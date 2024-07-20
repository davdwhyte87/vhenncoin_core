use core::num;
use std::borrow::Borrow;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fs::File;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use std::path::Path;
use std::thread::{self, Thread};
use std::time::Duration;
use awc::error;
use bigdecimal::num_traits::float;
use bigdecimal::ToPrimitive;
use chrono::format::StrftimeItems;
use futures::executor::block_on;
use futures::{FutureExt, TryFutureExt};
use lettre::transport::smtp::commands::Data;
use log::{debug, error, info};
use log4rs::Handle;
use tokio::net::tcp;
use walkdir::WalkDir;
use zip::write::SimpleFileOptions;
use zip::ZipWriter;

use std::env::{self, current_dir};
use std::str::FromStr;
use actix_web::{App, HttpServer};
use actix_web::dev::Server;
use hex_literal::len;
use itertools::Itertools;
use rand::Rng;
use serde_json::to_string;
use crate::blockchain::broadcast::{broadcast_request_tcp, get_node_list_c, get_node_list_http, get_node_list_net, get_node_wallet_list_C, get_servers, save_server_list};
use crate::blockchain::wallet::Wallet;
use crate::controllers::wallet_controller::create_wallet;
use crate::handlers::handlers::Handler;
use crate::models::server_list::ServerData;
use crate::models::wallet::MongoWallet;
use crate::utils::constants;
use crate::utils::env::get_env;
use crate::utils::formatter::Formatter;
use crate::utils::response::{Response, TCPResponse};

use super::broadcast::{get_node_wallet_list, get_seed_nodes, get_wallet_data, notify_network_new_node_bc, notify_new_node_http};


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

        // let data_set :Vec<&str>= data.split("\n").collect();
        // debug!("{}", data_set[0]);

        let data_set :Vec<&str>= data.split("\n").collect();
        debug!("data piece count {}", data_set.len() );
    
        let mut response = String::new();
        let action_name = data_set.get(0);
        let action_name = match action_name {
            Some(data)=>{data},
            None =>{
               
                
                let res= Formatter::response_formatter(
                    "0".to_string(),
                     "request data error. No action name".to_string(), 
                    "".to_string()
                    );
                TCPResponse::send_response_txt(res, &mut stream);
                return;
            }
        };
        let is_broadcasted = match data_set.get(4){
            Some(data)=>{data.to_string()},
            None =>{
                
                let res=  Formatter::response_formatter(
                    "0".to_string(),
                     "request data error. No is broadcasted".to_string(), 
                    "".to_string()
                    );
                TCPResponse::send_response_txt(res, &mut stream);
                return;
            } 
        };
    
        let message = match data_set.get(1){
            Some(data)=>{data.to_string()},
            None =>{
               
                let res=  Formatter::response_formatter(
                    "0".to_string(),
                     "request data error. No message".to_string(), 
                    "".to_string()
                    );
                TCPResponse::send_response_txt(res, &mut stream);
                return;
            }   
        };
    
        debug!("action name {}", action_name);
        debug!(" is broadcasted {}", is_broadcasted);
        match *action_name{
    
            "CreateWallet" =>{
                // thread::sleep(Duration::from_secs(5));
                debug!("Create wallet now");
                Handler::create_wallet_tcp(&message, &mut stream, is_broadcasted);
            },
            "Transfer"=>{
                Handler::transfer_c(message.clone(), &mut stream, is_broadcasted.clone());
            },
            "GetBalance"=>{
                thread::sleep(Duration::from_secs(5));
               Handler::get_balance_c(message.clone(), &mut stream);
            },
            "GetNodeBalance"=>{
              Handler::get_node_balance_c(message.clone(), &mut stream);
            },
            "GetNodeList"=>{
                // get all server nodes
                debug!("Handling node request");
                Handler::get_servers_c(&mut stream);
                
            },
            "AddNode"=>{
                Handler::add_node_c(&message.clone(), &mut stream);
            },
            "GetNodeWalletList"=>{
               Handler::get_node_wallet_list_c(&mut stream);
            },
            "GetWalletData"=>{
                thread::sleep(Duration::from_secs(1));
                Handler::get_single_wallet_c(message.clone(), &mut stream);
            },
            "GetZipChain"=>{
                Handler::get_chain_zip(&mut stream);
            }
    
            _ => {}
        }
        

        // match data_set[0]{

        //     "CreateWallet" =>{
        //        debug!("Create wallet now");
        //         Handler::create_wallet(&data_set[1].to_string(), &mut Some(stream), "0".to_string());
        //     },
        //     "Transfer"=>{
        //       Handler::transfer(data_set[1].to_string(), &mut Some(stream), "id".to_string());
        //     },

        //     _ => {}
        // }
        let response = "HTTP/1.1 200 OK\r\n\r\n";

        //stream.write(response.as_bytes()).unwrap();
        //stream.flush().unwrap();
    }

    pub fn discover_c()->Result<(), Box<dyn Error>>{
        info!("Starting node discovery");
        
        let mut rough_node_list:Vec<ServerData> = vec![];

        let servers = match  get_servers() {
            Ok(data)=>{data},
            Err(err)=>{
                error!("error getting servers list locally{}", err);
                return Err(err.into())
            }
        };

       

        // fetch server list of each initial node
        for node in servers {
            let c_server_list = get_node_list_c(&node);
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

           
        }


        // second level iteration. Going over all sync nodes childeren nodes
        for node in rough_node_list.to_owned() {

            // get all the node list in this node
            // let c_server_list=  block_on(async {get_node_list_http(&node).await});
            let c_server_list = get_node_list_c(&node);
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

           
        }
        
        // sort the rough list for unique enteries
        //rough_node_list.sort();
        let mut m: Vec<ServerData>= rough_node_list.into_iter().unique()
        .map(|servers| servers)
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();

        debug!("sorted final data .. {:?}", m);

        let mut i = 0;
        let tcp_address = get_env("TCP_ADDRESS");
        debug!("my tcp addres {}", tcp_address );
       
        m.retain(|server| server.ip_address != tcp_address);

        // write the discovered nodes into the serverlist 
        let data_string:String = serde_json::json!(m).to_string();

        match Self::save_server_list(data_string){
            Ok(_)=>{},
            Err(err)=>{
                error!("{}", err)
            }
        };

        Ok(())
    }


    // save data to the server_list.json
    pub fn save_server_list(data:String)->Result<(), Box<dyn Error> >{

        let data_path: String = format!("{}{}",current_dir().unwrap_or_default().to_str().unwrap_or_default(), "/server_list.json");
        debug!("serverlist file path {}",data_path);
    
        let file = File::options().truncate(true).write(true).open(data_path);
        let mut file =match file {
            Ok(file) => { file },
            Err(err) => { 
                error!("error opening data file {}", err.to_string());
                return Err(err.into()) 
            }
        };
    
        let write_ok = file.write_all(data.as_bytes());
        let write_ok = match write_ok{
            Ok(write_ok)=>{write_ok},
            Err(err) => {
                error!("error writing to data file {}", err.to_string());
                 return Err(err.into()) 
                }
        };
    
        
        Ok(())
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

    pub fn notifiy_network_new_node()->Result<(), Box<dyn Error>>{
        let nodes = get_servers();
        let nodes = match nodes {
            Ok(nodes)=>{nodes},
            Err(err)=>{
                error!("get seed nodes error ... {}", err);
                return Err(err.into());
            }
        };
        let new_node = ServerData{
            id:"".to_string(),
            ip_address:get_env("TCP_ADDRESS"),
            public_key:"".to_string(),
            http_address: "".to_string()
        };
        for node in nodes{
            let _ = notify_network_new_node_bc(&node, &new_node);
        }

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


    // connect with other nodes and get wallet data
    pub fn sync_wallets_new_node_c(){

          // get nodes

        debug!("Syncing wallets .....");
        let nodes = get_servers();
        let nodes = match nodes {
            Ok(nodes)=>{nodes},
            Err(err)=>{
                error!("get seed nodes error ... {}", err);
                return;
            }
        };

        let mut wallet_list:Vec<MongoWallet> =vec![];
     // get wallet data of all nodes 
     for node in &nodes {
        let node_wallet_list =  get_node_wallet_list_C(&node);
        let mut node_wallet_list = match node_wallet_list{
         Ok(data)=>{data},
         Err(err)=>{
             error!("error getting wallet list {}", err);
             vec![]
         }
        };

        // create a new wallet on the local server
        for wallet in node_wallet_list{

        //let res =  Wallet::create_wallet_node(address, wallet);
        //debug!("wallet create resp .. {}", res);

        }
        //wallet_list.append(&mut node_wallet_list);
     }
    }


    
    pub async fn sync_wallets_new_node(){
        // get nodes

        debug!("Syncing wallets .....");
        let nodes = get_servers();
        let nodes = match nodes {
            Ok(nodes)=>{nodes},
            Err(err)=>{
                error!("get seed nodes error ... {}", err);
                return;
            }
        };

        // get wallet list for each node 
            // sample random 20% of the network
        // let max = nodes.len().to_f64().unwrap();
        // let number_of_rolls= (20.0/100.0)*max;
        // let mut i = 0.0;


        let mut wallet_list:Vec<MongoWallet> =vec![];

        // get wallet data of all nodes 
        for node in &nodes {
           let node_wallet_list =  get_node_wallet_list(&node).await;
           let mut node_wallet_list = match node_wallet_list{
            Ok(data)=>{data},
            Err(err)=>{
                error!("error getting wallet list {}", err);
                vec![]
            }
           };

           for wallet in node_wallet_list{

           let res =  Handler::create_wallet_node(&wallet).await;
           debug!("wallet create resp .. {}", res);

           }
           //wallet_list.append(&mut node_wallet_list);
        }

        debug!("Fetched wallets .. {:?}", wallet_list);
        
        // make the wallet list unique
        // let final_wallet_list: Vec<MongoWallet>= wallet_list.into_iter().unique()
        // .map(|servers| servers)
        // .collect::<HashSet<_>>()
        // .into_iter()
        // .collect();

        // loop through the final wallet list and get wallet data
        // save the data, pass on if it exists 
        // for node in &nodes{
        //     for address in &final_wallet_list {
        //         let wallet = get_wallet_data(node, address.to_owned()).await;
        //         let wallet = match wallet {
        //             Ok(data)=>{data},
        //             Err(err)=>{
        //                 error!("{}", err);
        //                 MongoWallet::default()
        //             }
        //         };
        //         // result does not matter to us yet
        //         Handler::create_wallet_node(&wallet).await;
        //     }
        // }
        
    }


    // this gets data on a specific wallet, by making request to all the servers 
    // and then coming to a consensus
    fn get_wallet_data(){
        let nodes = get_servers();
        let nodes = match nodes {
            Ok(nodes)=>{nodes},
            Err(err)=>{
                error!("get seed nodes error ... {}", err);
                return;
            }
        };
        for node in nodes{

        }
    }


    // zip the nodes local chain 
    pub fn zipchain(){
        let path = Path::new("data.zip");
        let file = File::create(&path).unwrap();
        let options = SimpleFileOptions::default()
        .compression_method(zip::CompressionMethod::Bzip2)
        .unix_permissions(0o755);
    
        let mut zip = ZipWriter::new(file);
        let mut buffer = Vec::new();
        let data_path: String = format!("{}{}",current_dir().unwrap_or_default().to_str().unwrap_or_default(), "/data");
        for e in WalkDir::new(data_path.clone()).into_iter().filter_map(|e| e.ok()) {
            
           let name = e.path().strip_prefix(&data_path).unwrap().to_string_lossy();
            if e.metadata().unwrap().is_file() {
                
                println!("creating file : {}", name);
                zip.start_file(name, options);
                let mut f = match File::open(e.path()){
                    Ok(data)=>{data},
                    Err(err)=>{
                        println!("error reading file {}", err.to_string());
                        return;
                    }
                };
                
                f.read_to_end(&mut buffer);
                zip.write_all(&buffer);
                buffer.clear();
            }else {
                println!("creating folder : {}",name);
                zip.add_directory(name, options);
            }
        }
    
        zip.finish();
        
    }

}