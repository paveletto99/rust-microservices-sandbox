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
