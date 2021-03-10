use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Default)]
pub struct Invoice {
    id: Uuid,
    code: String,
    customerId: Uuid,
    createdOn: Option<DateTime<Utc>>
}

impl Invoice {

    pub fn setId(&mut self, id: Uuid) {
        self.id = id
    }

    pub fn getId(&self) -> &Uuid {
        &self.id
    }

    pub fn setCode(&mut self, code: String) {
        self.code = code
    }
    
    pub fn getCode(&self) -> &String {
        &self.code
    }

    pub fn setCustomerId(&mut self, customerId: Uuid) {
        self.customerId = customerId
    }

    pub fn getCustomerId(self) -> Uuid {
        self.customerId
    }

    pub fn setCreatedOn(&mut self, createdOn: DateTime<Utc>) {
        self.createdOn = Some(createdOn);
    }

    pub fn getCreatedOn(&self) -> DateTime<Utc> {
        // TODO: Must be enhanced
        self.createdOn.unwrap().to_owned()
    }
}