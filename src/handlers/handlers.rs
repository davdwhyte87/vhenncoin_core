use std::io::Write;
use std::net::TcpStream;
use std::str::FromStr;
use futures_util::future::err;
use log::{debug, error};
use crate::blockchain::kv_store::KvStore;
use crate::blockchain::transfer::Transfer;
use crate::blockchain::wallet::Wallet;
use crate::models::block::{Block, Chain};
use crate::models::request::{CreateWalletReq, TransferReq};
use crate::models::response::GenericResponse;
use crate::utils::response::TCPResponse;

pub struct Handler{

}
// handle sexternal communication from other sources to the blockchain module for any operations
impl Handler {
    pub fn transfer(data:String, stream:&mut TcpStream){
        // descode message
        let mut request: TransferReq = match  serde_json::from_str(data.as_str()) {
            Ok(data)=>{data},
            Err(err)=>{
                error!("{}",err.to_string());
                return
            }
        };

        match Transfer::transfer(request.sender, request.receiver,f32::from_str(request.amount.as_str()).unwrap()){
            Ok(_)=>{},
            Err(err)=>{
                error!("{}", err.to_string());

                let response = GenericResponse{
                    message : "Error making transfer".to_string(),
                    code : 0
                };
                TCPResponse::send_response(&response, stream);

                return;
            }
        }

        // send response
        let response = GenericResponse{
            message : "successfully created wallet".to_string(),
            code : 1
        };
        TCPResponse::send_response(&response, stream);
        return;

    }

    pub fn create_wallet(message:&String, stream: &mut TcpStream){
        // descode message
        let mut request: CreateWalletReq = match  serde_json::from_str(message.as_str()) {
            Ok(data)=>{data},
            Err(err)=>{
                error!("{}",err.to_string());
                return
            }
        };
        debug!("Done decoding message");

        match Wallet::create_wallet(request.address,"".to_string()){
            Ok(_)=>{},
            Err(err)=>{
                error!("{}", err.to_string());
                let response = GenericResponse{
                    message : "Error creating wallet".to_string(),
                    code : 0
                };
                TCPResponse::send_response(&response, stream);
                return
            }
        }

        // send response
        let response = GenericResponse{
            message : "successfully created wallet".to_string(),
            code : 1
        };
        TCPResponse::send_response(&response, stream);
        return;
    }
}