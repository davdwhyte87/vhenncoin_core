use std::io::Write;
use std::net::TcpStream;
use log::error;
use serde::{Deserialize, Serialize};


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
    pub fn send_response<T:Serialize>(response:&T, stream: &mut TcpStream){
        let resp_string:String = match serde_json::to_string(response){
            Ok(str)=>{str},
            Err(r)=>{
                error!("{}",r.to_string());
                "".to_string()
            }
        };

        match stream.write(resp_string.as_bytes()){
            Ok(_)=>{},
            Err(err)=>{
                error!("{}",err.to_string());
            }
        }

        stream.flush().unwrap();
    }
}