use std::borrow::Borrow;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use serde_json::to_string;
use crate::blockchain::handlers::Handler;

pub struct Node {

}

impl Node {

    pub fn serve(){
        let listener = TcpListener::bind("127.0.0.1:8080").unwrap();

        for stream in listener.incoming() {
            let stream = stream.unwrap();

             Node::handle_connection(stream);
        }
    }
    pub fn handle_connection(mut stream: TcpStream) {
        let mut buffer = [0; 512];

        stream.borrow().read(&mut buffer).unwrap();

        let data = String::from_utf8_lossy(&buffer).to_string();
        println!("Data : {}", data );

        let data_set :Vec<&str>= data.split("\n").collect();
        println!("{}", data_set[0]);

        match data_set[0]{

            "CREATE_WALLET" =>{
               println!("Create wallet now")
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