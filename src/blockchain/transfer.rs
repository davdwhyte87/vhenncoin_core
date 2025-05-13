use std::borrow::{Borrow, BorrowMut};
use std::error::Error;
use std::str::FromStr;
use std::sync::{Arc};
use base64::Engine;
use bigdecimal::BigDecimal;
use sha2::{Digest, Sha256};
use uuid::Uuid;

use crate::blockchain::wallet::{Wallet};
use crate::models::block::{Block, TBlock, VBlock};
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
use sled::Db;
use crate::blockchain::chain::{ChainX};
use crate::blockchain::kv_service::KVService;
use crate::blockchain::kv_store::KvStore;
use crate::blockchain::mongo_store::WalletService;
use crate::models::constants::{BLOCKS_TABLE, META_DATA_TABLE, TRANSACTIONS_LOG_TABLE};
use crate::models::db::{MongoService, DB};
use crate::models::mempool::Mempool;
use crate::models::request::TransferReq;
use crate::models::transaction::Transaction;



pub struct Transfer {

}
use thiserror::Error;
use tokio::sync::Mutex;
use crate::blockchain::kv_service2::KVService2;
use crate::utils::app_error::AppError;
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


    pub async fn get_all_transactions(mempool: Arc<Mutex<Mempool>>) -> Vec<Transaction> {
        let pool = mempool.lock().await; // Lock temporarily

        let mut all_tx = Vec::new();

        for txs in pool.transactions.values() {
            all_tx.extend_from_slice(txs);  // Just collecting data
        }

        all_tx
    }
    
    pub async fn process_single_bf_transaction(db:&Db, tx:Transaction)->Result<(), AppError>{
        // check if the accounts exist 
        let mut sender_account =match  Wallet::get_user_account(db,tx.sender.clone()).await?{
            Some(account) => account,
            None=>{ 
                return Err(AppError::AccountNotFound(tx.sender.clone()))
            }
        };

        let mut receiver_account =match  Wallet::get_user_account(db,tx.receiver.clone()).await?{
            Some(account) => account,
            None=>{
                return Err(AppError::AccountNotFound(tx.receiver.clone()))
            }
        }; 
        
        // verify the sender balance 
        let is_valid_amount =Wallet::verify_transaction_amount(db, &tx).await?;
        if !is_valid_amount {
            log::debug!("bad transaction amount {} - {} {}", sender_account.address.clone(),
                    receiver_account.address.clone(), tx.amount.clone()
                );
            return Err(AppError::InsufficientFunds);
        }
        //  create blocks
        let block = TBlock{
            id: tx.id.clone(),
            sender: tx.sender.clone(),
            receiver: tx.receiver.clone(),
            timestamp: tx.timestamp,
            amount: tx.amount.clone(),
        };
        // get tx id and check
        let derived_tx_id = Self::get_transaction_hash(tx.clone());
        if derived_tx_id != tx.id{
            return Err(AppError::IDMismatch);
        }
        // check if the tx exists
        
        for otx in sender_account.chain.clone(){
            if otx.id == derived_tx_id{
              return Err(AppError::TransactionAlreadyExists);  
            }
        }
        
        // add blocks
        let mut sender_chain = sender_account.chain.clone();
        sender_chain.push(block.clone());
        sender_account.chain = sender_chain;
        let mut receiver_chain = receiver_account.chain.clone();
        receiver_chain.push(block.clone());
        receiver_account.chain = receiver_chain;
      
        match KVService2::save_block(db, tx.sender.as_str(), tx.receiver.as_str(), &sender_account, &receiver_account).await{
            Ok(_)=>{},
            Err(err)=>{
                error!("Error while saving block: {}", err.to_string());
                return Err(err)
            }
        };
        
        Ok(())
    }
    
    pub fn get_transaction_hash(tx:Transaction)->String{
        let mut hasher = Sha256::new();

        // Convert everything to bytes and hash
        hasher.update(tx.timestamp.to_be_bytes());
        hasher.update(tx.sender.as_bytes());
        hasher.update(tx.receiver.as_bytes());
        hasher.update(tx.amount.normalized().to_string().as_bytes());

        // Finalize and convert to hex string
        let result = hasher.finalize();
        hex::encode(result)
    }
    // check all transactions from mem pool and do credits and debits

    pub async fn save_transactions_logs(db:&Db, transaction:Transaction)->Result<(), AppError> {
        // get the sender and receiver logs
        let sender_log =  KVService2::get_data::<Vec<Transaction>>(db, TRANSACTIONS_LOG_TABLE,transaction.sender.as_str() ).await?;
        let receiver_log =  KVService2::get_data::<Vec<Transaction>>(db, TRANSACTIONS_LOG_TABLE,transaction.receiver.as_str() ).await?;
        match sender_log {
            Some(mut sender_log) => {
                sender_log.push(transaction.clone());
               
                KVService2::save(db, TRANSACTIONS_LOG_TABLE, &transaction.sender.clone(),&sender_log ).await?;
            }
            None=>{
                let _log = vec![transaction.clone()]; 
                
                KVService2::save(db, TRANSACTIONS_LOG_TABLE, &transaction.sender.clone(),&_log ).await?;  
            }
        }

        match receiver_log {
            Some(mut receiver_log) => {
                receiver_log.push(transaction.clone());
               
                KVService2::save(db, TRANSACTIONS_LOG_TABLE, &transaction.receiver,&receiver_log ).await?;
            }
            None=>{
                let _log = vec![transaction.clone()];
            
                KVService2::save(db, TRANSACTIONS_LOG_TABLE, &transaction.receiver,&_log ).await?;
            }
        }
      Ok(())  
    }

    // pub fn remove_from_mempool(mempool: &mut Mempool, tx: &Transaction) {
    //     if let Some(pending_tx) = mempool.transactions.get_mut(&tx.sender) {
    //         // Remove the transaction by matching nonce or other unique identifiers
    //         pending_tx.retain(|t| t.nonce != tx.nonce); // Remove the transaction with the same nonce
    //     }
    // }
}