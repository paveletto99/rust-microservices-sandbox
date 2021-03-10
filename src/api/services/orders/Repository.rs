use std::io::Error;
use deadpool_postgres::{Client, Pool};
use super::Model::Order;
use uuid::Uuid;

pub struct Repository {
    pgPool: Pool
}

impl Repository {

    pub fn New( pgPool: Pool ) -> Self {
        Self { pgPool }
    }

    // TODO: Improve error handling
    pub async fn getOrder( &self, orderId: Uuid ) -> Result<Order, Error> {

        let client: Client = self.pgPool.get().await.unwrap();
        let stmt = client.prepare("SELECT $1::UUID as id, $1::UUID::TEXT as code, NOW() AS created_on").await.unwrap();
        let row = client.query_one(&stmt, &[&orderId]).await.unwrap();

        let mut order: Order = Order::default();
        order.setId(row.get(0));
        order.setCode(row.get(1));
        order.setCreatedOn(row.get(2));

        Ok(order)
    }

    // TODO: Improve error handling
    pub async fn getOrders( &self ) -> Result<Vec<Order>, Error> {
        // Build some random data
        const SQL: &'static str = r#"
                SELECT
                    MD5(RANDOM()::text || CLOCK_TIMESTAMP()::TEXT)::UUID AS id,
                    MD5(RANDOM()::text || CLOCK_TIMESTAMP()::TEXT)::UUID::TEXT AS code,
                    CLOCK_TIMESTAMP() AS created_on
                FROM generate_series(1, 500)
            "#;

        let client: Client = self.pgPool.get().await.unwrap();
        let stmt = client.prepare(SQL).await.unwrap();
        let rows = client.query(&stmt, &[]).await.unwrap();
        let mut orders: Vec<Order> = Vec::with_capacity(rows.len());
        let mut order: Order;

        for row in &rows {
            order = Order::default();
            order.setId(row.get(0));
            order.setCode(row.get(1));
            order.setCreatedOn(row.get(2));
            orders.push(order);
        }

        Ok(orders)
    }
}