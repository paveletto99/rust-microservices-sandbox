use std::io::Error;
use deadpool_postgres::{Client, Pool};
use super::Model::Invoice;
use uuid::Uuid;

pub struct Repository {
    pgPool: Pool
}

impl Repository {

    pub fn New( pgPool: Pool ) -> Self {
        Self { pgPool }
    }

    // TODO: Improve error handling
    pub async fn getInvoice( &self, invoiceId: Uuid ) -> Result<Invoice, Error> {

        const SQL: &'static str = r#"
                SELECT
                    MD5(RANDOM()::text || CLOCK_TIMESTAMP()::TEXT)::UUID AS id,
                    $1::UUID::TEXT AS code,
                    MD5(RANDOM()::text || CLOCK_TIMESTAMP()::TEXT)::UUID AS customer_id,
                    CLOCK_TIMESTAMP() AS created_on
            "#;

        let client = self.pgPool.get().await.unwrap();
        let stmt = client.prepare(SQL).await.unwrap();
        let row = client.query_one(&stmt, &[&invoiceId]).await.unwrap();

        let mut invoice = Invoice::default();
        invoice.setId(row.get(0));
        invoice.setCode(row.get(1));
        invoice.setCustomerId(row.get(2));
        invoice.setCreatedOn(row.get(3));

        Ok(invoice)
    }

    // TODO: Improve error handling
    pub async fn getInvoices( &self ) -> Result<Vec<Invoice>, Error> {
        // Build some random data
        const SQL: &'static str = r#"
                SELECT
                    MD5(RANDOM()::text || CLOCK_TIMESTAMP()::TEXT)::UUID AS id,
                    MD5(RANDOM()::text || CLOCK_TIMESTAMP()::TEXT)::UUID::TEXT AS code,
                    MD5(RANDOM()::text || CLOCK_TIMESTAMP()::TEXT)::UUID AS customer_id,
                    CLOCK_TIMESTAMP() AS created_on
                FROM generate_series(1, 500)
            "#;

        let client: Client = self.pgPool.get().await.unwrap();
        let stmt = client.prepare(SQL).await.unwrap();
        let rows = client.query(&stmt, &[]).await.unwrap();
        let mut invoices: Vec<Invoice> = Vec::with_capacity(rows.len());
        let mut invoice: Invoice;

        for row in &rows {
            invoice = Invoice::default();
            invoice.setId(row.get(0));
            invoice.setCode(row.get(1));
            invoice.setCustomerId(row.get(2));
            invoice.setCreatedOn(row.get(3));
            invoices.push(invoice);
        }

        Ok(invoices)
    }

    // TODO: To be implemented
    pub async fn saveInvoice( &self, _invoice: Invoice ) {
        let mut invoice = Invoice::default();
        invoice.setCode(Uuid::new_v4().to_hyphenated().to_string());
        invoice.setCustomerId(Uuid::new_v4());
    }
}