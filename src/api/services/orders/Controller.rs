use actix_web::{ web, Responder, HttpResponse };
use super::Resource::Order as ResourceOrder;
use super::OrderServiceManager;
use uuid::Uuid;

pub struct Controller;

impl Controller {
    
    pub fn setUpService( serviceCfg: &mut web::ServiceConfig ) {

        serviceCfg.service(web::resource("").route(web::get().to(Self::getOrdersHandler)));

        serviceCfg.service(
            web::resource("/{uuid}")
                .route(web::get().to(Self::getOrderHandler))
                .route(web::delete().to(Self::deleteOrderHandler))
        );

        serviceCfg.service(
            web::resource("/")
                .route(web::get().to(Self::getOrdersHandler))
                .route(web::post().to(Self::createOrderHandler))
                .route(web::put().to(Self::updateOrderHandler))
                .route(web::patch().to(Self::updateOrderHandler))
        );
    }
    
    async fn getOrderHandler(service: web::Data<OrderServiceManager>, orderId: web::Path<Uuid>) -> impl Responder {
        // TODO: Validate input !?
        match service.getOrder(orderId.into_inner()).await {
            Ok(order) => HttpResponse::Ok().json(order),
            Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
            //_ => HttpResponse::NotFound().body("Order Not found")
        }
    }

    async fn getOrdersHandler(service: web::Data<OrderServiceManager>) -> impl Responder {
        match service.getOrders().await {
            Ok(orders) => HttpResponse::Ok().json(orders),
            Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
            //_ => HttpResponse::NotFound().body("Orders Not found")
        }
    }

    // TODO: To be implemented
    async fn createOrderHandler(_service: web::Data<OrderServiceManager>, _order: web::Json<ResourceOrder>) -> impl Responder {
        web::Json(ResourceOrder::default())
    }

    // TODO: To be implemented
    async fn updateOrderHandler(_service: web::Data<OrderServiceManager>) -> impl Responder {
        web::Json(ResourceOrder::default())
    }

    // TODO: To be implemented
    async fn deleteOrderHandler(_service: web::Data<OrderServiceManager>) -> impl Responder {
        web::Json(ResourceOrder::default())
    }
}