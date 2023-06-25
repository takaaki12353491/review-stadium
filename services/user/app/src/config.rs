use once_cell::sync::Lazy;
use std::env;

const POSTGRES_HOST: &str = "POSTGRES_HOST";
const POSTGRES_PORT: &str = "POSTGRES_PORT";
const POSTGRES_USER: &str = "POSTGRES_USER";
const POSTGRES_PASSWORD: &str = "POSTGRES_PASSWORD";
const POSTGRES_DB: &str = "POSTGRES_DB";

pub struct DBConfig {
    postgres_host: String,
    postgres_port: String,
    postgres_user: String,
    postgres_password: String,
    postgres_database: String,
}

impl DBConfig {
    pub fn url(&self) -> String {
        format!(
            "postgres://{}:{}@{}:{}/{}",
            self.postgres_user,
            self.postgres_password,
            self.postgres_host,
            self.postgres_port,
            self.postgres_database
        )
    }
}

pub static DB_CONFIG: Lazy<DBConfig> = Lazy::new(|| DBConfig {
    postgres_host: env::var(POSTGRES_HOST)
        .unwrap_or_else(|_| panic!("{} must be set", POSTGRES_HOST)),
    postgres_port: env::var(POSTGRES_PORT)
        .unwrap_or_else(|_| panic!("{} must be set", POSTGRES_PORT)),
    postgres_user: env::var(POSTGRES_USER)
        .unwrap_or_else(|_| panic!("{} must be set", POSTGRES_USER)),
    postgres_password: env::var(POSTGRES_PASSWORD)
        .unwrap_or_else(|_| panic!("{} must be set", POSTGRES_PASSWORD)),
    postgres_database: env::var(POSTGRES_DB)
        .unwrap_or_else(|_| panic!("{} must be set", POSTGRES_DB)),
});
