use std::error::Error;
use log::error;
use mongodb::{options::ClientOptions, Client, Database};
use mongodb::bson::{bson, doc, to_bson, to_document};
use mongodb::results::{InsertOneResult, UpdateResult};
use crate::models::block::Chain;
use crate::models::wallet::MongoWallet;

const COLLECTION_NAME:&str = "Mongo_Wallet";


pub struct WalletService {

}

impl WalletService {
    pub async fn create(db: &Database, wallet: &MongoWallet) -> Result<InsertOneResult, Box<dyn Error>> {
        // Get a handle to a collection in the database.
        let collection = db.collection::<MongoWallet>(COLLECTION_NAME);
        let filter = doc! {"address":wallet.address.to_owned()};
        let res = collection.find_one(filter, None).await;
        match res {
            Ok(run_info_res)=>{
                match run_info_res {
                Some(run_info)=>{
                    return Err(Box::from("Wallet exists"))
                },
                None=>{
                }
            }},
            Err(err)=>{return Err(err.into())}
        };
        let res_diag =collection.insert_one(wallet, None).await;
        match res_diag {
            Ok(res_)=>{return Ok(res_)},
            Err(err)=>{return Err(err.into())}
        }
    }

    pub async fn get_by_address(db: &Database,address:String)->Result<Option<MongoWallet>, Box<dyn Error>>{
        let filter = doc! {"address":address};
        let collection = db.collection::<MongoWallet>(COLLECTION_NAME);
        let wallet_detail = collection.find_one(filter, None).await;
        let wallet_detail = match wallet_detail {
            Ok(user_detail)=>{
                user_detail
            },
            Err(err)=>{return Err(err.into())}
        };
        Ok(wallet_detail)
    }
    
    pub async fn update(db:&Database, address:String, wallet:&MongoWallet) ->Result<UpdateResult, Box<dyn Error>>{
        let filter = doc! {"address":address};
        let collection = db.collection::<MongoWallet>(COLLECTION_NAME);
        let new_doc = doc! {
            "$set": to_bson(wallet).unwrap()
        };
        let updated_doc = collection.update_one(filter,new_doc, None )
            .await;

        match updated_doc {
            Ok(updated_doc)=>{return Ok(updated_doc)},
            Err(err)=>{
                return Err(err.into())
            }
        }
    }


    pub async fn update_schema(db:&Database) ->Result<UpdateResult, Box<dyn Error>>{
        let filter = doc! {};
        let collection = db.collection::<MongoWallet>(COLLECTION_NAME);
        let new_doc = doc! {
            "$set": {"chain":{
                "chain":[{"transaction_id":""}]
            }
        }
        };
        let updated_doc = collection.update_many(filter,new_doc, None )
            .await;

        match updated_doc {
            Ok(updated_doc)=>{return Ok(updated_doc)},
            Err(err)=>{
                return Err(err.into())
            }
        }
    }
}
