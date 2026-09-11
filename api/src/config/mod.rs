use std::env;

pub struct Config {
    pub database_url: String,
    pub server_addr: String,
}

impl Config {
    pub fn load() -> Self {
        dotenvy::dotenv().ok();

        let database_url = env::var("DATABASE_URL").expect("DATABASE_URL HARUS ADA DI ENV");

        let server_addr = env::var("SERVER_ADDR").unwrap_or_else(|_| "0.0.0.0:3000".to_string());

        Self {
            database_url,
            server_addr,
        }
    }
}
