use std::env;

pub const DEFAULT_DATABASE_URL: &str = "postgresql://neondb_owner:npg_4RUErtCNydp6@ep-rough-cherry-b373m82z-pooler.c-4.ap-southeast-1.aws.neon.tech/neondb?sslmode=require&channel_binding=require";

#[derive(Debug, Clone)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
    pub min_connections: u32,
    pub connect_timeout_secs: u64,
}

impl Default for DatabaseConfig {
    fn default() -> Self {
        let url = env::var("DATABASE_URL").unwrap_or_else(|_| DEFAULT_DATABASE_URL.to_string());
        Self {
            url,
            max_connections: 20,
            min_connections: 2,
            connect_timeout_secs: 15,
        }
    }
}
