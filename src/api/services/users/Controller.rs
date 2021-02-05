use actix_web::{ web, Responder, HttpRequest, HttpResponse };
use serde::{Serialize};
use deadpool_postgres::{Pool};

#[derive(Serialize)]
struct User {
    name: String
}

pub struct Controller;

impl Controller {
    
    pub fn setUpService( serviceCfg: &mut web::ServiceConfig ) {
        serviceCfg.service(web::resource("/{uuid}").route(web::get().to(Self::getUserHandler)));
        serviceCfg.service(web::resource("/").route(web::get().to(Self::getUsersHandler)));
    }

    async fn getUserHandler(pgPool: web::Data<Pool>) -> impl Responder {

        let client = pgPool.get().await.unwrap();
        let stmt = client.prepare("SELECT CURRENT_USER").await.unwrap();
        let rows = client.query(&stmt, &[]).await.unwrap();
        let value: String = rows[0].get(0);
        
        web::Json(User { name: format!("{} {}", "Call : ", value) })
    }

    async fn getUsersHandler() -> impl Responder {
        web::Json(User { name: "Called getUsersHandler".to_owned() })
    }
}