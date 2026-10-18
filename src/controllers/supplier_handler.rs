use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};
use chrono::Utc;
use serde::Serialize;
use sqlx::Row;
use uuid::Uuid;

use crate::{
    models::{
        CreateSupplierRequestPayload, SupplierRequest, SupplierRequestStatus,
        UpdateSupplierStatusPayload,
    },
    services::{AppState, WhatsAppService, MEPC_ADMIN_WA},
};

#[derive(Serialize)]
pub struct CreateSupplierResponse {
    pub supplier_request: SupplierRequest,
    pub wa_admin_target: String,
    pub wa_payload_text: String,
    pub wa_click_to_chat_url: String,
    pub message: String,
}

fn row_to_supplier_request(row: &sqlx::postgres::PgRow) -> SupplierRequest {
    let id_uuid: Uuid = row.get("id");
    let status_str: String = row.get("status");
    let status = match status_str.to_uppercase().as_str() {
        "APPROVED" => SupplierRequestStatus::Approved,
        "CONTACTED" => SupplierRequestStatus::Contacted,
        "REJECTED" => SupplierRequestStatus::Rejected,
        _ => SupplierRequestStatus::Pending,
    };
    let created_at: chrono::DateTime<Utc> = row.get("created_at");

    SupplierRequest {
        id: id_uuid.to_string(),
        req_code: row.get("req_code"),
        supplier_name: row.get("supplier_name"),
        group_or_cv_name: row.get("group_or_cv_name"),
        location_regency: row.get("location_regency"),
        location_province: row.get("location_province"),
        whatsapp_number: row.get("whatsapp_number"),
        commodity_category: row.get("commodity_category"),
        commodity_name: row.get("commodity_name"),
        qty_monthly_capacity: row.get("qty_monthly_capacity"),
        unit: row.get("unit"),
        moisture_or_grade: row.get("moisture_or_grade"),
        offered_price_idr: row.get("offered_price_idr"),
        readiness_status: row.get("readiness_status"),
        notes: row.get("notes"),
        status,
        created_at: created_at.to_rfc3339(),
    }
}

pub async fn list_supplier_requests(State(state): State<AppState>) -> Json<Vec<SupplierRequest>> {
    let rows = sqlx::query("SELECT * FROM supplier_requests ORDER BY created_at DESC")
        .fetch_all(&state.pool)
        .await
        .unwrap_or_default();

    let list = rows.iter().map(row_to_supplier_request).collect();
    Json(list)
}

pub async fn create_supplier_request(
    State(state): State<AppState>,
    Json(payload): Json<CreateSupplierRequestPayload>,
) -> Result<(StatusCode, Json<CreateSupplierResponse>), StatusCode> {
    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM supplier_requests")
        .fetch_one(&state.pool)
        .await
        .unwrap_or(0);

    let year = Utc::now().format("%Y").to_string();
    let req_code = format!("SUP-MEPC-{}-{:04}", year, count + 1);

    let row = sqlx::query(
        r#"
        INSERT INTO supplier_requests (
            req_code, supplier_name, group_or_cv_name, location_regency, location_province,
            whatsapp_number, commodity_category, commodity_name, qty_monthly_capacity, unit,
            moisture_or_grade, offered_price_idr, readiness_status, notes, status, created_at, updated_at
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, 'PENDING', NOW(), NOW())
        RETURNING *
        "#,
    )
    .bind(&req_code)
    .bind(&payload.supplier_name)
    .bind(&payload.group_or_cv_name)
    .bind(&payload.location_regency)
    .bind(&payload.location_province)
    .bind(&payload.whatsapp_number)
    .bind(&payload.commodity_category)
    .bind(&payload.commodity_name)
    .bind(payload.qty_monthly_capacity)
    .bind(&payload.unit)
    .bind(&payload.moisture_or_grade)
    .bind(payload.offered_price_idr)
    .bind(&payload.readiness_status)
    .bind(&payload.notes)
    .fetch_one(&state.pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to insert supplier request into NeonDB: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let new_request = row_to_supplier_request(&row);

    let wa_text = WhatsAppService::format_supplier_request_message(&new_request);
    let wa_url = WhatsAppService::generate_click_to_chat_url(MEPC_ADMIN_WA, &wa_text);

    Ok((
        StatusCode::CREATED,
        Json(CreateSupplierResponse {
            supplier_request: new_request,
            wa_admin_target: format!("+{}", MEPC_ADMIN_WA),
            wa_payload_text: wa_text,
            wa_click_to_chat_url: wa_url,
            message: "Pengajuan suplai berhasil dicatat dalam NeonDB. Tim Sourcing MEPC akan segera menindaklanjuti via WhatsApp.".to_string(),
        }),
    ))
}

pub async fn update_supplier_status(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateSupplierStatusPayload>,
) -> Result<Json<SupplierRequest>, StatusCode> {
    let status_str = match payload.status {
        SupplierRequestStatus::Approved => "APPROVED",
        SupplierRequestStatus::Contacted => "CONTACTED",
        SupplierRequestStatus::Rejected => "REJECTED",
        SupplierRequestStatus::Pending => "PENDING",
    };

    let row = sqlx::query(
        r#"
        UPDATE supplier_requests
        SET status = $1,
            offered_price_idr = COALESCE($2, offered_price_idr),
            updated_at = NOW()
        WHERE id::text = $3 OR req_code = $3
        RETURNING *
        "#,
    )
    .bind(status_str)
    .bind(payload.agreed_price_idr)
    .bind(&id)
    .fetch_optional(&state.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    match row {
        Some(r) => Ok(Json(row_to_supplier_request(&r))),
        None => Err(StatusCode::NOT_FOUND),
    }
}
