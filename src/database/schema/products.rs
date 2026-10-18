use sqlx::PgPool;

pub async fn init_products_table(pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS products (
            id VARCHAR(64) PRIMARY KEY,
            category VARCHAR(64) NOT NULL,
            name VARCHAR(255) NOT NULL,
            latin_or_local_name VARCHAR(255) NOT NULL,
            hs_code VARCHAR(32) NOT NULL,
            min_order_qty DOUBLE PRECISION NOT NULL DEFAULT 1.0,
            unit VARCHAR(32) NOT NULL DEFAULT 'MT',
            price_estimate_usd DOUBLE PRECISION NOT NULL DEFAULT 0.0,
            specs JSONB NOT NULL DEFAULT '{}'::jsonb,
            image_url TEXT NOT NULL,
            description TEXT NOT NULL,
            featured BOOLEAN NOT NULL DEFAULT FALSE,
            created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
        );
        "#
    ).execute(pool).await?;

    sqlx::query("CREATE INDEX IF NOT EXISTS idx_products_category ON products(category);").execute(pool).await?;
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_products_featured ON products(featured);").execute(pool).await?;
    Ok(())
}
