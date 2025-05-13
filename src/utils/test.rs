use std::{env::current_dir, fs::File, io::{BufRead, BufReader, Read, Write}, net::{TcpListener, TcpStream}, path::Path, str::FromStr, vec};

use bigdecimal::BigDecimal;
use log::debug;

use walkdir::WalkDir;
use zip::{write::SimpleFileOptions, ZipWriter};



use crate::{blockchain::concensus, models::balance_pack::BalancePack};

use super::{response::Response, utils};
use miniserde::{json, Serialize, Deserialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct MongoWalletX {
    pub id: String,
    pub address: String,
    pub wallet_name: String,
    pub password_hash:String,
  
 
}


impl MongoWalletX {
        
    pub fn default()->MongoWalletX{
        return MongoWalletX{
            id: "".to_string(),
            address:  "".to_string(),
            wallet_name:  "".to_string(),
            password_hash:  "".to_string(),
       
         
        };
    }
}



pub fn test_dd_c(){
    
    // conver to string
    let mongo_wallet = MongoWalletX::default();
    let wallet_string = json::to_string(&mongo_wallet);
    
    //println!("wallet string  .... {}", wallet_string);
    let res = response_formatter("1".to_string(), "message".to_string(), wallet_string.clone());

    println!("response .... {}", res);
    let data_set :Vec<&str>= res.split(r"\n").collect();

    let response_code = match data_set.get(0){
        Some(data)=>{data},
        None=>{""}
    };
    let response_message = match data_set.get(2){
        Some(data)=>{data},
        None=>{""}
    };

    // 

    let clean_message = &response_message.replace(r"\","");

    //println!("clean message .... {}", clean_message);
    let wallet:MongoWalletX = match json::from_str(&response_message){
        Ok(data)=>{data},
        Err(err)=>{
            println!("error {:?}", err);
            return }
    };

    println!("wallet {:#?}", wallet.address);

}

pub fn response_formatter(code:String, message:String, data:String)->String{
    return format!("{}{}{}{}{}{}",code,"\n",message,"\n",data,"\n");
}

pub fn zip(){
    let path = Path::new("example.zip");
    let file = File::create(&path).unwrap();
    let options = SimpleFileOptions::default()
    .compression_method(zip::CompressionMethod::Bzip2)
    .unix_permissions(0o755);

    let mut zip = ZipWriter::new(file);
    let mut buffer = Vec::new();
    let data_path: String = format!("{}{}",current_dir().unwrap_or_default().to_str().unwrap_or_default(), "/data");
    for e in WalkDir::new(data_path.clone()).into_iter().filter_map(|e| e.ok()) {
        
       let name = e.path().strip_prefix(&data_path).unwrap().to_string_lossy();
        if e.metadata().unwrap().is_file() {
            
            println!("creating file : {}", name);
            zip.start_file(name, options);
            let mut f = match File::open(e.path()){
                Ok(data)=>{data},
                Err(err)=>{
                    println!("error reading file {}", err.to_string());
                    return;
                }
            };
            
            f.read_to_end(&mut buffer);
            zip.write_all(&buffer);
            buffer.clear();
        }else {
            println!("creating folder : {}",name);
            zip.add_directory(name, options);
        }
    }

    zip.finish();
}
pub fn file(){
   
    let listener = TcpListener::bind("127.0.0.1:4321").unwrap();
    let mut buf = [0; 4096];
    for stream in listener.incoming() {
        let mut stream = stream.unwrap();
        stream.set_write_timeout(None).unwrap();
        let mut file = File::open("white_paper.txt").unwrap();
        //let mut tcp = TcpStream::connect("127.0.0.1:4321").unwrap();
        // let reader = BufReader::new(file);
        // for line in reader.lines() {
        //     let line = line.unwrap();
        //     let _ = stream.write(line.as_bytes());
        // }
        loop {
            let n = file.read(&mut buf).unwrap();
            
            if n == 0 {
                // reached end of file
                break;
            }
            
            let _ = stream.write_all(&buf[..n]);
        }
       
    }
    
    

}
pub fn cons(){
    let balance_pack_list = vec![
        BalancePack{
            server_http_address :"s1".to_string(),
            balance: BigDecimal::from_str("10.0").unwrap()
        },
        BalancePack{
            server_http_address :"s2".to_string(),
            balance: BigDecimal::from_str("10.0").unwrap()
        },
        BalancePack{
            server_http_address :"s3".to_string(),
            balance: BigDecimal::from_str("10.0").unwrap()
        },  BalancePack{
            server_http_address :"s4".to_string(),
            balance: BigDecimal::from_str("10.0").unwrap()
        },
        BalancePack{
            server_http_address :"s5".to_string(),
            balance: BigDecimal::from_str("10.0").unwrap()
        },
        BalancePack{
            server_http_address :"s6".to_string(),
            balance: BigDecimal::from_str("9.0").unwrap()
        },
        BalancePack{
            server_http_address :"s7".to_string(),
            balance: BigDecimal::from_str("8.0").unwrap()
        },
        BalancePack{
            server_http_address :"s8".to_string(),
            balance: BigDecimal::from_str("10.0").unwrap()
        },
        BalancePack{
            server_http_address :"s9".to_string(),
            balance: BigDecimal::from_str("10.0").unwrap()
        },
        BalancePack{
            server_http_address :"s10".to_string(),
            balance: BigDecimal::from_str("10.0").unwrap()
        }
    ];
    let m = concensus::Concensus::vote_balance(balance_pack_list);
}