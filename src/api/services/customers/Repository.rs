use std::io::Error;
use deadpool_postgres::{Client, Pool};
use super::Model::Customer;
use uuid::Uuid;

pub struct Repository {
    pgPool: Pool
}

impl Repository {

    pub fn New( pgPool: Pool ) -> Self {
        Self { pgPool }
    }

    // TODO: Improve error handling
    pub async fn getCustomer( &self, customerId: Uuid ) -> Result<Customer, Error> {

        const SQL: &'static str = r#"
                SELECT
                    $1::UUID AS id,
                    MD5($1::UUID::TEXT)::TEXT AS vat_code,
                    'Customer001-' || MD5($1::UUID::TEXT)::TEXT AS name,
                    CLOCK_TIMESTAMP() AS created_at
            "#;

        let client = self.pgPool.get().await.unwrap();
        let stmt = client.prepare(SQL).await.unwrap();
        let row = client.query_one(&stmt, &[&customerId]).await.unwrap();

        let mut customer = Customer::default();
        customer.setId(row.get(0));
        customer.setVatCode(row.get(1));
        customer.setCustomerName(row.get(2));
        customer.setCreatedOn(row.get(3));

        Ok(customer)
    }

    // TODO: Improve error handling
    pub async fn getCustomers( &self ) -> Result<Vec<Customer>, Error> {
        // Build some random data
        const SQL: &'static str = r#"
                SELECT
                    MD5(RANDOM()::TEXT || CLOCK_TIMESTAMP()::TEXT)::UUID AS id,
                    MD5(RANDOM()::TEXT)::TEXT AS vat_code,
                    'Customer001-' || MD5(CLOCK_TIMESTAMP()::TEXT)::UUID::TEXT AS name,
                    CLOCK_TIMESTAMP() AS created_at
                FROM generate_series(1, 500)
            "#;

        let client: Client = self.pgPool.get().await.unwrap();
        let stmt = client.prepare(SQL).await.unwrap();
        let rows = client.query(&stmt, &[]).await.unwrap();
        let mut customers: Vec<Customer> = Vec::with_capacity(rows.len());
        let mut customer: Customer;

        for row in &rows {
            customer = Customer::default();
            customer.setId(row.get(0));
            customer.setVatCode(row.get(1));
            customer.setCustomerName(row.get(2));
            customer.setCreatedOn(row.get(3));
            customers.push(customer);
        }

        Ok(customers)
    }
}