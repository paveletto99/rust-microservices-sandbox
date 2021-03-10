use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Default)]
pub struct Order {
    id: Uuid,
    code: String,
    createdOn: Option<DateTime<Utc>>
}

impl Order {

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

    pub fn setCreatedOn(&mut self, createdOn: DateTime<Utc>) {
        self.createdOn = Some(createdOn);
    }

    pub fn getCreatedOn(&self) -> DateTime<Utc> {
        // TODO: Must be enhanced
        self.createdOn.unwrap().to_owned()
    }
}