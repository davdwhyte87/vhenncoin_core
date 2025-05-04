use log::debug;
use log::error;
use serde::Serialize;
use crate::utils::app_error::AppError;

pub struct Struct_H{

}

impl Struct_H {
    pub fn vec_to_string<T:Serialize>(data: Vec<T>)->String{
        let resp_string:String = match serde_json::to_string(&data){
            Ok(str)=>{str},
            Err(r)=>{
                error!("{}",r.to_string());
                "".to_string()
            }
        };
        
        debug!("response : {}", resp_string);
        return resp_string;
    }

    pub fn struct_to_string<T:Serialize>(data:&T)->String{
        let resp_string:String = match serde_json::to_string(data){
            Ok(str)=>{str},
            Err(r)=>{
                error!("{}",r.to_string());
                "".to_string()
            }
        }; 
        return resp_string;
    }

    pub fn struct_to_string2<T:Serialize>(data:&T)->Result<String,AppError>{
        let resp_string:String = match serde_json::to_string(data){
            Ok(str)=>{str},
            Err(r)=>{
                error!("{}",r.to_string());
                return Err(AppError::SerializationError(r.to_string()));
            }
        };
        return Ok(resp_string);
    }

   
}