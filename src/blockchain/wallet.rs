use std::borrow::Borrow;
use std::env::current_dir;
use std::error::Error;
use std::fmt::format;
use sha2::{Sha256, Digest};
use std::{fs, str};
use std::fs::{File, OpenOptions, read};
use std::io::{Read, Write};
use std::path::Path;
use std::ptr::null;
use actix_web::dev::ResourcePath;
use base64::Engine;
use base64::engine::general_purpose;
use log::{debug, error};
use rand::rngs::StdRng;
use ring::digest::SHA512;
use ring::error::Unspecified;
use ring::rand::SystemRandom;
use ring::signature::{Ed25519KeyPair, KeyPair};
use rsa::{Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey, pkcs1::DecodeRsaPrivateKey};
use rsa::pkcs1::{EncodeRsaPrivateKey, EncodeRsaPublicKey, LineEnding};
use rsa::pkcs8::EncodePrivateKey;
use rsa::rand_core;
use rsa::rand_core::SeedableRng;
use serde::__private::de::IdentifierDeserializer;
use serde::de::IntoDeserializer;
use sha256::digest;

use uuid::{Uuid, uuid};
use uuid::Version::Sha1;
use validator::HasLen;
use crate::blockchain::kv_store::KvStore;

use crate::models::block::{Block, Chain};
use crate::req_models::wallet_requests::CreateWalletReq;
use crate::utils::time::get_date_time;

pub struct Wallet{

}

impl Wallet {

    // check if data path exists
    pub fn wallet_exists (address:&String)->bool{
        let data_path = format!("{}{}{}",current_dir().unwrap_or_default().to_str().unwrap_or_default(),r"\data\" ,address.to_owned());
        println!("{}", data_path);
        if !Path::new(data_path.as_str()).exists() {
            return false
        }else {
            return true
        }
    }

    // create a wallet on the blockchain
    pub fn create_wallet(address:String, public_key:String)->Result<(), Box<dyn Error>>{

        // create database
        match  KvStore::create_db(address.clone(),"chain".to_string()){
            Ok(_)=>{
                debug!("Done creating wallet");
            },
            Err(err)=>{
                error!("{}",err.to_string());
                return Err(err.into());
            }
        };
        let mut block =Block{
            id: Uuid::new_v4().to_string(),
            sender_address: "000000000".to_string(),
            receiver_address: address.to_owned(),
            date_created: "".to_string(),
            hash: "".to_string(),
            amount: 100.0,
            prev_hash:"000000000".to_string(),
            public_key: "".to_string(),
        };

        let hash = digest(format!("{}{}{}{}{}",block.id, block.sender_address,
                                  block.receiver_address,block.amount, block.prev_hash ));

        block.hash = hash;
        let chain = Chain{ chain: vec![block] };

        // save data
        match KvStore::save(address,"chain".to_string(),Some(chain)){
            Ok(_)=>{
                debug!("Suscessful write to chain")
            },
            Err(err)=>{
                error!("{}",err.to_string())}
        }

        return Ok(())
    }

    // get a wallets last block
    // pub fn get_last_block(address:&String)->Result<>{
    //     let chain = match Wallet::get_wallet_chain(address){
    //
    //     }
    // }
    // this helps us get the wallet address
    pub fn get_wallet_chain(address:&String)->Result<Chain, Box<dyn Error>>{
        println!("data : Da");
        // create url path
        let data_path = format!("{}{}{}",current_dir().unwrap_or_default().to_str()
            .unwrap_or_default(), r"\data\",address.to_owned());
        let chain_path = format!("{}{}", data_path, r"\chain.bin");
        println!("data : {}", chain_path);
        //open file
        let mut file = File::open(chain_path);
        let mut file = match file {
            Ok(file)=>{file},
            Err(err)=>{return Err(err.into())}
        };
        // read data from file
        let mut contents = String::new();
        let read_ok =file.read_to_string(&mut contents);
        let read_ok = match read_ok {
            Ok(read_ok)=>{read_ok},
            Err(err)=>{return Err(err.into())}
        };
        println!("data : {}", contents);

        let mut chain: Chain = match  serde_json::from_str(contents.as_str()) {
            Ok(data)=>{data},
            Err(err)=>{
            return Err(err.into())
            }
        };

        return Ok(chain)
    }

    // save block
    pub fn save_block(address: &String, block:Block)->Result<(), Box<dyn Error>>{
        let data_path = format!("{}{}{}",current_dir().unwrap_or_default().to_str()
            .unwrap_or_default(), r"\data\",address.to_owned());
        let chain_path = format!("{}{}", data_path, r"\chain.bin");
        println!("data : {}", chain_path);
        //open file
        let mut file = OpenOptions::new().write(true).open(chain_path);
        let mut file = match file {
            Ok(file)=>{file},
            Err(err)=>{return Err(err.into())}
        };

        // get chain
        let mut chain = match Wallet::get_wallet_chain(address){
            Ok(chain)=>{chain},
            Err(err)=>{return Err(err.into())}
        };

        // append chain

        chain.chain.push(block);

        // save block to chain


        let json = serde_json::to_string(chain.borrow());
        let json =match json {
            Ok(json)=>{json},
            Err(err)=>{
                return Err(err.into())
            }
        };
        println!("{}", json);
        let write_ok =file.write_all(json.as_bytes());
        let write_ok = match write_ok {
            Ok(write_ok)=>{write_ok},
            Err(err)=>{
                return Err(err.into())
            }
        };

        return Ok(())
    }

    // gets a users wallet balance
    pub fn get_balance(address:&String)->Result<f32, Box<dyn Error>>{
        // get the chain
        let chain = match Wallet::get_wallet_chain(address){
            Ok(data)=>{data},
            Err(err)=>{
                return Err(err.into())
            }
        };
        // loop through and add all the block amounts
        let mut balance:f32 = 0.0;
        for block in chain.chain.into_iter(){
            balance = balance+block.amount;
        }

        return Ok(balance)
    }

    pub fn play(){
        let private_key1 = digest(Uuid::new_v4().to_string());
        let private_key2 = digest(Uuid::new_v4().to_string());
        let private_key3 = digest(Uuid::new_v4().to_string());
        let private_key4 = digest(Uuid::new_v4().to_string());
        println!("Private key 1: {}", private_key1.clone());
        println!("Private key 2: {}", private_key2);
        println!("Private key 3: {}", private_key3);
        println!("Private key 4 : {}", private_key4);

        let private_key = private_key1.clone()+ &private_key2 + &private_key3 + &private_key4;

        println!("Private key : {}", private_key);

        let public_key1 = digest(&private_key1);
        let public_key2 = digest(&private_key2);
        let public_key3 = digest(&private_key3);
        let public_key4 = digest(&private_key4);
        let public_key =  format!("{},{},{},{}",&public_key1, &public_key2 , &public_key3 ,&public_key4);
        println!("Public key : {}", public_key);

        let signature = format!("{},{}", &private_key1, &private_key4);
        let der_Pk =digest(&private_key1);
        let der_Pk2 =digest(&private_key4);
        let derived_public_key = format!("{},{},{},{}", &der_Pk,&public_key2,&public_key3,der_Pk2
        );

        //
        // reconstruct the public key from gignature

        let message = "Hello what is up man";
        let signed_message = digest(format!("{}{}", message,public_key));
        println!("Signed message : {}", signed_message);

        // derived signed message
        let der_signed_message = digest(format!("{}{}", message,derived_public_key));
        println!("Derived Signed message : {}", der_signed_message);

    }

    pub fn gen(){
        let mut dd= ("nomii the times are live".as_bytes());
        println!("{}",dd.length());
        //let mut rng = StdRng::from_seed(dd);
        use sha2::Sha256;
        use hkdf::Hkdf;
        use hex_literal::hex;

        let ikm = hex!("0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b0b");
        let salt = hex!("000102030405060708090a0b0c");
        let info = "the man is a benh".as_bytes().to_vec();

        let hk = Hkdf::<Sha256>::new(Some(&salt[..]), &ikm);
        let mut okm = [0u8; 32];
        hk.expand(&info, &mut okm)
            .expect("42 is a valid length for Sha256 to output");

        let mut rng = StdRng::from_seed(okm);
        //let mut rng = rand::thread_rng();

        let bits = 256;
        let u1priv_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
        let u1pub_key = RsaPublicKey::from(&u1priv_key);
        let m = format!("public key {} private key{} ",u1pub_key.to_pkcs1_pem(LineEnding::default()).unwrap().as_str(), u1priv_key.to_pkcs1_pem(LineEnding::default()).unwrap().as_str());
        println!("{}",m);
    }

    pub fn enc(){

        use base64::{engine, alphabet, Engine as _};



        // let encoded = crazy_engine.encode(b"abc 123");
        let dd = "nomii".as_bytes().try_into().unwrap_or_default();
        let mut rng = StdRng::from_seed(dd);
        let bits = 2048;
        let u1priv_key = RsaPrivateKey::new(&mut rng, bits).expect("failed to generate a key");
        let u1pub_key = RsaPublicKey::from(&u1priv_key);

        let u2priv_key = RsaPrivateKey::new(&mut rng, bits.clone()).expect("failed to generate a key");
        let u2pub_key = RsaPublicKey::from(&u2priv_key);

        let m = format!("{} {} ",u1pub_key.to_pkcs1_pem(LineEnding::default()).unwrap().as_str(), u1priv_key.to_pkcs1_pem(LineEnding::default()).unwrap().as_str());
        println!("{}",m);
// Encrypt
        let data = b"hello world";
        let enc_data = u1pub_key.encrypt(&mut rng, Pkcs1v15Encrypt, &data[..]).expect("failed to encrypt");
        assert_ne!(&data[..], &enc_data[..]);
        let enc_data_encode = general_purpose::URL_SAFE_NO_PAD.encode(&enc_data[..]);
        println!("before encode: {:?}",  &enc_data[..]);
        println!("encoded {}",enc_data_encode);
        println!("decoded {:?}",general_purpose::URL_SAFE_NO_PAD.decode(enc_data_encode).unwrap());

// Decryp
        let dec_data = u1priv_key.decrypt(Pkcs1v15Encrypt, &enc_data).expect("failed to decrypt");
        assert_eq!(&data[..], &dec_data[..]);
        println!("{:?}",String::from_utf8(dec_data[..].to_owned()).unwrap());
    }

    pub fn generate_key(){
        let mut rng = rand::thread_rng();
        let bits = 256;
        let priv_key = RsaPrivateKey::new(&mut rng, bits.clone()).expect("failed to generate a key");
        let pub_key = RsaPublicKey::from(&priv_key);


        let private_key_pem = match  priv_key.to_pkcs1_pem(LineEnding::default()){
            Ok(private_key_pem)=>{private_key_pem},
            Err(err)=>{
                println!("{:?}", err.to_string());
                return
            }
        };

        let public_key_pem = match pub_key.to_pkcs1_pem(LineEnding::default()){
            Ok(public_key_pem)=>{public_key_pem},
            Err(err)=>{
                println!("{:?}", err.to_string());
                return
            }
        };
        let l = RsaPrivateKey::from_pkcs1_pem(private_key_pem.to_string().as_str()).unwrap();
        // encode private and public keys
        let private_key_enc = general_purpose::URL_SAFE_NO_PAD.encode(private_key_pem.to_string());
        let public_key_enc = general_purpose::URL_SAFE_NO_PAD.encode(public_key_pem.to_string());
       // println!("{:?}",private_key_pem.to_string());
        println!("private key {:?}",private_key_enc);
        println!("public key {:?}",public_key_enc);
        let address = sha256::digest(public_key_pem);
        println!("wallet address {:?}",format!("{}{}","Vc",address));
    }

    pub fn edd_generate_keys(){
        let rand = SystemRandom::new();
        let pkcs8_bytes = Ed25519KeyPair::generate_pkcs8(&rand).unwrap(); // pkcs8 format used for persistent storage
        let key_pair = Ed25519KeyPair::from_pkcs8(pkcs8_bytes.as_ref()).map_err(|_| Unspecified).unwrap();

        println!("private key {}", general_purpose::URL_SAFE_NO_PAD.encode(pkcs8_bytes.as_ref()));
        println!("public key {}", general_purpose::URL_SAFE_NO_PAD.encode(key_pair.public_key()));

    }
}