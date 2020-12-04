use std::io::Error;
use deadpool_postgres::{Client, Pool};
use super::Model::Order;
use crate::api::commons::getUUID;

pub struct Repository {
    pgPool: Pool
}

impl Repository {

    pub fn New( pgPool: Pool ) -> Self {
        Self { pgPool }
    }
    
    pub async fn getOrder( &self, orderId: String ) -> Result<Order, Error> {
        
        let client: Client = self.pgPool.get().await.unwrap();
        let stmt = client.prepare("SELECT $1::TEXT::UUID::TEXT").await.unwrap();
        let rows = client.query(&stmt, &[&orderId]).await.unwrap();
        let value: String = rows[0].get(0);
        
        let mut order: Order = Order::default();
        order.setCode(value);
        
        Ok(order)
    }

    pub async fn getOrders( &self ) -> Result<Vec<Order>, Error> {
        
        let mut order01: Order = Order::default();
        order01.setCode(getUUID());

        let mut order02: Order = Order::default();
        order02.setCode(getUUID());

        Ok(vec![order01, order02])
    }
}