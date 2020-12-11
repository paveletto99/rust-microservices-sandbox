use super::UserServiceManager;
use crate::api::services::users::Model::User;
use actix_web::{web, HttpResponse, Responder};

pub struct Controller;

impl Controller {
    pub fn set_up_service(service_cfg: &mut web::ServiceConfig) {
        service_cfg
            .service(web::resource("/{user_id}").route(web::get().to(Self::get_user_handler)));
        service_cfg.service(web::resource("/").route(web::post().to(Self::add_users_handler)));
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
        user_param: web::Json<User>,
    ) -> impl Responder {
        // map user
        let mut user: User = User::default();
        user.set_username(user_param.get_username().to_string());
        user.set_password(user_param.get_password().to_string());
        user.set_email(user_param.get_email().to_string());
        match service.add_user(user).await {
            Ok(user) => HttpResponse::Ok().json(user),
            _ => HttpResponse::BadRequest().body("Error trying to read user from database"),
        }
    }
}
