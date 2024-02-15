use std::borrow::Borrow;
use std::fs;
use std::fs::File;
use std::path::Path;
use actix_web::{get, HttpResponse, post};
use actix_web::dev::ResourcePath;
use actix_web::web::{Data, Json, JsonBody, ReqData};

use validator::Validate;

use crate::req_models::wallet_requests::CreateWalletReq;



const ROUTE_NAME: &str = "wallet";

#[post("wallet/create")]
pub async fn create_wallet(req_data :Json<CreateWalletReq>) ->HttpResponse
{
    // lora rey
    // validate request
    // match req_data.validate() {
    //     Ok(_) => {},
    //     Err(err) => {
    //         return HttpResponse::BadRequest().json(err);
    //     }
    // }
    //
    // // check if wallet exists
    // let dp:&str = "/data/";
    // let data_path = format!("{}{}",dp,req_data.private_key.to_owned());
    // if !Path::new(data_path.path()).exists() {
    //     let folder = fs::create_dir_all(data_path.path());
    //     match folder {
    //         Ok(folder)=>folder,
    //         Err(err)=>{
    //             return HttpResponse::InternalServerError().json(Response{message:err.to_string()});
    //         }
    //     }
    //
    // }else{
    //     return HttpResponse::BadRequest().json(Response{
    //         message: "Wallet already exists".to_string()
    //     });
    // }
    //
    // // create necessary files
    // let bin_path = format!("{}{}", data_path, "/data.bin");
    // let file = File::create(bin_path.path());
    // match file {
    //     Ok(_)=>{},
    //     Err(err)=>{
    //         return HttpResponse::InternalServerError().json(Response{message:err.to_string()})
    //     }
    // }
    return HttpResponse::Ok().body("");
}

