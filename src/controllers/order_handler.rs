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
        BuyerOrder, CreateBuyerOrderPayload, OrderItem, OrderStatus, UpdateOrderStatusPayload,
    },
    services::{AppState, InvoiceService, WhatsAppService, MEPC_ADMIN_WA},
};

#[derive(Serialize)]
pub struct CreateOrderResponse {
    pub order: BuyerOrder,
    pub wa_admin_target: String,
    pub wa_payload_text: String,
    pub wa_click_to_chat_url: String,
    pub message: String,
}

fn row_to_buyer_order(row: &sqlx::postgres::PgRow) -> BuyerOrder {
    let id_uuid: Uuid = row.get("id");
    let status_str: String = row.get("status");
    let status = match status_str.to_uppercase().as_str() {
        "APPROVED" => OrderStatus::Approved,
        "CONTACTED" => OrderStatus::Contacted,
        "REJECTED" => OrderStatus::Rejected,
        _ => OrderStatus::Pending,
    };
    let items_val: serde_json::Value = row.get("items");
    let items: Vec<OrderItem> = serde_json::from_value(items_val).unwrap_or_default();
    let created_at: chrono::DateTime<Utc> = row.get("created_at");

    BuyerOrder {
        id: id_uuid.to_string(),
        order_code: row.get("order_code"),
        buyer_name: row.get("buyer_name"),
        company_name: row.get("company_name"),
        email: row.get("email"),
        whatsapp_number: row.get("whatsapp_number"),
        destination_country: row.get("destination_country"),
        destination_port: row.get("destination_port"),
        incoterms: row.get("incoterms"),
        items,
        status,
        total_amount: row.get("total_amount"),
        currency: row.get("currency"),
        notes: row.get("notes"),
        created_at: created_at.to_rfc3339(),
    }
}

pub async fn list_orders(State(state): State<AppState>) -> Json<Vec<BuyerOrder>> {
    let rows = sqlx::query("SELECT * FROM buyer_orders ORDER BY created_at DESC")
        .fetch_all(&state.pool)
        .await
        .unwrap_or_default();

    let orders = rows.iter().map(row_to_buyer_order).collect();
    Json(orders)
}

pub async fn get_order(
    State(state): State<AppState>,
    Path(id): Path<String>,
) -> Result<Json<BuyerOrder>, StatusCode> {
    let row = sqlx::query("SELECT * FROM buyer_orders WHERE id::text = $1 OR order_code = $1")
        .bind(&id)
        .fetch_optional(&state.pool)
        .await
        .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    match row {
        Some(r) => Ok(Json(row_to_buyer_order(&r))),
        None => Err(StatusCode::NOT_FOUND),
    }
}

pub async fn create_order(
    State(state): State<AppState>,
    Json(payload): Json<CreateBuyerOrderPayload>,
) -> Result<(StatusCode, Json<CreateOrderResponse>), StatusCode> {
    let count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM buyer_orders")
        .fetch_one(&state.pool)
        .await
        .unwrap_or(0);

    let year = Utc::now().format("%Y").to_string();
    let order_code = format!("ORD-MEPC-{}-{:04}", year, count + 1);
    let total_amount: f64 = payload.items.iter().map(|item| item.subtotal_usd).sum();

    let items_json = serde_json::to_value(&payload.items)
        .unwrap_or(serde_json::json!([]));

    let row = sqlx::query(
        r#"
        INSERT INTO buyer_orders (
            order_code, buyer_name, company_name, email, whatsapp_number,
            destination_country, destination_port, incoterms, items, status,
            total_amount, currency, notes, created_at, updated_at
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, 'PENDING', $10, 'USD', $11, NOW(), NOW())
        RETURNING *
        "#,
    )
    .bind(&order_code)
    .bind(&payload.buyer_name)
    .bind(&payload.company_name)
    .bind(&payload.email)
    .bind(&payload.whatsapp_number)
    .bind(&payload.destination_country)
    .bind(&payload.destination_port)
    .bind(&payload.incoterms)
    .bind(items_json)
    .bind(total_amount)
    .bind(&payload.notes)
    .fetch_one(&state.pool)
    .await
    .map_err(|e| {
        tracing::error!("Failed to insert buyer order into NeonDB: {}", e);
        StatusCode::INTERNAL_SERVER_ERROR
    })?;

    let new_order = row_to_buyer_order(&row);

    let wa_text = WhatsAppService::format_buyer_order_message(&new_order);
    let wa_url = WhatsAppService::generate_click_to_chat_url(MEPC_ADMIN_WA, &wa_text);

    Ok((
        StatusCode::CREATED,
        Json(CreateOrderResponse {
            order: new_order,
            wa_admin_target: format!("+{}", MEPC_ADMIN_WA),
            wa_payload_text: wa_text,
            wa_click_to_chat_url: wa_url,
            message: "Order placed successfully in NeonDB. Admin notified.".to_string(),
        }),
    ))
}

pub async fn update_order_status(
    State(state): State<AppState>,
    Path(id): Path<String>,
    Json(payload): Json<UpdateOrderStatusPayload>,
) -> Result<Json<BuyerOrder>, StatusCode> {
    let status_str = match payload.status {
        OrderStatus::Approved => "APPROVED",
        OrderStatus::Contacted => "CONTACTED",
        OrderStatus::Rejected => "REJECTED",
        OrderStatus::Pending => "PENDING",
    };

    let row = sqlx::query(
        r#"
        UPDATE buyer_orders
        SET status = $1,
            total_amount = COALESCE($2, total_amount),
            updated_at = NOW()
        WHERE id::text = $3 OR order_code = $3
        RETURNING *
        "#,
    )
    .bind(status_str)
    .bind(payload.adjusted_total_amount)
    .bind(&id)
    .fetch_optional(&state.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    let row = match row {
        Some(r) => r,
        None => return Err(StatusCode::NOT_FOUND),
    };

    let updated_order = row_to_buyer_order(&row);

    // If order was approved, auto-generate official Proforma Invoice in NeonDB if not existing
    if updated_order.status == OrderStatus::Approved {
        let inv_exists: bool = sqlx::query_scalar("SELECT EXISTS(SELECT 1 FROM invoices WHERE ref_code = $1)")
            .bind(&updated_order.order_code)
            .fetch_one(&state.pool)
            .await
            .unwrap_or(false);

        if !inv_exists {
            let inv_count: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM invoices")
                .fetch_one(&state.pool)
                .await
                .unwrap_or(0);

            let generated_invoice = InvoiceService::create_buyer_proforma_invoice(
                &updated_order,
                (inv_count + 1) as usize,
                "Admin Operasional Ekspor PT. MEPC",
            );

            let payload_json = serde_json::to_value(&generated_invoice.payload)
                .unwrap_or(serde_json::json!({}));

            let _ = sqlx::query(
                r#"
                INSERT INTO invoices (invoice_number, ref_code, payload, total_amount)
                VALUES ($1, $2, $3, $4)
                ON CONFLICT (invoice_number) DO NOTHING
                "#,
            )
            .bind(&generated_invoice.invoice_number)
            .bind(&updated_order.order_code)
            .bind(payload_json)
            .bind(generated_invoice.total_amount)
            .execute(&state.pool)
            .await;
        }
    }

    Ok(Json(updated_order))
}
