use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

#[derive(Debug, Serialize, Deserialize)]
pub struct ShippingResource {
    shippingId: String,
    shippingCode: String,
    shippingDateTime: DateTime<Utc>
}

impl ShippingResource {

    pub fn New() -> Self {
        Self {
            shippingId: "".to_string(),
            shippingCode: "".to_string(),
            shippingDateTime: Utc::now()
        }
    }
    
    pub fn setShippingId(&mut self, shippingId: String) {
        self.shippingId = shippingId
    }
    
    pub fn getShippingId(&self) -> &String {
        &self.shippingId
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