use async_trait::async_trait;
use actix_web::{web, Responder, HttpResponse};
use crate::api::commons::ApiController;
use super::{Resource::ShippingResource, ShippingServiceManager};
pub struct Controller {}

#[async_trait]
impl ApiController for Controller {
    
    fn setUpService(serviceCfg: &mut web::ServiceConfig ) {
        
        serviceCfg.service(
            web::resource("/{uuid}")
                .route(web::get().to(Self::getShippingHandler))
                .route(web::delete().to(Self::deleteShippingHandler))
        );
        
        serviceCfg.service(
            web::resource("/")
                .route(web::get().to(Self::getShippingsHandler))
                .route(web::post().to(Self::createShippingHandler))
                .route(web::put().to(Self::updateShippingHandler))
                .route(web::patch().to(Self::updateShippingHandler))
        );
    }
}

impl Controller {

    async fn getShippingHandler(service: web::Data<ShippingServiceManager>, shipping: web::Json<ShippingResource>) -> impl Responder {
        match service.getShipping(&shipping).await {
            Ok(shipping) => HttpResponse::Ok().json(shipping),
            Err(_err) => HttpResponse::NotFound().finish()
        }
    }

    async fn getShippingsHandler(service: web::Data<ShippingServiceManager>) -> impl Responder {
        match service.getShippings().await {
            Ok(shippings) => HttpResponse::Ok().json(shippings),
            Err(_err) => HttpResponse::InternalServerError().finish()
        }
    }
    
    async fn createShippingHandler(service: web::Data<ShippingServiceManager>, shipping: web::Json<ShippingResource>) -> impl Responder {
        match service.createShipping(&shipping).await {
            Ok(shipping) => HttpResponse::Ok().json(shipping),
            Err(_err) => HttpResponse::InternalServerError().finish()
        }
    }

    async fn deleteShippingHandler(service: web::Data<ShippingServiceManager>, shipping: web::Json<ShippingResource>) -> impl Responder {
        match service.deleteShipping(&shipping).await {
            Ok(shipping) => HttpResponse::Ok().json(shipping),
            Err(_err) => HttpResponse::InternalServerError().finish()
        }
    }

    async fn updateShippingHandler(service: web::Data<ShippingServiceManager>, shipping: web::Json<ShippingResource>) -> impl Responder {
        //HttpResponse::BadRequest()
        match service.updateShipping(&shipping).await {
            Ok(shipping) => HttpResponse::Ok().json(shipping),
            Err(_err) => HttpResponse::InternalServerError().finish()
        }
    }
}