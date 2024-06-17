use serde::{Serialize, Deserialize};
use crate::models::block::Chain;

#[derive(Debug, Serialize, Deserialize)]
pub struct Wallet {
    pub id: String,
    pub address: String,
    pub wallet_name: String,
    pub created_at: String,
    pub is_private: bool,
    pub transaction_limit: bool,
    pub transaction_limit_value: f32,
    pub limit_period: LimitPeriod,
    pub is_vault: bool,
    pub release_date: String
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


#[derive(Debug, Serialize, Deserialize)]
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
    pub chain:Chain
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
