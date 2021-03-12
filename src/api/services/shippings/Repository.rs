use mongodb::{Database, Collection, Cursor};
use bson::{Document, doc, from_document};
use crate::api::commons::Errors::ApplicationError;
use super::Model::Shipping;
use futures::{TryFutureExt, StreamExt};

pub struct Repository {
    MongoDB: Database
}

impl Repository {

    const COLLECTION_NAME : &'static str = "shippings";
    
    pub fn New( MongoDB: Database ) -> Self {
        Self { MongoDB }
    }

    fn getCollection(&self) -> Collection {
        self.MongoDB.collection(Self::COLLECTION_NAME)
    }

    fn documentToModel(&self, document: &Document) -> Result<Shipping, ApplicationError> {
        
        let mut model = Shipping::New();
        
        model.setShippingId(Some(document.get_object_id("_id")?.to_owned()));
        model.setShippingCode(document.get_str("code")?.to_string());
        //model.setShippingDateTime(document.get_timestamp::<chrono::DateTime::Utc>("shippingDateTime")?);
        model.setShippingDateTime(document.get_datetime("shippingDateTime")?.to_owned());
        
        Ok(model)
    }
    
    /*
    pub async fn getShipping( &self ) -> Result<Shipping, ApplicationError> {
        
        let result = self.getCollection()
                    .find_one(Some(doc!{ "shipping_code": "12345" }), None )
                    .await?;
        
        let shipping: Shipping;

        match result {
            Some(document) => shipping = bson::from_document(document)?,
            //None => mongodb::error::Error(err),
            //None => Err(ApplicationError::EntityNotFound),
            _ => shipping = Shipping::New()
        };

        //let shipping: Shipping = bson::from_bson(Bson::Document(result.unwrap()))?;

        Ok(shipping)
    }
    */

    pub async fn getShipping( &self, model: Shipping ) -> Result<Shipping, ApplicationError> {
        
        let result = self.getCollection()
                    .find_one(Some(doc!{ "_id": model.getShippingId().unwrap() }), None)
                    .await?;
        
        match result {
            Some(document) => Ok(bson::from_document::<Shipping>(document)?),
            None => Err(ApplicationError::EntityNotFound),
        }
    }

    pub async fn getShippings( &self ) -> Result<Vec<Shipping>, ApplicationError> {

        //Ok(vec![Shipping::New(), Shipping::New(), Shipping::New()])

        let mut cursor = self.getCollection().find(None, None).await?;
        let mut result: Vec<Shipping> = Vec::new();

        while let Some(doc) = cursor.next().await {
            result.push(bson::from_document::<Shipping>(doc.unwrap())?);
        }

        Ok(result)
    }

    pub async fn createShipping( &self, model: Shipping ) -> Result<Shipping, ApplicationError> {
        
        // Convert to a Bson document instance
        let serialized = bson::to_bson(&model)?;
        let serializedDocument = serialized.as_document();
        
        match serializedDocument {
            Some(document) => {
                let result = self.getCollection().insert_one(document.to_owned(), None).await?;
                
                let mut createdModel = model.clone();
                createdModel.setShippingId(Some(result.inserted_id.as_object_id().unwrap().to_owned()));
                Ok(createdModel)
            },
            None => Err(ApplicationError::BsonSerializationError)
        }

        /*
        let result = self.getCollection()
                //.insert_one(document.to_owned(), None).await?;
                .insert_one(serialized.as_document()?.unwrap(), None).await?;

        Ok(Shipping::New())
        */
    }

    pub async fn createShipping02( &self, shipping: &Shipping ) -> Result<Shipping, ApplicationError> {
        
        // Convert to a Bson document instance
        let document = bson::to_document(&shipping)?;
        let result = self.getCollection().insert_one(document.to_owned(), None).await?;

        let mut createdShipping = shipping.clone();
        createdShipping.setShippingId(Some(result.inserted_id.as_object_id().unwrap().to_owned()));
        
        Ok(createdShipping)
    }

    pub async fn createShipping03( &self, shipping: &mut Shipping ) -> Result<Shipping, ApplicationError> {

        let result = self.getCollection().insert_one(bson::to_document(&shipping)?.to_owned(), None).await?;

        shipping.setShippingId(Some(result.inserted_id.as_object_id().unwrap().to_owned()));

        //Ok(shipping.into())
        //Ok(shipping.clone())
        Ok(shipping.to_owned()) // Same as Clone
    }

    pub async fn deleteShipping( &self, shipping: &Shipping ) -> Result<Shipping, ApplicationError> {

        let document = bson::to_document(&shipping)?;

        self.getCollection()
            .find_one_and_delete(doc! { "_id": document.get_object_id("_id")? }, None)
            .await?;
        
        Ok(shipping.to_owned())
    }

    pub async fn deleteShipping01( &self, shipping: Shipping ) -> Result<Shipping, ApplicationError> {

        let result = self.getCollection()
                    .find_one_and_delete(doc! { "_id": shipping.getShippingId().unwrap() }, None)
                    //.map_err(|| ApplicationError::EntityNotFound)
                    //.map_ok_or_else()
                    .await?;

        match result {
            Some(shippingDocument) => Ok(bson::from_document::<Shipping>(shippingDocument)?),
            None => Err(ApplicationError::EntityNotFound)
        }
    }

    pub async fn updateShipping( &self, shipping: &Shipping ) -> Result<Shipping, ApplicationError> {
        
        let document = bson::to_document(&shipping)?;

        self.getCollection()
            // This method returns the previous version of the document so we discard the result and return the model instance passed in unless no error occurs with the update
            // We can build a specific doc! with more granular update on specific fields
            //.find_one_and_update(doc! { "_id": document.get_object_id("_id")? }, doc!{"$set" : { "shippingCode" : document.get_str("shippingCode")? } }, None)
            .find_one_and_update(doc! { "_id": document.get_object_id("_id")? }, doc!{ "$set" : document }, None)
            // TODO: test error handling
            .await?;

        Ok(shipping.to_owned())
    }
}