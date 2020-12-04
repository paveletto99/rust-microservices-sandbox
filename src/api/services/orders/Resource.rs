use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct Order {
    pub code: String
}