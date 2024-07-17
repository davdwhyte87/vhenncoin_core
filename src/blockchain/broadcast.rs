use std::borrow::Borrow;
use std::env::current_dir;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Read, Write};
use std::net::TcpStream;
use std::str::{from_utf8, FromStr};
use std::time::Duration;
use std::{f32, vec};
use actix_web::http;
use bigdecimal::BigDecimal;
use log::{debug, error};
use reqwest::header::CONTENT_TYPE;
use crate::blockchain::wallet;
use crate::models::request::{GetBalanceReq, GetWalletReq};
use crate::models::response::{GenericResponse, WalletNamesResp, WalletNamesRespC};
use crate::models::server_list::{ServerData, ServerList};
use crate::models::wallet::{MongoWallet, WalletC};
use crate::utils::constants;
use crate::utils::formatter::Formatter;
use crate::utils::struct_h::Struct_H;
use crate::utils::utils::request_formatter;
use reqwest;

use super::node;

pub fn get_servers() ->Result<Vec<ServerData>, Box<dyn Error>>{
    let data_path: String = format!("{}{}",current_dir().unwrap_or_default().to_str().unwrap_or_default(), "/server_list.json");
    debug!("serverlist file path {}",data_path);
    let mut file =match  File::open(data_path.clone()){
        Ok(file)=>{file},
        Err(err)=>{
            error!("error opening file {}",err.to_string());
            return Err(err.into())
        } 
    };
    let mut content = String::new();

    match file.read_to_string(&mut content){
        Ok(_)=>{},
        Err(err)=>{
            error!(" error reading file {}",err.to_string());
            return Err(err.into())
        }
    }

    debug!("file content {}", content);

    // decode data
    let server_list: Vec<ServerData> = match  serde_json::from_str(content.as_str()) {
        Ok(data)=>{data},
        Err(err)=>{
            error!("error parsing data {}",err.to_string());
            return Err(err.into())
        }
    };

    debug!("server list: {:?}",server_list);
    return Ok(server_list)
}



pub fn get_seed_nodes() ->Result<Vec<ServerData>, Box<dyn Error>>{
    let data_path: String = format!("{}{}",current_dir().unwrap_or_default().to_str().unwrap_or_default(), "/seed_nodes.json");
    debug!("serverlist file path {}",data_path);
    let mut file =match  File::open(data_path.clone()){
        Ok(file)=>{file},
        Err(err)=>{
            error!("error opening file {}",err.to_string());
            return Err(err.into())
        } 
    };
    let mut content = String::new();

    match file.read_to_string(&mut content){
        Ok(_)=>{},
        Err(err)=>{
            error!(" error reading file {}",err.to_string());
            return Err(err.into())
        }
    }

    debug!("file content {}", content);

    // decode data
    let server_list: Vec<ServerData> = match  serde_json::from_str(content.as_str()) {
        Ok(data)=>{data},
        Err(err)=>{
            error!("error parsing data {}",err.to_string());
            return Err(err.into())
        }
    };

    debug!("server list: {:?}",server_list);
    return Ok(server_list)
}

pub fn save_server_list(data:String)->Result<(), Box<dyn Error> >{

    let data_path: String = format!("{}{}",current_dir().unwrap_or_default().to_str().unwrap_or_default(), "/server_list.json");
    debug!("serverlist file path {}",data_path);

    let file = File::options().write(true).open(data_path);
    let mut file =match file {
        Ok(file) => { file },
        Err(err) => { return Err(err.into()) }
    };

    let write_ok = file.write_all(data.as_bytes());
    let write_ok = match write_ok{
        Ok(write_ok)=>{write_ok},
        Err(err) => { return Err(err.into()) }
    };

    
    Ok(())
}

// talks to other nodes and gets their node list
pub fn get_node_list_c(server_data:&ServerData)->Result<Vec<ServerData>, Box<dyn Error> >{
    let final_message = Formatter::request_formatter(
        constants::GET_NODE_LIST.to_owned(),
        "0".to_string(),
        "0".to_string(),
        "0".to_string(),
        "0".to_string()
    );

    let mut response = String::new();
    match TcpStream::connect(server_data.ip_address.to_owned()) {
        Ok(mut stream)=>{
            // send data to ip computer
            stream.write(final_message.as_ref());

            let mut reader = BufReader::new(&stream);
            
            let _ = reader.read_to_string(&mut response);
           
        },
        Err(err)=>{
            error!("error parsing data {}",err.to_string());
            return Err(err.into())
        }
    }
    
    debug!("response data .. {}", response);


    let data_set :Vec<&str>= response.split("\n").collect();
    let data = match data_set.get(2){
        Some(data)=>{data},
        None =>{return  Err(Box::from("No data in response ")); }
    }; 

    debug!("response data .. {}", data);

    let nodes = serde_json::from_str::<Vec<ServerData>>(&data);
    let nodes = match nodes{
        Ok(data)=>{data},
        Err(err)=>{
            error!("error {}",err.to_string());  
            return Err(err.into());
        }
    };

    Ok(nodes)
}
// talks to other nodes and gets their node list 
pub async fn get_node_list_http(server_data:&ServerData)->Result<Vec<ServerData>, Box<dyn Error> >{
    let url =format!("{}/send_message", server_data.http_address.to_owned());
    let mut c = awc::Client::default();
    debug!("{}", url);

    let message = request_formatter("GetNodeList".to_string(),
    "".to_string(),
     "".to_string(),
      "".to_string(), 
      "0".to_string());
   
    let resp = c.post(url.clone()).send_body(message).await;
    let mut resp =match resp {
        Ok(resp)=>{resp},
        Err(err)=>{
            error!("Request error ... {}", err);
            return Err(err.into())
        }
    };
    let bytese = resp.body().await;
    let bytese =match bytese {
        Ok(bytese)=>{bytese},
        Err(err)=>{
            error!("{}", err);
            return Err(err.into())
        }
    };
    let body = from_utf8(&bytese).unwrap().to_string();
    debug!("AWC RESPO {:?}",body);
    
   
    let text_data = "GetNodeList";

    // let client = reqwest::Client::new();
    // let response = client
    //     .post(url)
    //     .header(CONTENT_TYPE, "text/plain")
    //     .timeout(tokio::time::Duration::from_secs(20))
    //     .body(text_data.to_owned())
    //     .send()
    //     .await?;


    //debug!("Status Code: {}", response.status());
    let response_body = body;
    debug!("Req Body : {}", response_body);

    
    let data_set :Vec<&str>= response_body.split(r"\n").collect();
    let response_code = match data_set.get(0){
        Some(data)=>{data},
        None=>{""}
    };
    let response_message = match data_set.get(1){
        Some(data)=>{data},
        None=>{""}
    };
    let response_data = match data_set.get(2){
        Some(data)=>{data},
        None=>{""}
    };
    debug!("data 0 {}",response_code);
    debug!("data 1 {}",response_message);
    debug!("data 2 {}",response_data);

    let node_list: Vec<ServerData> =  match  serde_json::from_str(response_data){
        Ok(list)=>{list},
        Err(err)=>{
            error!("decode error {}", err);
            return Err(err.into())
        }
    };

    debug!("node list  ...{:?}", node_list);

    //let node_list = vec![ServerData{ id: "".to_string(), ip_address: "".to_string(), public_key: "".to_string(), http_address:"".to_string() }];
    Ok(node_list)

}
pub fn get_node_list_net(server_data:&ServerData)->Result<Vec<ServerData>, Box<dyn Error>>{
    // make call to ip address
    match TcpStream::connect(&server_data.ip_address) {
        Ok(mut stream)=>{
            let message = format!("GetNodeList{}",r"\n");

            // send data to ip computer
            stream.write(message.as_ref()).unwrap();

            // get response string
            let mut resp_string = String::new();
            let response =match  stream.read_to_string(&mut resp_string){
                Ok(x)=>{x},
                Err(err)=>{return Err(err.into())}
            };
            let resp_data : GenericResponse = match serde_json::from_str(resp_string.as_str()){
                Ok(data)=>{data},
                Err(err)=>{return Err(err.into());}
            };
            let server_data_list : Vec<ServerData> =match serde_json::from_str(resp_data.message.as_str()) {
                Ok(data)=>{data},
                Err(err)=>{return Err(err.into())}
            };

            return Ok(server_data_list)

        },
        Err(err)=>{
            error!("error parsing data {}",err.to_string());
            return  Err(err.into())
        }
    }
}

pub async fn  notify_new_node_http(server_data:&ServerData, new_node:&ServerData)->Result<(), Box<dyn Error> >{
    let url =format!("{}/send_message", server_data.http_address.to_owned());
    let mut c = awc::Client::default();
    debug!("{}", url);

    // prepare new node to string
    let nn_string = match serde_json::to_string(new_node){
        Ok(new_node_string)=>{new_node_string},
        Err(err)=>{
            error!("Request error ... {}", err);
            return Err(err.into())
        }
    };

    let request_string =  request_formatter("AddNode".to_string(),
    "".to_string(),
     "".to_string(),
      "".to_string(), 
      "0".to_string());
    let resp = c.post(url.clone()).send_body(request_string).await;
    let mut resp =match resp {
        Ok(resp)=>{resp},
        Err(err)=>{
            error!("Request error ... {}", err);
            return Err(err.into())
        }
    };
    let bytese = resp.body().await;
    let bytese =match bytese {
        Ok(bytese)=>{bytese},
        Err(err)=>{
            error!("{}", err);
            return Err(err.into())
        }
    };
    let body = from_utf8(&bytese).unwrap().to_string();
    debug!("AWC RESPO {:?}",body);

    let response_body = body;
    debug!("Req Body : {}", response_body);

    
    let data_set :Vec<&str>= response_body.split(r"\n").collect();

    Ok(())
}



pub fn notify_network_new_node_bc(node:&ServerData, new_node:&ServerData)->Result<(), Box<dyn Error>>{
    // conver node data into string 
    let nn_string = match serde_json::to_string(new_node){
        Ok(new_node_string)=>{new_node_string},
        Err(err)=>{
            error!("Request error ... {}", err);
            return Err(err.into())
        }
    };
    let message = Formatter::request_formatter(
        constants::ADD_NODE.to_owned(),
        nn_string,
        "".to_string(),
        "".to_string(),
        "0".to_string()
    );

    let mut response = String::new();
    match TcpStream::connect(node.ip_address.to_owned()) {
        Ok(mut stream)=>{
            // send data to ip computer
            stream.write(message.as_ref());
            
            // let mut reader = BufReader::new(&stream);
            
            // let _ = reader.read_to_string(&mut response);

            stream.flush();
        },
        Err(err)=>{
            error!("error parsing data {}",err.to_string());
            return Err(err.into());
        }
    }

    Ok(())
}
// telll other servers about the new request
pub fn broadcast_request_tcp(action:String, message:String){
    // get servers for the node 
    let servers = match get_servers() {
        Ok(data)=>{data},
        Err(err)=>{
            error!("{}", err.to_string());
            return;
        }
    };

    let final_message = Formatter::request_formatter(
        action,
        message,
        "".to_string(),
        "".to_string(),
        "1".to_string());

    for server in servers{
        match TcpStream::connect(server.ip_address) {
            Ok(mut stream)=>{
                // send data to ip computer
                stream.write(final_message.as_ref());
                // no need to read response
                // we do not care if it fails for now...
            },
            Err(err)=>{
                error!("error parsing data {}",err.to_string());
                return
            }
        } 
    }
}


pub fn broadcast_request(message:String, ip_address:String){
    // get all servers
    match TcpStream::connect(ip_address) {
        Ok(mut stream)=>{
            // send data to ip computer
            stream.write(message.as_ref()).unwrap();

            // no need to read response
        },
        Err(err)=>{
            error!("error parsing data {}",err.to_string());
            return
        }
    }
}


// this sends a request to other servers in the network about a new transaction 
pub async  fn  broadcast_request_http(action_name:String, message:String){

    // get servers to broadcast to 
    let servers = match get_servers(){
        Ok(data)=>{data},
        Err(err)=>{
            error!("Error getting servers ... {}", err);
            return ;
        }
    };


    for server in servers {
        let url =format!("{}/send_message", server.http_address);
        let mut c = awc::Client::default();
        debug!("{}", url);

        let request_data = request_formatter(action_name.to_owned(),
        message.to_owned(),
         "".to_string(),
          "".to_string(), 
          "1".to_string());

        debug!("request message {}", request_data);
        let resp = c.post(url.clone()).timeout(Duration::from_secs(1)).send_body(request_data).await;
        let mut resp =match resp {
            Ok(resp)=>{resp},
            Err(err)=>{
                error!("Request error ... {}", err);
                continue; 
            }
        };
        
        let bytese = resp.body().await;
        let bytese =match bytese {
            Ok(bytese)=>{bytese},
            Err(err)=>{
                error!("{}", err);
                return
            }
        };
        let body = from_utf8(&bytese).unwrap().to_string();
        debug!("AWC RESPO {:?}",body);
    }

    
}

pub async fn get_node_wallet_list(server_data:&ServerData)->Result<Vec<MongoWallet>, Box<dyn Error>>{
    let url =format!("{}/send_message", server_data.http_address.to_owned());
    let mut c = awc::Client::default();
    debug!("{}", url);

    let request_string = request_formatter("GetNodeWalletList".to_string(),
     "".to_string(),
      "".to_string(),
       "".to_string(), 
       "0".to_string());
    
    let resp = c.post(url.clone()).send_body(request_string).await;
    let mut resp =match resp {
        Ok(resp)=>{resp},
        Err(err)=>{
            error!("Request error ... {}", err);
            return Err(err.into())
        }
    };
    let bytese = resp.body().await;
    let bytese =match bytese {
        Ok(bytese)=>{bytese},
        Err(err)=>{
            error!("{}", err);
            return Err(err.into())
        }
    };
    let body = String::from_utf8_lossy(&bytese).to_string();
    //debug!("AWC RESPO {:?}",body);

    let response_body = body;
    //debug!("Req Body : {}", response_body);

    
    let data_set :Vec<&str>= response_body.split(r"\n").collect();

    let response_code = match data_set.get(0){
        Some(data)=>{data},
        None=>{""}
    };
    let response_message = match data_set.get(2){
        Some(data)=>{data.to_string()},
        None=>{"".to_string()}
    };

    //debug!("Response message : {}", response_message);
    let clean_message = &response_message.replace(r"\","");

    //debug!("Clean Response message : {}", clean_message);
    let wallet_llist:WalletNamesResp = match serde_json::from_str::<WalletNamesResp>(&clean_message){
        Ok(data)=>{data},
        Err(err)=>{
            error!("error decoding response {:?}",err);
            return Err(err.into());
        }
    };


    return Ok(wallet_llist.names)
}


pub struct WalletRIInfo{
     
}
pub async fn get_wallet_data(server_data:&ServerData, address:String)->Result<MongoWallet, Box<dyn Error>>{

    
    let url =format!("{}/send_message", server_data.http_address.to_owned());
    let mut c = awc::Client::default();
    debug!("{}", url);

    let request_string =  request_formatter("GetWalletData".to_string(),
     "".to_string(),
      "".to_string(),
       "".to_string(), 
       "0".to_string());
    let resp = c.post(url.clone()).send_body(request_string).await;
    let mut resp =match resp {
        Ok(resp)=>{resp},
        Err(err)=>{
            error!("Request error ... {}", err);
            return Err(err.into())
        }
    };
    let bytese = resp.body().await;
    let bytese =match bytese {
        Ok(bytese)=>{bytese},
        Err(err)=>{
            error!("{}", err);
            return Err(err.into())
        }
    };
    let body = from_utf8(&bytese).unwrap().to_string();
    debug!("AWC RESPO {:?}",body);

    let response_body = body;
    debug!("Req Body : {}", response_body);

    
    let data_set :Vec<&str>= response_body.split(r"\n").collect();

    let response_code = match data_set.get(0){
        Some(data)=>{data},
        None=>{""}
    };
    let response_data = match data_set.get(2){
        Some(data)=>{data},
        None=>{""}
    };

    let wallet_data:MongoWallet = match serde_json::from_str(&response_data){
        Ok(data)=>{data},
            Err(err)=>{
                error!("error ... {}", err);
                MongoWallet::default()
            }
    };
    // let wallet_data = match wallet_data {
    //     Ok(data)=>{data},
    //     Err(err)=>{
    //         error!("error ... {}", err);
    //         MongoWallet::default()
    //     }
    // };
    
    return Ok(wallet_data)

}


// communicate with other nodes and get their list of wallets
pub fn get_node_wallet_list_C(server_data:&ServerData)-> Result<Vec<String>, Box<dyn Error>>{
    let message = Formatter::request_formatter(
        constants::GET_NODE_WALLET_LIST.to_owned(),
        "".to_string(),
        "".to_string(),
        "".to_string(),
        "0".to_string()
    );
    let mut response = String::new();
    match TcpStream::connect(server_data.ip_address.to_owned()) {
        Ok(mut stream)=>{
            // send data to ip computer
            stream.write(message.as_ref());
            
            let mut reader = BufReader::new(&stream);
            
            let _ = reader.read_to_string(&mut response);

            stream.flush();
        },
        Err(err)=>{
            error!("error parsing data {}",err.to_string());
            return Err(err.into());
        }
    }

    debug!("remote balance response ..{}", response);

    // brrak down response 
    let data_set :Vec<&str>= response.split("\n").collect();
    let data = match data_set.get(2){
        Some(data)=>{data},
        None =>{return  Err(Box::from("No data in response ")); }
    }; 

    let wallet_names = serde_json::from_str::<WalletNamesRespC>(&data);
    let wallet_names = match wallet_names{
        Ok(data)=>{data},
        Err(err)=>{
            error!("error {}",err.to_string());  
            return Err(err.into());
        }
    };

    return Ok(wallet_names.names);

}

pub fn get_remote_wallet(server_data:&ServerData, address:&String)->Result<WalletC, Box<dyn Error>>{
    let request = Struct_H::struct_to_string(&GetWalletReq{address:address.to_owned()});
    let message = Formatter::request_formatter(
        constants::GET_NODE_BALANCE_ACTION.to_owned(),
        request,
        "".to_string(),
        "".to_string(),
        "0".to_string()
    );
    let mut response = String::new();
    match TcpStream::connect(server_data.ip_address.to_owned()) {
        Ok(mut stream)=>{
            // send data to ip computer
            stream.write(message.as_ref());
            
            let mut reader = BufReader::new(&stream);
            
            let _ = reader.read_to_string(&mut response);

            stream.flush();
        },
        Err(err)=>{
            error!("error parsing data {}",err.to_string());
            return Err(err.into())
        }
    }

    debug!("remote balance response ..{}", response);  
        // brrak down response 
        let data_set :Vec<&str>= response.split("\n").collect();
        let data = match data_set.get(2){
            Some(data)=>{data},
            None =>{return Err(Box::from("Could not break response down"))}
        };
    
        // get message  
        let wallet =  match serde_json::from_str::<WalletC>(data){
            Ok(data)=>{data},
            Err(err)=>{
                debug!("{}", err.to_string());
               WalletC::default()
            }
        };

        Ok(wallet)

}
pub fn get_remote_node_balance_c(server_data:&ServerData, address:&String)->Result<BigDecimal, Box<dyn Error>>{
    let request = Struct_H::struct_to_string(&GetBalanceReq{address:address.to_owned()});
    let message = Formatter::request_formatter(
        constants::GET_NODE_BALANCE_ACTION.to_owned(),
        request,
        "".to_string(),
        "".to_string(),
        "0".to_string()
    );
    let mut response = String::new();
    match TcpStream::connect(server_data.ip_address.to_owned()) {
        Ok(mut stream)=>{
            // send data to ip computer
            stream.write(message.as_ref());
            
            let mut reader = BufReader::new(&stream);
            
            let _ = reader.read_to_string(&mut response);

            stream.flush();
        },
        Err(err)=>{
            error!("error parsing data {}",err.to_string());
            return Err(err.into())
        }
    }

    debug!("remote balance response ..{}", response);

    // brrak down response 
    let data_set :Vec<&str>= response.split("\n").collect();
    let code = match data_set.get(0){
        Some(data)=>{data},
        None =>{return Err(Box::from("Could not break response down"))}
    };
    let data = match data_set.get(2){
        Some(data)=>{data},
        None =>{return Err(Box::from("Could not break response down"))}
    };

    // terminate and return error if we get no response from server
    if *code == "0"{
        return Err(Box::from("could not get data from remote server"))
    }

    // get message  
    use std::str::FromStr;
    let balance =  match BigDecimal::from_str(data){
        Ok(data)=>{data},
        Err(err)=>{
            debug!("{}", err.to_string());
            BigDecimal::from_str("0.0").unwrap()
        }
    };


    Ok(balance)
}

pub async fn get_node_balance(server_data:&ServerData, address:&String)->Result<BigDecimal, Box<dyn Error>>{
        
    let url =format!("{}/send_message", server_data.http_address.to_owned());
    let mut c = awc::Client::default();
    debug!("{}", url);

    let message = GetBalanceReq{
        address: address.to_string()
    };
    let message_string = match serde_json::to_string(&message){
        Ok(data)=>{data},
        Err(err)=>{
            return Err(err.into());
        }
    };

    let request_string =  request_formatter("GetNodeBalance".to_string(),
     message_string,
      "".to_string(),
       "".to_string(), 
       "0".to_string());

    let resp = c.post(url.clone()).send_body(request_string).await;
    let mut resp =match resp {
        Ok(resp)=>{resp},
        Err(err)=>{
            error!("Request error ... {}", err);
            return Err(err.into())
        }
    };
    let bytese = resp.body().await;
    let bytese =match bytese {
        Ok(bytese)=>{bytese},
        Err(err)=>{
            error!("{}", err);
            return Err(err.into())
        }
    };
    let body = from_utf8(&bytese).unwrap().to_string();
    debug!("AWC RESPO {:?}",body);

    let response_body = body;
    debug!("Req Body : {}", response_body);

    
    let data_set :Vec<&str>= response_body.split(r"\n").collect();

    let response_code = match data_set.get(0){
        Some(data)=>{data},
        None=>{""}
    };
    let response_data = match data_set.get(2){
        Some(data)=>{data},
        None=>{""}
    };

    if response_code == "0"{
        return Err(Box::from("Error from remote server"));
    }

    let balance = match BigDecimal::from_str(&response_data){
        Ok(data)=>{data},
            Err(err)=>{
                error!("error ... {}", err);
                BigDecimal::from_str("0.0").unwrap()
            }
    };
    // let wallet_data = match wallet_data {
    //     Ok(data)=>{data},
    //     Err(err)=>{
    //         error!("error ... {}", err);
    //         MongoWallet::default()
    //     }
    // };
    return Ok(balance)

}