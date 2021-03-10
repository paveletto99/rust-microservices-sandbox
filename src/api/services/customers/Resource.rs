use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Default)]
pub struct Customer {
    id: Uuid,
    vatCode: String,
    name: String,
    createdOn: Option<DateTime<Utc>>
}