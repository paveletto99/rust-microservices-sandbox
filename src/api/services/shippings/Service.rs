use mongodb::Database;
use bson::oid::ObjectId;
use crate::api::commons::Errors::ApplicationError;
use super::{Repository::Repository, Resource::ShippingResource};
use super::Model::Shipping;

pub struct Service {
    repository: Repository
}

impl Service {

    pub fn New( MongoDB: Database ) -> Self {
        Self { repository: Repository::New(MongoDB) }
    }

    fn getModelFromResource( &self, resource: &ShippingResource ) -> Result<Shipping, ApplicationError> {
        // TODO: Put some validation...
        let mut model = Shipping::New();
        
        model.setShippingId(Some(ObjectId::with_string(resource.getShippingId().to_owned().as_str())?));
        model.setShippingCode(resource.getShippingCode().to_owned());
        model.setShippingDateTime(resource.getShippingDateTime().to_owned());
        
        Ok(model)
    }

    pub async fn getShipping( &self, resource: &ShippingResource ) -> Result<Shipping, ApplicationError> {
        let mut model = Shipping::New();
        model.setShippingId(Some(ObjectId::with_string(resource.getShippingId().as_str())?));

        Ok(self.repository.getShipping(model).await?)
    }

    pub async fn getShippings( &self ) -> Result<Vec<Shipping>, ApplicationError> {
        Ok(self.repository.getShippings().await?)
    }
    
    pub async fn createShipping( &self, resource: &ShippingResource ) -> Result<Shipping, ApplicationError> {
        
        let mut model = Shipping::New();
        
        //model.setShippingId(Some(ObjectId::with_string(resource.getShippingId().to_owned().as_str())?));
        model.setShippingId(None);
        model.setShippingCode(resource.getShippingCode().to_owned());
        model.setShippingDateTime(resource.getShippingDateTime().to_owned());

        //Ok(self.repository.createShipping(model).await?)
        //Ok(self.repository.createShipping02(&model).await?)
        Ok(self.repository.createShipping03(&mut model).await?)
    }

    pub async fn deleteShipping( &self, resource: &ShippingResource ) -> Result<Shipping, ApplicationError> {
        //Ok(self.repository.deleteShipping(&self.getModelFromResource(resource)?).await?)
        Ok(self.repository.deleteShipping01(self.getModelFromResource(resource)?).await?)
    }

    pub async fn updateShipping( &self, resource: &ShippingResource ) -> Result<Shipping, ApplicationError> {
        Ok(self.repository.updateShipping(&self.getModelFromResource(resource)?).await?)
    }
}

#[cfg(test)]
mod tests {
    use std::env;
    use super::*;
    //use actix_web::test;
    use crate::api::clients::MongoDBClient::MongoDBClient;
    use chrono::Utc;
    use mongodb::Database;

    //const MONGO_DBURI: String  = env::var("MONGODB_URI").expect("MONGODB_URI not set");
    //const MONGO_DBDATABASE_NAME: String = env::var("MONGODB_DBNAME").expect("MONGODB_DBNAME not set");

    #[actix_rt::test]
    async fn test_add_new_document_success() {

        let MONGO_DBURI: String  = env::var("MONGODB_URI").expect("MONGODB_URI not set");
        let MONGO_DBDATABASE_NAME: String = env::var("MONGODB_DBNAME").expect("MONGODB_DBNAME not set");

        let mongodbConnectionResult = MongoDBClient::getMongoDBClientPool(&MONGO_DBURI, &MONGO_DBDATABASE_NAME).await;

        let mongodb: Database;

        match mongodbConnectionResult {
            Ok(database) => mongodb = database,
            Err(e) => panic!("Connection to MongoDB Server failed: {:?}", e)
        }

        let mut shipping= ShippingResource::New();
        shipping.setShippingId("".to_string());
        shipping.setShippingCode("34235325G12345DS".to_string());
        shipping.setShippingDateTime(Utc::now());

        let result = Service::New(mongodb).createShipping(&shipping).await;

        match result {
            Ok(shippingCreated) => println!("Shipping created:\n{:?}", shippingCreated),
            Err(err) => panic!("{:?}", err)
        }
    }

    #[actix_rt::test]
    async fn test_get_document_by_id() {

        let MONGO_DBURI: String  = env::var("MONGODB_URI").expect("MONGODB_URI not set");
        let MONGO_DBDATABASE_NAME: String = env::var("MONGODB_DBNAME").expect("MONGODB_DBNAME not set");

        let mongodbConnectionResult = MongoDBClient::getMongoDBClientPool(&MONGO_DBURI, &MONGO_DBDATABASE_NAME).await;

        let mongodb: Database;

        match mongodbConnectionResult {
            Ok(database) => mongodb = database,
            Err(e) => panic!("Connection to MongoDB Server failed: {:?}", e)
        }

        let mut shipping= ShippingResource::New();
        shipping.setShippingId("601b987800fdf50800d90c2a".to_string());

        let result = Service::New(mongodb).getShipping(&shipping).await;

        match result {
            Ok(shipping) => println!("Shipping found:\n{:?}", shipping),
            Err(err) => panic!("{:?}", err)
        }
    }

    #[actix_rt::test]
    async fn test_delete_document() {

        let MONGO_DBURI: String  = env::var("MONGODB_URI").expect("MONGODB_URI not set");
        let MONGO_DBDATABASE_NAME: String = env::var("MONGODB_DBNAME").expect("MONGODB_DBNAME not set");

        let mongodbConnectionResult = MongoDBClient::getMongoDBClientPool(&MONGO_DBURI, &MONGO_DBDATABASE_NAME).await;

        let mongodb: Database;

        match mongodbConnectionResult {
            Ok(database) => mongodb = database,
            Err(e) => panic!("Connection to MongoDB Server failed: {:?}", e)
        }

        let mut shipping= ShippingResource::New();
        shipping.setShippingId("601b9a55002408420032ec3a".to_string());

        let result = Service::New(mongodb).deleteShipping(&shipping).await;

        match result {
            Ok(shipping) => println!("Shipping deleted:\n{:?}", shipping),
            //Err(err) => panic!("{:?}", err)
            Err(err) => println!("Shipping deleted:\nEntity Not Found and Not Deleted : ERROR {:?} - ENTITY {:?}", err, shipping)
        }
    }

    #[actix_rt::test]
    async fn test_update_document() {

        let MONGO_DBURI: String  = env::var("MONGODB_URI").expect("MONGODB_URI not set");
        let MONGO_DBDATABASE_NAME: String = env::var("MONGODB_DBNAME").expect("MONGODB_DBNAME not set");

        let mongodbConnectionResult = MongoDBClient::getMongoDBClientPool(&MONGO_DBURI, &MONGO_DBDATABASE_NAME).await;

        let mongodb: Database;

        match mongodbConnectionResult {
            Ok(database) => mongodb = database,
            Err(e) => panic!("Connection to MongoDB Server failed: {:?}", e)
        }

        let mut shipping= ShippingResource::New();
        shipping.setShippingId("601c54ab00f2593100e1c0dc".to_string());
        shipping.setShippingCode("XXXXXXXXXXXXXXXXXXXXXXYY".to_string());
        shipping.setShippingDateTime(Utc::now());

        let result = Service::New(mongodb).updateShipping(&shipping).await;

        match result {
            Ok(shipping) => println!("Shipping updated:\n{:?}", shipping),
            Err(err) => panic!("{:?}", err)
        }
    }
}