use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize, Default)]
pub struct Invoice {
    id: Uuid,
    code: String,
    customerId: Uuid,
    createdOn: Option<DateTime<Utc>>
}