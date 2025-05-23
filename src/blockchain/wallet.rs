use std::error::Error;
use std::str::FromStr;
use actix_web::App;
use bigdecimal::{BigDecimal, Zero};
use chrono::{Local, NaiveDateTime};
use k256::ecdsa::{Signature, SigningKey, VerifyingKey};
use k256::ecdsa::signature::hazmat::PrehashVerifier;
use k256::ecdsa::signature::{DigestVerifier, Verifier};
use k256::elliptic_curve::weierstrass::add;
use k256::EncodedPoint;

use sha2::{Digest, Sha256};
use sled::Db;
use uuid::Uuid;
use crate::blockchain::kv_service2::KVService2;
use crate::blockchain::kv_service::KVService;
use crate::blockchain::transfer::Transfer;
use crate::models::account::Account;
use crate::models::block::TBlock;
use crate::models::constants::{ACCOUNTS_TABLE, META_DATA_TABLE, TRANSACTIONS_LOG_TABLE};
use crate::models::request::{CreateWalletReq, TransferReq};
use crate::models::transaction::Transaction;
use crate::utils::app_error::AppError;
use crate::utils::struct_h::Struct_H;

pub struct Wallet{

}


impl Wallet {

    // check if data path exists



    fn generate_compressed_pubkey(seed: &str) -> String {
        let hash = Sha256::digest(seed.as_bytes());
        let signing_key = SigningKey::from_bytes(&hash).unwrap();
        let verify_key = signing_key.verifying_key();
        hex::encode(verify_key.to_encoded_point(true).as_bytes())
    }
    pub async fn verify_transaction_signature(db:&Db, transaction:TransferReq)-> Result<bool, AppError>{
        log::debug!("amount {}",transaction.amount.to_string().clone());
        let transaction_data = format!(
        "{}{}{}{}{}",
            transaction.sender,
            transaction.receiver,
            transaction.amount,
            transaction.timestamp,
            transaction.id
        );

        let raw = transaction_data.as_bytes();
        log::debug!("🦀 raw preimage hex: {}", hex::encode(raw));
        
        let account = match Self::get_user_account(db,transaction.sender.clone()).await{
            Ok(account)=>{account},
            Err(err)=>{return Err(err.into())}
        };

        let pubk = &account.unwrap_or_default().public_key;
        let public_key_bytes =match hex::decode(pubk.clone()){
            Ok(public_key_bytes)=>{public_key_bytes},
            Err(err)=>{
                log::error!("{:?}", err);
                return Err(AppError::HexDecodeError(err.to_string()))
            }
        };

        log::debug!("Public key from DB: {}", pubk);
        log::debug!("Public key bytes: {:?}", public_key_bytes);
        log::debug!("Length: {}", public_key_bytes.len());
        
        let public_key = match VerifyingKey::from_sec1_bytes(&public_key_bytes){
            Ok(public_key)=>{public_key},
            Err(err)=>{
                log::error!("{:?}", err);
                return Err(AppError::VerifyKeyError(err.to_string()))
            }
        };

        log::debug!("public_key: {}", pubk.clone());
        log::debug!("signature : {}", transaction.signature.clone());
        let signature_bytes = match hex::decode(&transaction.signature){
            Ok(signature_bytes)=>{signature_bytes},
            Err(err)=>{
                log::error!("{:?}", err);
                return Err(AppError::HexDecodeError(err.to_string()))
            }
        };
        log::debug!("Signature bytes: {:?}", &signature_bytes);
        let signature = match Signature::from_slice(&signature_bytes){
            Ok(signature)=>{signature},
            Err(err)=>{
                log::error!("{}", err.to_string());
                return Err(AppError::SignatureError(err.to_string()))
            }
        }; 
        
        let is_valid = public_key.verify(transaction_data.as_bytes(), &signature).is_ok();
        log::debug!("is_valid: {}", is_valid);
        Ok(is_valid)
    }


    pub async fn verify_signature(db:&Db, message:String, address:String, signature_txt:String)-> Result<bool, AppError>{
        let transaction_data = format!(
            "{}", // Concatenate relevant fields for the transaction
            message
        );
        log::debug!("transaction_data: {}", transaction_data);
        let hash = Sha256::digest(transaction_data.as_bytes());
        log::debug!("tx_hash: {:x}", hash);

        let mut digest = Sha256::new();
        digest.update(message.as_bytes());
        
        let account = match Self::get_user_account(db, address.clone()).await{
            Ok(account)=>{account},
            Err(err)=>{return Err(err.into())}
        };

        let pubk = &account.unwrap_or_default().public_key;
        let public_key_bytes =match hex::decode(pubk.clone()){
            Ok(public_key_bytes)=>{public_key_bytes},
            Err(err)=>{
                log::error!("{:?}", err);
                return Err(AppError::HexDecodeError(err.to_string()))
            }
        };

        log::debug!("Public key from DB: {}", pubk);
        log::debug!("Public key bytes: {:?}", public_key_bytes);
        log::debug!("Length: {}", public_key_bytes.len());

        let public_key = match VerifyingKey::from_sec1_bytes(&public_key_bytes){
            Ok(public_key)=>{public_key},
            Err(err)=>{
                log::error!("{:?}", err);
                return Err(AppError::VerifyKeyError(err.to_string()))
            }
        };

        log::debug!("public_key: {}", pubk.clone());
     
        let signature_bytes = match hex::decode(signature_txt){
            Ok(signature_bytes)=>{signature_bytes},
            Err(err)=>{
                log::error!("{:?}", err);
                return Err(AppError::HexDecodeError(err.to_string()))
            }
        };
        log::debug!("Signature bytes: {:?}", &signature_bytes);
        let signature = match Signature::from_slice(&signature_bytes){
            Ok(signature)=>{signature},
            Err(err)=>{
                log::error!("{}", err.to_string());
                return Err(AppError::SignatureError(err.to_string()))
            }
        }; // This is the correct way.


        let is_valid = public_key.verify_digest(digest, &signature).is_ok();
        log::debug!("is_valid: {}", is_valid);
        Ok(is_valid)
    }

    // verify sender has enough funds
    pub async fn verify_transaction_amount(db:&Db, transaction:&Transaction)-> Result<bool, AppError>{
        let balance =Self::get_balance(db, transaction.sender.clone()).await?;
        // make sure user has enough coins
        if transaction.amount > balance{
            return Ok(false)
        }
        Ok(true)
    }
    // add transaction to mem pool
    
    
    pub async fn get_balance(db:&Db, sender:String)->Result<BigDecimal, AppError>{
        let account = match Self::get_user_account(db,sender.clone()).await{
            Ok(account)=>{account},
            Err(err)=>{return Err(err.into())}
        };
        
        if account.is_none(){
            return Err(AppError::AccountNotFound(sender))
        }
        
        // get chain 
        let mut balance = BigDecimal::zero();
        for tblock in account.unwrap_or_default().chain{
            if tblock.receiver == sender{
                balance = tblock.amount + balance; 
            } else if tblock.sender == sender {
                balance -= &tblock.amount;
            }

        }
        return Ok(balance);
    }
    


    // is account exists
    // pub async fn is_account_exists(address:String)->Result<bool, Box<dyn Error>>{
    //
    // }
    pub async fn get_user_account(db:&Db,address:String)->Result<Option<Account>,AppError>{
        let data:Option<Account> = KVService2::get_data::<Account>(db, ACCOUNTS_TABLE, &*address).await?;
        Ok(data)
    }


    pub async fn update_user_account(db:&Db, account: Account)-> Result<(), AppError>{
        
        // update senders wallet
        KVService2::save(db, ACCOUNTS_TABLE, &*account.address, &account).await?;
        Ok(())
    }

    pub async fn get_last_block_height(db:&Db)->Result<u64, AppError>{
      let latest_height = KVService2::get_data::<u64>(db, META_DATA_TABLE, "latest_height").await?;
        Ok(latest_height.unwrap_or_default())
    }

    pub async fn get_user_transactions_log(db:&Db, address:&str)->Result<Vec<Transaction>,AppError>{
        let transactions = KVService2::get_data::<Vec<Transaction>>(db, TRANSACTIONS_LOG_TABLE, address).await?;
        match transactions{
            Some(transactions)=>{
                Ok(transactions)
            },
            None=>{
                Ok(vec![])
            }
        }
    }
    
    

    // pub async fn save_block(block: VBlock, mempool: Arc<Mutex<Mempool>>)-> Result<(), Box<dyn std::error::Error>>{
    //
    //     // update last block
    // }



    // create wallet with rocks db kv store
    pub async fn create_wallet_r(db:&Db, req:CreateWalletReq)->Result<(), AppError>{

        let mut balance:BigDecimal = BigDecimal::zero();
        let mut chain:Vec<TBlock> = vec![];
        // for genesis wallets
        if req.address == "genesis"{
            balance = match BigDecimal::from_str("99900000000000"){
                Ok(data)=>{data},
                Err(err)=>{
                    log::error!("error creating genesis balance {}", err.to_string());
                    return Err(AppError::BigDecimalConversionError(err.to_string()))
                }
            };
            let mut block = TBlock{
                id: "".to_string(),
                sender: "000000000".to_string(),
                receiver: req.address.clone(),
                timestamp: 0,
                amount: balance.clone(),
            };
            let tx_id = Transfer::get_transaction_hash(Transaction{
                id: "".to_string(),
                sender: block.sender.clone(),
                receiver: block.receiver.clone(),
                amount: block.amount.clone(),
                signature: "".to_string(),
                timestamp: 0,
            });
            block.id = tx_id.clone();
            chain.push(block);
        }
        
        let account = Account{
            id: Uuid::new_v4().to_string(),
            address: req.address.clone(),
            chain: chain,
            created_at: Local::now().naive_local(),
            public_key: req.public_key,
            balance:BigDecimal::zero(),
        };
        KVService2::save(db, ACCOUNTS_TABLE, &account.address, &account).await?;
        return Ok(())
    }
}

    // creates a wallet but this is not accessible to client apps, this is for 
    //internal node syncing purposes
    // pub fn create_wallet_node(address: &String, wallet:WalletC)->Result<(), Box<dyn Error>>{
    //     let d_path = format!("data/{}", address) ;
    //     if !Path::new(d_path.as_str()).exists() {
    //         let folder = fs::create_dir_all(d_path.as_str());
    //         match folder {
    //             Ok(folder)=>folder,
    //             Err(err)=>{
    //                 error!("{}", err.to_string());
    //                 return Err(err.into())
    //             }
    //         }
    //
    //     }else{
    //         return Err(Box::from("Wallet path exists"))
    //     }
    //
    //
    //     let wallet_string:String = match serde_json::to_string(&wallet){
    //         Ok(str)=>{str},
    //         Err(r)=>{
    //             error!("error encoding wallet {}",r.to_string());
    //             return Err(r.into())
    //         }
    //     };
    //     // try creating the database
    //     let path = format!("data/{}/wallet.redb", address) ;
    //     const TABLE: TableDefinition<&str, String> = TableDefinition::new("my_data");
    //     let db =match  Database::create(path){
    //         Ok(data)=>{data},
    //         Err(err)=>{
    //             error!("error {}", err.to_string());
    //             return Err(err.into())
    //         }
    //     };
    //     let write_txn =match  db.begin_write(){
    //         Ok(data)=>{data},
    //         Err(err)=>{
    //             error!("error {}", err.to_string());
    //             return Err(err.into())
    //         }
    //     };
    //
    //
    //     {
    //         let mut table = match write_txn.open_table(TABLE){
    //             Ok(data)=>{data},
    //             Err(err)=>{
    //                 error!("error opening table  {}", err.to_string());
    //                 return Err(err.into())
    //             }
    //         };
    //         match table.insert("wallet_data", wallet_string){
    //             Ok(_)=>{},
    //             Err(err)=>{
    //                 error!("error inserting data {}", err.to_string());
    //                 return Err(err.into())
    //             }
    //         };
    //     }
    //     let _commit_res = match write_txn.commit(){
    //         Ok(data)=>{data},
    //         Err(err)=>{
    //             error!("commit error {}", err.to_string());
    //             return Err(err.into())
    //         }
    //     };
    //     let read_txn = match  db.begin_read(){
    //         Ok(data)=>{data},
    //         Err(err)=>{
    //             error!("error {}", err.to_string());
    //             return Err(err.into())
    //         }
    //     };
    //     let table = match read_txn.open_table(TABLE){
    //         Ok(data)=>{data},
    //         Err(err)=>{
    //             error!("error {}", err.to_string());
    //             return Err(err.into())
    //         }
    //     };
    //     let res_data =match  table.get("wallet_data") {
    //         Ok(data)=>{
    //             match data {
    //                 Some(data)=>{data},
    //                 None=>{return Err(Box::from("No data"))}
    //             }
    //             },
    //         Err(err)=>{
    //             error!("error {}", err.to_string());
    //             return Err(err.into())
    //         }
    //     };
    //     debug!("{}", res_data.value()  );
    //     return Ok(())
    // }

