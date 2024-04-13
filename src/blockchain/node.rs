use std::borrow::Borrow;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs::File;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use chrono::format::StrftimeItems;
use futures::executor::block_on;
use futures::{FutureExt, TryFutureExt};
use lettre::transport::smtp::commands::Data;
use log::{debug, error, info};

use std::env::{self, current_dir};
use std::str::FromStr;
use actix_web::{App, HttpServer};
use actix_web::dev::Server;
use hex_literal::len;
use itertools::Itertools;
use rand::Rng;
use serde_json::to_string;
use crate::blockchain::broadcast::{get_node_list_http, get_node_list_net, get_servers, save_server_list};
use crate::controllers::wallet_controller::create_wallet;
use crate::handlers::handlers::Handler;
use crate::models::server_list::ServerData;

use super::broadcast::notify_new_node_http;


pub struct Node {

}

impl Node {
    // #[actix_web::main]
    pub async fn server_http() -> Server {
        let http_port = match env::var("PORT"){
            Ok(data)=>{data},
            Err(err)=>{
                error!("{}",err);
                "8000".to_string()
            }
        };
        debug!("port number  {}", http_port);
        HttpServer::new(|| {
            App::new()
                .service(create_wallet)
        })
            .bind(("127.0.0.1", u16::from_str(http_port.as_str()).unwrap()))
            .unwrap()
            .run()
    }


    pub fn serve(){
        let port = match env::var("PORT"){
            Ok(data)=>{data},
            Err(err)=>{
                error!("{}",err);
                "8000".to_string()
            }
        };
        let address =format!("{}{}","127.0.0.1:", port);
        let listener = TcpListener::bind(address.to_owned()).unwrap();
        info!("Server running {}", address);
        for stream in listener.incoming() {
            let mut stream = stream.unwrap();

             Node::handle_connection(stream);
        }
    }
    pub fn handle_connection(mut stream: TcpStream) {
        let mut buffer = [0; 512];

        stream.borrow().read(&mut buffer).unwrap();

        let data = String::from_utf8_lossy(&buffer).to_string();
        debug!("Request Data : {}", data );

        let data_set :Vec<&str>= data.split("\n").collect();
        debug!("{}", data_set[0]);

        match data_set[0]{

            "CreateWallet" =>{
               debug!("Create wallet now");
                Handler::create_wallet(&data_set[1].to_string(), &mut Some(stream));
            },
            "Transfer"=>{
              Handler::transfer(data_set[1].to_string(), &mut Some(stream));
            },

            _ => {}
        }
        let response = "HTTP/1.1 200 OK\r\n\r\n";

        //stream.write(response.as_bytes()).unwrap();
        //stream.flush().unwrap();
    }



    // talks to other nodes and gets their node list, compares and establishes a truthful list
    // discover other nodes in the network
    pub async fn discover()->Result<(), Box<dyn Error>>{
        info!("Starting node discovery");
        
        let mut rough_node_list:Vec<ServerData> = vec![];

        let servers = match  get_servers() {
            Ok(data)=>{data},
            Err(err)=>{
                error!("{}", err);
                return Err(err.into())
            }
        };

        // sample random 20% of the network
        let max = servers.len();
        let number_of_rolls = (20/100)*max;

        let mut i = 0;

        // fetch server list of each initial node
        for node in servers {
            // randomly pick 1 out of max number of rolls times from the max bucket
            // let node_index = rand::thread_rng().gen_range(0..max);
            // let node =match servers.get(node_index) {
            //     Some(node)=>{node},
            //     None=>{ continue;}
            // };

            // get all the node list in this node
            // let c_server_list=  block_on(async {get_node_list_http(&node).await});
            let c_server_list = get_node_list_http(&node).await;
            let c_server_list =match c_server_list {
                Ok(cs)=>{cs},
                Err(err)=>{
                    error!("{}", err);
                    return Err(err.into())
                }
            };

            //add each item in the remote server list the rough list
            for s in c_server_list{
                rough_node_list.push(s);
            }

            i = i+1;
        }


        // second level iteration. Going over all sync nodes childeren nodes
        for node in rough_node_list.to_owned() {

            // get all the node list in this node
            // let c_server_list=  block_on(async {get_node_list_http(&node).await});
            let c_server_list = get_node_list_http(&node).await;
            let c_server_list =match c_server_list {
                Ok(cs)=>{cs},
                Err(err)=>{
                    error!("{}", err);
                    vec![]
                    //return Err(err.into())
                }
            };

            //add each item in the remote server list the rough list
            for s in c_server_list{
                rough_node_list.push(s);
            }

            i = i+1;
        }
        
        // sort the rough list for unique enteries
        //rough_node_list.sort();
        let m: Vec<ServerData>= rough_node_list.into_iter().unique()
        .map(|servers| servers)
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();

    debug!("sorted final data .. {:?}", m);

    // write the discovered nodes into the serverlist 
    let data_string:String = serde_json::json!(m).to_string();

    match save_server_list(data_string){
        Ok(_)=>{},
        Err(err)=>{
            error!("{}", err)
        }
    };
    // {
        // Ok(data_string) => {data_string}
        // Err(err) => {return Err(err.into()) }
    // };

    // let data_path: String = format!("{}{}",current_dir().unwrap_or_default().to_str().unwrap_or_default(), "/server_list.json");
    // debug!("serverlist file path {}",data_path);
    // let file = File::options().write(true).open(data_path);
    // let mut file =match file {
    //     Ok(file) => { file },
    //     Err(err) => { return Err(err.into()) }
    // };
    // let write_ok = file.write_all(data_string.as_bytes());
    // let write_ok = match write_ok{
    //     Ok(write_ok)=>{write_ok},
    //     Err(err) => { return Err(err.into()) }
    // };


    Ok(())

    }



    // this helps servers in this nodes server list know about it. 
    pub async fn  notify_servers_of_new_node()->Result<(), Box<dyn Error>>{
        debug!("{}","Starting notify servers of new node .....");
        let servers = match get_servers(){
            Ok(data)=>{data},
            Err(err)=>{
                error!("{}", err);
                vec![]
            }
        };
        // get node address
        let http_address = match env::var("HTTP_ADDRESS"){
            Ok(data)=>{data},
            Err(err)=>{
                error!("{}",err);
                "8000".to_string()
            }
        };
        let new_node = ServerData{ 
            id: "".to_string(),
             ip_address: "".to_string(),
              public_key: "".to_string(), 
              http_address: http_address
             };

        // we send all servers in the list a notification of this new node
        // we do not care much what the response is. maybe later we can cache failed requests and try again..
        for server in  servers{
            let r = notify_new_node_http(&server, &new_node).await;
        }
        Ok(())
    }

}