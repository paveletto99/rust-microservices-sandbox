use serde::{Serialize};

#[derive(Serialize)]
pub struct Invoice {
    pub customerId: String,
    pub code: String
}