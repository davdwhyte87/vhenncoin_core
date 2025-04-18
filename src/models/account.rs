use bigdecimal::BigDecimal;
use serde_derive::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
pub struct Account {
    pub id: String,
    pub address: String,
    pub wallet_name: String,
    pub nonce: u64,
    pub balance:BigDecimal,
    pub created_at: String,
    pub public_key: String,
}