use super::Model::User;
use super::Repository::Repository;
use actix_web::web;
use deadpool_postgres::Pool;
use std::io::Error;

pub struct Service {
    repository: Repository,
}

impl Service {
    pub fn New(pgPool: Pool) -> Self {
        Self {
            repository: Repository::New(pgPool),
        }
    }

    pub async fn get_user(&self, user_id: u32) -> Result<User, Error> {
        Ok(self.repository.get_user(user_id).await?)
    }

    pub async fn delete_user(&self, user_id: u32) -> Result<User, Error> {
        Ok(self.repository.delete_user(user_id).await?)
    }

    pub async fn add_user(&self, url_params: web::Json<User>) -> Result<User, Error> {
        // map user
        let mut user: User = User::default();
        user.set_username(url_params.get_username().to_string());
        user.set_password(url_params.get_password().to_string());
        user.set_email(url_params.get_email().to_string());
        Ok(self.repository.add_user(user).await?)
    }

    pub async fn update_user(&self, url_params: web::Json<User>) -> Result<User, Error> {
        // map user
        let mut user: User = User::default();
        user.set_id(url_params.get_id() as i32);
        user.set_username(url_params.get_username().to_string());
        user.set_password(url_params.get_password().to_string());
        user.set_email(url_params.get_email().to_string());
        Ok(self.repository.update_user(user).await?)
    }
}

#[cfg(test)]
mod tests {
    use std::env;

    use super::*;
    use actix_web::{http::header, http::StatusCode, test, web, App};
    use chrono::DateTime;
    use deadpool_postgres::{Manager, ManagerConfig, Pool, RecyclingMethod};
    use tokio_postgres::{Config, NoTls};

    fn get_db_pool() -> Pool {
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

    fn get_mock_user() -> User {
        let mut user = User::default();
        user.set_id(12345);
        user.set_username("12345".to_string());
        user.set_password("12345".to_string());
        user.set_email("validate@todo.do".to_string());
        user.set_created_on(chrono::Utc::now());
        user
    }

    #[actix_rt::test]
    async fn delete_user_works() {
        let user_id = 12345 as u32;
        let res = Service::New(get_db_pool()).delete_user(user_id).await;
        match res {
            Ok(user) => println!("User deleted:\n{:?}", user),
            Err(err) => panic!("{:?}", err),
        }
    }

    #[actix_rt::test]
    async fn add_user_works() {
        let url_param_mock = web::Json(get_mock_user());
        let res = Service::New(get_db_pool()).add_user(url_param_mock).await;
        match res {
            Ok(user) => println!("User found:\n{:?}", user),
            Err(err) => panic!("{:?}", err),
        }
    }

    #[actix_rt::test]
    async fn add_user_duplicated_username_return_error() {
        let url_param_mock = web::Json(get_mock_user());
        let res = Service::New(get_db_pool()).add_user(url_param_mock).await;

        match res {
            Err(err) => panic!("{:?}", err),
            Ok(user) => println!("User found:\n{:?}", user),
        }
    }
    #[actix_rt::test]
    async fn add_user_duplicated_email_return_error() {
        let url_param_mock = web::Json(get_mock_user());
        let res = Service::New(get_db_pool()).add_user(url_param_mock).await;

        match res {
            Err(err) => panic!("{:?}", err),
            Ok(user) => println!("User found:\n{:?}", user),
        }
    }

    #[actix_rt::test]
    async fn test_get_user_by_identifier_works() {
        let user_id = 0 as u32;
        let res = Service::New(get_db_pool()).get_user(user_id).await;

        match res {
            Ok(user) => println!("User found:\n{:?}", user),
            Err(err) => panic!("{:?}", err),
        }
    }
    #[actix_rt::test]
    async fn test_get_user_by_identifier_not_found_works() {
        let userId = 1000 as u32;
        let res = Service::New(get_db_pool()).get_user(userId).await;

        match res {
            Err(err) => panic!("{:?}", err),
            Ok(user) => println!("User found:\n{:?}", user),
        }
    }
}
