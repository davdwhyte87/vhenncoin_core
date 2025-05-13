use bigdecimal::BigDecimal;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Transaction {
    pub id:String,
    pub sender: String,
    pub receiver: String,
    pub amount: BigDecimal,
    pub signature: String,
    pub timestamp: u64
}