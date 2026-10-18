use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    response::Json,
};
use chrono::{DateTime, Utc};
use serde_json::{json, Value};
use sqlx::Row;

use crate::models::SecurityAuditLog;
use crate::services::{validate_jwt, AppState};

pub async fn get_security_audit_logs_handler(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<Vec<SecurityAuditLog>>, (StatusCode, Json<Value>)> {
    if let Some(auth_header) = headers.get("authorization").and_then(|h| h.to_str().ok()) {
        let _ = validate_jwt(auth_header);
    }

    let rows = sqlx::query(
        r#"
        SELECT id, user_nik, user_email, action, severity, ip_address, user_agent, details, created_at
        FROM security_audit_logs
        ORDER BY created_at DESC
        LIMIT 50
        "#,
    )
    .fetch_all(&state.pool)
    .await
    .map_err(|e| {
        tracing::error!("NeonDB audit log query error: {:?}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "success": false,
                "message": "Gagal mengambil log audit keamanan dari database."
            })),
        )
    })?;

    let logs: Vec<SecurityAuditLog> = rows
        .into_iter()
        .map(|r| {
            let raw_id: String = r.get("id");
            let id = uuid::Uuid::parse_str(&raw_id).unwrap_or_else(|_| uuid::Uuid::new_v4());
            let user_nik: Option<String> = r.get("user_nik");
            let user_email: Option<String> = r.get("user_email");
            let action: String = r.get("action");
            let severity: String = r.get("severity");
            let ip_address: Option<String> = r.get("ip_address");
            let user_agent: Option<String> = r.get("user_agent");
            let details: Option<serde_json::Value> = r.get("details");
            let created_at: DateTime<Utc> = r.get("created_at");

            SecurityAuditLog {
                id,
                user_nik,
                user_email,
                action,
                severity,
                ip_address,
                user_agent,
                details,
                created_at,
            }
        })
        .collect();

    Ok(Json(logs))
}
