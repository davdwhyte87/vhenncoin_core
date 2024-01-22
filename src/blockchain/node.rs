use std::borrow::Borrow;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use log::{debug, info};
use serde_json::to_string;
use crate::handlers::handlers::Handler;


pub struct Node {

}

impl Node {

    pub fn serve(){
        let address = "127.0.0.1:8080";
        let listener = TcpListener::bind(address).unwrap();
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
            "TRANSFER"=>{
              Handler::transfer(data_set[1].to_string(), stream.borrow());
            },

            _ => {}
        }
        let response = "HTTP/1.1 200 OK\r\n\r\n";

        stream.write(response.as_bytes()).unwrap();
        stream.flush().unwrap();
    }
}