use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Default)]
pub struct Customer {
    id: Uuid,
    vatCode: String,
    companyName: String,
    createdOn: Option<DateTime<Utc>>
}

impl Customer {

    pub fn setId(&mut self, id: Uuid) {
        self.id = id
    }

    pub fn getId(&self) -> &Uuid {
        &self.id
    }

    pub fn setVatCode(&mut self, vatCode: String) {
        self.vatCode = vatCode
    }

    pub fn getVatCode(&self) -> &String {
        &self.vatCode
    }

    pub fn setCompanyName(&mut self, companyName: String) {
        self.companyName = companyName
    }

    pub fn getCompanyName(&self) -> &String {
        &self.companyName
    }

    pub fn setCreatedOn(&mut self, createdOn: DateTime<Utc>) {
        self.createdOn = Some(createdOn);
    }

    pub fn getCreatedOn(&self) -> DateTime<Utc> {
        // TODO: Must be enhanced
        self.createdOn.unwrap().to_owned()
    }
}