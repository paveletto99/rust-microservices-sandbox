use deadpool_postgres::Pool;
use crate::api::services::invoices::Resource::Invoice as ResourceInvoice;
use crate::api::services::invoices::Model::Invoice as EntityInvoice;
use crate::api::services::invoices::Repository::Repository;

pub struct Service {
    repository: Repository
}

impl Service {

    pub fn New( pgPool: Pool ) -> Self {
        Self { repository: Repository::New(pgPool) }
    }

    pub async fn getInvoice( &self ) -> String {
        self.repository.getInvoice().await.to_string()
    }

    pub async fn saveInvoice( &self, invoice: ResourceInvoice ) {
        
        let mut invoice = EntityInvoice::default();
        invoice.setCustomerId("a88da591-659a-64da-d0bc-9d4c160a071f".to_string());
        invoice.setCode("a88da591-659a-64da-d0bc-9d4c160a071f".to_string());

        //self.repository.saveInvoice(invoice)
    }
}