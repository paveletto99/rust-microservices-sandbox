pub mod Errors;

use actix_web::web;
use async_trait::async_trait;

#[async_trait]
pub trait ApiController {
    fn setUpService(serviceCfg: &mut web::ServiceConfig);
}