use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Default)]
pub struct NewCustomer {
    vatCode: String,
    companyName: String
}

#[derive(Serialize, Deserialize, Default)]
pub struct UpdateCustomer {
    id: Uuid,
    vatCode: String,
    companyName: String
}