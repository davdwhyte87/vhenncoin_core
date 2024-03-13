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

#[derive(Debug, Serialize, Deserialize)]
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
