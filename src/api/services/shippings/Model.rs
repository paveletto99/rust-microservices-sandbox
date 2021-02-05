use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Shipping {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    shippingId: Option<bson::oid::ObjectId>,
    shippingCode: String,
    shippingDateTime: DateTime<Utc>
}

impl Shipping {

    pub fn New() -> Self {
        Self {
            shippingId: None,
            shippingCode: "".to_string(),
            shippingDateTime: Utc::now()
        }
    }
    
    pub fn setShippingId(&mut self, shippingId: Option<bson::oid::ObjectId>) {
        self.shippingId = shippingId
    }
    
    pub fn getShippingId(self) -> Option<bson::oid::ObjectId> {
        self.shippingId
    }
    
    pub fn setShippingCode(&mut self, shippingCode: String) {
        self.shippingCode = shippingCode
    }

    pub fn getShippingCode(&self) -> &String {
        &self.shippingCode
    }

    pub fn setShippingDateTime(&mut self, dt: DateTime<Utc>) {
        self.shippingDateTime = dt;
    }

    pub fn getShippingDateTime(&self) -> &DateTime<Utc> {
        &self.shippingDateTime
    }
}