use std::env;
use dotenv::dotenv;

pub struct Config {
    pub db_url: String,
}

impl Config {
    pub fn from_env() -> eyre::Result<Self> {
        dotenv().ok();
        let db_url = env::var("DATABASE_URL")?;
        Ok(Config { db_url })
    }
}