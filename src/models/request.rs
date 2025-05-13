use bigdecimal::BigDecimal;
use serde::{Serialize, Deserialize};


#[derive(Serialize, Deserialize)]
pub struct TransferReq {
    pub sender: String,
    pub amount:String,
    pub receiver: String,
    pub id: String,
    pub timestamp: u64,
    pub signature:String
}

#[derive(Serialize, Deserialize)]
pub struct GetBalanceReq {

    pub address: String,

}

#[derive(Serialize, Deserialize)]
pub struct GetUserTransactionsReq {
    pub address: String,
}

#[derive(Serialize, Deserialize)]
pub struct GetWalletReq {
    pub address: String,
}

#[derive(Serialize, Deserialize)]
pub struct VerifyWalletReq {
    pub address: String,
    pub message: String,
    pub signature: String,
}


#[derive(Serialize, Deserialize)]
pub struct CreateUserIDReq{
    pub user_name: String,
    pub password:String
}


#[derive(Serialize, Deserialize)]
pub struct ValidateUserIDReq{
    pub user_name: String,
    pub password:String
}





#[derive(Serialize, Deserialize)]
pub struct CreateWalletReq {
    pub address: String,
    pub wallet_name:String,
    pub public_key:String,
}

#[derive(Serialize, Deserialize)]
pub struct GetAccountReq {
    pub address: String,
}



pub struct NRequest <T>{
    pub action:String, 
    pub data:T,
}


#[derive(Serialize, Deserialize)]
pub struct AddNodeReq{
    pub id: String,
    pub ip_address:String,
    pub public_key:String,
    pub http_address:String
}


#[derive(Serialize, Deserialize)]
pub struct GetNodeListReq{

}


#[derive(Serialize, Deserialize)]
pub struct HttpMessage{
    pub message: String
}


