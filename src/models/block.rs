use std::clone;

use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};

// this represents a chunck of transaction data in a wallet
#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Block{
    pub id:String,
    pub transaction_id:String,
    pub sender_address:String,
    pub receiver_address:String,
    pub date_created:String,
    pub hash:String,
    pub prev_hash:String,
    pub amount: BigDecimal,
    pub public_key: String,
    pub balance:BigDecimal,
    pub trx_h:Option<String>
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Chain{
    pub chain: Vec<Block>
}

