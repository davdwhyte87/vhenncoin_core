use std::borrow::BorrowMut;
use std::env;
use std::io::Write;
use std::net::TcpStream;
use std::str::FromStr;
use futures::executor::block_on;
use futures_util::future::err;
use log::{debug, error};
use tokio::runtime::Runtime;
use crate::blockchain::kv_store::KvStore;
use crate::blockchain::transfer::Transfer;
use crate::blockchain::wallet::Wallet;
use crate::models::block::{Block, Chain};
use crate::models::request::{CreateWalletReq, GetBalanceReq, TransferReq};
use crate::models::response::{GenericResponse, GetBalanceResponse};
use crate::utils::response::{Response, TCPResponse};

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
    pub fn transfer(data:String,stream:&mut Option<TcpStream>)->String{
        let tcp_stream = match stream{
            Some(stream)=>{

                true},
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

        let mongodb_on = match env::var("MONGODB_ON"){
            Ok(data)=>{data},
            Err(err)=>{
                error!("{}",err);
                "8000".to_string()
            }
        };

        if mongodb_on == "1"{
            let res =block_on(async{
                Transfer::transfer_http(request.sender.to_owned(), request.receiver.to_owned(), f32::from_str(request.amount.as_str()).unwrap()).await
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

    pub fn create_wallet(message:&String, stream: &mut Option<TcpStream>)->String{
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
}

