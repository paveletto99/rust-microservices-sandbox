use std::io::Error;
use deadpool_postgres::Pool;
use super::Resource::Invoice as ResourceInvoice;
use super::Model::Invoice;
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
    pub async fn getInvoice( &self, invoiceId: Uuid ) -> Result<Invoice, Error> {
        Ok(self.repository.getInvoice(invoiceId).await?)
    }

    // TODO: Put here business logic and validations
    pub async fn getInvoices( &self ) -> Result<Vec<Invoice>, Error> {
        Ok(self.repository.getInvoices().await?)
    }

    // TODO: To be implemented - Put here business logic and validations
    pub async fn saveInvoice( &self, _invoice: ResourceInvoice ) {
        
        let mut invoice = Invoice::default();
        invoice.setCode("a88da591-659a-64da-d0bc-9d4c160a071f".to_string());
        invoice.setCustomerId(Uuid::parse_str("a88da591-659a-64da-d0bc-9d4c160a071f".to_string().as_str()).unwrap());

        //self.repository.saveInvoice(invoice)
    }
}