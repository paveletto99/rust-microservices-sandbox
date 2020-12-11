use super::Model::User;
use super::Repository::Repository;
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
    pub async fn add_user(&self, user: User) -> Result<User, Error> {
        Ok(self.repository.add_user(user).await?)
    }
}
