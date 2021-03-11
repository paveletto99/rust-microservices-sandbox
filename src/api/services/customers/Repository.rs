use std::io::Error;
use deadpool_postgres::{Client, Pool};
use super::Model::Customer;
use uuid::Uuid;

pub struct Repository {
    pgPool: Pool
}

impl Repository {
    pub fn New(pgPool: Pool) -> Self {
        Self { pgPool }
    }

    // TODO: Improve error handling
    pub async fn getCustomer(&self, customerId: Uuid) -> Result<Customer, Error> {

        const SQL: &'static str = r#"
                SELECT
                    $1::UUID AS id,
                    MD5($1::UUID::TEXT)::TEXT AS vat_code,
                    'Customer001-' || MD5($1::UUID::TEXT)::TEXT AS company_name,
                    CLOCK_TIMESTAMP() AS created_on
            "#;

        let client = self.pgPool.get().await.unwrap();
        let stmt = client.prepare(SQL).await.unwrap();
        let row = client.query_one(&stmt, &[&customerId]).await.unwrap();

        let mut customer = Customer::default();
        customer.setId(row.get(0));
        customer.setVatCode(row.get(1));
        customer.setCompanyName(row.get(2));
        customer.setCreatedOn(row.get(3));

        Ok(customer)
    }

    // TODO: Improve error handling
    pub async fn getCustomers(&self) -> Result<Vec<Customer>, Error> {
        // Build some random data
        const SQL: &'static str = r#"
                SELECT
                    MD5(RANDOM()::TEXT || CLOCK_TIMESTAMP()::TEXT)::UUID AS id,
                    MD5(RANDOM()::TEXT)::TEXT AS vat_code,
                    'Customer001-' || MD5(CLOCK_TIMESTAMP()::TEXT)::UUID::TEXT AS company_name,
                    CLOCK_TIMESTAMP() AS created_on
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
            customer.setCompanyName(row.get(2));
            customer.setCreatedOn(row.get(3));
            customers.push(customer);
        }

        Ok(customers)
    }

    // TODO: Improve error handling
    pub async fn createCustomer(&self, customer: &Customer) -> Result<Customer, Error> {

        const SQL: &'static str = r#"
                INSERT INTO tbl_customers ( id, vat_code, company_name, created_on )
                VALUES( DEFAULT, $1, $2, $3 )
                RETURNING *
            "#;

        let client: Client = self.pgPool.get().await.unwrap();
        let stmt = client.prepare(SQL).await.unwrap();
        /*
        let params = &[
            &customer.getVatCode(),
            &customer.getCompanyName(),
            &customer.getCreatedOn()
        ];

        let row = client.query_one(SQL, &params).await.unwrap();
        */

        let row = client.query_one(SQL, &[
                                                                &customer.getVatCode(),
                                                                &customer.getCompanyName(),
                                                                &customer.getCreatedOn()
                                                            ]).await.unwrap();

        let mut newCustomer = Customer::default();
        newCustomer.setId(row.get(0));
        newCustomer.setVatCode(row.get(1));
        newCustomer.setCompanyName(row.get(2));

        Ok(newCustomer)
    }

    // TODO: Improve error handling
    pub async fn updateCustomer(&self, customer: &Customer) -> Result<Customer, Error> {

        const SQL: &'static str = r#"
                UPDATE tbl_customers
                    SET vat_code = $1,
                        company_name = $2,
                        created_on = $3
                WHERE id = $4
                RETURNING *
            "#;

        let client: Client = self.pgPool.get().await.unwrap();
        let stmt = client.prepare(SQL).await.unwrap();

        let row = client.query_one(SQL, &[
                                                                &customer.getVatCode(),
                                                                &customer.getCompanyName(),
                                                                &customer.getCreatedOn(),
                                                                &customer.getId()
                                                            ]).await.unwrap();

        let mut updatedCustomer = Customer::default();
        updatedCustomer.setId(row.get(0));
        updatedCustomer.setVatCode(row.get(1));
        updatedCustomer.setCompanyName(row.get(2));
        updatedCustomer.setCreatedOn(row.get(3));

        Ok(updatedCustomer)
    }

    // TODO: Improve error handling
    pub async fn deleteCustomer(&self, customerId: &Uuid) -> Result<Uuid, Error> {
        let client: Client = self.pgPool.get().await.unwrap();
        let stmt = client.prepare("DELETE FROM tbl_customers WHERE id = $1").await.unwrap();
        client.execute(&stmt, &[&customerId]).await;

        Ok(customerId.to_owned())
    }
}