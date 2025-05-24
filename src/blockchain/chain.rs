use std::error::Error;

use serde_json::to_string;
use sha2::{Digest, Sha256};
use sled::Db;
use crate::blockchain::kv_service2::KVService2;
use crate::blockchain::kv_service::KVService;
use crate::models::block::VBlock;
use crate::models::constants::BLOCKS_TABLE;
use crate::utils::app_error::AppError;
use crate::utils::struct_h::Struct_H;

pub struct ChainX{

}

impl ChainX {
    pub fn calculate_block_hash(block: &VBlock) -> Result<String, AppError> {
        // Serialize the block data (excluding the hash itself)
        let block_data = Struct_H::struct_to_string2(&block)?;

        // Create a Sha256 hasher and input the block data
        let mut hasher = Sha256::new();
        hasher.update(block_data);

        // Get the hash as a byte array
        let hash_bytes = hasher.finalize();

        // Convert the hash to a hexadecimal string
        let hash_hex = format!("{:x}", hash_bytes);

        Ok(hash_hex)
    }
    
    // get all blocks 
    pub async fn get_all_blocks(db:&Db) -> Result<Vec<VBlock>, AppError> {
        let data = match KVService2::get_all_data::<VBlock>(db,BLOCKS_TABLE).await{
            Ok(data) => data,
            Err(e) => {
                log::error!("{}", e);
                return Err(e);
            }
        };
        let mut blocks= vec![];
        for item in data {
            blocks.push(item.1.clone());
        }
        return Ok(blocks);
    }
}
