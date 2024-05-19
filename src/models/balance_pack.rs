use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct BalancePack {
    pub server_http_address:String,
    pub balance:f32
}
