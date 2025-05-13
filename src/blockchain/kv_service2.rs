use actix_web::App;
use futures_util::TryStreamExt;
use log::error;
use serde::de::DeserializeOwned;
use serde::Serialize;
use sled::{Db, IVec};
use sled::transaction::TransactionError;
use tokio::task::spawn_blocking;
use crate::models::account::Account;
use crate::models::block::TBlock;
use crate::utils::app_error::AppError;

pub struct KVService2{
    
}
impl KVService2{
    
    pub async fn save_block(
        db:&Db,
        sender:&str,
        receiver:&str, 
        sender_account:&Account, 
        receiver_account:&Account 
    ) -> Result<(), AppError> {
        
        let tree = match db.open_tree("accounts"){
            Ok(r) => r,
            Err(err)=>{
                error!("{}", err);
                return Err(AppError::OpenTableError(err.to_string()))
            }
        };
        
        let sender_key = sender.as_bytes().to_vec();
        let receiver_key = receiver.as_bytes().to_vec();
        let sender_value = match serde_json::to_vec(sender_account){
            Ok(v) => v,
            Err(err)=>{
                error!("serialization error while inserting {}", err);
                return Err(AppError::SerializationError(err.to_string()));
            }
        };
        let receiver_value = match serde_json::to_vec(receiver_account){
            Ok(v) => v,
            Err(err)=>{
                error!("serialization error while inserting {}", err);
                return Err(AppError::SerializationError(err.to_string()));
            }
        };
        match spawn_blocking(move || {
            let res: Result<(), TransactionError<std::io::Error>> = tree.transaction(|tx| {
                // Attempt to write key1
                tx.insert(sender_key.clone(), sender_value.clone())?;
                // Attempt to write key2
                tx.insert(receiver_key.clone(), receiver_value.clone())?;

                // If we return Ok(()), sled will commit both writes.
                // If we `?` on any error, or explicitly return Err, *none* of the writes take effect.
                Ok(())
            });

            // 4) Check the result
            match res {
                Ok(_) => {Ok(())},
                Err(TransactionError::Abort(err)) => {
                    error!("error writing to db .. {}", err);
                    return Err(AppError::ErrorInsertingData(err.to_string()))
                }
                Err(TransactionError::Storage(err)) => {
                    error!("error writing to db .. {}", err);
                    return Err(AppError::ErrorInsertingData(err.to_string()))
                }
            }
        }).await{
            Ok(_)=>{},
            Err(err)=>{
                error!("{}", err);
                return Err(AppError::DatabaseInsertError(err.to_string()))
            }
        }
        Ok(())
    }

    pub async fn save<T: Serialize>(
        db: &Db,
        table: &str,
        key: &str,
        value: &T,
    ) -> Result<(), AppError> {
        let db = db.clone();
        let table = table.to_string();
        let key = key.to_string();
        let value = serde_json::to_vec(value).map_err(|e|AppError::SerializationError(e.to_string()))?;

        match spawn_blocking(move || {
            let tree = db.open_tree(table).map_err(|e|AppError::OpenTableError(e.to_string()))?;
            tree.insert(key.as_bytes(), value).map_err(|e|AppError::OpenTableError(e.to_string()))?;
            Ok::<(), AppError>(())
        }).await{
            Ok(_) => {}
            Err(err)=>{
                println!("Error: {:?}", err);
                return Err(AppError::DatabaseInsertError(err.to_string()));
            }
        };
        
        Ok(())
    }

    pub async fn get_data<T: DeserializeOwned>(
        db: &sled::Db,
        table: &str,
        key: &str,
    ) -> Result<Option<T>, AppError> {
        let db = db.clone();
        let table = table.to_string();
        let key = key.to_string();

        // Execute blocking operation
        let result: Option<sled::IVec> = match tokio::task::spawn_blocking(move || {
            let tree = match db.open_tree(&table){
                Ok(tree) => tree,
                Err(err)=>{
                    error!("Error: {:?}", err);
                    return Err(AppError::OpenTableError(err.to_string()));
                }
            };
            let value =match  tree.get(key.as_bytes()){
                Ok(tree) => tree,
                Err(err)=>{
                    error!("Error: {:?}", err);
                    return Err(AppError::OpenTableError(err.to_string()));
                }
            };
            Ok::<_, AppError>(value)
        })
            .await{
            Ok(value) => { 
              match value{
                  Ok(value) => value,
                  Err(err)=>{
                      error!("Error: {:?}", err);
                      return Err(AppError::OpenTableError(err.to_string()));
                  }
              }
            },
            Err(err) =>{
                error!("Error: {:?}", err);
                return Err(AppError::OpenTableError(err.to_string()));
            }
        };

        match result {
            Some(data) => {
                let deserialized = match serde_json::from_slice(&data){
                    Ok(deserialized) => deserialized,
                    Err(err) => {
                        error!("Error: {:?}", err);
                        return Err(AppError::SerializationError(err.to_string()))
                    }
                };
                Ok(Some(deserialized))
            }
            None => Ok(None),
        }
    }

    pub async fn get_all_data<T: DeserializeOwned>(
        db: &Db,
        table: &str,
    ) -> Result<Vec<(String, T)>, AppError> {
        let db = db.clone();
        let table = table.to_string();

        let pairs: Vec<(sled::IVec, sled::IVec)> = spawn_blocking(move || {
            let tree = db.open_tree(table).map_err(|e|AppError::OpenTableError(e.to_string()))?;
            Ok::<_, AppError>(tree.iter().collect::<Result<Vec<_>, _>>().map_err(|e|AppError::OpenTableError(e.to_string()))?)
        }).await.map_err(|e|AppError::OpenTableError(e.to_string()))??;

        let mut results = Vec::with_capacity(pairs.len());
        for (key, value) in pairs {
            let key_str = String::from_utf8(key.to_vec()).map_err(|e|AppError::SerializationError(e.to_string()))?;
            let value = serde_json::from_slice(&value).map_err(|e|AppError::SerializationError(e.to_string()))?;
            results.push((key_str, value));
        }

        Ok(results)
    }
}