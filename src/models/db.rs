use std::env;
use lazy_static::lazy_static;
use log::error;
use mongodb::{options::ClientOptions, Client, Database};
use once_cell::sync::OnceCell;


pub struct DB;

impl DB {
    pub fn say_hello() {}
    pub async fn initialize_db() -> Result<Database, mongodb::error::Error> {
        // Parse a connection string into an options struct.
        let mongo_url = match env::var("MONGO_URL"){
            Ok(data)=>{data},
            Err(err)=>{
                error!("{}",err);
                "8000".to_string()
            }
        };
        let mongodb_name = match env::var("MONGODB_NAME"){
            Ok(data)=>{data},
            Err(err)=>{
                error!("{}",err);
                "8000".to_string()
            }
        };
        let mut client_options = ClientOptions::parse(mongo_url).await?;

        // Manually set an option.
        client_options.app_name = Some("kuracoin".to_string());

        // Get a handle to the deployment.
        let client = Client::with_options(client_options)?;
        for db_name in client.list_database_names(None, None).await? {
            println!("{}", db_name);
        }

        let db = client.database(mongodb_name.as_str());
        return Result::Ok(db);
    }
}

static DB_SERVICE:OnceCell<MongoService> = OnceCell::new();


pub struct MongoService{
    pub db:Database
}

impl MongoService{
    pub async fn  init(){

        let mongo_url = match env::var("MONGO_URL"){
            Ok(data)=>{data},
            Err(err)=>{
                error!("{}",err);
                "8000".to_string()
            }
        };
        let mongodb_name = match env::var("MONGODB_NAME"){
            Ok(data)=>{data},
            Err(err)=>{
                error!("{}",err);
                "kuracoin".to_string()
            }
        };
        // Parse a connection string into an options struct.
        let mut client_options = ClientOptions::parse(mongo_url).await.unwrap();

        // Manually set an option.
        client_options.app_name = Some("hdos".to_string());

        // Get a handle to the deployment.
        let client = Client::with_options(client_options);


        let db = client.unwrap().database(mongodb_name.as_str());
        let mongo_service = MongoService{db};
        DB_SERVICE.set(mongo_service);
        return ;
    }

    pub fn get_db() -> Option<& 'static MongoService> {
        DB_SERVICE.get()
    }
}