use std::sync::Arc;
use tokio::{net::TcpListener, signal, spawn, sync::{Mutex, Notify}, time, time::{timeout, Duration}};
use log::{info, error, debug};
use anyhow::Result;

use sled::Db;
use tokio::io::{split, AsyncBufReadExt, AsyncReadExt, BufReader};
use tokio::net::TcpStream;

use crate::{ APP_CONFIG};
use crate::blockchain::transfer::Transfer;

use crate::models::mempool::Mempool;
use crate::models::response::NResponse;
use crate::utils::app_error::AppError;
use crate::utils::response::TCPResponse;

pub struct Node {

}

