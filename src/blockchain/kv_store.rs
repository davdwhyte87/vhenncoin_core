use std::borrow::Borrow;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::{Read, Write};
use serde;



use std::env::current_dir;
use std::path::Path;
use log::{debug, error};
use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;

use crate::models::block::{Block, Chain};


pub struct KvStore {
    db_address:String,
    db_name:String,

}

impl KvStore {



    pub fn create_db(address:String, db_name:String) -> Result<(), Box<dyn Error>>{
        let wallet_path = format!("{}{}{}",r"\data\",address,r"\");
        let db_path = format!("{}{}{}{}",current_dir().unwrap_or_default().to_str().unwrap_or_default(),wallet_path,db_name, ".bin");
        let real_wallet_path = format!("{}{}",current_dir().unwrap_or_default().to_str().unwrap_or_default(),wallet_path);
        // chekc if file exists
        // match fs::metadata(real_path.clone()){
        //     Ok(_)=>{},
        //     Err(err)=>{return Err(err.into())}
        // }
        // create new path

        println!("wallet path {}",wallet_path);
        println!("db path {}",db_path);
        println!("real wallet path {}", real_wallet_path);
        if !Path::new(real_wallet_path.as_str()).exists() {
            let folder = fs::create_dir_all(real_wallet_path.as_str());
            match folder {
                Ok(folder)=>folder,
                Err(err)=>{
                    println!("{}", err.to_string());
                    return Err(err.into())
                }
            }


            match File::create(db_path.as_str()){
                Ok(_)=>{},
                Err(err)=>{return Err(err.into())}
            };

        }else{
            println!("{}", "Path exists".to_string());
            return Err(Box::try_from("Error, wallet exists".to_string()).unwrap());

        } 
        // let file = File::create(real_path);
        // match file {
        //     Ok(file) => { file },
        //     Err(err) => { return Err(err.into()) }
        // };

        return Ok(())
    }

    pub fn save<T:Serialize>(address:String, db_name:String, data:Option<T>)->Result<(), Box<dyn Error>>{

        let wallet_path = format!("{}{}{}",r"\data\",address,r"\");
        let db_path = format!("{}{}{}{}",current_dir().unwrap_or_default().to_str().unwrap_or_default(),wallet_path,db_name, ".bin");

        //let chain = Chain{ chain: vec![] };


        let data = match data {
            Some(data)=>{data},
            None=>{return Err("Empty Data".into()) }
        };

        let data_string =match serde_json::to_string(data.borrow()){
            Ok(data_string) => {data_string}
            Err(err) => {return Err(err.into()) }
        };
        let file = File::options().write(true).open(db_path);
        let mut file =match file {
            Ok(file) => { file },
            Err(err) => { return Err(err.into()) }
        };

        let write_ok = file.write_all(data_string.as_bytes());
        let write_ok = match write_ok{
            Ok(write_ok)=>{write_ok},
            Err(err) => { return Err(err.into()) }
        };
        return Ok(());
    }

    // pub fn init(address:String,db_name:String ) ->KvStore{
    //     let wallet_path = format!("{}{}{}",r"\data\",address,r"\");
    //     let db_path = format!("{}{}{}{}",current_dir().unwrap_or_default().to_str().unwrap_or_default(),wallet_path,db_name, ".bin");
    //     let real_wallet_path = format!("{}{}",current_dir().unwrap_or_default().to_str().unwrap_or_default(),wallet_path);
    //
    //     return KvStore{
    //         db_address: address,
    //         db_name,
    //     }
    // }
    
    pub fn get<T: DeserializeOwned>(address:String, db_name:String) ->Result<T, Box<dyn Error>>{
        let wallet_path = format!("{}{}{}",r"\data\",address,r"\");
        let db_path = format!("{}{}{}{}",current_dir().unwrap_or_default().to_str().unwrap_or_default(),wallet_path,db_name, ".bin");
        let real_wallet_path = format!("{}{}",current_dir().unwrap_or_default().to_str().unwrap_or_default(),wallet_path);
        // read from disk
        let mut file =match  File::open(db_path.clone()){
            Ok(file)=>{file},
            Err(err)=>{
                error!("error opening file {}",err.to_string());
                return Err(err.into())
            }
        };
        let mut content = String::new();

        match file.read_to_string(&mut content){
            Ok(_)=>{},
            Err(err)=>{

                error!(" error reading file {}",err.to_string());
                return Err(err.into())
            }
        }
        debug!("db path {}", db_path.clone());
        debug!("file content {}", content);
        let m = content.clone();




        // decode data
        let chain: T = match  serde_json::from_str::<T>(content.as_str()) {
            Ok(data)=>{data},
            Err(err)=>{
                error!("error parsing data {}",err.to_string());
                return Err(err.into())
            }
        };
        return  Ok(chain)

    }


}