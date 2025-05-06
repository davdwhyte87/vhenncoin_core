use bigdecimal::BigDecimal;
use chrono::NaiveDateTime;
use serde_derive::{Deserialize, Serialize};
use crate::models::block::TBlock;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
pub struct Account {
    pub id: String,
    pub address: String,
    pub public_key: String,
    pub chain: Vec<TBlock>,
    pub(crate) created_at: NaiveDateTime
}