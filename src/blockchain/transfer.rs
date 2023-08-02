use std::borrow::Borrow;
use std::error::Error;
use uuid::Uuid;

use crate::blockchain::wallet::Wallet;
use crate::models::block::Block;
use crate::utils::time::get_date_time;

pub struct Transfer {

}

impl Transfer {
    // transfer value from one wallet to another
    pub fn transfer(sender:String, receiver:String, amount:f32)->Result<(), Box<dyn Error>>{
        // get sender public key from last block
        // check if both wallets exist
        let sender_exists = Wallet::wallet_exists(&sender);
        let receiver_exists = Wallet::wallet_exists(&receiver);
        if sender_exists!= true || receiver_exists !=true {
            return Err(Box::from("Wallet does not exist"))
        }
        //check if sender has the money available
        let sender_chain = match Wallet::get_wallet_chain(&sender){
            Ok(sender_chian)=>{sender_chian},
            Err(err)=>{return Err(err.into())}
        };
        let receiver_chain = match Wallet::get_wallet_chain(&receiver){
            Ok(receiver_chain)=>{receiver_chain},
            Err(err)=>{return Err(err.into())}
        };
        let sender_balance = match Wallet::get_balance(&sender){
            Ok(sender_balance)=>{sender_balance},
            Err(err)=> {return Err(err.into())}
        };

        if sender_balance < amount{
            return Err(Box::from("Insufficient funds"))
        }
        // create minus block
        let sender_h = serde_json::to_string(&sender_chain.borrow().chain.last().unwrap());
        let sender_h =match sender_h {
            Ok(json)=>{json},
            Err(err)=>{
                return Err(err.into())
            }
        };
        let receiver_h = serde_json::to_string(&receiver_chain.borrow().chain.last().unwrap());
        let receiver_h =match receiver_h {
            Ok(json)=>{json},
            Err(err)=>{
                return Err(err.into())
            }
        };
        let sender_block = Block{
            id: Uuid::new_v4().to_string(),
            sender_address: sender.to_owned(),
            receiver_address: receiver.to_owned(),
            date_created: get_date_time(),
            hash:sender_h,
            amount: -amount.clone(),
            public_key: sender_chain.chain.last().unwrap().public_key.clone()
        };
        // create add block for receiver
        let receiver_block = Block{
            id: Uuid::new_v4().to_string(),
            sender_address: sender.to_owned(),
            receiver_address: receiver.to_owned(),
            date_created: get_date_time(),
            hash:receiver_h,
            amount: amount.clone(),
            public_key: sender_chain.chain.last().unwrap().public_key.clone()
        };
        // if two blocks are saved well, send response
        match Wallet::save_block(&sender, sender_block){
            Ok(_)=>{},
            Err(err)=>{return Err(err.into())}
        }

        match Wallet::save_block(&receiver, receiver_block){
            Ok(_)=>{},
            Err(err)=>{return Err(err.into())}
        }


        return Ok(())
    }
}