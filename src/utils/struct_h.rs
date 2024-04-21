use log::debug;
use log::error;
use serde::Serialize;


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
}