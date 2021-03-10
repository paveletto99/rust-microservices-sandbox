use deadpool_postgres::{config::ConfigError, Manager, ManagerConfig, Pool, RecyclingMethod};
use std::env;
use tokio_postgres::Error;
use tokio_postgres::{Config, NoTls};

const MAX_POOL_SIZE: usize = 16;

pub struct PostgresClient;

impl PostgresClient {
    pub fn new() -> Self {
        PostgresClient {}
    }

    /// Return default configuration reading env vars or ConfigError
    /// @todo[PG] manage error ewhen read vars
    pub fn get_default_config() -> Result<Config, ConfigError> {
        let mut pg_config: Config = Config::new();
        pg_config.host(env::var("PG_HOST").unwrap().as_str());
        pg_config.port(env::var("PG_PORT").unwrap().parse::<u16>().unwrap());
        pg_config.user(env::var("PG_USER").unwrap().as_str());
        pg_config.password(env::var("PG_PASS").unwrap().as_str());
        pg_config.dbname(env::var("PG_DBNAME").unwrap().as_str());
        Ok(pg_config)
    }

    pub async fn get_custom_pool(
        app_name: Option<&String>,
        db_name: &String,
        host: &String,
        pass: &String,
        port: Option<u16>,
        user: &String,
    ) -> Result<Pool, Error> {
        // PostgreSQL Environment Config
        let mut pg_config: Config = Config::new();
        if let Some(_x) = app_name {
            pg_config.application_name(&app_name.unwrap());
        }
        pg_config.dbname(&db_name);
        pg_config.host(&host);
        pg_config.password(&pass);
        pg_config.port(port.unwrap_or(5432));
        pg_config.user(&user);
        // PostgreSQL Connection Pool
        let pool = Pool::new(
            Manager::from_config(
                pg_config,
                NoTls,
                ManagerConfig {
                    recycling_method: RecyclingMethod::Fast,
                },
            ),
            MAX_POOL_SIZE,
        );
        Ok(pool)
    }

    pub async fn get_default_pool() -> Result<Pool, Error> {
        // PostgreSQL Connection Pool
        let pool = Pool::new(
            Manager::from_config(
                Self::get_default_config().unwrap(),
                NoTls,
                ManagerConfig {
                    recycling_method: RecyclingMethod::Fast,
                },
            ),
            MAX_POOL_SIZE,
        );
        Ok(pool)
    }

    pub async fn get_pool_from_config(config: Config) -> Result<Pool, Error> {
        // PostgreSQL Connection Pool
        let pool = Pool::new(
            Manager::from_config(
                config,
                NoTls,
                ManagerConfig {
                    recycling_method: RecyclingMethod::Fast,
                },
            ),
            MAX_POOL_SIZE,
        );
        Ok(pool)
    }
}
