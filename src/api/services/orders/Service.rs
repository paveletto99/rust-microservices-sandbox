use std::io::Error;
use deadpool_postgres::Pool;
use super::Model::Order;
use super::Repository::Repository;
use crate::api::commons::getUUID;

pub struct Service {
    repository: Repository
}

impl Service {

    pub fn New( pgPool: Pool ) -> Self {
        Self { repository: Repository::New(pgPool) }
    }

    pub async fn getOrder( &self ) -> Result<Order, Error> {
        Ok(self.repository.getOrder(getUUID()).await?)
    }
}