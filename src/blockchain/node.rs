use std::sync::Arc;
use tokio::{net::TcpListener, select, signal, spawn, sync::{Mutex, Notify}, time, time::{timeout, Duration}};
use log::{info, error, debug};
use anyhow::Result;

use sled::Db;
use tokio::io::{split, AsyncReadExt, BufReader};
use tokio::net::TcpStream;
use tokio_util::codec::{FramedRead, LinesCodec};
use crate::{ APP_CONFIG};
use crate::blockchain::transfer::Transfer;
use crate::handlers::handlers::Handler;
use crate::models::mempool::Mempool;
use crate::models::response::NResponse;
use crate::utils::app_error::AppError;
use crate::utils::response::TCPResponse;

pub struct Node {

}

impl Node {

    pub async fn serve() -> Result<(), AppError> {
        let address = format!("0.0.0.0:{}", APP_CONFIG.port);
        let listener = match TcpListener::bind(&address).await
            .map_err(|e| { error!("bind error: {}", e); AppError::UnexpectedError(e.to_string()) }){
            Ok(l) => l,
            Err(e)=>{
                error!("{:?}", e);
                panic!()
            }
        };
        info!("🚀 Listening on {}", address);

        // Shutdown notifier
        let shutdown = Arc::new(Notify::new());
        {
            let shutdown_signal = shutdown.clone();
            spawn(async move {
                signal::ctrl_c().await.expect("failed to install Ctrl+C handler");
                info!("🔌 Ctrl+C received, shutting down…");
                shutdown_signal.notify_waiters();
            });
        }
        let sled_db = sled::open("blockchain_db")
            .map_err(|e| {
                error!("Failed to open sled database: {}", e);
                AppError::CreateDatabaseError(e.to_string())
            })?;
        let mempool = Arc::new(tokio::sync::Mutex::new(Mempool::new()));
        let db = Arc::new(sled_db);
        
        // spawn miner
        // {
        //     let db_clone     = db.clone();
        //     let mempool_clone = mempool.clone();
        //     tokio::spawn(async move {
        //         // This will loop forever (or until you decide to shut it down)
        //         Self::mine_blocks(&db_clone, mempool_clone).await;
        //     });
        // }

        loop {
            select! {
            // New incoming connection
            conn = listener.accept() => {
                match conn {
                        
                    Ok((stream, addr)) => {
                        info!("📡 Accepted connection from {}", addr);
                        let db_clone = db.clone();
                        let pool_clone = mempool.clone();
                            // let socket = Arc::new(tokio::sync::Mutex::new(stream));
                        // Spawn handling with timeout
                                tokio::spawn(async move {
                            if let Err(err) = Self::handle_connection(stream, pool_clone, db_clone).await {
                                error!("Connection handling error: {:?}", err);
                            }
                            });
                    }
                    Err(e) => error!("❌ Accept error: {}", e),
                }
            },

            // Shutdown signal fired
            _ = shutdown.notified() => {
                info!("🛑 Shutdown notified; leaving accept loop");
                break;
            },
        } // end select
        } // end loop

        info!("👋 Server has shut down gracefully");
        Ok(())
    }
    pub async fn handle_connection(mut stream:TcpStream, mempool: Arc<Mutex<Mempool>>, db: Arc<Db>) ->Result<(),AppError>{
        let mut buffer = [0u8; 512];
        // let mut guard = stream_guard.lock().await;
        // let mut stream = &mut *guard;
        let size = match stream.read(&mut buffer).await{
            Ok(size) if size > 0 => size,
            _ => {
                error!("No data read from stream");
                TCPResponse::send_response_x::<String>(NResponse{
                    status:0,
                    message: "error with data".to_string(),
                    data:None
                }, &mut stream).await;
                return Ok(());
            }
        };
        
        let data = String::from_utf8_lossy(&buffer[..size]).to_string();
        debug!("request data {}", data);
        let message: serde_json::Value = match serde_json::from_str(&data){
            Ok(data )=>{data},
            Err(err)=>{
                log::error!("error {}", err.to_string());
                TCPResponse::send_response_x::<String>(NResponse {
                    status: 0,
                    message: "error decoding request".to_string(),
                    data: None
                }, &mut stream).await;
                return Ok(());
            }
        };
        let action_name = match message["action"].as_str(){
            Some(action)=>{action},
            None=>{
                TCPResponse::send_response_x::<String>(NResponse {
                    status: 0,
                    message: "action is not defined".to_string(),
                    data: None
                }, &mut stream).await;
                return Ok(());
            }
        };
        let message_data = &message["data"].to_string();
        
        match action_name{
            "create_wallet" =>{
                // thread::sleep(Duration::from_secs(5));
                debug!("Create wallet now");
                Handler::create_wallet_tcp(&db, message_data, &mut stream, "0".to_string()).await;
                Ok(())
            },
            "transfer"=>{
                Handler::transfer_c(message_data, &mut stream,"0".to_string(), mempool, &db).await;
                Ok(())
            },
            "get_mempool"=>{
                Handler::get_mempool(mempool, &mut stream).await;
                Ok(())
            },
            "get_account"=>{
                Handler::get_account(&db, message_data, &mut stream).await;
                Ok(())
            },
            "get_balance"=>{
                Handler::get_balance(&db, message_data, &mut stream).await;
                Ok(())
            },
            "get_last_block_height"=>{ 
                Handler::get_last_block_height(&db, &mut stream).await;
                Ok(())
            },
            "get_last_block"=>{
                Handler::get_last_block(&db, &mut stream).await;
                Ok(())
            },
            "get_all_blocks"=>{
               Handler::get_all_blocks(&db, &mut stream).await;
                Ok(())
            },
            "get_user_transactions"=>{
                Handler::get_user_transactions(message_data, &db, &mut stream).await;
                Ok(())
            },
            "verify_wallet"=>{
               Handler::verify_wallet(message_data, &db, &mut stream).await;
                Ok(())
            },
            "hello"=>{
                Handler::hello(&mut stream).await;
                Ok(())
            }
            _ => {
                TCPResponse::send_response_x::<String>(NResponse {
                    status: 0,
                    message: "action not found".to_string(),
                    data: None
                }, &mut stream).await;
                return Ok(());
            }
        }
    }

    // pub async fn mine_blocks(db:&Db, mempool: Arc<Mutex<Mempool>>) {
    //    
    //     let mut interval = time::interval(Duration::from_secs(30));
    //     loop {
    //         interval.tick().await;
    //         log::info!("Minning new block ...............");
    //         let transactions = Transfer::get_all_transactions(mempool.clone()).await;
    //         match Transfer::process_transactions(db, mempool.clone(), transactions.clone()).await{
    //             Ok(_) => {},
    //             Err(err) => {
    //                 log::error!("{}", err.to_string());
    //                 continue;
    //             }
    //         };
    //     }
    // }

    // pub fn discover_c()->Result<(), Box<dyn Error>>{
    //     info!("Starting node discovery");
    //
    //     let mut rough_node_list:Vec<ServerData> = vec![];
    //
    //     let servers = match  get_servers() {
    //         Ok(data)=>{data},
    //         Err(err)=>{
    //             error!("error getting servers list locally{}", err);
    //             return Err(err.into())
    //         }
    //     };
    //
    //
    //
    //     // fetch server list of each initial node
    //     for node in servers {
    //         let c_server_list = get_node_list_c(&node);
    //         let c_server_list =match c_server_list {
    //             Ok(cs)=>{cs},
    //             Err(err)=>{
    //                 error!("{}", err);
    //                 return Err(err.into())
    //             }
    //         };
    //
    //         //add each item in the remote server list the rough list
    //         for s in c_server_list{
    //             rough_node_list.push(s);
    //         }
    //
    //
    //     }
    //
    //
    //     // second level iteration. Going over all sync nodes childeren nodes
    //     for node in rough_node_list.to_owned() {
    //
    //         // get all the node list in this node
    //         // let c_server_list=  block_on(async {get_node_list_http(&node).await});
    //         let c_server_list = get_node_list_c(&node);
    //         let c_server_list =match c_server_list {
    //             Ok(cs)=>{cs},
    //             Err(err)=>{
    //                 error!("{}", err);
    //                 vec![]
    //                 //return Err(err.into())
    //             }
    //         };
    //
    //         //add each item in the remote server list the rough list
    //         for s in c_server_list{
    //             rough_node_list.push(s);
    //         }
    //
    //
    //     }
    //
    //     // sort the rough list for unique enteries
    //     //rough_node_list.sort();
    //     let mut m: Vec<ServerData>= rough_node_list.into_iter().unique()
    //     .map(|servers| servers)
    //     .collect::<HashSet<_>>()
    //     .into_iter()
    //     .collect();
    //
    //     debug!("sorted final data .. {:?}", m);
    //
    //     let mut i = 0;
    //     let tcp_address = get_env("TCP_ADDRESS");
    //     debug!("my tcp addres {}", tcp_address );
    //
    //     m.retain(|server| server.ip_address != tcp_address);
    //
    //     // write the discovered nodes into the serverlist
    //     let data_string:String = serde_json::json!(m).to_string();
    //
    //     match Self::save_server_list(data_string){
    //         Ok(_)=>{},
    //         Err(err)=>{
    //             error!("{}", err)
    //         }
    //     };
    //
    //     Ok(())
    // }


    // save data to the server_list.json
    // pub fn save_server_list(data:String)->Result<(), Box<dyn Error> >{
    //
    //     let data_path: String = format!("{}{}",current_dir().unwrap_or_default().to_str().unwrap_or_default(), "/server_list.json");
    //     debug!("serverlist file path {}",data_path);
    //
    //     let file = File::options().truncate(true).write(true).open(data_path);
    //     let mut file =match file {
    //         Ok(file) => { file },
    //         Err(err) => {
    //             error!("error opening data file {}", err.to_string());
    //             return Err(err.into())
    //         }
    //     };
    //
    //     let write_ok = file.write_all(data.as_bytes());
    //     let write_ok = match write_ok{
    //         Ok(write_ok)=>{write_ok},
    //         Err(err) => {
    //             error!("error writing to data file {}", err.to_string());
    //              return Err(err.into())
    //             }
    //     };
    //
    //
    //     Ok(())
    // }
    // talks to other nodes and gets their node list, compares and establishes a truthful list
    // discover other nodes in the network
    // pub async fn discover()->Result<(), Box<dyn Error>>{
    //     info!("Starting node discovery");
    //
    //     let mut rough_node_list:Vec<ServerData> = vec![];
    //
    //     let servers = match  get_servers() {
    //         Ok(data)=>{data},
    //         Err(err)=>{
    //             error!("{}", err);
    //             return Err(err.into())
    //         }
    //     };
    //
    //     // sample random 20% of the network
    //     let max = servers.len();
    //     let number_of_rolls = (20/100)*max;
    //
    //     let mut i = 0;
    //
    //     // fetch server list of each initial node
    //     for node in servers {
    //         // randomly pick 1 out of max number of rolls times from the max bucket
    //         // let node_index = rand::thread_rng().gen_range(0..max);
    //         // let node =match servers.get(node_index) {
    //         //     Some(node)=>{node},
    //         //     None=>{ continue;}
    //         // };
    //
    //         // get all the node list in this node
    //         // let c_server_list=  block_on(async {get_node_list_http(&node).await});
    //         let c_server_list = get_node_list_http(&node).await;
    //         let c_server_list =match c_server_list {
    //             Ok(cs)=>{cs},
    //             Err(err)=>{
    //                 error!("{}", err);
    //                 return Err(err.into())
    //             }
    //         };
    //
    //         //add each item in the remote server list the rough list
    //         for s in c_server_list{
    //             rough_node_list.push(s);
    //         }
    //
    //         i = i+1;
    //     }
    //
    //
    //     // second level iteration. Going over all sync nodes childeren nodes
    //     for node in rough_node_list.to_owned() {
    //
    //         // get all the node list in this node
    //         // let c_server_list=  block_on(async {get_node_list_http(&node).await});
    //         let c_server_list = get_node_list_http(&node).await;
    //         let c_server_list =match c_server_list {
    //             Ok(cs)=>{cs},
    //             Err(err)=>{
    //                 error!("{}", err);
    //                 vec![]
    //                 //return Err(err.into())
    //             }
    //         };
    //
    //         //add each item in the remote server list the rough list
    //         for s in c_server_list{
    //             rough_node_list.push(s);
    //         }
    //
    //         i = i+1;
    //     }
    //
    //     // sort the rough list for unique enteries
    //     //rough_node_list.sort();
    //     let m: Vec<ServerData>= rough_node_list.into_iter().unique()
    //     .map(|servers| servers)
    //     .collect::<HashSet<_>>()
    //     .into_iter()
    //     .collect();
    //
    // debug!("sorted final data .. {:?}", m);
    //
    // // write the discovered nodes into the serverlist
    // let data_string:String = serde_json::json!(m).to_string();
    //
    // match save_server_list(data_string){
    //     Ok(_)=>{},
    //     Err(err)=>{
    //         error!("{}", err)
    //     }
    // };
    // // {
    //     // Ok(data_string) => {data_string}
    //     // Err(err) => {return Err(err.into()) }
    // // };
    //
    // // let data_path: String = format!("{}{}",current_dir().unwrap_or_default().to_str().unwrap_or_default(), "/server_list.json");
    // // debug!("serverlist file path {}",data_path);
    // // let file = File::options().write(true).open(data_path);
    // // let mut file =match file {
    // //     Ok(file) => { file },
    // //     Err(err) => { return Err(err.into()) }
    // // };
    // // let write_ok = file.write_all(data_string.as_bytes());
    // // let write_ok = match write_ok{
    // //     Ok(write_ok)=>{write_ok},
    // //     Err(err) => { return Err(err.into()) }
    // // };
    //
    //
    // Ok(())
    //
    // }

    // pub fn notifiy_network_new_node()->Result<(), Box<dyn Error>>{
    //     let nodes = get_servers();
    //     let nodes = match nodes {
    //         Ok(nodes)=>{nodes},
    //         Err(err)=>{
    //             error!("get seed nodes error ... {}", err);
    //             return Err(err.into());
    //         }
    //     };
    //     let new_node = ServerData{
    //         id:"".to_string(),
    //         ip_address:get_env("TCP_ADDRESS"),
    //         public_key:"".to_string(),
    //         http_address: "".to_string()
    //     };
    //     for node in nodes{
    //         let _ = notify_network_new_node_bc(&node, &new_node);
    //     }
    //
    //     Ok(())
    //
    // }


    // this helps servers in this nodes server list know about it. 
    // pub async fn  notify_servers_of_new_node()->Result<(), Box<dyn Error>>{
    //     debug!("{}","Starting notify servers of new node .....");
    //     let servers = match get_servers(){
    //         Ok(data)=>{data},
    //         Err(err)=>{
    //             error!("{}", err);
    //             vec![]
    //         }
    //     };
    //     // get node address
    //     let http_address = match env::var("HTTP_ADDRESS"){
    //         Ok(data)=>{data},
    //         Err(err)=>{
    //             error!("{}",err);
    //             "8000".to_string()
    //         }
    //     };
    //     let new_node = ServerData{
    //         id: "".to_string(),
    //          ip_address: "".to_string(),
    //           public_key: "".to_string(),
    //           http_address: http_address
    //          };
    //
    //     // we send all servers in the list a notification of this new node
    //     // we do not care much what the response is. maybe later we can cache failed requests and try again..
    //     for server in  servers{
    //         let r = notify_new_node_http(&server, &new_node).await;
    //     }
    //     Ok(())
    // }


    // connect with other nodes and get wallet data
    // pub fn sync_wallets_new_node_c(){
    //
    //       // get nodes
    //
    //     debug!("Syncing wallets .....");
    //     let nodes = get_servers();
    //     let nodes = match nodes {
    //         Ok(nodes)=>{nodes},
    //         Err(err)=>{
    //             error!("get seed nodes error ... {}", err);
    //             return;
    //         }
    //     };
    //
    //     let mut wallet_list:Vec<MongoWallet> =vec![];
    //  // get wallet data of all nodes
    //  for node in &nodes {
    //     let node_wallet_list =  get_node_wallet_list_C(&node);
    //     let mut node_wallet_list = match node_wallet_list{
    //      Ok(data)=>{data},
    //      Err(err)=>{
    //          error!("error getting wallet list {}", err);
    //          vec![]
    //      }
    //     };
    //
    //     // create a new wallet on the local server
    //     for wallet in node_wallet_list{
    //
    //     //let res =  Wallet::create_wallet_node(address, wallet);
    //     //debug!("wallet create resp .. {}", res);
    //
    //     }
    //     //wallet_list.append(&mut node_wallet_list);
    //  }
    // }


    
    // pub async fn sync_wallets_new_node(){
    //     // get nodes
    //
    //     debug!("Syncing wallets .....");
    //     let nodes = get_servers();
    //     let nodes = match nodes {
    //         Ok(nodes)=>{nodes},
    //         Err(err)=>{
    //             error!("get seed nodes error ... {}", err);
    //             return;
    //         }
    //     };
    //
    //     // get wallet list for each node
    //         // sample random 20% of the network
    //     // let max = nodes.len().to_f64().unwrap();
    //     // let number_of_rolls= (20.0/100.0)*max;
    //     // let mut i = 0.0;
    //
    //
    //     let mut wallet_list:Vec<MongoWallet> =vec![];
    //
    //     // get wallet data of all nodes
    //     for node in &nodes {
    //        let node_wallet_list =  get_node_wallet_list(&node).await;
    //        let mut node_wallet_list = match node_wallet_list{
    //         Ok(data)=>{data},
    //         Err(err)=>{
    //             error!("error getting wallet list {}", err);
    //             vec![]
    //         }
    //        };
    //
    //        for wallet in node_wallet_list{
    //
    //        let res =  Handler::create_wallet_node(&wallet).await;
    //        debug!("wallet create resp .. {}", res);
    //
    //        }
    //        //wallet_list.append(&mut node_wallet_list);
    //     }
    //
    //     debug!("Fetched wallets .. {:?}", wallet_list);
    //
    //     // make the wallet list unique
    //     // let final_wallet_list: Vec<MongoWallet>= wallet_list.into_iter().unique()
    //     // .map(|servers| servers)
    //     // .collect::<HashSet<_>>()
    //     // .into_iter()
    //     // .collect();
    //
    //     // loop through the final wallet list and get wallet data
    //     // save the data, pass on if it exists
    //     // for node in &nodes{
    //     //     for address in &final_wallet_list {
    //     //         let wallet = get_wallet_data(node, address.to_owned()).await;
    //     //         let wallet = match wallet {
    //     //             Ok(data)=>{data},
    //     //             Err(err)=>{
    //     //                 error!("{}", err);
    //     //                 MongoWallet::default()
    //     //             }
    //     //         };
    //     //         // result does not matter to us yet
    //     //         Handler::create_wallet_node(&wallet).await;
    //     //     }
    //     // }
    //
    // }


    // this gets data on a specific wallet, by making request to all the servers 
    // and then coming to a consensus
    // fn get_wallet_data(){
    //     let nodes = get_servers();
    //     let nodes = match nodes {
    //         Ok(nodes)=>{nodes},
    //         Err(err)=>{
    //             error!("get seed nodes error ... {}", err);
    //             return;
    //         }
    //     };
    //     for node in nodes{
    //
    //     }
    // }


    // zip the nodes local chain 
    // pub fn zipchain(){
    //     let path = Path::new("data.zip");
    //     let file = File::create(&path).unwrap();
    //     let options = SimpleFileOptions::default()
    //     .compression_method(zip::CompressionMethod::Bzip2)
    //     .unix_permissions(0o755);
    //
    //     let mut zip = ZipWriter::new(file);
    //     let mut buffer = Vec::new();
    //     let data_path: String = format!("{}{}",current_dir().unwrap_or_default().to_str().unwrap_or_default(), "/data");
    //     for e in WalkDir::new(data_path.clone()).into_iter().filter_map(|e| e.ok()) {
    //
    //        let name = e.path().strip_prefix(&data_path).unwrap().to_string_lossy();
    //         if e.metadata().unwrap().is_file() {
    //
    //             println!("creating file : {}", name);
    //             zip.start_file(name, options);
    //             let mut f = match File::open(e.path()){
    //                 Ok(data)=>{data},
    //                 Err(err)=>{
    //                     println!("error reading file {}", err.to_string());
    //                     return;
    //                 }
    //             };
    //
    //             f.read_to_end(&mut buffer);
    //             zip.write_all(&buffer);
    //             buffer.clear();
    //         }else {
    //             println!("creating folder : {}",name);
    //             zip.add_directory(name, options);
    //         }
    //     }
    //
    //     zip.finish();
    //
    // }



    // setupp teh folders and files needed for storing digital ID data
    // pub fn setup_digital_id_folders()->Result<(), Box<dyn Error>>{
    //     let path_str = format!("id_data/");
    //
    //     let path = Path::new(path_str.as_str());
    //
    //     if !path.exists() {
    //         match fs::create_dir_all(path){
    //             Ok(_)=>{},
    //             Err(err)=>{
    //                 println!("Error creating path: {}", err.to_string());
    //                 return Err(err.into());
    //             }
    //         };
    //         println!("Path created: {}", path.display());
    //     } else {
    //
    //         println!("Path already exists: {}", path.display());
    //     }
    //
    //     return Ok(())
    // }

}