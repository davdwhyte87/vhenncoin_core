use serde::{Serialize,Deserialize};


#[derive(Serialize, Deserialize)]
pub struct TransferReq {
    pub sender: String,
    pub receiver: String,
    pub amount: String,
}

#[derive(Serialize, Deserialize)]
pub struct GetBalanceReq {

    pub address: String,

}



#[derive(Serialize, Deserialize)]
pub struct CreateWalletReq {
    pub address: String,
    pub password: String,
    pub wallet_name:String
}


#[derive(Serialize, Deserialize)]
pub struct GetNodeListReq{

}


#[derive(Serialize, Deserialize)]
pub struct HttpMessage{
    pub message: String
}


