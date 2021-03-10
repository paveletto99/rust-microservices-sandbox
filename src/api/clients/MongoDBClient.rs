use mongodb::error::Error;
use mongodb::{Client, Database};
use std::env;

pub struct MongoDBClient;

impl MongoDBClient  {

    /*
    pub async fn getMongoDBClientPool(MongoDBURI: &String, MongoDBDatabaseName: &String) -> Result<Database, mongodb::error::Error> {
        Ok(Client::with_uri_str(&MongoDBURI).await?.database(&MongoDBDatabaseName))
    }
    */
    pub async fn getMongoDBClientPool(MongoDBURI: &String, MongoDBDatabaseName: &String) -> Result<Database, Error> {
        //Ok(Client::with_uri_str(&MongoDBURI).await.unwrap().database(&MongoDBDatabaseName))

        let client = Client::with_uri_str(&MongoDBURI).await.unwrap();
        Ok(client.database(&MongoDBDatabaseName))
    }

    pub async fn getMongoDB() -> Result<Database, Error> {
        // TODO: Move to Module const
        let MONGO_DBURI: String = env::var("MONGODB_URI").expect("MONGODB_URI not set");
        let MONGO_DBDATABASE_NAME: String = env::var("MONGODB_DBNAME").expect("MONGODB_DBNAME not set");

        Ok(Client::with_uri_str(&MONGO_DBURI).await.unwrap().database(&MONGO_DBDATABASE_NAME))
    }
}