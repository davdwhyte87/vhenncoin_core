use bigdecimal::BigDecimal;
use serde_derive::{Deserialize, Serialize};

use super::wallet::{MongoWallet, WalletC};

#[derive(Serialize,Deserialize)]
pub struct GenericResponse {
    pub message: String,
    pub code:i32
    
}

#[derive(Serialize,Deserialize)]
pub struct GetBalanceResponse {
    pub message: String,
    pub code:i32,
    pub balance: f32

}

#[derive(Serialize,Deserialize)]
pub struct GetBalanceResp {
    pub balance: BigDecimal,
    pub address: String,
}

#[derive(Serialize,Deserialize)]
pub struct WalletNamesResp{
    pub names: Vec<MongoWallet>

}

#[derive(Serialize,Deserialize)]
pub struct WalletNamesRespC{
    pub names: Vec<String>
}


#[derive(Serialize,Deserialize, Clone, Debug)]
pub struct NResponse<T>{
    pub status: i32, // 0=fail 1=success
    pub message: String,
    pub data: Option<T>
}

// 0 = error 1= success 4 = not found 6= unauthorized


