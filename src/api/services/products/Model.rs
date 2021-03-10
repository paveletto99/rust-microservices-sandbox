use serde::{Serialize, Deserialize};
use bson::serde_helpers::uuid_as_binary;
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Product {
    #[serde(rename = "_id", with = "uuid_as_binary")]
    id: Uuid,
    code: String,
    createdOn: DateTime<Utc>
}

impl Product {

    pub fn New() -> Self {
        Self {
            id: Uuid::new_v4(),
            code: "".to_string(),
            createdOn: Utc::now()
        }
    }
    
    pub fn setId(&mut self, productId: Uuid) {
        self.id = productId
    }
    
    pub fn getId(self) -> Uuid {
        self.id
    }
    
    pub fn setCode(&mut self, productCode: String) {
        self.code = productCode
    }

    pub fn getCode(&self) -> &String {
        &self.code
    }

    pub fn setCreatedOn(&mut self, createdOn: DateTime<Utc>) {
        self.createdOn = createdOn;
    }

    pub fn getCreatedOn(&self) -> &DateTime<Utc> {
        &self.createdOn
    }
}