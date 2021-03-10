use async_trait::async_trait;
use actix_web::{web, Responder, HttpResponse};
use crate::api::commons::ApiController;
use super::{Resource::Product as ProductResource, ProductServiceManager};
use uuid::Uuid;

pub struct Controller {}

#[async_trait]
impl ApiController for Controller {
    
    fn setUpService(serviceCfg: &mut web::ServiceConfig ) {

        serviceCfg.service(web::resource("").route(web::get().to(Self::getProductsHandler)));

        serviceCfg.service(
            web::resource("/{uuid}")
                .route(web::get().to(Self::getProductHandler))
                .route(web::delete().to(Self::deleteProductHandler))
        );
        
        serviceCfg.service(
            web::resource("/")
                .route(web::get().to(Self::getProductsHandler))
                .route(web::post().to(Self::createProductHandler))
                .route(web::put().to(Self::updateProductHandler))
                .route(web::patch().to(Self::updateProductHandler))
        );
    }
}

impl Controller {

    async fn getProductHandler(service: web::Data<ProductServiceManager>, productId: web::Path<Uuid>) -> impl Responder {
        match service.getProduct(&productId).await {
            Ok(product) => HttpResponse::Ok().json(product),
            Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
            //_ => HttpResponse::NotFound().finish()
        }
    }

    async fn getProductsHandler(service: web::Data<ProductServiceManager>) -> impl Responder {
        match service.getProducts().await {
            Ok(products) => HttpResponse::Ok().json(products),
            Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
            //_ => HttpResponse::NotFound().finish()
        }
    }
    
    async fn createProductHandler(service: web::Data<ProductServiceManager>, product: web::Json<ProductResource>) -> impl Responder {
        match service.createProduct(&product).await {
            Ok(product) => HttpResponse::Ok().json(product),
            Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
            //_ => HttpResponse::NotFound().finish()
        }
    }

    async fn deleteProductHandler(service: web::Data<ProductServiceManager>, productId: web::Path<Uuid>) -> impl Responder {
        match service.deleteProduct(&productId.into_inner()).await {
            Ok(product) => HttpResponse::Ok().json(product),
            Err(err) => HttpResponse::InternalServerError().body(err.to_string()),
            //_ => HttpResponse::NotFound().finish()
        }
    }

    async fn updateProductHandler(service: web::Data<ProductServiceManager>, product: web::Json<ProductResource>) -> impl Responder {
        match service.updateProduct(&product).await {
            Ok(Product) => HttpResponse::Ok().json(Product),
            Err(_err) => HttpResponse::InternalServerError().finish()
        }
    }
}