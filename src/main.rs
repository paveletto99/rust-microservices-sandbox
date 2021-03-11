#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused)]

use actix_files::Files;
use actix_web::{http, middleware, web, App, HttpResponse, HttpServer, error};
use mongodb::Client;
use std::env;
use uuid::Uuid;

use crate::api::clients::PostgresClient::PostgresClient;

// Application Modules
mod api;
use api::commons::ApiController;
// Customers API
use api::services::customers::CustomerController;
use api::services::customers::CustomerServiceManager;
// Products API
use api::services::products::ProductController;
use api::services::products::ProductServiceManager;
// Orders API
use api::services::orders::OrderController;
use api::services::orders::OrderServiceManager;
// Invoices API
use api::services::invoices::InvoiceController;
use api::services::invoices::InvoiceServiceManager;
// Shippings API
use api::services::shippings::ShippingController;
use api::services::shippings::ShippingServiceManager;
// Users API
use api::services::users::UserController;
use api::services::users::UserServiceManager;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Logging
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    // PostgreSQL Connection Pool
    let pool = PostgresClient::get_default_pool().await.unwrap();

    // MongoDB Connection Pool
    let MongoDBURI = env::var("MONGODB_URI").expect("MONGODB_URI not set");
    let MongoDBDatabaseName = env::var("MONGODB_DBNAME").expect("MONGODB_DBNAME not set");
    let MongoDBClient = Client::with_uri_str(&MongoDBURI).await.unwrap();
    let MongoDB = MongoDBClient.database(&MongoDBDatabaseName);

    // MongoDB Alternative connection method for more control over the Connection Pooler and Read/Write Concerns
    /*
    let MongoDBClientOptions = ClientOptions::builder()
                                      .hosts(vec![StreamAddress{hostname: "localhost".to_string(), port: Some(27017)}])
                                      .max_pool_size(Some(200))
                                      .min_pool_size(Some(5))
                                      .build();

    let MongoDBClient = Client::with_options(MongoDBClientOptions).unwrap();
    let MongoDB = MongoDBClient.database(&env::var("MONGODB_DBNAME").expect("MONGODB_DBNAME not set"));
    */

    // HttpServer
    HttpServer::new(move || {
        App::new()
            // Default Middlewares
            .wrap(middleware::Logger::default())
            .wrap(middleware::Compress::default())
            .wrap(
                middleware::DefaultHeaders::new()
                    .header("X-Version", "1.0")
                    .header("X-Request-ID", Uuid::new_v4().to_hyphenated().to_string())
                    .header(http::header::ACCESS_CONTROL_ALLOW_ORIGIN, "*")
                    .header(http::header::ACCESS_CONTROL_ALLOW_METHODS, "GET, POST, PUT, PATCH")
                    /*
                    .header(http::header::CONTENT_SECURITY_POLICY, "default-src 'self'")
                    .header(http::header::X_FRAME_OPTIONS, "sameorigin")
                    .header(http::header::X_CONTENT_TYPE_OPTIONS, "nosniff")
                    .header(http::header::REFERRER_POLICY, "origin-when-cross-origin")
                    .header("Clear-Site-Data", "*")
                    */
            )
            // Liveness probe | Readiness probe
            .route("/healthz", web::get().to(|| HttpResponse::Ok().finish()))
            .data( // TODO: debug Path Extractor
                web::PathConfig::default()
                    .error_handler(|err, _| error::InternalError::from_response(err,HttpResponse::InternalServerError().body("Invalid Path")).into())
            )
            .service(
                web::scope("/api").service(
                    web::scope("/v1")
                        .data(
                            // Json extractor configuration
                            web::JsonConfig::default()
                                // Limit request payload size
                                .limit(4096)
                                // Handling deserialization errors
                                .error_handler(|err, _| error::InternalError::from_response(err, HttpResponse::InternalServerError().body("Invalid JSON payload")).into())
                        )
                        .wrap(middleware::DefaultHeaders::new().header(
                            "Authorization",
                            format!("{}{}", "Bearer ", Uuid::new_v4().to_simple()),
                        )) // Example setting response Headers e.g: JWT Token
                        .data(pool.clone()) // Passing PostgreSQL Connection Pooler to the Extractor
                        .service(
                            // Mount CustomerServiceManager
                            web::scope("/customers")
                                .data(CustomerServiceManager::New(pool.clone())) // Passing Service Manager Instance to the Extractor
                                .configure(CustomerController::setUpService), // Mount routes
                        )
                        .service(
                            // Mount ProductServiceManager
                            web::scope("/products")
                                .data(ProductServiceManager::New(MongoDB.clone()))
                                .configure(ProductController::setUpService),
                        )
                        .service(
                            // Mount OrderServiceManager
                            web::scope("/orders")
                                .data(OrderServiceManager::New(pool.clone()))
                                .configure(OrderController::setUpService),
                        )
                        .service(
                            // Mount InvoiceServiceManager
                            web::scope("/invoices")
                                .data(InvoiceServiceManager::New(pool.clone()))
                                .configure(InvoiceController::setUpService),
                        )
                        .service(
                            // Mount ShippingServiceManager
                            web::scope("/shippings")
                                .data(ShippingServiceManager::New(MongoDB.clone()))
                                .configure(ShippingController::setUpService),
                        )
                        .service(
                            // Mount UserServiceManager
                            web::scope("/users")
                                .data(UserServiceManager::New(pool.clone()))
                                .configure(UserController::setUpService),
                        )
                        .default_service(
                            web::route().to(|| async { HttpResponse::MethodNotAllowed() }),
                        ),
                ),
            )
            .service(
                Files::new("/www", "www")
                    .prefer_utf8(true)
                    .index_file("index.html"),
            ) // Static resources
            .default_service(web::route().to(|| HttpResponse::NotFound())) // Default route
    })
    .bind(format!(
        "0.0.0.0:{}",
        env::var("HTTP_PORT").unwrap_or("9000".to_string())
    ))?
    .run()
    .await
}
