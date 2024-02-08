use std::env::current_dir;
use std::error::Error;
use std::fs::File;
use std::io::{Read, Write};
use std::net::TcpStream;
use log::{debug, error};
use crate::models::server_list::{ServerData, ServerList};

pub fn get_servers() ->Result<(), Box<dyn Error>>{
    let data_path = format!("{}{}",current_dir().unwrap_or_default().to_str().unwrap_or_default(), "/server_list.json");
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
    return Ok(())
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