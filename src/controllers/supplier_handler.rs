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

fn get_f64_val(row: &sqlx::postgres::PgRow, col: &str) -> f64 {
    if let Ok(v) = row.try_get::<f64, _>(col) {
        v
    } else if let Ok(v) = row.try_get::<i64, _>(col) {
        v as f64
    } else if let Ok(v) = row.try_get::<i32, _>(col) {
        v as f64
    } else {
        0.0
    }
}

fn get_id_str(row: &sqlx::postgres::PgRow) -> String {
    if let Ok(s) = row.try_get::<String, _>("id") {
        s
    } else if let Ok(u) = row.try_get::<Uuid, _>("id") {
        u.to_string()
    } else {
        Uuid::new_v4().to_string()
    }
}

fn row_to_supplier_request(row: &sqlx::postgres::PgRow) -> SupplierRequest {
    let id_str = get_id_str(row);
    let status_str: String = row.try_get("status").unwrap_or_else(|_| "PENDING".to_string());
    let status = match status_str.to_uppercase().as_str() {
        "APPROVED" => SupplierRequestStatus::Approved,
        "CONTACTED" => SupplierRequestStatus::Contacted,
        "REJECTED" => SupplierRequestStatus::Rejected,
        _ => SupplierRequestStatus::Pending,
    };
    let created_at: chrono::DateTime<Utc> = row.try_get("created_at").unwrap_or_else(|_| Utc::now());

    SupplierRequest {
        id: id_str,
        req_code: row.try_get("req_code").unwrap_or_default(),
        supplier_name: row.try_get("supplier_name").unwrap_or_default(),
        group_or_cv_name: row.try_get("group_or_cv_name").unwrap_or_default(),
        location_regency: row.try_get("location_regency").unwrap_or_default(),
        location_province: row.try_get("location_province").unwrap_or_default(),
        whatsapp_number: row.try_get("whatsapp_number").unwrap_or_default(),
        commodity_category: row.try_get("commodity_category").unwrap_or_default(),
        commodity_name: row.try_get("commodity_name").unwrap_or_default(),
        qty_monthly_capacity: get_f64_val(row, "qty_monthly_capacity"),
        unit: row.try_get("unit").unwrap_or_else(|_| "Ton".to_string()),
        moisture_or_grade: row.try_get("moisture_or_grade").unwrap_or_default(),
        offered_price_idr: get_f64_val(row, "offered_price_idr"),
        readiness_status: row.try_get("readiness_status").unwrap_or_else(|_| "Ready Stock".to_string()),
        notes: row.try_get("notes").ok(),
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
    let new_id = format!("sup-{}", Uuid::new_v4());

    let row = sqlx::query(
        r#"
        INSERT INTO supplier_requests (
            id, req_code, supplier_name, group_or_cv_name, location_regency, location_province,
            whatsapp_number, commodity_category, commodity_name, qty_monthly_capacity, unit,
            moisture_or_grade, offered_price_idr, readiness_status, notes, status, created_at
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, 'PENDING', NOW())
        RETURNING *
        "#,
    )
    .bind(&new_id)
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
            offered_price_idr = COALESCE($2, offered_price_idr)
        WHERE id::text = $3 OR req_code = $3
        RETURNING *
        "#,
    )
    .bind(status_str)
    .bind(payload.agreed_price_idr)
    .bind(&id)
    .fetch_optional(&state.pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to update supplier status: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    match row {
        Some(r) => Ok(Json(row_to_supplier_request(&r))),
        None => Err(StatusCode::NOT_FOUND),
    }
}
