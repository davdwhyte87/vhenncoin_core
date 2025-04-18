extern crate core;

use std::env;
use std::str::FromStr;
use actix_web::{rt, get, web, App, HttpServer, Responder, post};
use actix_web::web::{Data, resource, route, service};

use log::{debug, error, info, LevelFilter};
use log4rs::append::console::ConsoleAppender;
use log4rs::{Config, Handle};
use log4rs::config::{Appender, Root};

mod controllers;
use controllers::{
    wallet_controller

};
mod models;
mod handlers;
use models::{response};
mod blockchain;
use blockchain::wallet;
use blockchain::transfer;
use sha2::digest::consts::U256;
use utils::env::get_env;
use crate::blockchain::broadcast::{broadcast_request_http, get_servers};
use crate::blockchain::kv_store::KvStore;
use crate::blockchain::mongo_store::WalletService;
use crate::blockchain::node::Node;
use crate::blockchain::wallet::Wallet;
use crate::models::block::{Block, Chain};
use crate::req_models::wallet_requests::CreateWalletReq;
use crate::utils::test::test_dd;

mod utils;
mod req_models;
mod middlewares;


use env_file_reader::read_file;
use std::thread;
use actix_web::dev::Server;
use bigdecimal::BigDecimal;
use dotenv::dotenv;
use futures_util::future::join_all;
use once_cell::sync::Lazy;
use redb::{Database, TableDefinition};
use crate::handlers::handlers::Handler;
use crate::models::constants::{ACCOUNTS_TABLE, BLOCKS_TABLE, META_DATA_TABLE, TRANSACTIONS_LOG_TABLE};
use crate::models::db::MongoService;
use crate::models::request::HttpMessage;


#[get("/")]
async fn index() -> impl Responder {
    "Hello, Bread!"
}

#[get("/{name}")]
async fn hello(name: web::Path<String>) -> impl Responder {
    format!("Hello {:?}!", &name)
}

// std::io::Result<()>


#[tokio::main]
async fn main() {
    // test_dd();
   // utils::test::zip();
//    match Wallet::seed_gen_keys("wet_whitej***"){
//     Ok(_)=>{},
//     Err(err)=>{println!("{}", err.to_string())}
//    }

//    return;
    env::set_var("RUST_BACKTRACE", "full");

    log4rs::init_file("log4rs.yaml", Default::default()).unwrap();
    info!("Starting server..");

    dotenv::dotenv().ok();

    if get_env("TCP_ADDRESS") == ""{
        error!("IP address not configured");
        return ;
    }

    match Node::setup_digital_id_folders(){
        Ok(_)=>{},
        Err(err)=>{
            error!("error setting up id folders{}", err.to_string());
            //panic!()
            return;
        }
    }

    // match Node::discover_c() {
    //     Ok(_)=>{},
    //     Err(err)=>{
    //         debug!("{}", err.to_string());
    //     }
    // }
    // match Node::notifiy_network_new_node(){
    //     Ok(_)=>{}, 
    //     Err(err)=>{
    //         debug!("{}", err.to_string()); 
    //     }
    // }
    Node::serve().await;

}


fn create_database()->Result<Database, Box<dyn std::error::Error>>{
    let path = format!("data/db.redb") ;
    let db =match  Database::create(path){
        Ok(data)=>{data},
        Err(err)=>{
            error!("error creating db {}", err.to_string());
            return Err(err.into())
        }
    };

    let METADATA: TableDefinition<&str, String> = TableDefinition::new(META_DATA_TABLE);
    let BLOCKS: TableDefinition<&str, String> = TableDefinition::new(BLOCKS_TABLE);
    let ACCOUNTS: TableDefinition<&str, String> = TableDefinition::new(ACCOUNTS_TABLE);
    let TRANSACTIONS_LOG: TableDefinition<&str, String> = TableDefinition::new(TRANSACTIONS_LOG_TABLE);

    let write_txn = db.begin_write()?;
    {
        write_txn.open_table(METADATA)?;
        write_txn.open_table(BLOCKS)?;
        write_txn.open_table(ACCOUNTS)?;
        write_txn.open_table(TRANSACTIONS_LOG)?;
    }
    write_txn.commit()?;
    
    Ok(db)
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