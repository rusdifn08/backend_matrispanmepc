use axum::{extract::State, response::Json};
use chrono::Utc;
use serde::Serialize;
use crate::services::AppState;

#[derive(Serialize)]
pub struct DashboardStats {
    pub pending_orders_count: usize,
    pub approved_orders_count: usize,
    pub total_orders_count: usize,
    pub pending_suppliers_count: usize,
    pub total_suppliers_count: usize,
    pub unread_messages_count: usize,
    pub total_products_count: usize,
    pub total_invoices_issued: usize,
    pub total_export_value_usd: f64,
    pub belawan_port_status: String,
    pub operational_hours_status: String,
    pub server_time_utc: String,
    pub server_time_wib: String,
}

pub async fn get_dashboard_stats(State(state): State<AppState>) -> Json<DashboardStats> {
    let pending_orders: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM buyer_orders WHERE status = 'PENDING'")
        .fetch_one(&state.pool)
        .await
        .unwrap_or(0);

    let approved_orders: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM buyer_orders WHERE status = 'APPROVED'")
        .fetch_one(&state.pool)
        .await
        .unwrap_or(0);

    let total_orders: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM buyer_orders")
        .fetch_one(&state.pool)
        .await
        .unwrap_or(0);

    let pending_suppliers: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM supplier_requests WHERE status = 'PENDING'")
        .fetch_one(&state.pool)
        .await
        .unwrap_or(0);

    let total_suppliers: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM supplier_requests")
        .fetch_one(&state.pool)
        .await
        .unwrap_or(0);

    let unread_messages: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM messages WHERE status = 'UNREAD'")
        .fetch_one(&state.pool)
        .await
        .unwrap_or(0);

    let total_products: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM products")
        .fetch_one(&state.pool)
        .await
        .unwrap_or(0);

    let total_invoices: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM invoices")
        .fetch_one(&state.pool)
        .await
        .unwrap_or(0);

    let total_export_val: Option<f64> = sqlx::query_scalar(
        "SELECT SUM(total_amount) FROM buyer_orders WHERE status IN ('APPROVED', 'CONTACTED')",
    )
    .fetch_one(&state.pool)
    .await
    .unwrap_or(None);

    let total_export_value_usd = total_export_val.unwrap_or(0.0);

    let now_utc = Utc::now();
    let wib_offset = chrono::FixedOffset::east_opt(7 * 3600).unwrap();
    let now_wib = now_utc.with_timezone(&wib_offset);
    let hour = now_wib.format("%H").to_string().parse::<u32>().unwrap_or(12);

    let operational_hours_status = if (7..17).contains(&hour) {
        "OPEN - Active Trading Floor (07:00 - 17:00 WIB)".to_string()
    } else {
        "STANDBY - 24/7 Global Inquiries Queue Active".to_string()
    };

    Json(DashboardStats {
        pending_orders_count: pending_orders as usize,
        approved_orders_count: approved_orders as usize,
        total_orders_count: total_orders as usize,
        pending_suppliers_count: pending_suppliers as usize,
        total_suppliers_count: total_suppliers as usize,
        unread_messages_count: unread_messages as usize,
        total_products_count: total_products as usize,
        total_invoices_issued: total_invoices as usize,
        total_export_value_usd,
        belawan_port_status: "OPERATIONAL - Berthing & Loading Normal (IDBLW)".to_string(),
        operational_hours_status,
        server_time_utc: now_utc.format("%Y-%m-%d %H:%M:%S UTC").to_string(),
        server_time_wib: now_wib.format("%Y-%m-%d %H:%M:%S WIB (UTC+7)").to_string(),
    })
}
