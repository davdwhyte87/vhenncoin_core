use std::borrow::Borrow;
use std::env::current_dir;
use std::error::Error;
use std::fs::File;
use std::io::{Read, Write};
use std::net::TcpStream;
use std::str::from_utf8;
use std::vec;
use log::{debug, error};
use reqwest::header::CONTENT_TYPE;
use crate::models::response::GenericResponse;
use crate::models::server_list::{ServerData, ServerList};
use reqwest;

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
pub async fn get_node_list_http(server_data:&ServerData)->Result<Vec<ServerData>, Box<dyn Error> >{
    let url =format!("{}/send_message", server_data.http_address.to_owned());
    let mut c = awc::Client::default();
    debug!("{}", url);

    let resp = c.post(url.clone()).send_body("GetNodeList").await;
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
    
    debug!("data 0 {}",data_set[0]);
    debug!("data 1 {}",data_set[1]);
    let node_list: Vec<ServerData> =  match  serde_json::from_str(data_set[1]){
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