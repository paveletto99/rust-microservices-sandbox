mod MongoDB;
mod PostgreSQL;

pub use MongoDB::MongoDB as MongoDBMigration;
pub use PostgreSQL::PostgreSQL as PostgreSQLMigration;