use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct Product {
    id: String,
    code: String,
    createdOn: DateTime<Utc>
}

impl Product {

    pub fn New() -> Self {
        Self {
            id: "".to_string(),
            code: "".to_string(),
            createdOn: Utc::now()
        }
    }
    
    pub fn setId(&mut self, productId: String) {
        self.id = productId
    }
    
    pub fn getId(&self) -> &String {
        &self.id
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