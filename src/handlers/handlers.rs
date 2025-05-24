use std::borrow::BorrowMut;
use std::env::{self, current_dir};
use std::fs::{self, File};
use std::io::{Read, Write};

use std::num::ParseIntError;
use std::str::FromStr;
use std::sync::{Arc};
use bigdecimal::BigDecimal;
use log::{debug, error, info};
use log4rs::append::file;

use sha256::digest;
use sled::Db;
use tokio::net::TcpStream;
use tokio::runtime::Runtime;
use tokio::sync::Mutex;
use uuid::Uuid;
use crate::blockchain::concensus::Concensus;

use crate::blockchain::kv_store::KvStore;
use crate::blockchain::node::Node;
use crate::blockchain::transfer::{Transfer, TransferError};
use crate::blockchain::wallet::{self, Wallet};
use crate::{models, APP_CONFIG};
use crate::models::balance_pack::{BalanceCPack, BalancePack, WalletCPack};
use crate::models::block::{Block, Chain, VBlock};
use crate::models::request::{AddNodeReq, CreateUserIDReq, CreateWalletReq, GetAccountReq, GetBalanceReq, GetUserTransactionsReq, GetWalletReq, TransferReq, ValidateUserIDReq, VerifyWalletReq};
use crate::models::response::{ GetBalanceResp, GetBalanceResponse, NResponse, WalletNamesResp, WalletNamesRespC};
use crate::models::server_list::ServerData;
use crate::models::user_id::UserID;
use crate::models::wallet::{MongoWallet, WalletC};
use crate::utils::constants;

use crate::utils::formatter::Formatter;
use crate::utils::response::{Response, TCPResponse};
use crate::utils::struct_h::Struct_H;

use crate::utils::time::get_date_time;
use crate::utils::utils::{validate_user_name, MyError, MyErrorTypes};
use models::balance_pack;
use crate::blockchain::chain::ChainX;
use crate::blockchain::kv_service2::KVService2;
use crate::blockchain::kv_service::KVService;
use crate::models::account::Account;
use crate::models::constants::BLOCKS_TABLE;
use crate::models::mempool::Mempool;
use crate::models::transaction::Transaction;
use crate::utils::app_error::AppError;

pub struct Handler{

}
// handle sexternal communication from other sources to the blockchain module for any operations




