use std::borrow::Borrow;
use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream};
use log::{debug, error, info};
use std::env;
use std::str::FromStr;
use actix_web::{App, HttpServer};
use actix_web::dev::Server;
use hex_literal::len;
use rand::Rng;
use serde_json::to_string;
use crate::blockchain::broadcast::{get_node_list_net, get_servers};
use crate::controllers::wallet_controller::create_wallet;
use crate::handlers::handlers::Handler;


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
              Handler::transfer(data_set[1].to_string(), &mut stream);
            },

            _ => {}
        }
        let response = "HTTP/1.1 200 OK\r\n\r\n";

        //stream.write(response.as_bytes()).unwrap();
        //stream.flush().unwrap();
    }


    // discover other nodes in the network
    pub fn discover(){
        let servers = match  get_servers() {
            Ok(data)=>{data},
            Err(err)=>{
                error!("{}", err);
                return
            }
        };
        // sample random 20% of the network
        let max = servers.len();
        let number_of_rolls = (20/100)*max;

        let mut i = 0;

        // fetch server list of each initial node
        while (i < number_of_rolls) {
            // randomly pick 1 out of max number of rolls times from the max bucket
            let node_index = rand::thread_rng().gen_range(0..max);
            let node =match servers.get(node_index) {
                Some(node)=>{node},
                None=>{ continue;}
            };

            // get all the node list in this node
            let c_server_list = match get_node_list_net(node){
                Ok(data)=>{data},
                Err(err)=>{continue;}
            };
            i = i+1;
        }
    }

}