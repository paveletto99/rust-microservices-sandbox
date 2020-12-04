
use std::io::Error;
use deadpool_postgres::{Client, Pool};
use super::Model::User;

pub struct Repository {
    pgPool: Pool
}

impl Repository {

    pub fn New( pgPool: Pool ) -> Self {
        Self { pgPool }
    }

    pub async fn getUser( &self ) -> Result<Option<User>, Error> {
        let _client: Client = self.pgPool.get().await.unwrap();
        Ok(Some(User{}))
    }

    pub async fn getAllUsers( &self ) -> Result<Vec<User>, Error> {
        Ok(vec![User{}, User{}])
    }

    pub async fn getAllUsersOption( &self ) -> Result<Option<Vec<User>>, Error> {
        let _client: Client = self.pgPool.get().await.unwrap();
        Ok(Some(vec![User{}, User{}]))
    }
}