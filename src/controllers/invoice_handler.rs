use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::Json,
};
use chrono::Utc;
use sqlx::Row;
use uuid::Uuid;

use crate::{
    models::{Invoice, InvoicePayload, InvoiceRefType},
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

fn row_to_invoice(row: &sqlx::postgres::PgRow) -> Invoice {
    let id_str = get_id_str(row);
    let invoice_number: String = row.try_get("invoice_number").unwrap_or_default();
    let ref_code: String = row.try_get("ref_code").unwrap_or_default();
    let payload_val: serde_json::Value = row.try_get("payload").unwrap_or(serde_json::Value::Null);
    let payload: InvoicePayload = serde_json::from_value(payload_val.clone()).unwrap_or_else(|_| {
        serde_json::from_value(serde_json::json!({
            "invoice_number": invoice_number,
            "title": "PROFORMA EXPORT INVOICE",
            "status": "APPROVED",
            "issued_date": "2026-10-16",
            "due_date": "2026-10-30",
            "ref_type": "BUYER_ORDER",
            "ref_code": ref_code,
            "company_name": "PT. MATRISPAN EXIM PHOENIX CORP",
            "company_address": "Jl. Ir. H. Juanda No. 21 LK. II, Kota Binjai, Sumatera Utara",
            "company_email": "export@matrispanmepc.com",
            "company_phone": "+62 822-7777-1234",
            "port_of_loading": "Belawan Port (IDBLW)",
            "recipient_name": "-",
            "recipient_company": "-",
            "recipient_email": "-",
            "recipient_phone": "-",
            "destination_country": "-",
            "destination_port": "-",
            "incoterms": "FOB Belawan",
            "payment_terms": "30% T/T Advance, 70% L/C at sight",
            "items": [],
            "subtotal": 0.0,
            "shipping_and_insurance_fee": 0.0,
            "total_amount": 0.0,
            "currency": "USD",
            "bank_instructions": {
                "bank_name": "PT. BANK MANDIRI (PERSERO) TBK",
                "account_name": "PT. MATRISPAN EXIM PHOENIX CORP",
                "account_number": "106-00-1899214-7",
                "swift_code": "BMRIIDJA",
                "branch": "KCP BINJAI JUANDA"
            },
            "qr_verification_code": "",
            "authorized_signatory": "H. Rusdi Matrispan"
        }))
        .unwrap()
    });

    let issued_at: chrono::DateTime<Utc> = row.try_get("issued_at").unwrap_or_else(|_| Utc::now());

    Invoice {
        id: id_str.clone(),
        invoice_number,
        ref_type: InvoiceRefType::BuyerOrder,
        ref_id: id_str,
        ref_code,
        approved_by_admin: "Admin Operasional Ekspor PT. MEPC".to_string(),
        issued_at: issued_at.to_rfc3339(),
        total_amount: get_f64_val(row, "total_amount"),
        currency: "USD".to_string(),
        payload,
    }
}

pub async fn list_invoices(State(state): State<AppState>) -> Json<Vec<Invoice>> {
    let rows = sqlx::query("SELECT * FROM invoices ORDER BY issued_at DESC")
        .fetch_all(&state.pool)
        .await
        .unwrap_or_default();

    let invoices = rows.iter().map(row_to_invoice).collect();
    Json(invoices)
}

pub async fn get_invoice(
    State(state): State<AppState>,
    Path(param): Path<String>,
) -> Result<Json<Invoice>, StatusCode> {
    let row = sqlx::query(
        "SELECT * FROM invoices WHERE id::text = $1 OR invoice_number = $1 OR ref_code = $1",
    )
    .bind(&param)
    .fetch_optional(&state.pool)
    .await
    .map_err(|_| StatusCode::INTERNAL_SERVER_ERROR)?;

    match row {
        Some(r) => Ok(Json(row_to_invoice(&r))),
        None => Err(StatusCode::NOT_FOUND),
    }
}
