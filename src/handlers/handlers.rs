use std::borrow::BorrowMut;
use std::env::{self, current_dir};
use std::fs::File;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::str::FromStr;
use futures::executor::block_on;
use futures_util::future::err;
use log::{debug, error};
use tokio::runtime::Runtime;
use crate::blockchain::broadcast::{broadcast_request_http, get_node_balance, get_servers, save_server_list};
use crate::blockchain::concensus::Concensus;
use crate::blockchain::kv_store::KvStore;
use crate::blockchain::mongo_store::WalletService;
use crate::blockchain::transfer::Transfer;
use crate::blockchain::wallet::Wallet;
use crate::models;
use crate::models::balance_pack::BalancePack;
use crate::models::block::{Block, Chain};
use crate::models::db::MongoService;
use crate::models::request::{AddNodeReq, CreateWalletReq, GetBalanceReq, TransferReq};
use crate::models::response::{GenericResponse, GetBalanceResponse, WalletNamesResp};
use crate::models::server_list::ServerData;
use crate::models::wallet::MongoWallet;
use crate::utils::response::{Response, TCPResponse};
use crate::utils::struct_h::Struct_H;
use models::balance_pack;

pub struct Handler{

}
// handle sexternal communication from other sources to the blockchain module for any operations
impl Handler {

    pub async fn get_node_balalnce(data:String)->String{
     
        // descode message
        let request: GetBalanceReq = match  serde_json::from_str(data.as_str()) {
            Ok(data)=>{data},
            Err(err)=>{
                error!("{}",err.to_string());
                return Response::response_formatter(
                    "0".to_string(),
                     "Error persing data".to_string(), 
                     err.to_string()
                    )
            }
        };

        let res = Wallet::get_balance_http(request.address).await;
     
        let balance = match res {
            Ok(balance)=>{balance},
            Err(err)=>{
                error!("{}",err.to_string());
                return Response::response_formatter(
                    "0".to_string(),
                     "Error getting balance".to_string(), 
                     err.to_string()
                    )
            }
        };

        return Response::response_formatter(
            "1".to_string(),
             "Ok".to_string(), 
             balance.to_string()
            )

    }

    pub async fn get_balalnce(data:String,stream:&mut Option<TcpStream>)->String{
        let tcp_stream = match stream{
            Some(stream)=>{

                true},
            None=>{false }
        };
        // descode message
        let mut request: GetBalanceReq = match  serde_json::from_str(data.as_str()) {
            Ok(data)=>{data},
            Err(err)=>{
                error!("{}",err.to_string());
                let response = GenericResponse{
                    message : "Error getting balance".to_string(),
                    code : 0
                };
                return Response::response_formatter(
                    "0".to_string(),
                     "Error persing data".to_string(), 
                     err.to_string()
                    )
            }
        };

        let res =Wallet::get_balance_http(request.address.clone()).await;
       

        let balance = match res {
            Ok(balance)=>{balance},
            Err(err)=>{
                error!("{}",err.to_string());
                let response = GenericResponse{
                    message : "Error getting balance ".to_string(),
                    code : 0
                };
                return Response::response_formatter(
                    "0".to_string(),
                     "Error getting balance".to_string(), 
                     err.to_string()
                    )
            }
        };

        // send query to other servers 
        let servers = match get_servers(){
            Ok(data)=>{data},
            Err(err)=>{
                return Response::response_formatter(
                    "0".to_string(),
                     "Error getting servers".to_string(), 
                     err.to_string()
                    )
            }
        };
        
        let mut balance_pack_list:Vec<BalancePack> = vec![];

         for server in servers{
            // get balance from other servers 
            let r_balance = get_node_balance(&server,
                 &request.address).await;
            let r_balance = match r_balance {
                Ok(data)=>{data},
                Err(err)=>{
                    error!("error ... {}", err);
                    0.0
                }
            };


            balance_pack_list.push(BalancePack{server_http_address:server.http_address.to_owned(), balance:r_balance})
            
         }

         // for this certain wallet address, these are the balances in their balance pack 
         // if there are 10 nodes in the network, then we will have 10 balances 
         // we now have to do some voting 

         let b_vote = Concensus::vote_balance(balance_pack_list);


        return Response::response_formatter(
            "1".to_string(),
             "Ok".to_string(), 
             b_vote.balance.to_string()
            )

    }

    // transfer ...
    pub fn transfer(data:String,stream:&mut Option<TcpStream>, is_broadcasted:String)->String{
        let tcp_stream = match stream{
            Some(stream)=>{
                true
            },
            None=>{false }
        };
        // descode message
        let mut request: TransferReq = match  serde_json::from_str(data.as_str()) {
            Ok(data)=>{data},
            Err(err)=>{
                error!("{}",err.to_string());
                let response = GenericResponse{
                    message : "Error transferring".to_string(),
                    code : 0
                };
                return Response::response_formatter(
                    "0".to_string(),
                     "Error persing data".to_string(), 
                     err.to_string()
                    )
            }
        };

        debug!("transaction ID {}", request.transaction_id);

        let mongodb_on = match env::var("MONGODB_ON"){
            Ok(data)=>{data},
            Err(err)=>{
                error!("{}",err);
                "8000".to_string()
            }
        };

        
        let res =block_on(async{
            Transfer::transfer_http(
                request.sender.to_owned(),
                request.receiver.to_owned(),
                f32::from_str(request.amount.as_str()).unwrap(),
                request.transaction_id,
                request.sender_password
        ).await
        }
        );
        match res {
            Ok(_)=>{},
            Err(err)=>{
                error!("transfer error {}", err.to_string());
                let response = GenericResponse{
                    message : err.to_string(),
                    code : 0
                };
                return Response::response_formatter(
                    "0".to_string(),
                     "Error transfering".to_string(), 
                     err.to_string()
                    )
            }
        
        }
        return Response::response_formatter(
            "1".to_string(),
             "Coins Transfered".to_string(), 
             "".to_string()
            )

    }

    pub fn http_ceate_wallet(message:&String){

    }

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

    pub fn create_wallet(message:&String, stream: &mut Option<TcpStream>, is_broadcasted:String)->String{

        // descode message
        let tcp_stream = match stream{
            Some(stream)=>{

                true},
            None=>{false }
        };
        let mut request: CreateWalletReq = match  serde_json::from_str(message.as_str()) {
            Ok(data)=>{data},
            Err(err)=>{
                error!("{}",err.to_string());
                let response = GenericResponse{
                    message : "Error creating wallet".to_string(),
                    code : 0
                };
                return Response::response_formatter(
                    "0".to_string(),
                     "Error persing data".to_string(), 
                     err.to_string()
                    )
            }
        };
        debug!("Done decoding message");

        let mongodb_on = match env::var("MONGODB_ON"){
            Ok(data)=>{data},
            Err(err)=>{
                error!("{}",err);
                "8000".to_string()
            }
        };
        // if mongodatabase is on, use the create wallet http method, else, use the KVstore
        if (mongodb_on == "1"){
            let res =block_on(async{
                    Wallet::create_wallet_http(request.address.to_owned(), "".to_string(), request.password ).await
                }
            );
            match res {
                Ok(_)=>{},
                Err(err)=>{
                    error!("{}", err.to_string());
                    let response = GenericResponse{
                        message : "Error creating wallet".to_string(),
                        code : 0
                    };
                    return Response::response_formatter(
                        "0".to_string(),
                         "Error creating wallet".to_string(), 
                         err.to_string()
                        )
                }
            }

        }else {
            match Wallet::create_wallet(request.address,"".to_string()){
                Ok(_)=>{},
                Err(err)=>{
                    error!("{}", err.to_string());
                    let response = GenericResponse{
                        message : "Error creating wallet".to_string(),
                        code : 0
                    };
                    if tcp_stream {
                        TCPResponse::send_response(&response,  stream.as_mut().unwrap().borrow_mut());
                    }

                    return Response::string_response(&response)
                }
            }
        }


        // send response
        let response = GenericResponse{
            message : "successfully created wallet".to_string(),
            code : 1
        };
        if tcp_stream {
            TCPResponse::send_response(&response, stream.as_mut().unwrap().borrow_mut());
        }

       
        return Response::response_formatter(
            "1".to_string(),
             "Wallet created".to_string(), 
             "".to_string()
            )
    }


    pub fn get_servers()->String{
        let data_path: String = format!("{}{}",current_dir().unwrap_or_default().to_str().unwrap_or_default(), "/server_list.json");
        debug!("serverlist file path {}",data_path);
        let mut file =match  File::open(data_path.clone()){
            Ok(file)=>{file},
            Err(err)=>{
                error!("error opening file {}",err.to_string());
                let response = GenericResponse{
                    message : "Error fetching server list".to_string(),
                    code : 0
                };
                return Response::response_formatter(
                    "0".to_string(),
                     "Error fetching server list ".to_string(), 
                     err.to_string()
                    )
            } 
        };
        let mut content = String::new();
    
        match file.read_to_string(&mut content){
            Ok(_)=>{},
            Err(err)=>{
                error!(" error reading file {}",err.to_string());
                let response = GenericResponse{
                    message : "Error fetching server list".to_string(),
                    code : 0
                };
                return Response::response_formatter(
                    "0".to_string(),
                     "Error  fetching server list".to_string(), 
                     err.to_string()
                    )
            }
        }

        // let response = GenericResponse{
        //     message : content,
        //     code : 1
        // };
        
        
        return Response::response_formatter(
            "1".to_string(),
             "Error persing data".to_string(), 
             content
            )

        // let servers = get_servers();
        // let servers = match servers{
        //     Ok(servers)=>{servers},
        //     Err(err)=>{
        //         error!("{:?}",err);
        //         let response = GenericResponse{
        //             message : "Error fetching server list".to_string(),
        //             code : 0
        //         };
        //         return Response::string_response(&response)
        //     }
        // };

        // let res = Struct_H::vec_to_string(servers);
        // return res

    }


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



    // this gets all the wallet addresses 
    pub async fn get_node_wallet_list()->String{
        let wallets = Wallet::get_all_wallets().await;
        let wallets = match wallets {
            Ok(wallets)=>{wallets},
            Err(err)=>{
                error!("error getting walets from db ...{}", err);
                vec![]
            }
        };

        let wallet_names = WalletNamesResp{
            names :wallets
        };

        let wallets_str = serde_json::to_string(&wallet_names);
        let wallets_string =match wallets_str {
            Ok(data)=>{data},
            Err(err)=>{
                return Response::response_formatter(
                    "0".to_string(),
                     "Error persing result data".to_string(), 
                     err.to_string()
                    )
                }
        };
        return Response::response_formatter(
            "1".to_string(),
             "Ok".to_string(), 
             wallets_string
            )
    }

    pub async fn get_single_wallet(address:String)->String{
        let wallets = Wallet::get_single_wallet(address).await;
        let wallets = match wallets {
            Ok(wallets)=>{wallets},
            Err(err)=>{
                error!("error ... {}", err);
                return Response::response_formatter(
                    "0".to_string(),
                     "Error getting single wallet from db".to_string(), 
                     err.to_string()
                    )
            }
        } ;

        let wallets_str = serde_json::to_string(&wallets);
        let wallets_string =match wallets_str {
            Ok(data)=>{data},
            Err(err)=>{ 
                return Response::response_formatter(
                    "0".to_string(),
                     "Error persing wallet struct to string".to_string(), 
                     err.to_string()
                    )
                }
        };
        //  return format!("1 {}{}{}{:?}",r"\n","Ok","r\n",wallets_string);
         return Response::response_formatter(
            "1".to_string(),
             "Ok".to_string(), 
             wallets_string
            )

    }

}

