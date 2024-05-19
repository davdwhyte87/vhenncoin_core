use serde_derive::{Deserialize, Serialize};

use super::wallet::MongoWallet;

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
pub struct WalletNamesResp{
    pub names: Vec<MongoWallet>

}


// 0 = error 1= success 4 = not found 6= unauthorized


