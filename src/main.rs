#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(unused)]

use actix_web::{middleware, web, App, HttpResponse, HttpServer};
use deadpool_postgres::{Manager, ManagerConfig, Pool, RecyclingMethod};
use std::env;
use tokio_postgres::{Config, NoTls};
use uuid::Uuid;
use mongodb::Client;

// Application Modules
mod api;
use api::commons::ApiController;
// Users API
use api::services::users::UserController;
use api::services::users::UserServiceManager;
// Customers API
use api::services::customers::CustomerController;
use api::services::customers::CustomerServiceManager;
// Invoices API
use api::services::invoices::InvoiceController;
use api::services::invoices::InvoiceServiceManager;
// Orders API
use api::services::orders::OrderController;
use api::services::orders::OrderServiceManager;
// Shippings API
use api::services::shippings::ShippingController;
use api::services::shippings::ShippingServiceManager;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Logging
    env::set_var("RUST_LOG", "actix_web=debug,actix_server=info");
    env_logger::init();

    // PostgreSQL Environment Config
    const MAX_POOL_SIZE: usize = 16;
    let mut pgConfig: Config = tokio_postgres::Config::new();
    pgConfig.host(env::var("PG_HOST").unwrap().as_str());
    pgConfig.port(env::var("PG_PORT").unwrap().parse::<u16>().unwrap());
    pgConfig.user(env::var("PG_USER").unwrap().as_str());
    pgConfig.password(env::var("PG_PASS").unwrap().as_str());
    pgConfig.dbname(env::var("PG_DBNAME").unwrap().as_str());

    // PostgreSQL Connection Pool
    let pool = Pool::new(Manager::from_config(pgConfig, NoTls, ManagerConfig{ recycling_method: RecyclingMethod::Fast }), MAX_POOL_SIZE);

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
            .wrap(middleware::Compress::default())
            .wrap(middleware::DefaultHeaders::new().header("X-Version", "1.0"))
            .wrap(
                middleware::DefaultHeaders::new()
                    .header("X-Request-ID", Uuid::new_v4().to_hyphenated().to_string()),
            )
            .wrap(middleware::Logger::default())
            // Liveness probe | Readiness probe
             .route("/healthz", web::get().to(|| HttpResponse::Ok().finish()))
            .service(
                web::scope("/api").service(
                    web::scope("/v1")
                        .data(web::JsonConfig::default().limit(2048))
                        .wrap(middleware::DefaultHeaders::new().header(
                            "Authorization",
                            format!("{}{}", "Bearer ", Uuid::new_v4().to_simple()),
                        )) // Example setting response Headers e.g: JWT Token
                        .data(pool.clone()) // Passing PostgreSQL Connection Pooler to the Extractor
                        // .configure(UserController::set_up_service) // Mount routes
                        .service(
                            // Mount CustomerServiceManager
                            web::scope("/customers")
                                .data(CustomerServiceManager::New(pool.clone())) // Passing Service Manager Instance to the Extractor
                                .configure(CustomerController::setUpService), // Mount routes
                        )
                        .service(
                            // Mount InvoiceServiceManager
                            web::scope("/invoices")
                                .data(InvoiceServiceManager::New(pool.clone()))
                                .configure(InvoiceController::setUpService),
                        )
                        .service(
                            // Mount OrderServiceManager
                            web::scope("/orders")
                                .data(OrderServiceManager::New(pool.clone()))
                                .configure(OrderController::setUpService),
                        )
                        .service(
                            // Mount OrderServiceManager
                            web::scope("/users")
                                .data(UserServiceManager::New(pool.clone()))
                                .configure(UserController::set_up_service),
                        )
                        .service( // Mount ShippingServiceManager
                            web::scope("/shippings")
                                .data(ShippingServiceManager::New(MongoDB.clone()))
                                .configure(ShippingController::setUpService)
                        )
                        .default_service(
                            web::route().to(|| async { HttpResponse::MethodNotAllowed() }),
                        ),
                ),
            )
            .default_service(web::route().to(|| HttpResponse::NotFound())) // Default route
    })
    .bind(format!(
        "{}{}",
        "0.0.0.0:",
        env::var("HTTP_PORT").unwrap_or("9000".to_string())
    ))?
    .run()
    .await
}
