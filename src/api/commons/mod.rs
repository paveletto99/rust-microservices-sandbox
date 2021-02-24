pub mod Errors;

//use deadpool_postgres::{ManagerConfig, Pool};
use actix_web::web;
use async_trait::async_trait;
use uuid::Uuid;

#[async_trait]
pub trait ApiController {
    fn setUpService(serviceCfg: &mut web::ServiceConfig);
}

pub fn getUUID() -> String {
    Uuid::new_v4().to_hyphenated().to_string()
}