use std::error::Error;
use redb::Database;
use serde_json::to_string;
use sha2::{Digest, Sha256};
use crate::blockchain::kv_service::KVService;
use crate::models::block::VBlock;
use crate::models::constants::BLOCKS_TABLE;

pub struct ChainX{

}

impl ChainX {
    pub fn calculate_block_hash(block: &VBlock) -> Result<String, Box<dyn Error>> {
        // Serialize the block data (excluding the hash itself)
        let block_data = to_string(&block)?;

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
    pub fn get_all_blocks(db:&Database) -> Result<Vec<VBlock>, Box<dyn Error>> {
        let blocks = match KVService::get_all_data::<VBlock>(db,BLOCKS_TABLE){
            Ok(blocks) => blocks,
            Err(e) => {
                log::error!("{}", e);
                return Err(e);
            }
        };
        
        return Ok(blocks);
    }
}
