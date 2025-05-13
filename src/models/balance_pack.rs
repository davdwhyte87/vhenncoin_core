use bigdecimal::BigDecimal;
use serde::{Deserialize, Serialize};

use super::wallet::WalletC;


#[derive(Debug, Serialize, Deserialize)]
pub struct BalancePack {
    pub server_http_address:String,
    pub balance:BigDecimal
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BalanceCPack {
    pub ip_address:String,
    pub balance:BigDecimal
}


#[derive(Debug, Serialize, Deserialize)]
pub struct WalletCPack {
    pub ip_address:String,
    pub wallet:WalletC
}


