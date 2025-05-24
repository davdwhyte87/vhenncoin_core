use std::{clone, error::Error, path::Path};

use log::{debug, error};

use serde::{de::{self, DeserializeOwned}, Deserialize};
use crate::models::block::VBlock;

pub struct KVService{

}

// impl KVService {
//     pub fn save(
//        db: &Database,
//         table_name:&str,
//         key:String,
//         value:String
//     )->Result<(), Box<dyn Error>>{
//
//         // let path = format!("id_data/user_data.redb");
//
//         let TABLE: TableDefinition<&str, String> = TableDefinition::new(table_name);
//
//         let write_txn =match  db.begin_write(){
//             Ok(data)=>{data},
//             Err(err)=>{
//                 error!("error {}", err.to_string());
//                 return Err(err.into())
//             }
//         };
//
//         {
//             let mut table = match write_txn.open_table(TABLE){
//                 Ok(data)=>{data},
//                 Err(err)=>{
//                     error!("error opening table  {}", err.to_string());
//                     return Err(err.into())
//                 }
//             };
//             match table.insert(&*key, value){
//                 Ok(_)=>{},
//                 Err(err)=>{
//                     error!("error inserting data {}", err.to_string());
//                     return Err(err.into())
//                 }
//             };
//         }
//
//         let _commit_res = match write_txn.commit(){
//             Ok(data)=>{data},
//             Err(err)=>{
//                 error!("commit error {}", err.to_string());
//                 return Err(err.into())
//             }
//         };
//
//         return Ok(())
//     }
//
//
//
//     pub fn get_data<T>(
//         db:&Database,
//         table_name:&str,
//         key:&str,
//     )->Result<Option<T>, Box<dyn Error>>where T: DeserializeOwned,{
//
//         let TABLE: TableDefinition<&str, String> = TableDefinition::new(table_name);
//         // check if wallet exists
//
//
//         let read_txn = match  db.begin_read(){
//             Ok(data)=>{data},
//             Err(err)=>{
//                 error!("error {}", err.to_string());
//                 return Err(err.into())
//             }
//         };
//         let table = match read_txn.open_table(TABLE){
//             Ok(data)=>{data},
//             Err(err)=>{
//                 error!("error {}", err.to_string());
//                 return Err(err.into())
//             }
//         };
//         let res_data=match  table.get(key) {
//             Ok(data)=>{
//                 match data {
//                     Some(data)=>{data.value().to_owned()},
//
//                     None=>{
//                         debug!("data not found {}" ,key);
//                         return Ok(None)
//                     }
//                 }
//                 },
//             Err(err)=>{
//                 error!("error {}", err.to_string());
//                 return Err(err.into())
//             }
//         };
//         // debug!("{}", res_data.value());
//
//
//         // convert db string data
//         let data = match serde_json::from_str::<T>(&res_data){
//             Ok(data)=>{data},
//             Err(err)=>{
//                 error!("{}", err.to_string());
//                 return Err(err.into());
//             }
//         };
//
//         return Ok(Some(data));
//     }
//
//
//
//     pub fn get_all_data<T>(
//         db:&Database,
//         table_name:&str,
//     )->Result<Vec<T>, Box<dyn Error>>where T: DeserializeOwned,{
//
//         let TABLE: TableDefinition<&str, String> = TableDefinition::new(table_name);
//         // check if wallet exists
//
//
//         let read_txn = match  db.begin_read(){
//             Ok(data)=>{data},
//             Err(err)=>{
//                 error!("error {}", err.to_string());
//                 return Err(err.into())
//             }
//         };
//         let table = match read_txn.open_table(TABLE){
//             Ok(data)=>{data},
//             Err(err)=>{
//                 error!("error {}", err.to_string());
//                 return Err(err.into())
//             }
//         };
//
//         let mut blocks:Vec<T> = vec![];
//         for result in table.iter()?{
//             let (height, value) = result?;
//             let block: T = match serde_json::from_str(&value.value()){
//                 Ok(block)=>{block},
//                 Err(err)=>{
//                     continue;
//                 }
//             };
//             blocks.push(block);
//         }
//         return Ok(blocks);
//     }
//
// }


