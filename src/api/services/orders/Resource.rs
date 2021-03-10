use chrono::{DateTime, Utc};
use serde::{Serialize, Deserialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Default)]
pub struct Order {
    id: Uuid,
    code: String,
    createdOn: Option<DateTime<Utc>>
}