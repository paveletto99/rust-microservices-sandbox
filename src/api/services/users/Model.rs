// use chrono::{DateTime, Utc};
use serde::Serialize;
use serde::Deserialize;
#[derive(Default, Serialize, Deserialize)]
pub struct User {
    user_id: Option<i32>,
    username: String,
    password: String,
    email: String,
    created_on: Option<String>,
    // last_login: DateTime<Utc>,
}

impl User {
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
    pub fn set_created_on(&mut self, created_on: String) {
        self.created_on = Some(created_on)
    }

    // pub fn get_created_on(&self) -> &String {
    //     &self.created_on.unwrap()
    // }
}
