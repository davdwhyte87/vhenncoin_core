use std::borrow::Borrow;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use log::{debug, error, info};
use std::env;
use serde_json::to_string;
use crate::handlers::handlers::Handler;


pub struct Node {

}

impl Node {

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
                Handler::create_wallet(&data_set[1].to_string(), &mut stream)
            },
            "Transfer"=>{
              Handler::transfer(data_set[1].to_string(), &mut stream);
            },

            _ => {}
        }
        let response = "HTTP/1.1 200 OK\r\n\r\n";

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}