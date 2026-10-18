use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use std::time::Duration;
use crate::config::DEFAULT_DATABASE_URL;

pub async fn create_neon_pool(url: Option<&str>) -> Result<PgPool, sqlx::Error> {
    let conn_str = url.unwrap_or(DEFAULT_DATABASE_URL);
    tracing::info!("Connecting to NeonDB PostgreSQL cluster: {}", conn_str.split('@').last().unwrap_or(""));
    
    let pool = PgPoolOptions::new()
        .max_connections(25)
        .min_connections(2)
        .acquire_timeout(Duration::from_secs(15))
        .idle_timeout(Duration::from_secs(600))
        .connect(conn_str)
        .await?;

    tracing::info!("✅ Connected to NeonDB PostgreSQL successfully!");
    Ok(pool)
}
