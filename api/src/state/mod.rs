use sqlx::PgPool;
use sqlx::postgres::PgPoolOptions;

use crate::config::Config;

#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
}

impl AppState {
    pub async fn new(config: &Config) -> Self {
        let pool = PgPoolOptions::new()
            .max_connections(5)
            .connect(&config.database_url)
            .await
            .expect("Failed koneksi ke database");

        println!("Database terhubung!");

        Self { db: pool }
    }
}
