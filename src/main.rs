extern crate core;

use std::env;
use std::str::FromStr;
use std::sync::Arc;
use actix_web::{rt, get, web, App, HttpServer, Responder, post, HttpResponse};
use actix_web::web::{Data, resource, route, service, ServiceConfig};

use log::{debug, error, info, LevelFilter};
use log4rs::append::console::ConsoleAppender;
use log4rs::{Config, Handle};
use log4rs::config::{Appender, Root};

mod controllers;
use controllers::{
    wallet_controller

};
mod models;

use models::{response};
mod blockchain;
use blockchain::wallet;
use sha2::digest::consts::U256;
use crate::blockchain::kv_store::KvStore;
use crate::blockchain::node::Node;
use crate::blockchain::wallet::Wallet;
use crate::models::block::{Block, Chain};



mod utils;


use std::thread;
use actix_web::dev::Server;
use bigdecimal::BigDecimal;
use dotenv::dotenv;

use hex_literal::hex;
use k256::ecdsa::{Signature, VerifyingKey};
use k256::ecdsa::signature::Verifier;
use once_cell::sync::Lazy;
use sha2::{Digest, Sha256};
use crate::controllers::wallet_controller::{create_wallet, get_account, get_balance, transfer, verify_account};

use crate::models::constants::{ACCOUNTS_TABLE, BLOCKS_TABLE, META_DATA_TABLE, TRANSACTIONS_LOG_TABLE};

use crate::models::mempool::Mempool;
use crate::models::request::HttpMessage;
use crate::utils::app_error::AppError;

#[get("/{name}")]
async fn hello(name: web::Path<String>) -> impl Responder {
    format!("Hello {:?}!", &name)
}

// std::io::Result<()>


#[actix_web::main]
async fn main()-> std::io::Result<()>  {
    env::set_var("RUST_BACKTRACE", "full");
    //
    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
    info!("Starting server..");
    dotenv::dotenv().ok();

    // Node::serve().await;
    let sled_db = match sled::open("blockchain_db")
        .map_err(|e| {
            error!("Failed to open sled database: {}", e);
            AppError::CreateDatabaseError(e.to_string())
        }){
        Ok(db) => db,
        Err(err)=>{
            error!("{}", err);
            panic!("Failed to open sled database: {}", err);
        }
    };
    let mempool = Arc::new(tokio::sync::Mutex::new(Mempool::new()));
    let db = Arc::new(sled_db);
    let port: u16 = APP_CONFIG.port.to_owned();
    let address = ("0.0.0.0", port);
    HttpServer::new(move|| {
        App::new()
            .app_data(Data::new(db.clone()))
            .configure(configure_services)
    })
        .bind(address)?
        .run()
        .await
}

fn configure_services(cfg: &mut ServiceConfig) {
    cfg
        .service(
            web::scope("/wallet")
                .service(create_wallet)
                .service(transfer)
                .service(get_account)
                .service(get_balance)
                .service(verify_account)
        )
        .service(index)
    ;
}

pub struct AppConfig {
    pub port: u16,
    pub tcp_address: String,
    pub version: BigDecimal
}

pub static APP_CONFIG: Lazy<AppConfig> = Lazy::new(|| {
    dotenv().ok(); 
    AppConfig {
        port: env::var("PORT")
            .unwrap_or_else(|_| "3000".into())
            .parse()
            .expect("Invalid SERVER_PORT"),
        tcp_address: "".to_string(),
        version: BigDecimal::from_str("0.2").unwrap_or_default()
    }
});

#[get("/hello")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello bread!")
}