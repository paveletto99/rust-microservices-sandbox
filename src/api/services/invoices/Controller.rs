use actix_web::{ web, Responder };
use crate::api::services::invoices::Resource::Invoice;
use crate::api::services::invoices::InvoiceServiceManager;

pub struct Controller;

impl Controller {
    
    pub fn setUpService( serviceCfg: &mut web::ServiceConfig ) {
        serviceCfg.service(web::resource("/{uuid}").route(web::get().to(Self::getInvoiceHandler)));
        serviceCfg.service(
            web::resource("/")
                .route(web::get().to(Self::getInvoicesHandler))
                .route(web::post().to(Self::createInvoiceHandler))
                .route(web::delete().to(Self::deleteInvoiceHandler))
                .route(web::put().to(Self::updateInvoiceHandler))
                .route(web::patch().to(Self::updateInvoiceHandler))
        );
    }

    async fn getInvoiceHandler(service: web::Data<InvoiceServiceManager>) -> impl Responder {
        web::Json(Invoice{ customerId: "".to_string(), code: format!("{} {}", "Call INVOICE : ", service.getInvoice().await.to_string()) })
    }

    async fn getInvoicesHandler() -> impl Responder {
        web::Json(Invoice{ customerId: "".to_string(), code: "Call INVOICE : ".to_owned() })
    }

    async fn createInvoiceHandler(service: web::Data<InvoiceServiceManager>) -> impl Responder {
        web::Json(Invoice{ customerId: "".to_string(), code: format!("{} {}", "Call INVOICE : ", service.getInvoice().await.to_string()) })
    }

    async fn updateInvoiceHandler(service: web::Data<InvoiceServiceManager>) -> impl Responder {
        web::Json(Invoice{ customerId: "".to_string(), code: format!("{} {}", "Call INVOICE : ", service.getInvoice().await.to_string()) })
    }

    async fn deleteInvoiceHandler(service: web::Data<InvoiceServiceManager>) -> impl Responder {
        web::Json(Invoice{ customerId: "".to_string(), code: format!("{} {}", "Call INVOICE : ", service.getInvoice().await.to_string()) })
    }
}