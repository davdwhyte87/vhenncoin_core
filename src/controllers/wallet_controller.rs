use std::borrow::Borrow;
use std::fs;
use std::fs::File;
use std::path::Path;
use std::str::FromStr;
use std::sync::Arc;
use actix_web::{get, HttpResponse, post, web};
use actix_web::dev::ResourcePath;
use actix_web::web::{Data, Json, JsonBody, ReqData};
use bigdecimal::BigDecimal;
use log::{debug, error};
use sled::Db;
use validator::Validate;
use crate::blockchain::transfer::Transfer;
use crate::blockchain::wallet::Wallet;
use crate::models::account::Account;
use crate::models::request::{CreateWalletReq, GetAccountReq, GetBalanceReq, TransferReq, VerifyWalletReq};
use crate::models::response::{GenericResp, GetBalanceResp, NResponse};
use crate::models::transaction::Transaction;
use crate::utils::app_error::AppError;
use crate::utils::response::TCPResponse;

const ROUTE_NAME: &str = "wallet";

#[post("/create_wallet")]
pub async fn create_wallet(
    db:Data<Arc<Db>>,
    req: Result<web::Json<CreateWalletReq>, actix_web::Error>,
)->HttpResponse {
    let mut resp_data = GenericResp::<String> {
        message: "".to_string(),
        status: 0,
        data: None
    };

    let req = match req {
        Ok(data)=>{data},
        Err(err)=>{
            log::error!("validation  error  {}", err.to_string());
            resp_data.message = "Validation error".to_string();
            resp_data.status = 0;
            resp_data.data = None;
            return HttpResponse::BadRequest().json( resp_data);
        }
    };
    let s = &req.address;
    if !s.chars().all(|c| !c.is_alphabetic() || c.is_lowercase()) {
        log::debug!("{}", "address must be lowercase");
        resp_data.message = "address must be lowercase".to_string();
        resp_data.status = 0;
        resp_data.data = None;
        return HttpResponse::BadRequest().json( resp_data);
    }
    if req.address.to_owned().contains(char::is_whitespace){
        log::debug!("{}", "address contains white space");
        resp_data.message = "address contains white space".to_string();
        resp_data.status = 0;
        resp_data.data = None;
        return HttpResponse::BadRequest().json( resp_data);
    }

    // check if account exists
    let account = match Wallet::get_user_account(&db,req.address.clone()).await{
        Ok(account)=>{account},
        Err(err)=>{
            error!("{}", err.to_string());
            resp_data.message = "error gettting account".to_string();
            resp_data.status = 0;
            resp_data.data = None;
            return HttpResponse::InternalServerError().json( resp_data);
        }
    };
    if account.is_some(){
        resp_data.message = "Account already exists".to_string();
        resp_data.status = 0;
        resp_data.data = None;
        return HttpResponse::BadRequest().json( resp_data);
    }

    //create wallet
    match Wallet::create_wallet_r(&db, req.into_inner()).await{
        Ok(_)=>{
        },
        Err(err)=>{
            error!("{}", err.to_string());
            resp_data.message = "error creating wallet".to_string();
            resp_data.status = 0;
            resp_data.data = None;
            return HttpResponse::InternalServerError().json( resp_data);
        }
    };

    resp_data.status = 1;
    resp_data.data = None;
    resp_data.message = "Wallet created!".to_string();
    return HttpResponse::Ok().json(resp_data)
}

#[post("/transfer")]
pub async fn transfer(
    db:Data<Arc<Db>>,
    req: Result<web::Json<TransferReq>, actix_web::Error>,
)->HttpResponse {
    let mut resp_data = GenericResp::<String> {
        message: "".to_string(),
        status: 0,
        data: None
    };

    let req = match req {
        Ok(data)=>{data},
        Err(err)=>{
            log::error!("validation  error  {}", err.to_string());
            resp_data.message = "Validation error".to_string();
            resp_data.status = 0;
            resp_data.data = None;
            return HttpResponse::InternalServerError().json( resp_data);
        }
    };

    if &req.sender == &req.receiver{
        resp_data.message = "cannot send to self".to_string();
        resp_data.status = 0;
        resp_data.data = None;
        return HttpResponse::BadRequest().json( resp_data);
    }


    let amount = match BigDecimal::from_str(&*req.amount.clone()){
        Ok(amount)=>{amount},
        Err(err)=>{
            debug!("{}",err.to_string());
            resp_data.message = "Invalid amount".to_string();
            resp_data.status = 0;
            resp_data.data = None;
            return HttpResponse::BadRequest().json( resp_data);
        }
    };



    let tx = Transaction{
        id:req.id.clone(),
        sender: req.sender.clone(),
        receiver: req.receiver.clone(),
        amount: amount.clone(),
        signature: req.signature.clone(),
        timestamp: req.timestamp.clone(),
    };

    let mut message = String::new();
    match Wallet::verify_transaction_signature(&db, req.into_inner()).await{
        Ok(is_ok) => {
            if !is_ok{
                resp_data.message = "Error verifying signature".to_string();
                resp_data.status = 0;
                resp_data.data = None;
                return HttpResponse::InternalServerError().json( resp_data);
            }
        },
        Err(err)=>{
            error!("{}", err.to_string());
            match err{
                AppError::AccountNotFound(..)=>{
                    message = "Account not found".to_string()
                },
                _=>{
                    message = "Error verifying transaction signature".to_string();
                }
            }
            resp_data.message = message;
            resp_data.status = 0;
            resp_data.data = None;
            return HttpResponse::InternalServerError().json( resp_data);

        }
    };

    match Transfer::process_single_bf_transaction(&db, tx).await{
        Ok(_)=>{},
        Err(err)=>{
            error!("{}", err.to_string());
            let mut message = String::new();
            match err{
                AppError::InsufficientFunds =>{
                    message = "insufficient funds!".to_string();
                },
                AppError::IDMismatch=>{
                    message = "Invalid transaction!".to_string();
                },
                AppError::TransactionAlreadyExists=>{
                    message = "Transaction already exists!".to_string();
                },
                AppError::AccountNotFound(..)=>{
                    message = "Account not found!".to_string();
                }
                _=>{
                    message = "Error transferring coin".to_string();
                }
            }
            resp_data.message = message;
            resp_data.status = 0;
            resp_data.data = None;
            return HttpResponse::InternalServerError().json( resp_data);
        }
    };
    resp_data.message = "Successful transfer".to_string();
    resp_data.status = 1;
    resp_data.data = None;
    HttpResponse::Ok().json(resp_data)
}


#[post("/get_account")]
pub async fn get_account(
    db:Data<Arc<Db>>,
    req: Result<web::Json<GetAccountReq>, actix_web::Error>,
)->HttpResponse {
    let mut resp_data = GenericResp::<Account> {
        message: "".to_string(),
        status: 0,
        data: None
    };

    let req = match req {
        Ok(data)=>{data},
        Err(err)=>{
            log::error!("validation  error  {}", err.to_string());
            resp_data.message = "Validation error".to_string();
            resp_data.status = 0;
            resp_data.data = None;
            return HttpResponse::BadRequest().json( resp_data);
        }
    };


    // check if account exists
    let account = match Wallet::get_user_account(&db,req.address.clone()).await{
        Ok(account)=>{account},
        Err(err)=>{
            error!("{}", err.to_string());
            resp_data.message = "error gettting account".to_string();
            resp_data.status = 0;
            resp_data.data = None;
            return HttpResponse::InternalServerError().json( resp_data);
        }
    };
    if account.is_none(){
        resp_data.message = "Account does not ecist".to_string();
        resp_data.status = 0;
        resp_data.data = None;
        return HttpResponse::BadRequest().json( resp_data);
    }

    let mut my_account = account.unwrap_or_default();
    my_account.get_balance();
    resp_data.status = 1;
    resp_data.data = Some(my_account);
    resp_data.message = "Ok!".to_string();
    return HttpResponse::Ok().json(resp_data)
}

#[post("/get_balance")]
pub async fn get_balance(
    db:Data<Arc<Db>>,
    req: Result<web::Json<GetBalanceReq>, actix_web::Error>,
)->HttpResponse {
    let mut resp_data = GenericResp::<GetBalanceResp> {
        message: "".to_string(),
        status: 0,
        data: None
    };

    let req = match req {
        Ok(data)=>{data},
        Err(err)=>{
            log::error!("validation  error  {}", err.to_string());
            resp_data.message = "Validation error".to_string();
            resp_data.status = 0;
            resp_data.data = None;
            return HttpResponse::BadRequest().json( resp_data);
        }
    };


    // check if account exists
    let account = match Wallet::get_user_account(&db,req.address.clone()).await{
        Ok(account)=>{account},
        Err(err)=>{
            error!("{}", err.to_string());
            resp_data.message = "error gettting account".to_string();
            resp_data.status = 0;
            resp_data.data = None;
            return HttpResponse::InternalServerError().json( resp_data);
        }
    };
    if account.is_none(){
        resp_data.message = "Account does not ecist".to_string();
        resp_data.status = 0;
        resp_data.data = None;
        return HttpResponse::BadRequest().json( resp_data);
    }

    let mut my_account = account.unwrap_or_default();
    my_account.get_balance();
    resp_data.status = 1;
    resp_data.data = Some(GetBalanceResp{
        address: my_account.address,
        balance: my_account.balance,
    });
    resp_data.message = "Ok!".to_string();
    return HttpResponse::Ok().json(resp_data)
}

#[post("/verify_account")]
pub async fn verify_account(
    db:Data<Arc<Db>>,
    req: Result<web::Json<VerifyWalletReq>, actix_web::Error>,
)->HttpResponse {
    let mut resp_data = GenericResp::<Account> {
        message: "".to_string(),
        status: 0,
        data: None
    };

    let req = match req {
        Ok(data)=>{data},
        Err(err)=>{
            log::error!("validation  error  {}", err.to_string());
            resp_data.message = "Validation error".to_string();
            resp_data.status = 0;
            resp_data.data = None;
            return HttpResponse::BadRequest().json( resp_data);
        }
    };

    let mut message = String::new();
    let is_ok =match Wallet::verify_signature(&db, req.message.to_owned(), req.address.to_owned(), req.signature.to_owned()).await{
        Ok(d) => {
            d
        },
        Err(err)=>{
            error!("{}", err.to_string());
            match err{
                AppError::AccountNotFound(..)=>{
                    message = "Account not found!".to_string();
                },
                _=>{
                    message = "Error verifying signature".to_string();
                }
            }
            resp_data.message = message.clone();
            resp_data.status = 0;
            resp_data.data = None;
            return HttpResponse::InternalServerError().json( resp_data);
        }
    };

    if(!is_ok) {
        resp_data.message = "Verification failed.".to_string();
        resp_data.status = 0;
        resp_data.data = None;
        return HttpResponse::BadRequest().json( resp_data);
    }

    resp_data.status = 1;
    resp_data.data = None;
    resp_data.message = "Ok!".to_string();
    HttpResponse::Ok().json(resp_data)
}