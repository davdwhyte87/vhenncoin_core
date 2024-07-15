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
use futures_util::future::join_all;
use crate::handlers::handlers::Handler;
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



fn main() {
    // test_dd();
//    utils::test::zip();
//    return;


    env::set_var("RUST_BACKTRACE", "full");


    let stdout = ConsoleAppender::builder().build();
    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .build(Root::builder().appender("stdout").build(LevelFilter::Debug))
        .unwrap();
    let _handle = log4rs::init_config(config).unwrap();
    info!("Starting server..");

    dotenv::dotenv().ok();

    if get_env("TCP_ADDRESS") == ""{
        error!("IP address not configured");
        return ;
    }

    match Node::discover_c() {
        Ok(_)=>{},
        Err(err)=>{
            debug!("{}", err.to_string());
        }
    }
    match Node::notifiy_network_new_node(){
        Ok(_)=>{}, 
        Err(err)=>{
            debug!("{}", err.to_string()); 
        }
    }
    Node::serve();

}


//#[actix_web::main]
#[tokio::main]
async fn start_http_server()  ->Server{
    debug!("Stat http func works");
    let http_port = match env::var("HTTP_PORT"){
        Ok(data)=>{data},
        Err(err)=>{
            error!("{}",err);
            "8000".to_string()
        }
    };
    debug!("port number  {}", http_port);
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(route_to_tcp)

    })
        .bind(("127.0.0.1", u16::from_str(http_port.as_str()).unwrap()))
        .unwrap()
        .run()



    //let srv_handle = srv.handle();
    //rt::spawn(srv);
    //info!("running http server on localhost:{}", http_port);

    //srv_handle.stop(false).await;

}

#[post("/send_message")]
async fn route_to_tcp(req: String) -> String {
    let message = req;
    debug!("{}", message.to_owned());
    let data = message;
    debug!("Request Data : {}", data );

    let data_set :Vec<&str>= data.split(r"\n").collect();
    debug!("raw action name {}", data_set[0]);

    let mut response = String::new();
    let action_name = data_set.get(0);
    let action_name = match action_name {
        Some(data)=>{data},
        None =>{
            return format!("0{}{}",r"\n","request data error. No action name");
        }
    };
    let is_broadcasted = match data_set.get(4){
        Some(data)=>{data.to_string()},
        None =>{
            return format!("0{}{}",r"\n","request data error. No is broadcasted");
        } 
    };

    let message = match data_set.get(1){
        Some(data)=>{data.to_string()},
        None =>{
            return format!("0{}{}",r"\n","request data error. No message");
        }   
    };

    debug!("action name {}", action_name);
    debug!(" is broadcasted {}", is_broadcasted);
    match *action_name{

        "CreateWallet" =>{
            debug!("Create wallet now");
            response = Handler::create_wallet(&data_set[1].to_string(), &mut None, is_broadcasted.clone());
            if is_broadcasted == "0" {
                debug!("broadcasting ");
                broadcast_request_http("CreateWallet".to_string(),data_set[1].to_string()).await
            }
        },
        "Transfer"=>{
            response = Handler::transfer(message.clone(), &mut None, is_broadcasted.clone());
            if is_broadcasted == "0" {
                debug!("broadcasting ");
                broadcast_request_http("Transfer".to_string(),message).await
            }
        },
        "GetBalance"=>{
            response = Handler::get_balalnce(message.clone(), &mut None).await;
        },
        "GetNodeBalance"=>{
            response = Handler::get_node_balalnce(message.clone()).await;
        },
        "GetNodeList"=>{
            // get all server nodes
            debug!("Handling node request");
            response = Handler::get_servers();
            debug!("{}", response);
        },
        "AddNode"=>{
            response = Handler::add_node(data_set[1].to_string());
        },
        "GetNodeWalletList"=>{
            response = Handler::get_node_wallet_list().await;
        },
        "GetWalletData"=>{
            response = Handler::get_single_wallet(message).await
        },

        _ => {}
    }
    
    response
}