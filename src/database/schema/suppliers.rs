use sqlx::PgPool;

pub async fn init_suppliers_table(pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS supplier_requests (
            id VARCHAR(64) PRIMARY KEY,
            req_code VARCHAR(64) UNIQUE NOT NULL,
            supplier_name VARCHAR(128) NOT NULL,
            group_or_cv_name VARCHAR(255) NOT NULL,
            location_regency VARCHAR(128) NOT NULL,
            location_province VARCHAR(128) NOT NULL,
            whatsapp_number VARCHAR(64) NOT NULL,
            commodity_category VARCHAR(64) NOT NULL,
            commodity_name VARCHAR(255) NOT NULL,
            qty_monthly_capacity DOUBLE PRECISION NOT NULL DEFAULT 1.0,
            unit VARCHAR(32) NOT NULL DEFAULT 'Ton',
            moisture_or_grade VARCHAR(128) NOT NULL,
            offered_price_idr DOUBLE PRECISION NOT NULL DEFAULT 0.0,
            readiness_status VARCHAR(64) NOT NULL DEFAULT 'Ready Stock',
            notes TEXT,
            status VARCHAR(32) NOT NULL DEFAULT 'PENDING',
            created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
        );
        "#
    ).execute(pool).await?;

    sqlx::query("CREATE INDEX IF NOT EXISTS idx_suppliers_status ON supplier_requests(status);").execute(pool).await?;
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_suppliers_code ON supplier_requests(req_code);").execute(pool).await?;
    Ok(())
}
