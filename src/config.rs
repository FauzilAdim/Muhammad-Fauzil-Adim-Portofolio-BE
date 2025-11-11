use std::env;
use deadpool_postgres::{Config, Pool, Runtime};

pub fn get_pg_pool() -> Pool {
    // Try DATABASE_URL first (for Railway/production)
    if let Ok(database_url) = env::var("DATABASE_URL") {
        let config: Config = database_url.parse().expect("Invalid DATABASE_URL");
        return config.create_pool(Some(Runtime::Tokio1), tokio_postgres::NoTls).unwrap();
    }
    
    // Fallback to individual variables (for local development)
    let mut cfg = Config::new();
    cfg.host = Some(env::var("PG_HOST").expect("PG_HOST not set"));
    cfg.user = Some(env::var("PG_USER").expect("PG_USER not set"));
    cfg.password = env::var("PG_PASS").ok();
    cfg.dbname = Some(env::var("PG_DB").expect("PG_DB not set"));
    cfg.create_pool(Some(Runtime::Tokio1), tokio_postgres::NoTls).unwrap()
}