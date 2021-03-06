use chrono::{DateTime, Utc};
use serde::Deserialize;
use serde::Serialize;

#[derive(Default, Serialize, Deserialize, Debug)]
pub struct User {
    user_id: Option<u32>,
    username: String,
    password: String,
    email: String,
    created_on: Option<String>,
    // last_login: DateTime<Utc>,
}

impl User {
    pub fn set_id(&mut self, id: i32) {
        self.user_id = Some(id as u32)
    }

    pub fn get_id(&self) -> i32 {
        self.user_id.unwrap() as i32
    }

    pub fn set_username(&mut self, username: String) {
        self.username = username
    }

    pub fn get_username(&self) -> &String {
        &self.username
    }

    pub fn set_password(&mut self, password: String) {
        self.password = password
    }

    pub fn get_password(&self) -> &String {
        &self.password
    }

    pub fn set_email(&mut self, email: String) {
        self.email = email
    }

    pub fn get_email(&self) -> &String {
        &self.email
    }
    pub fn set_created_on(&mut self, created_on: DateTime<Utc>) {
        self.created_on = Some(created_on.to_rfc3339())
    }

    // pub fn get_created_on(&self) -> String {
    //     self.created_on.unwrap_or_default()
    // }

    // @todo[PG] add https://docs.rs/validator/0.12.0/validator/
}
