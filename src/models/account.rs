use bigdecimal::{BigDecimal, Zero};
use chrono::NaiveDateTime;

use serde_derive::{Deserialize, Serialize};
use crate::models::block::TBlock;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Default)]
pub struct Account {
    pub id: String,
    pub address: String,
    pub public_key: String,
    pub chain: Vec<TBlock>,
    pub created_at: NaiveDateTime,
    pub balance:BigDecimal
}

impl Account{
    pub fn get_balance(&mut self) -> BigDecimal {
        let mut balance = BigDecimal::zero();
        for tblock in &self.chain{
            if tblock.receiver.to_owned() == self.address{
                balance = tblock.amount.to_owned() + balance;
            } else if tblock.sender == self.address {
                balance -= &tblock.amount;
            }

        }
        self.balance = balance.clone();
        return balance;
    }
}