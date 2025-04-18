use std::collections::HashMap;
use crate::models::transaction::Transaction;
type Address = String;
#[derive(Debug, Default, Clone)]
pub struct Mempool {
    pub transactions: HashMap<Address, Vec<Transaction>>,
}


impl Mempool {
    pub fn new() -> Self {
        Mempool {
            transactions: HashMap::new(),
        }
    }
}