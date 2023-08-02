extern crate core;

use std::env;
use actix_web::{get, web, App, HttpServer, Responder};
use actix_web::web::{Data, resource, route, service};


mod controllers;
use controllers::{
    wallet_controller

};
mod models;
use models::{response};
mod blockchain;
use blockchain::wallet;
use blockchain::transfer;
use crate::req_models::wallet_requests::CreateWalletReq;

mod utils;
mod req_models;
mod middlewares;





#[get("/")]
async fn index() -> impl Responder {
    "Hello, Bread!"
}

#[get("/{name}")]
async fn hello(name: web::Path<String>) -> impl Responder {
    format!("Hello {:?}!", &name)
}

// std::io::Result<()>
#[actix_web::main]
async fn main() {
    env::set_var("RUST_BACKTRACE", "full");

    // wallet::Wallet::create_wallet("David".to_string());
    // match transfer::Transfer::transfer("David".to_string(), "john".to_string(), 10.0) {
    //     Ok(_)=>{},
    //     Err(err)=>{
    //         println!("{:?}", err)
    //     }
    // }


    match wallet::Wallet::get_balance(&"David".to_string()){
        Ok(data)=>{println!("{:?}", data)},
        Err(err)=>{
            println!("{:?}", err)
        }
    }
    // HttpServer::new(move|| {
    //     App::new()
    //
    //
    //         // USER CONTROLLERS
    //
    //         .service(
    //             // all authenticated endpoints
    //             web::scope("api/v1/")
    //                 .service(wallet_controller::create_wallet)
    //         )
    //         // .service(user_controller::create_user)
    //         // .service(user_controller::login_user)
    //         // .service(power_ups_controller::use_power_up)
    //         // .service(user_controller::get_code)
    //         //
    // })
    //     .bind(("127.0.0.1", 80))?
    //     .run()
    //     .await
}