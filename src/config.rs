use std::env;
use deadpool_postgres::{Config, Pool, ManagerConfig, RecyclingMethod};

pub fn get_pg_pool() -> Pool {
    // Try DATABASE_URL first (for Railway/production)
    if let Ok(database_url) = env::var("DATABASE_URL") {
        let pg_config: tokio_postgres::Config = database_url.parse().expect("Invalid DATABASE_URL");
        let mgr_config = ManagerConfig {
            recycling_method: RecyclingMethod::Fast,
        };
        let mgr = deadpool_postgres::Manager::from_config(pg_config, tokio_postgres::NoTls, mgr_config);
        return Pool::builder(mgr).max_size(16).build().unwrap();
    }
    
    // Fallback to individual variables (for local development)
    let mut cfg = Config::new();
    cfg.host = Some(env::var("PG_HOST").expect("PG_HOST not set"));
    cfg.user = Some(env::var("PG_USER").expect("PG_USER not set"));
    cfg.password = env::var("PG_PASS").ok();
    cfg.dbname = Some(env::var("PG_DB").expect("PG_DB not set"));
    cfg.create_pool(None, tokio_postgres::NoTls).unwrap()
}