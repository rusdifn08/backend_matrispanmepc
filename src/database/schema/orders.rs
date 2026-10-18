use sqlx::PgPool;

pub async fn init_orders_table(pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS buyer_orders (
            id VARCHAR(64) PRIMARY KEY,
            order_code VARCHAR(64) UNIQUE NOT NULL,
            buyer_name VARCHAR(128) NOT NULL,
            company_name VARCHAR(255) NOT NULL,
            email VARCHAR(128) NOT NULL,
            whatsapp_number VARCHAR(64) NOT NULL,
            destination_country VARCHAR(128) NOT NULL,
            destination_port VARCHAR(128) NOT NULL,
            incoterms VARCHAR(64) NOT NULL,
            items JSONB NOT NULL DEFAULT '[]'::jsonb,
            status VARCHAR(32) NOT NULL DEFAULT 'PENDING',
            total_amount DOUBLE PRECISION NOT NULL DEFAULT 0.0,
            currency VARCHAR(16) NOT NULL DEFAULT 'USD',
            notes TEXT,
            created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
        );
        "#
    ).execute(pool).await?;

    sqlx::query("CREATE INDEX IF NOT EXISTS idx_orders_status ON buyer_orders(status);").execute(pool).await?;
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_orders_code ON buyer_orders(order_code);").execute(pool).await?;
    Ok(())
}
