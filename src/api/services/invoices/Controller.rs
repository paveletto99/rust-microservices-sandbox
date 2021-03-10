use actix_web::{ web, Responder, HttpResponse };
use super::Resource::Invoice as ResourceInvoice;
use super::InvoiceServiceManager;
use uuid::Uuid;

pub struct Controller;

impl Controller {
    
    pub fn setUpService( serviceCfg: &mut web::ServiceConfig ) {

        serviceCfg.service(web::resource("").route(web::get().to(Self::getInvoicesHandler)));

        serviceCfg.service(
            web::resource("/{uuid}")
                    .route(web::get().to(Self::getInvoiceHandler))
                    .route(web::delete().to(Self::deleteInvoiceHandler))
        );

        serviceCfg.service(
            web::resource("/")
                .route(web::get().to(Self::getInvoicesHandler))
                .route(web::post().to(Self::createInvoiceHandler))
                .route(web::put().to(Self::updateInvoiceHandler))
                .route(web::patch().to(Self::updateInvoiceHandler))
        );
    }

    async fn getInvoiceHandler(service: web::Data<InvoiceServiceManager>, invoiceId: web::Path<Uuid>) -> impl Responder {
        // TODO: Validate input !?
        match service.getInvoice(invoiceId.into_inner()).await {
            Ok(order) => HttpResponse::Ok().json(order),
            Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
            //_ => HttpResponse::NotFound().body("Invoice Not found")
        }
    }

    async fn getInvoicesHandler(service: web::Data<InvoiceServiceManager>) -> impl Responder {
        match service.getInvoices().await {
            Ok(invoices) => HttpResponse::Ok().json(invoices),
            Err(e) => HttpResponse::InternalServerError().body(e.to_string()),
            //_ => HttpResponse::NotFound().body("Invoices Not found")
        }
    }

    // TODO: To be implemented
    async fn createInvoiceHandler(_service: web::Data<InvoiceServiceManager>, _invoice: web::Json<ResourceInvoice>) -> impl Responder {
        web::Json(ResourceInvoice::default())
    }

    // TODO: To be implemented
    async fn updateInvoiceHandler(_service: web::Data<InvoiceServiceManager>) -> impl Responder {
        web::Json(ResourceInvoice::default())
    }

    // TODO: To be implemented
    async fn deleteInvoiceHandler(_service: web::Data<InvoiceServiceManager>) -> impl Responder {
        web::Json(ResourceInvoice::default())
    }
}