use sqlx::PgPool;

pub async fn init_invoices_table(pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS invoices (
            id VARCHAR(64) PRIMARY KEY,
            invoice_number VARCHAR(64) UNIQUE NOT NULL,
            ref_type VARCHAR(32) NOT NULL DEFAULT 'BUYER_ORDER',
            ref_id VARCHAR(64),
            ref_code VARCHAR(64) NOT NULL,
            approved_by_admin VARCHAR(128) NOT NULL DEFAULT 'Admin Operasional PT. MEPC',
            issued_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
            total_amount DOUBLE PRECISION NOT NULL DEFAULT 0.0,
            currency VARCHAR(16) NOT NULL DEFAULT 'USD',
            payload JSONB NOT NULL DEFAULT '{}'::jsonb,
            created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
        );
        "#
    ).execute(pool).await?;

    // Idempotent column migrations for existing instances
    let _ = sqlx::query("ALTER TABLE invoices ADD COLUMN IF NOT EXISTS ref_id VARCHAR(64);").execute(pool).await;
    let _ = sqlx::query("ALTER TABLE invoices ADD COLUMN IF NOT EXISTS ref_type VARCHAR(32) DEFAULT 'BUYER_ORDER';").execute(pool).await;
    let _ = sqlx::query("ALTER TABLE invoices ADD COLUMN IF NOT EXISTS approved_by_admin VARCHAR(128) DEFAULT 'Admin Operasional PT. MEPC';").execute(pool).await;

    sqlx::query("CREATE INDEX IF NOT EXISTS idx_invoices_num ON invoices(invoice_number);").execute(pool).await?;
    let _ = sqlx::query("CREATE INDEX IF NOT EXISTS idx_invoices_ref_id ON invoices(ref_id);").execute(pool).await;
    Ok(())
}
