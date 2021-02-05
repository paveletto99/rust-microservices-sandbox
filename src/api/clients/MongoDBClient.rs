use mongodb::error::Error;
use mongodb::{Client, Database};

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
}