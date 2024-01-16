use std::borrow::Borrow;
use std::error::Error;
use std::fs;
use std::fs::File;
use std::io::Write;
use serde;
use serde::Serialize;


use std::env::current_dir;
use std::path::Path;

pub struct KvStore {
    db_name:String,
    db_path:String,

}

impl KvStore {



    pub fn create_db(db_name:String,  path:String) -> Result<KvStore, Box<dyn Error>>{
        let real_path = format!("{}{}",current_dir().unwrap_or_default().to_str().unwrap_or_default(),path);
        // chekc if file exists
        // match fs::metadata(real_path.clone()){
        //     Ok(_)=>{},
        //     Err(err)=>{return Err(err.into())}
        // }
        // create new path

        println!("{}",real_path);
        if !Path::new(real_path.as_str()).exists() {
            let folder = fs::create_dir_all(real_path.as_str());
            match folder {
                Ok(folder)=>folder,
                Err(err)=>{
                    println!("{}", err.to_string());
                    return Err(err.into())
                }
            }

            let db_path = format!("{}{}{}", real_path, db_name,".bin");
            match File::create(db_path.as_str()){
                Ok(_)=>{},
                Err(err)=>{return Err(err.into())}
            };

        }else{
            println!("{}", "Path exists".to_string())
        }
        // let file = File::create(real_path);
        // match file {
        //     Ok(file) => { file },
        //     Err(err) => { return Err(err.into()) }
        // };

        return Ok(KvStore { db_name: db_name, db_path: path })
    }

    pub fn save<T:Serialize>(self, data:Option<T>)->Result<(), Box<dyn Error>>{
        let real_path = format!("{}{}{}{}",current_dir().unwrap_or_default().to_str().unwrap_or_default(),self.db_path,self.db_name, ".bin");
        //let chain = Chain{ chain: vec![] };


        let data = match data {
            Some(data)=>{data},
            None=>{return Err("Empty Data".into()) }
        };

        let data_string =match serde_json::to_string(data.borrow()){
            Ok(data_string) => {data_string}
            Err(err) => {return Err(err.into()) }
        };
        let file = File::options().write(true).open(real_path);
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

    pub fn get(){

    }


}