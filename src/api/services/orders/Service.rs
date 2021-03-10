use std::io::Error;
use deadpool_postgres::Pool;
use super::Model::Order;
use super::Repository::Repository;
use uuid::Uuid;

pub struct Service {
    repository: Repository
}

impl Service {

    pub fn New( pgPool: Pool ) -> Self {
        Self { repository: Repository::New(pgPool) }
    }

    // TODO: Put here business logic and validations
    pub async fn getOrder( &self, orderId: Uuid ) -> Result<Order, Error> {
        Ok(self.repository.getOrder(orderId).await?)
    }

    // TODO: Put here business logic and validations
    pub async fn getOrders( &self ) -> Result<Vec<Order>, Error> {
        Ok(self.repository.getOrders().await?)
    }
}