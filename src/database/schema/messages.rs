use sqlx::PgPool;

pub async fn init_messages_table(pool: &PgPool) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS messages (
            id VARCHAR(64) PRIMARY KEY,
            sender_name VARCHAR(128) NOT NULL,
            sender_company VARCHAR(255),
            sender_phone VARCHAR(64) NOT NULL,
            sender_email VARCHAR(128),
            channel VARCHAR(32) NOT NULL DEFAULT 'WEB_INQUIRY',
            subject VARCHAR(255) NOT NULL,
            content TEXT NOT NULL,
            ref_code VARCHAR(64),
            status VARCHAR(32) NOT NULL DEFAULT 'UNREAD',
            reply_content TEXT,
            replied_by VARCHAR(128),
            replied_at TIMESTAMPTZ,
            created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
        );
        "#
    ).execute(pool).await?;

    sqlx::query("CREATE INDEX IF NOT EXISTS idx_messages_status ON messages(status);").execute(pool).await?;
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_messages_channel ON messages(channel);").execute(pool).await?;
    Ok(())
}
