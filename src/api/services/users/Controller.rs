use super::UserServiceManager;
use crate::api::services::users::Model::User;
use actix_web::{web, HttpResponse, Responder};

pub struct Controller;

impl Controller {
    pub fn set_up_service(service_cfg: &mut web::ServiceConfig) {
        service_cfg.service(web::resource("/{user_id}")
            .route(web::delete().to(Self::delete_user_handler))
            .route(web::get().to(Self::get_user_handler)));
        service_cfg.service(web::resource("/")
            .route(web::patch().to(Self::update_users_handler))
            .route(web::post().to(Self::add_users_handler)));
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
            _ => HttpResponse::BadRequest().body("Error trying to read user from database"),
        }
    }

    async fn update_users_handler(
        service: web::Data<UserServiceManager>,
        url_params: web::Json<User>,
    ) -> impl Responder {
        match service.update_user(url_params).await {
            Ok(user) => HttpResponse::Ok().json(user),
            _ => HttpResponse::BadRequest().body("Error trying to read user from database"),
        }
    }

    async fn delete_user_handler(
        service: web::Data<UserServiceManager>,
        user_id: web::Path<u32>,
    ) -> impl Responder {
        match service.delete_user(user_id.into_inner()).await {
            Ok(user) => HttpResponse::Ok().json(user),
            _ => HttpResponse::BadRequest().body("Error trying to read user from database"),
        }
    }
}
