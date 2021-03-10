use mongodb::{Database, Collection};
use bson::{Document, doc};
use crate::api::commons::Errors::ApplicationError;
use super::Model::Product;
use uuid::Uuid;

pub struct Repository {
    MongoDB: Database
}

impl Repository {

    const COLLECTION_NAME : &'static str = "products";
    
    pub fn New( MongoDB: Database ) -> Self {
        Self { MongoDB }
    }

    fn getCollection(&self) -> Collection {
        self.MongoDB.collection(Self::COLLECTION_NAME)
    }

    pub async fn getProduct( &self, productId: &Uuid ) -> Result<Product, ApplicationError> {

        let mut product = Product::New();
        product.setId(productId.to_owned());

        let result = self.getCollection().find_one(bson::to_document(&product)?, None).await?;

        match result {
            Some(document) => Ok(bson::from_document::<Product>(document)?),
            None => Err(ApplicationError::EntityNotFound),
        }
    }

    pub async fn getProducts( &self ) -> Result<Vec<Product>, ApplicationError> {
        Ok(vec![Product::New(), Product::New(), Product::New()])
    }

    pub async fn createProduct( &self, product: &mut Product ) -> Result<Product, ApplicationError> {

        product.setId(Uuid::new_v4());
        self.getCollection().insert_one(bson::to_document(&product)?.to_owned(), None).await?;

        Ok(product.to_owned())
    }

    pub async fn deleteProduct( &self, productId: &Uuid ) -> Result<Uuid, ApplicationError> {

        let mut product = Product::New();
        product.setId(productId.to_owned());

        self.getCollection().find_one_and_delete(bson::to_document(&product)?, None).await?;
        
        Ok(productId.to_owned())
    }

    pub async fn updateProduct( &self, product: &Product ) -> Result<Product, ApplicationError> {
        
        let document = bson::to_document(&product)?;

        self.getCollection()
            .find_one_and_update(doc! { "_id": document.get_object_id("_id")? }, doc!{ "$set" : document }, None)
            .await?;

        Ok(product.to_owned())
    }
}