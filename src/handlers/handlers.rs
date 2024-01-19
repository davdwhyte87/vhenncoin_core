use std::net::TcpStream;
use crate::blockchain::kv_store::KvStore;
use crate::blockchain::wallet::Wallet;
use crate::models::block::{Block, Chain};
use crate::models::request::{CreateWalletReq, TransferReq};

pub struct Handler{

}
// handle sexternal communication from other sources to the blockchain module for any operations
impl Handler {
    pub fn transfer(data:String, stream: &TcpStream){
        // descode message
        let mut request: TransferReq = match  serde_json::from_str(data.as_str()) {
            Ok(data)=>{data},
            Err(err)=>{
                println!("{}",err.to_string());
                return
            }
        };

        // get sender wallet
        // let kv_store = KvStore::init("chain".to_string(), request.sender);
        // let chain = kv_store.get()
    }

    pub fn create_wallet(message:&String, stream: &TcpStream){
        // descode message
        let mut request: CreateWalletReq = match  serde_json::from_str(message.as_str()) {
            Ok(data)=>{data},
            Err(err)=>{
                println!("{}",err.to_string());
                return
            }
        };
        println!("Done decoding message");

        match Wallet::create_wallet(request.address,"".to_string()){
            Ok(_)=>{},
            Err(err)=>{
                println!("{}", err.to_string())
            }
        }

        return;
    }
}