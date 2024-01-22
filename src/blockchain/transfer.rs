use std::borrow::Borrow;
use std::error::Error;
use base64::Engine;
use uuid::Uuid;

use crate::blockchain::wallet::Wallet;
use crate::models::block::Block;
use crate::utils::time::get_date_time;
use base64::engine::general_purpose;
use futures_util::AsyncReadExt;
use jsonwebtoken::crypto::sign;
use mongodb::options::AuthMechanism::ScramSha256;
use ring::agreement::{Algorithm, UnparsedPublicKey};
use ring::error::Unspecified;
use ring::rand::SystemRandom;
use ring::signature;
use ring::signature::{ECDSA_P256_SHA256_ASN1, ECDSA_P256_SHA256_ASN1_SIGNING, EcdsaKeyPair, EcdsaSigningAlgorithm, ED25519, Ed25519KeyPair, KeyPair, VerificationAlgorithm};
use rsa::{Pkcs1v15Encrypt, RsaPrivateKey, RsaPublicKey, pkcs1::DecodeRsaPrivateKey, pkcs1::DecodeRsaPublicKey, Pkcs1v15Sign};
use rsa::pkcs8::spki::Error::AlgorithmParametersMissing;
use rsa::traits::SignatureScheme;
use sha256::digest;
use crate::models::request::TransferReq;


pub struct Transfer {

}

impl Transfer {

    pub fn sign_messafe(private_key:String, message:String)->Result<(), Box<dyn Error>>{
        // decode private key
        let private_key = match general_purpose::URL_SAFE_NO_PAD.decode(private_key){
            Ok(private_key)=>{private_key},
            Err(err)=>{
                return Err(err.into())
            }
        };

        let private_key:RsaPrivateKey = match RsaPrivateKey::from_pkcs1_pem(String::from_utf8_lossy(private_key.borrow()).to_string().as_str()){
            Ok(private_key)=>{private_key},
            Err(err)=>{return Err(err.into())}
        };
        match private_key.sign(Pkcs1v15Sign { hash_len: Some(message.as_bytes().len()), prefix: Box::new([]) }, message.as_bytes()){
            // encode signature as bytes
            Ok(data)=>{println!("signature : {:?}", base64::engine::general_purpose::URL_SAFE_NO_PAD.encode(data))},
            Err(err)=>{return Err(err.into())}
        }
        return Ok(())
    }

    pub fn verify(public_key:String, signature:String, message:String)->Result<(), Box<dyn Error>>{
        // decode signature to bytes
        let signature = match general_purpose::URL_SAFE_NO_PAD.decode(signature){
            Ok(signature)=>{signature},
            Err(err)=>{
                return Err(err.into())
            }
        };
        // decode public key
        let public_key = match general_purpose::URL_SAFE_NO_PAD.decode(public_key){
            Ok(public_key)=>{public_key},
            Err(err)=>{
                return Err(err.into())
            }
        };
        let public_key:RsaPublicKey = match RsaPublicKey::from_pkcs1_pem(String::from_utf8_lossy(public_key.borrow()).to_string().as_str()){
            Ok(public_key)=>{public_key},
            Err(err)=>{return Err(err.into())}
        };


        match public_key.verify(Pkcs1v15Sign { hash_len: Some(message.as_bytes().len()), prefix: Box::new([]) },
                          message.as_bytes(),
             &signature
        ){
            Ok(_)=>{ println!("Ver")},
            Err(err)=>{return Err(err.into())}
        }
        return Ok(())
    }

    pub fn generate_wallet(){
        let rand = SystemRandom::new();
        let pkcs8_bytes = Ed25519KeyPair::generate_pkcs8(&rand).unwrap(); // pkcs8 format used for persistent storage
        let key_pair = Ed25519KeyPair::from_pkcs8(pkcs8_bytes.as_ref()).map_err(|_| Unspecified).unwrap();

        println!("private key {}", general_purpose::URL_SAFE_NO_PAD.encode(pkcs8_bytes.as_ref()));
        println!("public key {}", general_purpose::URL_SAFE_NO_PAD.encode(key_pair.public_key()));

    }

    pub fn edd_sign(private_key:String, message:String){
        // decode private key
        let private_key = match general_purpose::URL_SAFE_NO_PAD.decode(private_key){
            Ok(private_key)=>{private_key},
            Err(err)=>{
                return println!("{}", err.to_string())
            }
        };
        let rand = SystemRandom::new();
        let private_key = signature::Ed25519KeyPair::from_pkcs8(private_key.as_slice()).unwrap();
        let sig = private_key.sign(message.as_bytes());
        println!("Signature: {:?}", general_purpose::URL_SAFE_NO_PAD.encode(sig.as_ref()))
    }
    pub fn edd_verify(public_key:String, signature:String, message:String)->bool{
        // decode private key
        let public_key = match general_purpose::URL_SAFE_NO_PAD.decode(public_key){
            Ok(public_key)=>{public_key},
            Err(err)=>{
                println!("{}", err.to_string());
                return false
            }
        };
        let signature = match general_purpose::URL_SAFE_NO_PAD.decode(signature){
            Ok(signature)=>{signature},
            Err(err)=>{
                println!("{}", err.to_string());
                return false
            }
        };
        let public_key = signature::UnparsedPublicKey::new(&ED25519,public_key.as_slice());
        match public_key.verify(message.as_bytes(), signature.as_slice()){
            Ok(_)=>{
                return true
            },
            Err(err)=>{
                println!("error: {}",err);
                return false
            }
        }


    }
    pub fn validate(sender:String, receiver:String, hashed:String, signature:String)->Result<(), Box<dyn Error>>{
        // get sender public kety
        let sender_chain = match Wallet::get_wallet_chain(&sender){
            Ok(sender_chain)=>{sender_chain},
            Err(err)=>{return Err(err.into())}
        };
        let public_key_enc = sender_chain.chain[0].public_key.clone();
        //decode public key
        // let public_key_vec = match general_purpose::URL_SAFE_NO_PAD.decode(public_key_enc){
        //     Ok(public_key_vec)=>{public_key_vec},
        //     Err(err)=>{return Err(err.into())}
        // };
        // construct public key

        let public_key:RsaPublicKey = match RsaPublicKey::from_pkcs1_pem(public_key_enc.as_str()){
            Ok(public_key)=>{public_key},
            Err(err)=>{return Err(err.into())}
        };

        match public_key.verify(Pkcs1v15Sign { hash_len: Some(hashed.as_bytes().len()), prefix: Box::new([]) }, hashed.as_bytes(), signature.as_bytes()){
            Ok(_)=>{},
            Err(err)=>{return Err(err.into())}
        }

        return Ok(())

    }
    pub fn make_transfer(){
        
        
    }
    // transfer value from one wallet to another
    pub fn transfer(sender:String, receiver:String, amount:f32)->Result<(), Box<dyn Error>>{
        
        // get sender public key from last block
        // check if both wallets exist
        let sender_exists = Wallet::wallet_exists(&sender);
        let receiver_exists = Wallet::wallet_exists(&receiver);
        if sender_exists!= true || receiver_exists !=true {
            return Err(Box::from("Wallet does not exist"))
        }
        //check if sender has the money available
        let sender_chain = match Wallet::get_wallet_chain(&sender){
            Ok(sender_chian)=>{sender_chian},
            Err(err)=>{return Err(err.into())}
        };
        let receiver_chain = match Wallet::get_wallet_chain(&receiver){
            Ok(receiver_chain)=>{receiver_chain},
            Err(err)=>{return Err(err.into())}
        };
        let sender_balance = match Wallet::get_balance(&sender){
            Ok(sender_balance)=>{sender_balance},
            Err(err)=> {return Err(err.into())}
        };

        if sender_balance < amount{
            return Err(Box::from("Insufficient funds"))
        }
        // create minus block
        let sender_h = serde_json::to_string(&sender_chain.borrow().chain.last().unwrap());
        let sender_h =match sender_h {
            Ok(json)=>{json},
            Err(err)=>{
                return Err(err.into())
            }
        };
        let receiver_h = serde_json::to_string(&receiver_chain.borrow().chain.last().unwrap());
        let receiver_h =match receiver_h {
            Ok(json)=>{json},
            Err(err)=>{
                return Err(err.into())
            }
        };
        let sender_block = Block{
            id: Uuid::new_v4().to_string(),
            sender_address: sender.to_owned(),
            receiver_address: receiver.to_owned(),
            date_created: get_date_time(),
            hash:sender_h,
            amount: -amount.clone(),
            prev_hash :"".to_string(),
            public_key: sender_chain.chain.last().unwrap().public_key.clone()
        };
        // create add block for receiver
        let receiver_block = Block{
            id: Uuid::new_v4().to_string(),
            sender_address: sender.to_owned(),
            prev_hash :"".to_string(),
            receiver_address: receiver.to_owned(),
            date_created: get_date_time(),
            hash:receiver_h,
            amount: amount.clone(),
            public_key: sender_chain.chain.last().unwrap().public_key.clone()
        };
        // if two blocks are saved well, send response
        match Wallet::save_block(&sender, sender_block){
            Ok(_)=>{},
            Err(err)=>{return Err(err.into())}
        }

        match Wallet::save_block(&receiver, receiver_block){
            Ok(_)=>{},
            Err(err)=>{return Err(err.into())}
        }


        return Ok(())
    }
}