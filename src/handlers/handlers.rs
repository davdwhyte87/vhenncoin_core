use std::borrow::BorrowMut;
use std::env::{self, current_dir};
use std::fs::{self, File};
use std::io::{Read, Write};

use std::num::ParseIntError;
use std::str::FromStr;
use std::sync::{Arc};
use bigdecimal::BigDecimal;
use futures::executor::block_on;
use futures_util::future::err;
use itertools::Format;
use log::{debug, error, info};
use log4rs::append::file;
use redb::Database;
use sha256::digest;
use sled::Db;
use tokio::net::TcpStream;
use tokio::runtime::Runtime;
use tokio::sync::Mutex;
use uuid::Uuid;
use crate::blockchain::broadcast::{broadcast_request_http, broadcast_request_tcp, get_node_balance, get_remote_node_balance_c, get_remote_wallet, get_servers, save_server_list};
use crate::blockchain::concensus::Concensus;

use crate::blockchain::kv_store::KvStore;
use crate::blockchain::mongo_store::WalletService;
use crate::blockchain::node::Node;
use crate::blockchain::transfer::{Transfer, TransferError};
use crate::blockchain::wallet::{self, Wallet};
use crate::{models, APP_CONFIG};
use crate::models::balance_pack::{BalanceCPack, BalancePack, WalletCPack};
use crate::models::block::{Block, Chain, VBlock};
use crate::models::db::MongoService;
use crate::models::request::{AddNodeReq, CreateUserIDReq, CreateWalletReq, GetAccountReq, GetBalanceReq, GetUserTransactionsReq, GetWalletReq, TransferReq, ValidateUserIDReq, VerifyWalletReq};
use crate::models::response::{GenericResponse, GetBalanceResponse, NResponse, WalletNamesResp, WalletNamesRespC};
use crate::models::server_list::ServerData;
use crate::models::user_id::UserID;
use crate::models::wallet::{MongoWallet, WalletC};
use crate::utils::constants;
use crate::utils::env::get_env;
use crate::utils::formatter::Formatter;
use crate::utils::response::{Response, TCPResponse};
use crate::utils::struct_h::Struct_H;
use crate::utils::test::response_formatter;
use crate::utils::time::get_date_time;
use crate::utils::utils::{validate_user_name, MyError, MyErrorTypes};
use models::balance_pack;
use crate::blockchain::chain::ChainX;
use crate::blockchain::kv_service2::KVService2;
use crate::blockchain::kv_service::KVService;
use crate::models::account::Account;
use crate::models::constants::BLOCKS_TABLE;
use crate::models::mempool::Mempool;
use crate::models::transaction::Transaction;

pub struct Handler{

}
// handle sexternal communication from other sources to the blockchain module for any operations
impl Handler {

      // transfer ... 
    pub async fn transfer_c(data:&String,stream:&mut TcpStream, is_broadcasted:String, mempool: Arc<Mutex<Mempool>>, db: &Db){
        // descode message
        let request: TransferReq = match  serde_json::from_str(data.as_str()) {
            Ok(data)=>{data},
            Err(err)=>{
                error!("{}",err.to_string());
                TCPResponse::send_response_x::<String>(NResponse{
                    status:0,
                    message: "error decoding request".to_string(),
                    data:None
                }, stream).await;
                return;
            }
        };
          if request.sender == request.receiver{
              TCPResponse::send_response_x::<String>(NResponse{
                  status:0,
                  message: "cannot send to self".to_string(),
                  data:None
              }, stream).await;
              return;  
          }
            
          
        let amount = match BigDecimal::from_str(&*request.amount.clone()){
          Ok(amount)=>{amount},
          Err(err)=>{
              debug!("{}",err.to_string());
              TCPResponse::send_response_x::<String>(NResponse{
                  status:0,
                  message: "invalid amount".to_string(),
                  data:None
              }, stream).await;
              return;
          }
        };
          
        let nonce:u64 = match u64::from_str(&*request.nonce.clone()){
          Ok(nonce)=>{nonce},
          Err(err)=>{
              debug!("{}",err.to_string());
              TCPResponse::send_response_x::<String>(NResponse{
                  status:0,
                  message: "invalid nonce".to_string(),
                  data:None
              }, stream).await;
              return;
          }
        };
          
          let tx = Transaction{
              sender: request.sender.clone(),
              receiver: request.receiver.clone(),
              amount: amount.clone(),
              nonce: nonce.clone(),
              signature: request.signature.clone(),
          };
          
          match Wallet::verify_transaction_signature(db, request).await{
              Ok(_) => {
                  
              },
              Err(err)=>{
                  error!("{}", err.to_string());
                  TCPResponse::send_response_x::<String>(NResponse{
                      status:0,
                      message: "error verifying transaction signature".to_string(),
                      data:None
                  }, stream).await;
                  return;
              }
          }; 
          // prevent sending to same sender wallet
          
          
       
        match Transfer::add_to_mempool(db,mempool, tx).await{
            Ok(_)=>{},
            Err(err)=>{
                error!("{}", err.to_string());
                TCPResponse::send_response_x::<String>(NResponse{
                    status:0,
                    message: err.to_string(),
                    data:None
                }, stream).await;
                return;
            }
        };

          TCPResponse::send_response_x::<String>(NResponse{
              status:1,
              message: "successfully submitted transaction".to_string(),
              data:None
          }, stream).await;
          return;
    }

    
    // get all blocks
    pub async fn get_all_blocks(db:&Db, stream: &mut TcpStream){
        let chain =match  ChainX::get_all_blocks(db).await{
            Ok(data)=>{data},
            Err(err)=>{
                log::error!("error getting blocks  ..{}", err);
                TCPResponse::send_response_x::<String>(NResponse{
                    status:0,
                    message: "error getting blocks".to_string(),
                    data:None
                }, stream).await;
                return; 
            }
        };

        TCPResponse::send_response_x::<Vec<VBlock>>(NResponse{
            status:1,
            message: "ok".to_string(),
            data:Some(chain)
        }, stream).await;
    }
    
    pub async fn hello(stream:&mut TcpStream){
        TCPResponse::send_response_x::<String>(NResponse{
            status:1,
            message: format!("This is a vhenncoin server, version:{}", APP_CONFIG.version),
            data:None
        }, stream).await;
        return;
    }

    pub async fn verify_wallet(data:&String ,db:&Db, stream: &mut TcpStream) {
        let request: VerifyWalletReq = match serde_json::from_str(data.as_str()) {
            Ok(data) => { data },
            Err(err) => {
                error!("{}",err.to_string());
                TCPResponse::send_response_x::<String>(NResponse {
                    status: 0,
                    message: "error decoding request".to_string(),
                    data: None
                }, stream).await;
                return;
            }
        };
        
        // check if wallet exists 
        let account = match Wallet::get_user_account(db,request.address.to_owned()).await{
            Ok(data)=>{data},
            Err(err)=>{
                error!("{}",err.to_string());
                TCPResponse::send_response_x::<String>(NResponse{
                    status:0,
                    message: "error getting wallet data".to_string(),
                    data:None
                }, stream).await;
                return;
            }
        };
        match account{
            Some(account)=>{},
            None=>{
                TCPResponse::send_response_x::<String>(NResponse{
                    status:0,
                    message: "Wallet does not exist".to_string(),
                    data:None
                }, stream).await;
                return;
            }
        }
        
        let is_ok =match Wallet::verify_signature(db, request.message, request.address, request.signature).await{
            Ok(d) => {
                d
            },
            Err(err)=>{
                error!("{}", err.to_string());
                TCPResponse::send_response_x::<String>(NResponse{
                    status:0,
                    message: "error verifying signature".to_string(),
                    data:None
                }, stream).await;
                return;
            }
        };
        
        if(is_ok){
            TCPResponse::send_response_x::<String>(NResponse{
                status:1,
                message: "Ok".to_string(),
                data:None
            }, stream).await;
            return;
        }else{
            TCPResponse::send_response_x::<String>(NResponse{
                status:0,
                message: "invalid signature".to_string(),
                data:None
            }, stream).await;
            return;
        }
    }
    pub async fn get_user_transactions(data:&String ,db:&Db, stream: &mut TcpStream){

        let request: GetUserTransactionsReq = match  serde_json::from_str(data.as_str()) {
            Ok(data)=>{data},
            Err(err)=>{
                error!("{}",err.to_string());
                TCPResponse::send_response_x::<String>(NResponse{
                    status:0,
                    message: "error decoding request".to_string(),
                    data:None
                }, stream).await;
                return;
            }
        };
        let data =match  Wallet::get_user_transactions_log(db, request.address.as_str()).await{
            Ok(data)=>{data},
            Err(err)=>{
                log::error!("error getting user transactions  ..{}", err);
                TCPResponse::send_response_x::<String>(NResponse{
                    status:0,
                    message: "error getting transactions".to_string(),
                    data:None
                }, stream).await;
                return;
            }
        };

        TCPResponse::send_response_x::<Vec<Transaction>>(NResponse{
            status:1,
            message: "ok".to_string(),
            data:Some(data)
        }, stream).await;
    }

    

    // transfer ...
  
  
    pub async fn create_wallet_node(wallet:&MongoWallet)->String{
        let database = match MongoService::get_db(){
            Some(database)=>{database.db.to_owned()},
            None=>{ 
                return Response::response_formatter(
                    "0".to_string(),
                     "Error creating wallet".to_string(), 
                    "".to_string()
                    )
                //Err(Box::from("No database connection"))
            }
        };

        let res = WalletService::create(&database, wallet).await;
        match res {
            Ok(_)=>{},
            Err(err)=>{
                error!("{}", err);
                return Response::response_formatter(
                    "0".to_string(),
                     "Error saving wallet".to_string(), 
                     err.to_string()
                    )
            }
        }

        return Response::response_formatter(
            "1".to_string(),
             "Wallet Created".to_string(), 
             "".to_string()
            )

    }


    pub async fn get_account(db: &Db, message:&String, stream: &mut TcpStream) {
      
        let request: GetAccountReq = match serde_json::from_str(message.as_str()) {
            Ok(data) => { data },
            Err(err) => {
                error!("{}",err.to_string());
                TCPResponse::send_response_x::<Account>(NResponse {
                    status: 0,
                    message: "error decoding request".to_string(),
                    data: None
                }, stream).await;
                return;
            }
        };
        
        let account =match Wallet::get_user_account(db, request.address).await{
            Ok(data)=>{
                match data{
                    Some(account)=>{account},
                    None=>{
                        TCPResponse::send_response_x::<Account>(NResponse {
                            status: 0,
                            message: "account not found".to_string(),
                            data: None
                        }, stream).await;
                        return;
                    }
                }
            },
            Err(err)=>{
                error!("{}",err.to_string());
                TCPResponse::send_response_x::<Account>(NResponse {
                    status: 0,
                    message: "error getting account".to_string(),
                    data: None
                }, stream).await;
                return;
            }
        };

        TCPResponse::send_response_x::<Account>(NResponse {
            status: 1,
            message: "ok".to_string(),
            data: Some(account)
        }, stream).await;
        return;
    }


    pub async fn get_last_block_height(db:&Db, stream: &mut TcpStream) {

        let data =match Wallet::get_last_block_height(db).await{
            Ok(data)=>{
               data
            },
            Err(err)=>{
                error!("{}",err.to_string());
                TCPResponse::send_response_x::<Account>(NResponse {
                    status: 0,
                    message: "error getting account".to_string(),
                    data: None
                }, stream).await;
                return;
            }
        };

        TCPResponse::send_response_x::<u64>(NResponse {
            status: 1,
            message: "ok".to_string(),
            data: Some(data)
        }, stream).await;
        return;
    }


    pub async fn get_last_block(db:&Db, stream: &mut TcpStream) {

        let data =match Wallet::get_last_block_height(db).await{
            Ok(data)=>{
                data
            },
            Err(err)=>{
                error!("{}",err.to_string());
                TCPResponse::send_response_x::<Account>(NResponse {
                    status: 0,
                    message: "error getting account".to_string(),
                    data: None
                }, stream).await;
                return;
            }
        };

        let last_block:Option<VBlock> =  match  KVService2::get_data::<VBlock>(db, BLOCKS_TABLE, &data.to_string() ).await{
            Ok(data)=>{data},
            Err(err)=>{
                error!("{}",err.to_string());
                TCPResponse::send_response_x::<VBlock>(NResponse {
                    status: 0,
                    message: "error getting block".to_string(),
                    data: None
                }, stream).await;
                return;
            }
        };
        
        let last_block = match last_block{
            Some(block)=>{block},
            None=>{
                TCPResponse::send_response_x::<VBlock>(NResponse {
                    status: 0,
                    message: "No block found".to_string(),
                    data: None
                }, stream).await;
                return;
            }
        };

        TCPResponse::send_response_x::<VBlock>(NResponse {
            status: 1,
            message: "ok".to_string(),
            data: Some(last_block)
        }, stream).await;
        return;
    }

    pub async fn get_mempool(mempool: Arc<Mutex<Mempool>>, stream: &mut TcpStream) {
        log::debug!("starting creation");
        
        let trnasactions =  Transfer::get_all_transactions(mempool).await;
        TCPResponse::send_response_x::<Vec<Transaction>>(NResponse {
            status: 1,
            message: "ok".to_string(),
            data: Some(trnasactions)
        }, stream).await;
        return;
    }


    pub async fn create_wallet_tcp(db: &Db, message:&String, stream: &mut TcpStream, is_broadcasted:String){
        log::debug!("starting creation");
        let request: CreateWalletReq = match  serde_json::from_str(message.as_str()) {
            Ok(data)=>{data},
            Err(err)=>{
                error!("{}",err.to_string());
                TCPResponse::send_response_x::<String>(NResponse {
                    status: 0,
                    message: "error decoding request".to_string(),
                    data: None
                }, stream).await;
                return;
            }
        };
        log::debug!("checking white space");
        // check if there is white space in the wallet address
        if request.address.contains(char::is_whitespace){
            log::debug!("{}", "address contains white space");
            TCPResponse::send_response_x::<String>(NResponse{
                status:0,
                message: "invalid address, has white space".to_string(),
                data:None
            }, stream).await;
            return; 
        }

        // make sure address is lower case
        //let tr_address = request.address.trim().to_lowercase();
        debug!("Done decoding message");
        
        // check if wallet exists 
        let account = match Wallet::get_user_account(db,request.address.clone()).await{
            Ok(account)=>{account},
            Err(err)=>{
                error!("{}", err.to_string());
                TCPResponse::send_response_x::<String>(NResponse{
                    status:0,
                    message: "error getting account".to_string(),
                    data:None
                }, stream).await;
                return;
            }
        };
        if account.is_some(){
            TCPResponse::send_response_x::<String>(NResponse{
                status:0,
                message: "This wallet already exists".to_string(),
                data:None
            }, stream).await;
            return;
        }
        let resp = match Wallet::create_wallet_r(db, request).await{
            Ok(_)=>{
                TCPResponse::send_response_x::<String>(NResponse{
                    status:1,
                    message: "wallet created!".to_string(),
                    data:None
                }, stream).await;
                return;

                    // broadcast and tell other servers about the newly created wallet
                    // if is_broadcasted == "0" {
                    //     debug!("broadcasting ...... ");
                    //     broadcast_request_tcp("CreateWallet".to_string(),message.to_string());
                    // }
                    return;
            },
            Err(err)=>{
                error!("{}", err.to_string());
                TCPResponse::send_response_x::<String>(NResponse{
                    status:0,
                    message: "error creating wallet".to_string(),
                    data:None
                }, stream).await;
                return;
            }
        };
    }
   

    // get list of servers locally and send to remote server requesting for it
    // pub fn get_servers_c(stream: &mut TcpStream){
    // 
    //     let servers = get_servers();
    //     let servers = match servers{
    //         Ok(data)=>{data},
    //         Err(err)=>{
    //             error!("{}", err.to_string());
    //             let response = Formatter::response_formatter(
    //                 "0".to_string(),
    //                 "Error sending issues".to_string(),
    //                 "".to_string()
    //             );
    //             TCPResponse::send_response_txt(response, stream);
    //             return;
    //         }
    //     };
    // 
    //     // convert to string
    //     let content = match serde_json::to_string(&servers){
    //         Ok(data)=>{data},
    //         Err(err)=>{
    //             error!("{}", err.to_string());
    //             let err_response = Formatter::response_formatter(
    //                 "0".to_string(),
    //                 "Error converting to string".to_string(),
    //                 "".to_string()
    //             );
    //             TCPResponse::send_response_txt(err_response, stream);
    //             return;
    //         }
    //     };
    // 
    //     debug!("content data to be sent {}", content.clone());      
    //     let response = Formatter::response_formatter(
    //         "1".to_string(),
    //          "0".to_string(), 
    //          content
    //         );
    //        
    //     TCPResponse::send_response_txt(response, stream);
    //     return;
    // }

    // pub fn get_servers()->String{
    //     let data_path: String = format!("{}{}",current_dir().unwrap_or_default().to_str().unwrap_or_default(), "/server_list.json");
    //     debug!("serverlist file path {}",data_path);
    //     let mut file =match  File::open(data_path.clone()){
    //         Ok(file)=>{file},
    //         Err(err)=>{
    //             error!("error opening file {}",err.to_string());
    //             let response = GenericResponse{
    //                 message : "Error fetching server list".to_string(),
    //                 code : 0
    //             };
    //             return Response::response_formatter(
    //                 "0".to_string(),
    //                  "Error fetching server list ".to_string(), 
    //                  err.to_string()
    //                 )
    //         } 
    //     };
    //     let mut content = String::new();
    // 
    //     match file.read_to_string(&mut content){
    //         Ok(_)=>{},
    //         Err(err)=>{
    //             error!(" error reading file {}",err.to_string());
    //             let response = GenericResponse{
    //                 message : "Error fetching server list".to_string(),
    //                 code : 0
    //             };
    //             return Response::response_formatter(
    //                 "0".to_string(),
    //                  "Error  fetching server list".to_string(), 
    //                  err.to_string()
    //                 )
    //         }
    //     }
    // 
    //     // let response = GenericResponse{
    //     //     message : content,
    //     //     code : 1
    //     // };
    //     
    //     
    //     return Response::response_formatter(
    //         "1".to_string(),
    //          "Error persing data".to_string(), 
    //          content
    //         )
    // 
    //     // let servers = get_servers();
    //     // let servers = match servers{
    //     //     Ok(servers)=>{servers},
    //     //     Err(err)=>{
    //     //         error!("{:?}",err);
    //     //         let response = GenericResponse{
    //     //             message : "Error fetching server list".to_string(),
    //     //             code : 0
    //     //         };
    //     //         return Response::string_response(&response)
    //     //     }
    //     // };
    // 
    //     // let res = Struct_H::vec_to_string(servers);
    //     // return res
    // 
    // }

    // add new node to list of remote server 
    // pub fn add_node_c(message:&String, stream: &mut TcpStream){
    //     let mut request: AddNodeReq = match  serde_json::from_str(message.as_str()) {
    //         Ok(data)=>{data},
    //         Err(err)=>{
    //             error!("persing message {}",err.to_string());
    //             // return format!("0{}{}",r"\n","Error persing message");
    //             let response = Formatter::response_formatter(
    //                 "0".to_string(),
    //                  "Error persing message".to_string(), 
    //                  err.to_string()
    //                 );
    //             TCPResponse::send_response_txt(response, stream);
    //             return;
    //             // AddNodeReq{ id: todo!(), ip_address: todo!(), public_key: todo!(), http_address: todo!() }
    //             // return Response::string_response(&response)
    //         }
    //     }; 
    // 
    //     let new_server_data = ServerData{
    //         ip_address :request.ip_address,
    //         id: request.id,
    //         http_address:request.http_address,
    //         public_key: request.public_key
    //     };
    // 
    //     // get local node server list
    //     let mut  node_list = match get_servers() {
    //         Ok(data)=>{data},
    //         Err(err)=>{
    //         
    //             error!("fetching server list {}", err);
    //             // return format!("0{}{}",r"\n","Error fetching server list");
    //             let response: String = Formatter::response_formatter(
    //                 "0".to_string(),
    //                  "Error fetching server list".to_string(), 
    //                  err.to_string()
    //                 );
    // 
    //             TCPResponse::send_response_txt(response, stream);
    //             return;
    //         }
    //         
    //     };
    // 
    //     // add new node 
    //     // check if node exists
    //     let has_new_node = node_list.contains(&new_server_data);
    //     if !has_new_node {
    //         node_list.push(new_server_data);
    //     }
    //    
    // 
    //     // let data_string:String = serde_json::json!(node_list).to_string();
    //     let data_string = match serde_json::to_string(&node_list){
    //         Ok(new_node_string)=>{new_node_string},
    //         Err(err)=>{
    //             error!("Request error ... {}", err);
    //             // return  format!("0{}{}",r"\n","Error converting to string");
    //             let response = Formatter::response_formatter(
    //                 "0".to_string(),
    //                  "Error converting to string".to_string(), 
    //                  err.to_string()
    //                 );
    // 
    //             TCPResponse::send_response_txt(response, stream);
    //             return;
    //         }
    //     };
    //     // save to disk
    //     match Node::save_server_list(data_string){
    //         Ok(_)=>{},
    //         Err(err)=>{
    //             error!("saving list to disk {}", err);
    //             // return format!("0{}{}",r"\n","Error saving data");
    //             let response = Response::response_formatter(
    //                 "0".to_string(),
    //                  "Error saving data".to_string(), 
    //                  err.to_string()
    //                 );
    //             TCPResponse::send_response_txt(response, stream);
    //             return;
    //         }
    //     };
    //     // return format!("1 {}{}",r"\n","Node added successfully");
    //     let response = Formatter::response_formatter(
    //         "1".to_string(),
    //          "Node added successfully".to_string(), 
    //          "".to_string()
    //         );
    // 
    //     TCPResponse::send_response_txt(response, stream);
    //     return;
    // }

    // add new node to list of nodes in server list
    // later to be moved to Node struct
    pub fn add_node(message:String)->String{
        let mut request: AddNodeReq = match  serde_json::from_str(message.as_str()) {
            Ok(data)=>{data},
            Err(err)=>{
                error!("persing message {}",err.to_string());
                // return format!("0{}{}",r"\n","Error persing message");
                return Response::response_formatter(
                    "0".to_string(),
                     "Error persing message".to_string(), 
                     err.to_string()
                    )
                // AddNodeReq{ id: todo!(), ip_address: todo!(), public_key: todo!(), http_address: todo!() }
                // return Response::string_response(&response)
            }
        }; 

        let new_server_data = ServerData{
            ip_address :request.ip_address,
            id: request.id,
            http_address:request.http_address,
            public_key: request.public_key
        };

        // get local node server list
        let mut  node_list = match get_servers() {
            Ok(data)=>{data},
            Err(err)=>{
            
                error!("fetching server list {}", err);
                // return format!("0{}{}",r"\n","Error fetching server list");
                return Response::response_formatter(
                    "0".to_string(),
                     "Error fetching server list".to_string(), 
                     err.to_string()
                    )
            }
            
        };

        // add new node 
        // check if node exists
        let has_new_node = node_list.contains(&new_server_data);
        if !has_new_node {
            node_list.push(new_server_data);
        }
       

        // let data_string:String = serde_json::json!(node_list).to_string();
        let data_string = match serde_json::to_string(&node_list){
            Ok(new_node_string)=>{new_node_string},
            Err(err)=>{
                error!("Request error ... {}", err);
                // return  format!("0{}{}",r"\n","Error converting to string");
                return Response::response_formatter(
                    "0".to_string(),
                     "Error converting to string".to_string(), 
                     err.to_string()
                    )
            }
        };
        // save to disk
        match save_server_list(data_string){
            Ok(_)=>{},
            Err(err)=>{
                error!("saving list to disk {}", err);
                // return format!("0{}{}",r"\n","Error saving data");
                return Response::response_formatter(
                    "0".to_string(),
                     "Error saving data".to_string(), 
                     err.to_string()
                    )
            }
        };
        // return format!("1 {}{}",r"\n","Node added successfully");
        return Response::response_formatter(
            "1".to_string(),
             "Node added successfully".to_string(), 
             "".to_string()
            )
    }


    // execute request to create wallet from broadcast 
    // these requests are usually user requests
    pub fn receive_create_wallet_http_broadcast_request(message:String)->String{
        // perse request data
        let mut request: CreateWalletReq = match  serde_json::from_str(message.as_str()) {
            Ok(data)=>{data},
            Err(err)=>{
                error!("persing message {}",err.to_string());
                return Response::response_formatter(
                    "0".to_string(),
                     "Error persing data".to_string(), 
                     err.to_string()
                    )
                
            }
        }; 


        // return format!("1 {}{}",r"\n","OK");
        return Response::response_formatter(
            "1".to_string(),
             "Ok".to_string(), 
             "".to_string()
            )

    }

    // this gets all the wallet addresses for the local node 
    // pub fn get_node_wallet_list_c(stream: &mut TcpStream){
    //   
    //     let entries = match fs::read_dir("/data"){
    //         Ok(data)=>{data},
    //         Err(err)=>{
    //             error!("{}", err.to_string());
    //             let response: String = Formatter::response_formatter(
    //                 "0".to_string(),
    //                  "Wrror with read dir".to_string(), 
    //                  err.to_string()
    //                 );
    //             TCPResponse::send_response_txt(response, stream);
    //             return;
    //         }
    //     };
    // 
    //     let mut wallets:Vec<WalletC> =vec![];
    //     // Extract the filenames from the directory entries and store them in a vector
    //     let file_names: Vec<String> = entries
    //         .filter_map(|entry| {
    //             let path = entry.ok()?.path();
    //             if path.is_file() {
    //                 let address = path.file_name()?.to_str().map(|s| s.to_owned()).unwrap();
    //                 let wallet = Wallet::get_wallet_c(&address).unwrap();
    //                 wallets.push(wallet);
    //                 path.file_name()?.to_str().map(|s| s.to_owned())
    //                
    //             } else {
    //                 None
    //             }
    //         })
    //         .collect();
    // 
    //     
    //     let wallet_names = WalletNamesRespC{
    //         names :file_names
    //     };
    // 
    //     let wallets_str = serde_json::to_string(&wallet_names);
    //     let wallets_string =match wallets_str {
    //         Ok(data)=>{data},
    //         Err(err)=>{ 
    //             let response: String = Formatter::response_formatter(
    //                 "0".to_string(),
    //                  "persing wallets to struct".to_string(), 
    //                  err.to_string()
    //                 );
    //             TCPResponse::send_response_txt(response, stream);
    //             return;
    //             }
    //     };
    //    
    //     let response: String = Formatter::response_formatter(
    //         "1".to_string(),
    //          "Ok".to_string(), 
    //          wallets_string
    //         ) ;
    // 
    //     TCPResponse::send_response_txt(response, stream);
    //     return;
    // }


    // this gets all the wallet addresses 
    // pub async fn get_node_wallet_list()->String{
    //     let wallets = Wallet::get_all_wallets().await;
    //     let wallets = match wallets {
    //         Ok(wallets)=>{wallets},
    //         Err(err)=>{
    //             error!("error getting walets from db ...{}", err);
    //             vec![]
    //         }
    //     };
    // 
    //     let wallet_names = WalletNamesResp{
    //         names :wallets
    //     };
    // 
    //     let wallets_str = serde_json::to_string(&wallet_names);
    //     let wallets_string =match wallets_str {
    //         Ok(data)=>{data},
    //         Err(err)=>{
    //             return Response::response_formatter(
    //                 "0".to_string(),
    //                  "Error persing result data".to_string(), 
    //                  err.to_string()
    //                 )
    //             }
    //     };
    //     return Response::response_formatter(
    //         "1".to_string(),
    //          "Ok".to_string(), 
    //          wallets_string
    //         )
    // }


    // pub fn get_single_wallet_c(message:String,  stream: &mut TcpStream){
    // 
    //     let mut request: GetWalletReq = match  serde_json::from_str(&message.as_str()) {
    //         Ok(data)=>{data},
    //         Err(err)=>{
    //             error!("{}",err.to_string());
    //           
    //             let response = Formatter::response_formatter(
    //                 "0".to_string(),
    //                  "Error persing data".to_string(), 
    //                  err.to_string()
    //                 );
    //             TCPResponse::send_response_txt(response, stream);
    //             return;
    //         }
    //     };
    //     let wallet = Wallet::get_wallet_c(&request.address);
    //     let wallet = match wallet {
    //         Ok(wallet)=>{wallet},
    //         Err(err)=>{
    //             error!("error ... {}", err);
    //             let response: String = Formatter::response_formatter(
    //                 "0".to_string(),
    //                  "Error getting single wallet from db".to_string(), 
    //                  err.to_string()
    //                 );
    // 
    //             TCPResponse::send_response_txt(response, stream);
    //             return;
    //         }
    //     } ;
    // 
    //     let wallets_str = serde_json::to_string(&wallet);
    //     let wallets_string =match wallets_str {
    //         Ok(data)=>{data},
    //         Err(err)=>{ 
    //             error!("error ... {}", err);
    //             let response: String = Formatter::response_formatter(
    //                 "0".to_string(),
    //                  "Error persing wallet to string".to_string(), 
    //                  err.to_string()
    //                 );
    // 
    //             TCPResponse::send_response_txt(response, stream);
    //             return;
    //             }
    //     };
    //     //  return format!("1 {}{}{}{:?}",r"\n","Ok","r\n",wallets_string);
    //     let response: String = Formatter::response_formatter(
    //         "1".to_string(),
    //          "Ok".to_string(), 
    //          wallets_string
    //         );
    // 
    //     TCPResponse::send_response_txt(response, stream);
    //     return;
    // 
    // }
    // pub async fn get_single_wallet(address:String)->String{
    //     let wallets = Wallet::get_single_wallet(address).await;
    //     let wallets = match wallets {
    //         Ok(wallets)=>{wallets},
    //         Err(err)=>{
    //             error!("error ... {}", err);
    //             return Response::response_formatter(
    //                 "0".to_string(),
    //                  "Error getting single wallet from db".to_string(), 
    //                  err.to_string()
    //                 )
    //         }
    //     } ;
    // 
    //     let wallets_str = serde_json::to_string(&wallets);
    //     let wallets_string =match wallets_str {
    //         Ok(data)=>{data},
    //         Err(err)=>{ 
    //             return Response::response_formatter(
    //                 "0".to_string(),
    //                  "Error persing wallet struct to string".to_string(), 
    //                  err.to_string()
    //                 )
    //             }
    //     };
    //     //  return format!("1 {}{}{}{:?}",r"\n","Ok","r\n",wallets_string);
    //      return Response::response_formatter(
    //         "1".to_string(),
    //          "Ok".to_string(), 
    //          wallets_string
    //         )
    // 
    // }


    // zips chains and sends the data to the client computer 
    // pub fn get_chain_zip(stream:&mut TcpStream){
    //     Node::zipchain();
    //     
    //     let mut buf = [0; 4096];
    //     stream.set_write_timeout(None).unwrap();
    //     let mut file = File::open("data.zip").unwrap();
    //     loop {
    //         let n = file.read(&mut buf).unwrap();
    //         
    //         if n == 0 {
    //             // reached end of file
    //             break;
    //         }
    //         
    //         let _ = stream.write_all(&buf[..n]);
    //     } 
    // }
}



