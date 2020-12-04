use deadpool_postgres::Pool;
use super::Repository::Repository;

pub struct Service {
    repository: Repository
}

impl Service {

    pub fn New( pgPool: Pool ) -> Self {
        Self { repository: Repository::New(pgPool) }
    }

    pub async fn getCustomer( &self ) -> String {
        self.repository.getCustomer().await.to_string()
    }
}