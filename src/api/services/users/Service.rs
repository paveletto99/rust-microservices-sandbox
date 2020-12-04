use std::io::Error;
use deadpool_postgres::Pool;
use super::Model::User;
use super::Repository::Repository;

pub struct Service {
    repository: Repository
}

impl Service {

    pub fn New( pgPool: Pool ) -> Self {
        Self { repository: Repository::New(pgPool) }
    }

    pub async fn getUser( &self ) -> Result<User, Error> {
        Ok(User{})
    }
}