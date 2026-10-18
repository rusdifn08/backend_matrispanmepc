use sqlx::PgPool;

pub async fn init_audit_table(pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS security_audit_logs (
            id VARCHAR(64) PRIMARY KEY,
            user_nik VARCHAR(32),
            user_email VARCHAR(128),
            action VARCHAR(64) NOT NULL,
            severity VARCHAR(32) NOT NULL DEFAULT 'INFO',
            ip_address VARCHAR(64),
            user_agent TEXT,
            details JSONB,
            created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
        );
        "#
    ).execute(pool).await?;

    sqlx::query("CREATE INDEX IF NOT EXISTS idx_audit_action ON security_audit_logs(action);").execute(pool).await?;
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_audit_nik ON security_audit_logs(user_nik);").execute(pool).await?;
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_audit_created ON security_audit_logs(created_at DESC);").execute(pool).await?;
    Ok(())
}
