use std::{clone, error::Error, path::Path};

use futures::future::ok;
use log::error;
use redb::{Database, TableDefinition};
use serde::{de::{self, DeserializeOwned}, Deserialize};


pub struct KVService{

}

impl KVService {
    pub fn save(
        path:String,
        table_name:&str,
        key:String,
        value:String
    )->Result<(), Box<dyn Error>>{
    
        // let path = format!("id_data/user_data.redb");
    
        let TABLE: TableDefinition<&str, String> = TableDefinition::new(table_name);
            let db =match  Database::create(path){
                Ok(data)=>{data},
                Err(err)=>{
                    error!("error {}", err.to_string());
                    return Err(err.into())
                }
        };
    
        let write_txn =match  db.begin_write(){
            Ok(data)=>{data},
            Err(err)=>{
                error!("error {}", err.to_string());
                return Err(err.into())
            }
        };
    
        {
            let mut table = match write_txn.open_table(TABLE){
                Ok(data)=>{data},
                Err(err)=>{
                    error!("error opening table  {}", err.to_string());
                    return Err(err.into())
                } 
            };
            match table.insert(&*key, value){
                Ok(_)=>{},
                Err(err)=>{
                    error!("error inserting data {}", err.to_string());
                    return Err(err.into())
                } 
            };
        }
    
        let _commit_res = match write_txn.commit(){
            Ok(data)=>{data},
            Err(err)=>{
                error!("commit error {}", err.to_string());
                return Err(err.into())
            }   
        };
    
        return Ok(())
    }



    pub fn get_data<T>(
        path:&str,
        table_name:&str,
        key:&str,
    )->Result<(T), Box<dyn Error>>where T: DeserializeOwned,{

        let TABLE: TableDefinition<&str, String> = TableDefinition::new(table_name);
        // check if wallet exists 
        
        if !Path::new(path).exists() {
            return  Err(Box::from("DB does not exist"));
        }
        // open database 
        let db = match Database::open(path){
            Ok(data)=>{data},
            Err(err)=>{
                error!("{}", err.to_string());
                return  Err(err.into());
            }
        };
        let read_txn = match  db.begin_read(){
            Ok(data)=>{data},
            Err(err)=>{
                error!("error {}", err.to_string());
                return Err(err.into())
            } 
        };
        let table = match read_txn.open_table(TABLE){
            Ok(data)=>{data},
            Err(err)=>{
                error!("error {}", err.to_string());
                return Err(err.into())
            }
        };
        let res_data=match  table.get(key) {
            Ok(data)=>{
                match data {
                    Some(data)=>{data.value().to_owned()},
                    None=>{return Err(Box::from("No data"))}
                }
                },
            Err(err)=>{
                error!("error {}", err.to_string());
                return Err(err.into())
            }  
        };
        // debug!("{}", res_data.value());

       
        // convert db string data 
        let data = match serde_json::from_str::<T>(&res_data){
            Ok(data)=>{data},
            Err(err)=>{
                error!("{}", err.to_string());
                return Err(err.into());
            }
        };

        return Ok(data)
    }
     
}


