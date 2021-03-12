use mongodb::error::Error;
use mongodb::{Client, Database};

// TODO: Remove it - defined here for bulk loading some data
use serde::{Serialize, Deserialize};
use bson::{Document, serde_helpers::uuid_as_binary};
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Product {
    #[serde(rename = "_id", with = "uuid_as_binary")]
    id: Uuid,
    code: String,
    createdOn: DateTime<Utc>
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Shipping {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    shippingId: Option<bson::oid::ObjectId>,
    shippingCode: String,
    shippingDateTime: DateTime<Utc>
}

//

pub struct MongoDB;

impl MongoDB  {

    pub async fn migrate(mongodb: Database) -> Result<(), Error> {

        let howMany: i32 = 100;

        // Bulk load some data
        mongodb.collection("products").drop(None).await; // Only for example data!!
        mongodb.collection("shippings").drop(None).await; // Only for example data!!

        let productsCollection = mongodb.collection("products");
        let mut productsDocs: Vec<Document> = Vec::with_capacity(howMany as usize);
        let mut product: Product;

        for i in 1..=howMany {

            product = Product{
                id: Uuid::new_v4(),
                code: Uuid::new_v4().to_string(),
                createdOn: Utc::now()
            };

            productsDocs.push(bson::to_document(&product).unwrap());
        }

        if productsDocs.len() > 0 {
            productsCollection.insert_many(productsDocs, None).await?;
        }

        let shippingsCollection = mongodb.collection("shippings");
        let mut shippingsDocs: Vec<Document> = Vec::with_capacity(howMany as usize);
        let mut shipping: Shipping;

        for i in 1..=howMany {

            shipping = Shipping{
                shippingId: None,
                shippingCode: Uuid::new_v4().to_string(),
                shippingDateTime: Utc::now()
            };

            shippingsDocs.push(bson::to_document(&shipping).unwrap());
        }

        if shippingsDocs.len() > 0 {
            shippingsCollection.insert_many(shippingsDocs, None).await?;
        }

        Ok(())
    }
}