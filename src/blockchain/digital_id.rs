use std::error::Error;

use futures::future::ok;
use log::error;
use redb::{Database, TableDefinition};
use sha256::digest;

use crate::{models::user_id::{self, UserID}, utils::{self}};

use super::kv_service::KVService;

pub struct  DigitalID{

}

#[derive(strum_macros::Display, Debug)]
pub enum WalletCreationError {
    WalletExists
}
impl Error for WalletCreationError {
    
}
impl DigitalID {

    pub fn create_user(user_name:&str, user_id:UserID)->Result<(), Box<dyn Error>>{
        let path = format!("id_data/user_data.redb");
        let data_string = utils::struct_h::Struct_H::struct_to_string::<UserID>(&user_id);

        let userID = match KVService::get_data::<UserID>(&path, "user_data", user_name){
            Ok(data)=>{data},
            Err(err)=>{
               UserID::default()
            }
        };
        if !userID.id.is_empty(){
            // instead of returning error we want to return ok
            return Err(Box::from(WalletCreationError::WalletExists));
        }



        match KVService::save(path, "user_data", user_name.to_owned(), data_string){
            Ok(_)=>{},
            Err(err)=>{
                error!(" error saving data on disk {}", err.to_string());
                return Err(err.into())
            }
        }
        return Ok(())
    }

    pub fn validate_user(user_name:&str, password:String)->Result<(), Box<dyn Error>>{
        let path = format!("id_data/user_data.redb");
        let userID = match KVService::get_data::<UserID>(&path, "user_data", user_name){
            Ok(data)=>{data},
            Err(err)=>{
                error!(" error getting user data {}", err.to_string());
                return Err(err.into())
            }
        };
    
        let hash = digest(format!("{}", password ));
    
        if userID.password_hash != hash{
            return Err(Box::from("Validation error"));
        }
    
        return  Ok(());
    }


    pub fn get_user(user_name:&str)->Result<UserID, Box<dyn Error>>{
        let path = format!("id_data/user_data.redb");
        let userID = match KVService::get_data::<UserID>(&path, "user_data", user_name){
            Ok(data)=>{data},
            Err(err)=>{
                error!(" error saving data on disk {}", err.to_string());
                return Err(err.into())
            }
        };
    
    
        return  Ok(userID);
    }
      
}





