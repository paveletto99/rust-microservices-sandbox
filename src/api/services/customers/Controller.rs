use async_trait::async_trait;
use actix_web::{ web, Responder };
use serde::{Serialize};
use crate::api::services::customers::CustomerServiceManager;
use crate::api::commons::ApiController;

#[derive(Serialize)]
struct User {
    name: String
}

pub struct Controller {}

#[async_trait]
impl ApiController for Controller {
    
    fn setUpService(serviceCfg: &mut web::ServiceConfig ) {
        serviceCfg.service(web::resource("/customer").route(web::get().to(Self::getCustomerHandler)));
        serviceCfg.service(web::resource("/{uuid}").route(web::get().to(Self::getCustomerHandler)));
        serviceCfg.service(web::resource("/customers").route(web::get().to(Self::getCustomersHandler)));
    }
}

impl Controller {

    async fn getCustomerHandler(service: web::Data<CustomerServiceManager>) -> impl Responder {
        web::Json(User{ name: format!("{} {}", "Call CUSTOMER : ", service.getCustomer().await.to_string()) })
    }

    async fn getCustomersHandler(_service: web::Data<CustomerServiceManager>) -> impl Responder {
        web::Json(User{ name: "Call CUSTOMER : ".to_string() })
    }
}