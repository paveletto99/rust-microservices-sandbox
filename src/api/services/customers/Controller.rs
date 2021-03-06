use async_trait::async_trait;
use actix_web::{ web, Responder, HttpResponse };
use super::CustomerServiceManager;
use super::{Resource::NewCustomer as NewCustomerResource, Resource::UpdateCustomer as UpdateCustomerResource};
use crate::api::commons::ApiController;
use uuid::Uuid;

pub struct Controller {}

#[async_trait]
impl ApiController for Controller {
    
    fn setUpService(serviceCfg: &mut web::ServiceConfig ) {

        serviceCfg.service(web::resource("").route(web::get().to(Self::getCustomersHandler)));

        serviceCfg.service(
            web::resource("/{uuid}")
                .route(web::get().to(Self::getCustomerHandler))
                .route(web::delete().to(Self::deleteCustomerHandler))
        );

        serviceCfg.service(
            web::resource("/")
                .route(web::get().to(Self::getCustomersHandler))
                .route(web::post().to(Self::createCustomerHandler))
                .route(web::put().to(Self::updateCustomerHandler))
                .route(web::patch().to(Self::updateCustomerHandler))
        );
    }
}

impl Controller {

    async fn getCustomerHandler(service: web::Data<CustomerServiceManager>, customerId: web::Path<Uuid>) -> impl Responder {
        // TODO: Validate input !?
        match service.getCustomer(customerId.into_inner()).await {
            Ok(customer) => HttpResponse::Ok().json(customer),
            Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
            //_ => HttpResponse::NotFound().body("Customer Not found")
        }
    }

    async fn getCustomersHandler(service: web::Data<CustomerServiceManager>) -> impl Responder {
        match service.getCustomers().await {
            Ok(customers) => HttpResponse::Ok().json(customers),
            Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
            //_ => HttpResponse::NotFound().body("Customers Not found")
        }
    }

    // TODO: To be implemented
    async fn createCustomerHandler(_service: web::Data<CustomerServiceManager>, customer: web::Json<NewCustomerResource>) -> impl Responder {
        HttpResponse::Created().json(customer.into_inner())
    }

    // TODO: To be implemented
    async fn updateCustomerHandler(service: web::Data<CustomerServiceManager>, customer: web::Json<UpdateCustomerResource>) -> impl Responder {
        HttpResponse::Ok().json(customer.into_inner())
    }

    // TODO: To be implemented
    async fn deleteCustomerHandler(_service: web::Data<CustomerServiceManager>, customerId: web::Path<Uuid>) -> impl Responder {
        HttpResponse::Accepted().json(customerId.into_inner())
    }
}