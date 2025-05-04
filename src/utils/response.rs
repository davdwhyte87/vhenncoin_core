use std::io::Write;
use log::error;
use serde::{Deserialize, Serialize};
use serde::de::DeserializeOwned;
use tokio::io::AsyncWriteExt;
use tokio::net::TcpStream;
use crate::models::response::NResponse;

pub struct Response {
    
}

impl Response{
    pub fn string_response<T:Serialize>(response:&T)->String{
        let resp_string:String = match serde_json::to_string(response){
            Ok(str)=>{str},
            Err(r)=>{
                error!("{}",r.to_string());
                "".to_string()
            }
        };
        
        return resp_string;
    }

    pub fn response_formatter(code:String, message:String, data:String)->String{
        return format!("{}{}{}{}{}{}",code,r"\n",message,r"\n",data,r"\n");
    }
}
pub struct TCPResponse {

}

impl TCPResponse {
    // this function handles sending of messages back to the ip making the request using the given stream
    pub async  fn send_response<T:Serialize>(response:&T, stream: &mut TcpStream){
        let resp_string:String = match serde_json::to_string(response){
            Ok(str)=>{str},
            Err(r)=>{
                error!("{}",r.to_string());
                "".to_string()
            }
        };

        match stream.write(resp_string.as_bytes()).await{
            Ok(_)=>{},
            Err(err)=>{
                error!("{}",err.to_string());
            }
        }

        stream.flush().await.unwrap();
    }

    pub async fn send_response_txt(response:String, stream: &mut TcpStream){
        match stream.write(response.as_bytes()).await{
            Ok(_)=>{},
            Err(err)=>{
                error!("{}", err.to_string())
            }
        }
    }

    pub async fn send_response_x<T>(response:NResponse<T>, stream: &mut TcpStream) where T: Serialize{
        let mut resp_string = match serde_json::to_string(&response){
            Ok(str)=>str,
            Err(err)=>{
                error!("{}", err);
                return;
            }
        };
        resp_string.push('\n');
        match stream.write_all(resp_string.as_bytes()).await{
            Ok(_)=>{},
            Err(err)=>{
                error!("{}", err.to_string())
            }
        }

        // Flush to ensure all bytes are sent
        if let Err(err) = stream.flush().await {
            error!("{}", err.to_string());
            return;
        }

        // Shutdown the write side so the client knows it's the end of data
        if let Err(err) = stream.shutdown().await {
            error!("{}", err.to_string());
        }
    }
    
    
}