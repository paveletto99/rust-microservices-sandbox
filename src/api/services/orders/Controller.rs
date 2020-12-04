use actix_web::{ web, Responder, HttpResponse };
use super::Resource::Order;
use super::OrderServiceManager;
pub struct Controller;

impl Controller {
    
    pub fn setUpService( serviceCfg: &mut web::ServiceConfig ) {
        
        serviceCfg.service(web::resource("/{uuid}").route(web::get().to(Self::getOrderHandler)));
        serviceCfg.service(
            web::resource("/")
                .route(web::get().to(Self::getOrdersHandler))
                .route(web::post().to(Self::createOrderHandler))
                .route(web::delete().to(Self::deleteOrderHandler))
                .route(web::put().to(Self::updateOrderHandler))
                .route(web::patch().to(Self::updateOrderHandler))
        );
    }
    
    async fn getOrderHandler(service: web::Data<OrderServiceManager>) -> impl Responder {
        match service.getOrder().await {
            Ok(order) => HttpResponse::Ok().json(Order{ code: order.getCode().to_string() }),
            _ => HttpResponse::BadRequest().body("Error trying to read order from database")
        }
    }

    async fn getOrdersHandler() -> impl Responder {
        web::Json(Order{ code : "Order GET ".to_string() })
    }

    async fn createOrderHandler(service: web::Data<OrderServiceManager>, order: web::Json<Order>) -> impl Responder {
        web::Json(Order{ code: format!("{} {}", "Order POST : ", "".to_string()) })
    }

    async fn updateOrderHandler(service: web::Data<OrderServiceManager>) -> impl Responder {
        web::Json(Order{ code: format!("{} {}", "Order PUT | PATCH : ", "".to_string()) })
    }

    async fn deleteOrderHandler(service: web::Data<OrderServiceManager>) -> impl Responder {
        web::Json(Order{ code: format!("{} {}", "Order : DELETE", "".to_string()) })
    }
}