use deadpool_postgres::Pool;
use tokio_postgres::Error;

pub struct PostgreSQL;

impl PostgreSQL {

    pub async fn migrate(pgPool: Pool) -> Result<(), Error> {

        const SQL: &'static str = r#"

                CREATE TABLE IF NOT EXISTS tbl_customers (
                     id UUID NOT NULL PRIMARY KEY DEFAULT gen_random_uuid(),
                     vat_code VARCHAR NOT NULL,
                     company_name VARCHAR NOT NULL,
                     created_on TIMESTAMP WITH TIME ZONE NOT NULL,
                     CONSTRAINT tbl_customers_vat_code_uq UNIQUE ( vat_code )
                );

                CREATE TABLE IF NOT EXISTS tbl_accounts (
                    user_id INTEGER NOT NULL PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
                    username VARCHAR NOT NULL,
                    password VARCHAR NOT NULL,
                    email VARCHAR NOT NULL,
                    created_on TIMESTAMP WITH TIME ZONE NOT NULL,
                    CONSTRAINT tbl_accounts_username_uq UNIQUE ( username ),
                    CONSTRAINT tbl_accounts_email_uq UNIQUE ( email )
                );
            "#;

        let client = pgPool.get().await.unwrap();
        client.batch_execute(SQL).await
    }
}