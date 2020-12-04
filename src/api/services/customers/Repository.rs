use deadpool_postgres::Pool;

pub struct Repository {
    pgPool: Pool
}

impl Repository {

    pub fn New( pgPool: Pool ) -> Self {
        Self { pgPool }
    }

    pub async fn getCustomer( &self ) -> String {
        let client = self.pgPool.get().await.unwrap();
        let stmt = client.prepare("SELECT 'Customer001' || MD5(CURRENT_TIMESTAMP::TEXT)::UUID::TEXT AS customer").await.unwrap();
        let rows = client.query(&stmt, &[]).await.unwrap();
        let value: String = rows[0].get(0);
        value.into()
    }
}