use std::io::Error;
use deadpool_postgres::Pool;
use super::Model::Customer;
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
    pub async fn getCustomer( &self, customerId: Uuid ) -> Result<Customer, Error> {
        Ok(self.repository.getCustomer(customerId).await?)
    }

    // TODO: Put here business logic and validations
    pub async fn getCustomers( &self ) -> Result<Vec<Customer>, Error> {
        Ok(self.repository.getCustomers().await?)
    }

    // TODO: Put here business logic and validations
    pub async fn createCustomers( &self, customer: &Customer ) -> Result<Customer, Error> {
        Ok(self.repository.createCustomer(customer).await?)
    }

    // TODO: Put here business logic and validations
    pub async fn updateCustomers( &self, customer: &Customer ) -> Result<Customer, Error> {
        Ok(self.repository.updateCustomer(customer).await?)
    }

    // TODO: Put here business logic and validations
    pub async fn deleteCustomers( &self, customerId: &Uuid ) -> Result<Uuid, Error> {
        Ok(self.repository.deleteCustomer(customerId).await?)
    }
}