use std::collections::HashMap;
use std::env;
use std::string::ToString;
use env_file_reader::read_file;
use log::error;

pub struct Env  {
    pub env_vars:HashMap<String,String>
}
pub fn init_env(){
    // get environment vars
    let env_vars = match read_file(".env"){
        Ok(vars)=>{vars},
        Err(err)=>{
            error!("{}", err);
            return;
        }
    };
}

// pub fn get_env(){

// }

pub fn get_env(key:&str)->String{
    let data = match env::var(key){
        Ok(data)=>{data},
        Err(err)=>{
            error!("{}",err);
            "".to_string()
        }
    };

    return data;
}