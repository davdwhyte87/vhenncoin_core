use serde_derive::{Deserialize, Serialize};

// this represents a chunck of transaction data in a wallet
#[derive(Debug, Serialize, Deserialize)]
pub struct Block{
    pub id:String,
    pub sender_address:String,
    pub receiver_address:String,
    pub date_created:String,
    pub hash:String,
    pub prev_hash:String,
    pub amount: f32,
    pub public_key: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Chain{
    pub chain: Vec<Block>
}

