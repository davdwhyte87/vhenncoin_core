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
use crate::blockchain::broadcast::{broadcast_request_http, get_servers, save_server_list};
use crate::blockchain::kv_store::KvStore;
use crate::blockchain::transfer::Transfer;
use crate::blockchain::wallet::Wallet;
use crate::models::block::{Block, Chain};
use crate::models::request::{AddNodeReq, CreateWalletReq, GetBalanceReq, TransferReq};
use crate::models::response::{GenericResponse, GetBalanceResponse};
use crate::models::server_list::ServerData;
use crate::utils::response::{Response, TCPResponse};
use crate::utils::struct_h::Struct_H;

pub struct Handler{

}
// handle sexternal communication from other sources to the blockchain module for any operations
impl Handler {

    pub fn get_balalnce(data:String,stream:&mut Option<TcpStream>)->String{
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
                return Response::string_response(&response)
            }
        };

        let res = block_on(async {
            Wallet::get_balance_http(request.address).await
        });

        let balance = match res {
            Ok(balance)=>{balance},
            Err(err)=>{
                error!("{}",err.to_string());
                let response = GenericResponse{
                    message : "Error getting balance ".to_string(),
                    code : 0
                };
                return Response::string_response(&response)
            }
        };

        let response = GetBalanceResponse{
            message : "successfully m".to_string(),
            code : 1,
            balance: balance
        };

        return Response::string_response(&response);

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
                return Response::string_response(&response)
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

        if mongodb_on == "1"{
            let res =block_on(async{
                Transfer::transfer_http(request.sender.to_owned(), request.receiver.to_owned(), f32::from_str(request.amount.as_str()).unwrap(),
            request.transaction_id
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
                    return Response::string_response(&response)
                }
            }
        }else{
            match Transfer::transfer(request.sender, request.receiver,f32::from_str(request.amount.as_str()).unwrap()){
                Ok(_)=>{},
                Err(err)=>{
                    error!("{}", err.to_string());

                    let response = GenericResponse{
                        message : "Error making transfer".to_string(),
                        code : 0
                    };
                    if tcp_stream {
                        TCPResponse::send_response(&response, stream.as_mut().unwrap().borrow_mut());
                    }


                    return Response::string_response(&response);
                }
            }
        }



        // send response
        let response = GenericResponse{
            message : "successfully made transfer".to_string(),
            code : 1
        };
        if tcp_stream {
            TCPResponse::send_response(&response, stream.as_mut().unwrap().borrow_mut());
        }

        return Response::string_response(&response);

    }

    pub fn http_ceate_wallet(message:&String){

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
                return Response::string_response(&response)
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
                    Wallet::create_wallet_http(request.address.to_owned(), "".to_string()).await
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
                    return Response::string_response(&response)
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

       
        return Response::string_response(&response);
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
                return format!("0{}{}",r"\n","Error fetching server list");
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
                return format!("0{}{}",r"\n","Error fetching server list");
            }
        }

        // let response = GenericResponse{
        //     message : content,
        //     code : 1
        // };
        
        
        return format!("1 {}{}",r"\n",content);

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
                return format!("0{}{}",r"\n","Error persing message");
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
                return format!("0{}{}",r"\n","Error fetching server list");
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
                return  format!("0{}{}",r"\n","Error converting to string");
            }
        };
        // save to disk
        match save_server_list(data_string){
            Ok(_)=>{},
            Err(err)=>{
                error!("saving list to disk {}", err);
                return format!("0{}{}",r"\n","Error saving data");
            }
        };
        return format!("1 {}{}",r"\n","Node added successfully");
    }


    // execute request to create wallet from broadcast 
    // these requests are usually user requests
    pub fn receive_create_wallet_http_broadcast_request(message:String)->String{
        // perse request data
        let mut request: CreateWalletReq = match  serde_json::from_str(message.as_str()) {
            Ok(data)=>{data},
            Err(err)=>{
                error!("persing message {}",err.to_string());
                return format!("0{}{}",r"\n","Error persing message");
                // AddNodeReq{ id: todo!(), ip_address: todo!(), public_key: todo!(), http_address: todo!() }
                // return Response::string_response(&response)
            }
        }; 


        return format!("1 {}{}",r"\n","OK");

    }

}

