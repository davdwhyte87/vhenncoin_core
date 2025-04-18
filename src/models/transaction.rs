use bigdecimal::BigDecimal;
use serde_derive::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone, PartialEq)]
pub struct Transaction {
    pub sender: String,
    pub receiver: String,
    pub amount: BigDecimal,
    pub nonce: u64,
    pub signature: String,
}