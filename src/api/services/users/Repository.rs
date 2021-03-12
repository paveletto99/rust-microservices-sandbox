use super::Model::User;
use chrono::DateTime;
use chrono::Local;
use chrono::Utc;
use deadpool_postgres::{Client, Pool};
use std::io::Error;

pub struct Repository {
    pgPool: Pool,
}

impl Repository {
    pub fn New(pgPool: Pool) -> Self {
        Self { pgPool }
    }

    pub async fn get_user(&self, user_id: u32) -> Result<User, Error> {
        let pg_id = user_id as i32; // cast to i32 https://doc.rust-lang.org/1.30.0/book/first-edition/casting-between-types.html
        let client: Client = self.pgPool.get().await.unwrap();
        let stmt = client
            .prepare(
                "SELECT user_id, username, password, email, created_on FROM tbl_accounts WHERE user_id = $1",
            )
            .await
            .unwrap();
        let rows = client.query(&stmt, &[&pg_id]).await.unwrap();
        // prepare ouput
        let mut user: User = User::default();
        if rows.len() == 1 {
            user.set_id(rows[0].get(0));
            user.set_username(rows[0].get(1));
            user.set_password(rows[0].get(2));
            user.set_email(rows[0].get(3));
            user.set_created_on(rows[0].get(4));
        }
        Ok(user)
    }

    pub async fn delete_user(&self, user_id: u32) -> Result<User, Error> {
        let pg_id = user_id as i32; // cast to i32 https://doc.rust-lang.org/1.30.0/book/first-edition/casting-between-types.html
        let client: Client = self.pgPool.get().await.unwrap();
        let stmt = client
            .prepare("DELETE FROM tbl_accounts WHERE user_id = $1 RETURNING *")
            .await
            .unwrap();
        let rows = client.query(&stmt, &[&pg_id]).await.unwrap();
        // prepare ouput
        let mut user: User = User::default();
        if rows.len() == 1 {
            user.set_id(rows[0].get(0));
            user.set_username(rows[0].get(1));
            user.set_password(rows[0].get(2));
            user.set_email(rows[0].get(3));
            user.set_created_on(rows[0].get(4));
        }
        Ok(user)
    }

    pub async fn add_user(&self, user: User) -> Result<User, Error> {
        let client: Client = self.pgPool.get().await.unwrap();
        // covert timezone
        let local_time = Local::now();
        let utc_time = DateTime::<Utc>::from_utc(local_time.naive_utc(), Utc);
        client
            .execute(
                "INSERT INTO tbl_accounts (username, password, email, created_on) VALUES ($1, $2, $3, $4)",
                &[
                    &user.get_username(),
                    &user.get_password(),
                    &user.get_email(),
                    &utc_time,
                ],
            )
            .await
            .unwrap();

        //@todo[pg] temp code
        let user: User = User::default();
        Ok(user)
    }

    pub async fn update_user(&self, user: User) -> Result<User, Error> {
        let client: Client = self.pgPool.get().await.unwrap();
        client
            .execute(
                "
                UPDATE tbl_accounts
                SET username = $2, password = $3, email = $4
                WHERE user_id = $1
                ",
                &[
                    &user.get_id(),
                    &user.get_username(),
                    &user.get_password(),
                    &user.get_email(),
                ],
            )
            .await
            .unwrap();

        //temp code
        let user: User = User::default();
        Ok(user)
    }
}