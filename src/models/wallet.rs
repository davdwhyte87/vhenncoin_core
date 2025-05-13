use chrono::NaiveDateTime;
use serde::{Serialize, Deserialize};
use crate::models::block::{Chain, TBlock};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Wallet {
    pub id: String,
    pub address: String,
    pub public_key: String,
    pub chain: Vec<TBlock>,
    created_at: NaiveDateTime
}

#[derive(Debug, Serialize, Deserialize,Clone, PartialEq, Eq,)]
pub enum LimitPeriod{
    Daily,
    Weekly,
    Monthly,
    Yearly
}



#[derive(Debug, Serialize, Deserialize)]
pub struct MongoWallet {
    pub id: String,
    pub address: String,
    pub wallet_name: String,
    pub password_hash:String,
    pub created_at: String,
    pub public_key: String,
    pub is_private: bool,
    pub transaction_limit: bool,
    pub transaction_limit_value: f32,
    pub limit_period: LimitPeriod,
    pub is_vault: bool,
    pub release_date: String,
    pub chain:Chain
}


#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct WalletC {
    pub id: String,
    pub address: String,
    pub wallet_name: String,
    pub password_hash:String,
    pub created_at: String,
    pub public_key: String,
    pub is_private: bool,
    pub transaction_limit: bool,
    pub transaction_limit_value: f32,
    pub limit_period: LimitPeriod,
    pub is_vault: bool,
    pub release_date: String,
    pub chain:Chain,
    pub is_vcid_id:Option<bool>,    // is the wallet using vhenncoin id for authentication
    pub vcid_id_user_name:Option<String> 
}

impl WalletC {
    
    pub fn default()->WalletC{
        return WalletC{
            id: "".to_string(),
            address:  "".to_string(),
            wallet_name:  "".to_string(),
            password_hash:  "".to_string(),
            created_at:  "".to_string(),
            public_key:  "".to_string(),
            is_private: false,
            transaction_limit:false,
            transaction_limit_value: 2.2,
            limit_period: LimitPeriod::Daily,
            is_vault: false,
            release_date:  "".to_string(),
            chain:Chain { chain: vec![] },
            is_vcid_id : Some(false),
            vcid_id_user_name : Some("".to_string())
        };
    }
}


impl MongoWallet {
    
    pub fn default()->MongoWallet{
        return MongoWallet{
            id: "".to_string(),
            address:  "".to_string(),
            wallet_name:  "".to_string(),
            password_hash:  "".to_string(),
            created_at:  "".to_string(),
            public_key:  "".to_string(),
            is_private: false,
            transaction_limit:false,
            transaction_limit_value: 2.2,
            limit_period: LimitPeriod::Daily,
            is_vault: false,
            release_date:  "".to_string(),
            chain:Chain { chain: vec![] },
        };
    }
}
