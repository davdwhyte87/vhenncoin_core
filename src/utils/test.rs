use std::vec;

use log::debug;



use crate::{blockchain::concensus, models::balance_pack::BalancePack};

use super::{response::Response, utils};
use miniserde::{json, Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct MongoWalletX {
    pub id: String,
    pub address: String,
    pub wallet_name: String,
    pub password_hash:String,
  
 
}


impl MongoWalletX {
        
    pub fn default()->MongoWalletX{
        return MongoWalletX{
            id: "".to_string(),
            address:  "".to_string(),
            wallet_name:  "".to_string(),
            password_hash:  "".to_string(),
       
         
        };
    }
}

pub fn test_dd(){
    
    // conver to string
    let mongo_wallet = MongoWalletX::default();
    let wallet_string = json::to_string(&mongo_wallet);
    
    //println!("wallet string  .... {}", wallet_string);
    let res = response_formatter("1".to_string(), "message".to_string(), wallet_string.clone());

    println!("response .... {}", res);
    let data_set :Vec<&str>= res.split(r"\n").collect();

    let response_code = match data_set.get(0){
        Some(data)=>{data},
        None=>{""}
    };
    let response_message = match data_set.get(2){
        Some(data)=>{data},
        None=>{""}
    };

    // 

    let clean_message = &response_message.replace(r"\","");

    //println!("clean message .... {}", clean_message);
    let wallet:MongoWalletX = match json::from_str(&response_message){
        Ok(data)=>{data},
        Err(err)=>{
            println!("error {:?}", err);
            return }
    };

    println!("wallet {:#?}", wallet.address);

}

pub fn response_formatter(code:String, message:String, data:String)->String{
    return format!("{}{}{}{}{}{}",code,r"\n",message,r"\n",data,r"\n");
}

pub fn cons(){
    let balance_pack_list = vec![
        BalancePack{
            server_http_address :"s1".to_string(),
            balance: 10.0
        },
        BalancePack{
            server_http_address :"s2".to_string(),
            balance: 10.0
        },
        BalancePack{
            server_http_address :"s3".to_string(),
            balance: 10.0
        },  BalancePack{
            server_http_address :"s4".to_string(),
            balance: 10.0
        },
        BalancePack{
            server_http_address :"s5".to_string(),
            balance: 10.0
        },
        BalancePack{
            server_http_address :"s6".to_string(),
            balance: 9.0
        },
        BalancePack{
            server_http_address :"s7".to_string(),
            balance: 8.0
        },
        BalancePack{
            server_http_address :"s8".to_string(),
            balance: 10.0
        },
        BalancePack{
            server_http_address :"s9".to_string(),
            balance: 10.0
        },
        BalancePack{
            server_http_address :"s10".to_string(),
            balance: 10.0
        }
    ];
    let m = concensus::Concensus::vote_balance(balance_pack_list);
}