use mongodb::Database;
use crate::api::commons::Errors::ApplicationError;
use super::{Repository::Repository, Resource::Product as ResourceProduct};
use super::Model::Product;
use uuid::Uuid;

pub struct Service {
    repository: Repository
}

impl Service {

    pub fn New( MongoDB: Database ) -> Self {
        Self { repository: Repository::New(MongoDB) }
    }

    fn getModelFromResource( &self, resource: &ResourceProduct ) -> Result<Product, ApplicationError> {
        // TODO: Put some validation...
        let mut model = Product::New();
        
        model.setId(Uuid::parse_str(resource.getId().to_owned().as_str())?);
        model.setCode(resource.getCode().to_owned());
        model.setCreatedOn(resource.getCreatedOn().to_owned());
        
        Ok(model)
    }

    pub async fn getProduct( &self, productId: &Uuid ) -> Result<Product, ApplicationError> {
        Ok(self.repository.getProduct(productId).await?)
    }

    pub async fn getProducts( &self ) -> Result<Vec<Product>, ApplicationError> {
        Ok(self.repository.getProducts().await?)
    }
    
    pub async fn createProduct( &self, resource: &ResourceProduct ) -> Result<Product, ApplicationError> {
        Ok(self.repository.createProduct(&mut self.getModelFromResource(resource)?).await?)
    }

    pub async fn deleteProduct( &self, productId: &Uuid ) -> Result<Uuid, ApplicationError> {
        Ok(self.repository.deleteProduct(productId).await?)
    }

    pub async fn updateProduct( &self, resource: &ResourceProduct ) -> Result<Product, ApplicationError> {
        Ok(self.repository.updateProduct(&self.getModelFromResource(resource)?).await?)
    }
}

// W.I.P Unit test
#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::clients::MongoDBClient::MongoDBClient;
    use chrono::Utc;
    use mongodb::Database;
    use crate::api::services::products::ProductServiceManager;

    async fn newProduct( service: ProductServiceManager ) -> Result<Product, ApplicationError> {
        let mut product = ResourceProduct::New();
        product.setId(Uuid::new_v4().to_hyphenated().to_string());
        product.setCode("34235325G12345DS".to_string());
        product.setCreatedOn(Utc::now());

        Ok(service.createProduct(&product).await?)
    }

    #[actix_rt::test]
    async fn test_add_new_document_success() {

        let mongodb = MongoDBClient::getMongoDB().await.unwrap();

        match newProduct(Service::New(mongodb)).await {
            Ok(productCreated) => println!("Product created:\n{:?}", productCreated),
            Err(creationErr) => panic!("{:?}", creationErr)
        }
    }

    #[actix_rt::test]
    async fn test_get_document_by_id() {

        let mongodb = MongoDBClient::getMongoDB().await.unwrap();
        let service = ProductServiceManager::New(mongodb.clone());

        // Add a document and return it
        match newProduct(service).await {
            Ok(product) => {
                println!("New Product created: {:?}\nSearching it into the database...", product);

                let createdProductId = product.getId();
                match ProductServiceManager::New(mongodb).getProduct(&createdProductId.clone()).await {
                    Ok(productFound) => {
                        println!("Product found: {:?}\n", productFound);
                        assert_eq!(createdProductId, productFound.getId());
                    },
                    Err(searchErr) => panic!("{:?}", searchErr)
                }
            },
            Err(creationErr) => panic!("{:?}", creationErr)
        }
    }

    #[actix_rt::test]
    async fn test_delete_document() {

        let mongodb = MongoDBClient::getMongoDB().await.unwrap();
        let service = ProductServiceManager::New(mongodb.clone());

        // Add a document and remove it
        match newProduct(service).await {
            Ok(product) => {
                println!("New Product created: {:?}\nDeleting it from the database...", product);

                let newProductId = product.getId();
                match ProductServiceManager::New(mongodb).deleteProduct(&newProductId.clone()).await {
                    Ok(deletedProductId) => println!("Product with ID : {:?} has been deleted; was: {:?}", deletedProductId, newProductId),
                    Err(deletionErr) => panic!("{:?}", deletionErr)
                }
            },
            Err(creationErr) => panic!("{:?}", creationErr)
        }
    }

    #[actix_rt::test]
    async fn test_update_document() {

        let mongodb = MongoDBClient::getMongoDB().await.unwrap();
        let service = ProductServiceManager::New(mongodb.clone());

        // Add a document and update it
        match newProduct(service).await {
            Ok(product) => {
                println!("New Product created: {:?}\nUpdating it...", product);

                let mut newProductVersion = ResourceProduct::New();
                let currentProduct = product.clone();
                newProductVersion.setId(currentProduct.getId().to_string());
                newProductVersion.setCode("ZZZZZZZZZZZZZZZZZZ".to_string());
                newProductVersion.setCreatedOn(Utc::now());

                match ProductServiceManager::New(mongodb).updateProduct(&newProductVersion).await {
                    Ok(updatedProduct) => println!("Product updated : {:?}\nwas: {:?}", updatedProduct, product),
                    Err(updateErr) => panic!("{:?}", updateErr)
                }
            },
            Err(creationErr) => panic!("{:?}", creationErr)
        }
    }
}