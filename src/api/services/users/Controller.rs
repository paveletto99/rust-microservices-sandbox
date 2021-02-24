use super::UserServiceManager;
use crate::api::services::users::Model::User;
use actix_web::{web, HttpResponse, Responder};

pub struct Controller;

impl Controller {
    pub fn set_up_service(service_cfg: &mut web::ServiceConfig) {
        service_cfg.service(
            web::resource("/{user_id}")
                .route(web::delete().to(Self::delete_user_handler))
                .route(web::get().to(Self::get_user_handler)),
        );
        service_cfg.service(
            web::resource("/")
                .route(web::patch().to(Self::update_users_handler))
                .route(web::post().to(Self::add_users_handler)),
        );
    }

    async fn get_user_handler(
        service: web::Data<UserServiceManager>,
        user_id: web::Path<u32>,
    ) -> impl Responder {
        match service.get_user(user_id.into_inner()).await {
            Ok(user) => HttpResponse::Ok().json(user),
            _ => HttpResponse::BadRequest().body("Error trying to read user from database"),
        }
    }

    async fn add_users_handler(
        service: web::Data<UserServiceManager>,
        url_params: web::Json<User>,
    ) -> impl Responder {
        match service.add_user(url_params).await {
            Ok(user) => HttpResponse::Ok().json(user),
            _ => HttpResponse::BadRequest().body("Error trying to add new user from database"),
        }
    }

    async fn update_users_handler(
        service: web::Data<UserServiceManager>,
        url_params: web::Json<User>,
    ) -> impl Responder {
        match service.update_user(url_params).await {
            Ok(user) => HttpResponse::Ok().json(user),
            _ => HttpResponse::BadRequest().body("Error trying to update user from database"),
        }
    }

    async fn delete_user_handler(
        service: web::Data<UserServiceManager>,
        user_id: web::Path<u32>,
    ) -> impl Responder {
        match service.delete_user(user_id.into_inner()).await {
            Ok(user) => HttpResponse::Ok().json(user),
            _ => HttpResponse::BadRequest().body("Error trying to delete user from database"),
        }
    }
}

// https://docs.rs/actix-web/3.3.2/actix_web/test/fn.read_body_json.html
// https://actix.rs/docs/testing/
// https://www.lpalmieri.com/posts/an-introduction-to-property-based-testing-in-rust/
// https://www.lpalmieri.com/posts/2020-08-09-zero-to-production-3-how-to-bootstrap-a-new-rust-web-api-from-scratch/#4-1-how-do-you-test-an-endpoint
// https://www.lpalmieri.com/posts/2020-04-13-wiremock-async-http-mocking-for-rust-applications/

#[cfg(test)]
mod tests {
    use std::env;

    use super::*;
    use actix_web::{http::header, http::StatusCode, test, web, App};
    use deadpool_postgres::{Manager, ManagerConfig, Pool, RecyclingMethod};
    use tokio_postgres::{Config, NoTls};

    pub fn get_db_pool() -> Pool {
        // try to mock db pool
        let mut pg_config: Config = tokio_postgres::Config::new();
        pg_config.host(env::var("PG_HOST").unwrap().as_str());
        pg_config.port(env::var("PG_PORT").unwrap().parse::<u16>().unwrap());
        pg_config.user(env::var("PG_USER").unwrap().as_str());
        pg_config.password(env::var("PG_PASS").unwrap().as_str());
        pg_config.dbname(env::var("PG_DBNAME").unwrap().as_str());
        // PostgreSQL Connection Pool
        let pool = Pool::new(
            Manager::from_config(
                pg_config,
                NoTls,
                ManagerConfig {
                    recycling_method: RecyclingMethod::Fast,
                },
            ),
            16,
        );
        pool
    }
    #[actix_rt::test]
    async fn get_user_by_identifier_works() {
        let mut srv = test::init_service(
            App::new()
                .data(UserServiceManager::New(get_db_pool().clone()))
                .service(web::resource("/users/{user_id}").to(Controller::get_user_handler)),
        )
        .await;

        let req = test::TestRequest::get().uri("/users/0").to_request();
        // Call application
        let resp = test::call_service(&mut srv, req).await;
        assert_eq!(resp.status(), StatusCode::OK);
    }

    #[actix_rt::test]
    async fn add_new_user_works() {
        let mut app = test::init_service(
            App::new()
                .data(UserServiceManager::New(get_db_pool().clone()))
                .service(
                    web::resource("/users").route(
                        web::post()
                            .to(Controller::add_users_handler),
                    ),
                ),
        )
        .await;
        let payload =
            r#"{"username":"12345","password":"User name","email":"validate@todo.do"}"#.as_bytes();

        let resp = test::TestRequest::post()
            .uri("/users")
            .header(header::CONTENT_TYPE, "application/json")
            .set_payload(payload)
            .send_request(&mut app)
            .await;

        // assert_eq!(resp.status(), StatusCode::CONFLICT);  // @todo[PG] must be implemented with RC 409
        assert_eq!(resp.status(), StatusCode::INTERNAL_SERVER_ERROR);

        let _result: User = test::read_body_json(resp).await;
    }

    #[actix_rt::test]
    #[ignore]
    async fn subscribe_returns_a_400_when_fields_are_present_but_invalid() {
        let client = reqwest::Client::new();
        let test_cases = vec![(
            r#"{"username":"12345","password":"User name","email":"validate@todo.do"}"#,
            "empty",
        )];

        for (body, description) in test_cases {
            // Act
            let response = client
                .post("http://localhost:9000/api/v1/users")
                .header("Content-Type", "application/json")
                .body(body)
                .send()
                .await
                .expect("Failed to execute request.");

            // Assert
            assert_eq!(
                400,
                response.status().as_u16(),
                "The API did not return a 400 Bad Request when the payload was {}.",
                description
            );
        }
    }
}
