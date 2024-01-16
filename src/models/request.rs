
use serde_derive::Serialize;

#[derive(Serialize)]
pub struct TransferReq {
    pub message: String,
    pub sender: String,
    pub receiver: String,
    pub amount: String,
    pub transaction_key: String
}

