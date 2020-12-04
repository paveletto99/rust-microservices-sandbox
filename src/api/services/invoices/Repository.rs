use deadpool_postgres::Pool;
use crate::api::services::invoices::Model::Invoice;

pub struct Repository {
    pgPool: Pool
}

impl Repository {

    pub fn New( pgPool: Pool ) -> Self {
        Self { pgPool }
    }

    pub async fn getInvoice( &self ) -> String {

        let mut invoice = Invoice::default();
        invoice.setCustomerId("a88da591-659a-64da-d0bc-9d4c160a071f".to_string());

        let client = self.pgPool.get().await.unwrap();
        let stmt = client.prepare("SELECT MD5(CURRENT_TIMESTAMP::TEXT)::UUID::TEXT AS code").await.unwrap();
        let rows = client.query(&stmt, &[]).await.unwrap();
        let value: String = rows[0].get(0);
        value.into()
    }

    pub async fn saveInvoice( &self, invoice: Invoice ) {

        let mut invoice = Invoice::default();
        invoice.setCustomerId("a88da591-659a-64da-d0bc-9d4c160a071f".to_string());
        invoice.setCode("a88da591-659a-64da-d0bc-9d4c160a071f".to_string());
    }
}