use std::borrow::{Borrow, BorrowMut};
use std::error::Error;
use std::str::FromStr;
use std::sync::{Arc, Mutex};
use base64::Engine;
use bigdecimal::BigDecimal;
use sha2::{Digest, Sha256};
use uuid::Uuid;

use crate::blockchain::wallet::{Wallet};
use crate::models::block::{Block, VBlock};
use crate::utils::time::get_date_time;
use crate::utils::utils::{MyError, MyErrorTypes};
use base64::engine::general_purpose;
use chrono::Utc;
use futures_util::AsyncReadExt;
use jsonwebtoken::crypto::sign;
use log::{debug, error, log};
use mongodb::options::AuthMechanism::ScramSha256;
use mongodb::results::UpdateResult;
use r2d2_mongodb::mongodb::ErrorCode::OK;
use redb::{Database, TableDefinition};
use ring::agreement::{Algorithm, UnparsedPublicKey};
use ring::error::Unspecified;
use ring::rand::SystemRandom;
use ring::signature;
use ring::signature::{ECDSA_P256_SHA256_ASN1, ECDSA_P256_SHA256_ASN1_SIGNING, EcdsaKeyPair, EcdsaSigningAlgorithm, ED25519, Ed25519KeyPair, KeyPair, VerificationAlgorithm};
use rsa::{Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey, pkcs1::DecodeRsaPrivateKey, pkcs1::DecodeRsaPublicKey, Pkcs1v15Sign};
use rsa::pkcs8::spki::Error::AlgorithmParametersMissing;
use rsa::traits::SignatureScheme;
use serde_json::to_string;
use sha256::digest;
use crate::blockchain::chain::{ChainX};
use crate::blockchain::kv_service::KVService;
use crate::blockchain::kv_store::KvStore;
use crate::blockchain::mongo_store::WalletService;
use crate::models::constants::{BLOCKS_TABLE, META_DATA_TABLE, TRANSACTIONS_LOG_TABLE};
use crate::models::db::MongoService;
use crate::models::mempool::Mempool;
use crate::models::request::TransferReq;
use crate::models::transaction::Transaction;



pub struct Transfer {

}
use thiserror::Error;
use crate::utils::struct_h::Struct_H;

#[derive(Error, Debug)]
pub enum TransferError{
    #[error("Error with transaction signature")]
    InvalidSignature,
    #[error("Error while verifying transaction")]
    ErrorWhileVerifyingTransaction,
    #[error("Trnasaction already exists in blockchain")]
    TransactionExistsInMempool,
    
    
}

impl Transfer {

    pub async fn add_to_mempool(db:&Database, mempool: Arc<Mutex<Mempool>>,  tx: Transaction)->Result<(), TransferError>{
        
        let mut  pool = mempool.lock().unwrap();
        let pending_tx =pool.transactions.entry(tx.sender.clone()).or_insert_with(Vec::new);

        // Check if this sender already has a transaction with the same nonce
        if pending_tx.iter().any(|t| t.nonce == tx.nonce) {
            return Err(TransferError::TransactionExistsInMempool);
        }
        pending_tx.push(tx);
        log::debug!("Current state of mempool: {:?}", pool.transactions);
        return Ok(());
    }

    pub async fn get_all_transactions(mempool: Arc<Mutex<Mempool>>) -> Vec<Transaction> {
        let pool = mempool.lock().unwrap(); // Lock temporarily

        let mut all_tx = Vec::new();

        for txs in pool.transactions.values() {
            all_tx.extend_from_slice(txs);  // Just collecting data
        }

        all_tx
    }
    // check all transactions from mem pool and do credits and debits
    pub async fn process_transactions(db:&Database, mempool: Arc<Mutex<Mempool>>, transactions:Vec< Transaction>) -> Result<(), Box<dyn Error>> {
        let mut processed_tx: Vec<Transaction> = vec![];
        if transactions.len() == 0 {
            log::debug!("Transactions empty ...");
            return Ok(());
        }
        for tx in transactions {
            log::debug!("processing transaction {} -- {}", tx.sender.clone(), tx.receiver.clone());
            // get sender/ reciever account
            let mut sender_account =match  Wallet::get_user_account(db,tx.sender.clone()).await?{
                Some(account) => account,
                None=>{ continue }
            };

            let mut receiver_account =match  Wallet::get_user_account(db,tx.receiver.clone()).await?{
                Some(account) => account,
                None=>{ continue }
            };

            // verify transaction amount
            let is_valid_amount =Wallet::verify_transaction_amount(db, &tx).await?;
            if !is_valid_amount {
                log::debug!("bad transaction amount {} - {} {}", sender_account.address.clone(),
                    receiver_account.address.clone(), tx.amount.clone()
                );
                // remove transaction from mempool
                let mut pool = mempool.lock().unwrap();
                Self::remove_from_mempool(&mut pool, &tx);
                continue;
            }


            // update accounts
            sender_account.balance = sender_account.balance - tx.amount.clone();
            if tx.nonce != sender_account.nonce {
                continue
            }
            sender_account.nonce = sender_account.nonce + 1;

            receiver_account.balance = receiver_account.balance + tx.amount.clone();
            

            match Wallet::update_user_account(db,sender_account).await{
                Ok(_) => {}
                Err(e)=>{
                    log::error!("Error updating user account : {}", e);
                    continue
                }
            };
            match Wallet::update_user_account(db, receiver_account).await{
                Ok(_) => {}
                Err(e)=>{
                    log::error!("Error updating user account : {}", e);
                    continue
                } 
            };

            processed_tx.push(tx.clone());
            // Remove the transaction from the mempool after processing it
            let mut pool = mempool.lock().unwrap();
            Self::remove_from_mempool(&mut pool, &tx);
        }
        
        // prevent block creation when there are no successful transactions
        if(processed_tx.len() == 0) {
            return Ok(());
        }
        
        // save block
        let previous_block_height = Wallet::get_last_block_height(db).await?;
        // get last block
        let last_block:Option<VBlock> = KVService::get_data::<VBlock>(db, BLOCKS_TABLE, previous_block_height.to_string().as_str() )?;
        
        let mut prev_hash:String;
        if last_block.is_some() {
            prev_hash = last_block.unwrap().hash.clone();
        }else{
            prev_hash = "000000000".to_string();
        }
        let mut new_block = VBlock{
            previous_hash: prev_hash,
            transactions: processed_tx.clone(),
            timestamp: Utc::now().timestamp(),
            hash: "".to_string(),
            block_height: previous_block_height+1,
        };
        let hash = ChainX::calculate_block_hash(&new_block)?;
        new_block.hash = hash.clone();
        // save new block
        KVService::save(db,BLOCKS_TABLE, (previous_block_height+1).to_string(), to_string(&new_block)?)?;
        KVService::save(db, META_DATA_TABLE, "latest_height".to_string(), (previous_block_height+1).to_string()  )?;
        
        
        // store transaction logs
        for tx in processed_tx{
            match Self::save_transactions_logs(db, tx).await{
                Ok(_) => {}
                Err(e)=>{
                    log::error!("Error updating user account : {}", e);
                    continue
                }
            };
        }
        Ok(())
    }
    
    pub async fn save_transactions_logs(db:&Database, transaction:Transaction)->Result<(), Box<dyn Error>> {
        // get the sender and receiver logs
        let sender_log =  KVService::get_data::<Vec<Transaction>>(db, TRANSACTIONS_LOG_TABLE,transaction.sender.as_str() )?;
        let receiver_log =  KVService::get_data::<Vec<Transaction>>(db, TRANSACTIONS_LOG_TABLE,transaction.receiver.as_str() )?;
        match sender_log {
            Some(mut sender_log) => {
                sender_log.push(transaction.clone());
                let log_string = Struct_H::vec_to_string::<Transaction>(sender_log);
                KVService::save(db, TRANSACTIONS_LOG_TABLE, transaction.sender.clone(),log_string )?;
            }
            None=>{
                let _log = vec![transaction.clone()]; 
                let log_string = Struct_H::vec_to_string::<Transaction>(_log);
                KVService::save(db, TRANSACTIONS_LOG_TABLE, transaction.sender.clone(),log_string )?;  
            }
        }

        match receiver_log {
            Some(mut receiver_log) => {
                receiver_log.push(transaction.clone());
                let log_string = Struct_H::vec_to_string::<Transaction>(receiver_log);
                KVService::save(db, TRANSACTIONS_LOG_TABLE, transaction.receiver,log_string )?;
            }
            None=>{
                let _log = vec![transaction.clone()];
                let log_string = Struct_H::vec_to_string::<Transaction>(_log);
                KVService::save(db, TRANSACTIONS_LOG_TABLE, transaction.receiver,log_string )?;
            }
        }
      Ok(())  
    }

    pub fn remove_from_mempool(mempool: &mut Mempool, tx: &Transaction) {
        if let Some(pending_tx) = mempool.transactions.get_mut(&tx.sender) {
            // Remove the transaction by matching nonce or other unique identifiers
            pending_tx.retain(|t| t.nonce != tx.nonce); // Remove the transaction with the same nonce
        }
    }
}