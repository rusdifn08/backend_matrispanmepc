use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};
use chrono::Utc;
use sqlx::Row;
use uuid::Uuid;

use crate::{
    models::{AdminMessage, CreateMessagePayload, MessageChannel, MessageStatus, ReplyMessagePayload},
    services::AppState,
};

fn get_id_str(row: &sqlx::postgres::PgRow) -> String {
    if let Ok(s) = row.try_get::<String, _>("id") {
        s
    } else if let Ok(u) = row.try_get::<Uuid, _>("id") {
        u.to_string()
    } else {
        Uuid::new_v4().to_string()
    }
}

fn row_to_message(row: &sqlx::postgres::PgRow) -> AdminMessage {
    let id_str = get_id_str(row);
    let chan_str: String = row.try_get("channel").unwrap_or_else(|_| "WEB_INQUIRY".to_string());
    let channel = match chan_str.to_uppercase().as_str() {
        "WHATSAPP" => MessageChannel::WhatsApp,
        "EMAIL" => MessageChannel::Email,
        _ => MessageChannel::WebInquiry,
    };
    let status_str: String = row.try_get("status").unwrap_or_else(|_| "UNREAD".to_string());
    let status = match status_str.to_uppercase().as_str() {
        "READ" => MessageStatus::Read,
        "REPLIED" => MessageStatus::Replied,
        _ => MessageStatus::Unread,
    };
    let created_at: chrono::DateTime<Utc> = row.try_get("created_at").unwrap_or_else(|_| Utc::now());

    AdminMessage {
        id: id_str,
        sender_name: row.try_get("sender_name").unwrap_or_default(),
        sender_company: row.try_get("sender_company").unwrap_or_default(),
        sender_phone: row.try_get("sender_phone").unwrap_or_default(),
        sender_email: row.try_get("sender_email").unwrap_or_default(),
        channel,
        subject: row.try_get("subject").unwrap_or_default(),
        content: row.try_get("content").unwrap_or_default(),
        ref_code: row.try_get("ref_code").unwrap_or_default(),
        status,
        created_at: created_at.to_rfc3339(),
    }
}

pub async fn list_messages(State(state): State<AppState>) -> Json<Vec<AdminMessage>> {
    let rows = sqlx::query("SELECT * FROM messages ORDER BY created_at DESC")
        .fetch_all(&state.pool)
        .await
        .unwrap_or_default();

    let msgs = rows.iter().map(row_to_message).collect();
    Json(msgs)
}

pub async fn create_message(
    State(state): State<AppState>,
    Json(payload): Json<CreateMessagePayload>,
) -> Result<(StatusCode, Json<AdminMessage>), StatusCode> {
    let chan_str = match payload.channel {
        MessageChannel::WhatsApp => "WHATSAPP",
        MessageChannel::WebInquiry => "WEB_INQUIRY",
        MessageChannel::Email => "EMAIL",
    };

    let row = sqlx::query(
        r#"
        INSERT INTO messages (
            sender_name, sender_company, sender_phone, sender_email,
            channel, subject, content, ref_code, status, created_at
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, 'UNREAD', NOW())
        RETURNING *
        "#,
    )
    .bind(&payload.sender_name)
    .bind(&payload.sender_company)
    .bind(&payload.sender_phone)
    .bind(&payload.sender_email)
    .bind(chan_str)
    .bind(&payload.subject)
    .bind(&payload.content)
    .bind(&payload.ref_code)
    .fetch_one(&state.pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to insert message into NeonDB: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    Ok((StatusCode::CREATED, Json(row_to_message(&row))))
}

pub async fn mark_message_read(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<AdminMessage>, StatusCode> {
    let row = sqlx::query(
        "UPDATE messages SET status = 'READ' WHERE id::text = $1 RETURNING *",
    )
    .bind(&id)
    .fetch_optional(&state.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    match row {
        Some(r) => Ok(Json(row_to_message(&r))),
        None => Err(StatusCode::NOT_FOUND),
    }
}

pub async fn reply_message(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(payload): Json<ReplyMessagePayload>,
) -> Result<Json<AdminMessage>, StatusCode> {
    let row = sqlx::query(
        r#"
        UPDATE messages
        SET status = 'REPLIED',
            reply_content = $1,
            replied_by = $2,
            replied_at = NOW()
        WHERE id::text = $3
        RETURNING *
        "#,
    )
    .bind(&payload.reply_content)
    .bind(&payload.admin_user)
    .bind(&id)
    .fetch_optional(&state.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    match row {
        Some(r) => Ok(Json(row_to_message(&r))),
        None => Err(StatusCode::NOT_FOUND),
    }
}
