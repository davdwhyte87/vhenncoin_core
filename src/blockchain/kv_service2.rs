use actix_web::App;
use futures_util::TryStreamExt;
use log::error;
use serde::de::DeserializeOwned;
use serde::Serialize;
use sled::{Db, IVec};
use tokio::task::spawn_blocking;
use crate::utils::app_error::AppError;

pub struct KVService2{
    
}
impl KVService2{
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