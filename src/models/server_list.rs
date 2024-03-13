use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ServerList {
    pub server_list: Vec<ServerData>
}


#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct ServerData {
    pub id: String,
    pub ip_address:String,
    pub public_key:String,
    pub http_address:String
}