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
        .expect(format!("{} must be set", POSTGRES_HOST).as_str()),
    postgres_port: env::var(POSTGRES_PORT)
        .expect(format!("{} must be set", POSTGRES_PORT).as_str()),
    postgres_user: env::var(POSTGRES_USER)
        .expect(format!("{} must be set", POSTGRES_USER).as_str()),
    postgres_password: env::var(POSTGRES_PASSWORD)
        .expect(format!("{} must be set", POSTGRES_PASSWORD).as_str()),
    postgres_database: env::var(POSTGRES_DB)
        .expect(format!("{} must be set", POSTGRES_DB).as_str()),
});
