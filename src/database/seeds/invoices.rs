use sqlx::PgPool;
use serde_json::json;

pub async fn seed_invoices(pool: &PgPool) -> Result<(), sqlx::Error> {
    let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM invoices")
        .fetch_one(pool)
        .await?;

    if count.0 == 0 {
        tracing::info!("Seeding verified commercial export proforma invoices into NeonDB...");
        let inv_payload = json!({
            "invoice_number": "INV/MEPC/2026/0089",
            "title": "COMMERCIAL / PROFORMA EXPORT INVOICE",
            "status": "APPROVED BY ADMIN",
            "issued_date": "15 October 2026",
            "due_date": "29 October 2026",
            "ref_type": "BUYER_ORDER",
            "ref_code": "ORD-MEPC-2026-089",
            "company_name": "PT. MATRISPAN EXIM PHOENIX CORP",
            "company_address": "Jl. Ir. H. Juanda No. 21 LK. II, Kota Binjai, Sumatera Utara 20714, Indonesia",
            "company_email": "info@matrispanmepc.com",
            "company_phone": "+62 851-8311-0510",
            "port_of_loading": "Belawan International Port (IDBLW), North Sumatra, Indonesia",
            "recipient_name": "Hans Dietrich",
            "recipient_company": "Global Spice GmbH",
            "recipient_email": "purchasing@globalspice.de",
            "recipient_phone": "+49 152 9841 2300",
            "destination_country": "Germany",
            "destination_port": "Port of Hamburg (DEHAM)",
            "incoterms": "FOB Belawan",
            "payment_terms": "30% T/T Advance Deposit upon Proforma confirmation, 70% Irrevocable L/C at sight upon Bill of Lading (B/L)",
            "items": [
                {
                    "description": "Nutmeg Whole ABCD Grade (Biji Pala Utuh)",
                    "hs_code": "0908.11.00",
                    "quantity": 15.0,
                    "unit": "MT",
                    "unit_price": 8400.0,
                    "subtotal": 126000.0
                },
                {
                    "description": "Cloves Lal Pari Quality (Cengkeh Lal Pari)",
                    "hs_code": "0907.10.00",
                    "quantity": 10.0,
                    "unit": "MT",
                    "unit_price": 9800.0,
                    "subtotal": 98000.0
                }
            ],
            "subtotal": 224000.0,
            "shipping_and_insurance_fee": 0.0,
            "total_amount": 224000.0,
            "currency": "USD",
            "bank_instructions": {
                "bank_name": "PT. BANK MANDIRI (PERSERO) TBK",
                "account_name": "PT. MATRISPAN EXIM PHOENIX CORP",
                "account_number": "106-00-1899214-7 (USD / IDR FX ACCOUNT)",
                "swift_code": "BMRIIDJA",
                "branch": "KCP BINJAI JUANDA - SUMATERA UTARA, INDONESIA"
            },
            "qr_verification_code": "https://matrispanmepc.com/verify-invoice?code=INV/MEPC/2026/0089",
            "authorized_signatory": "Direktur Operasional & Ekspor / M. Rusdi"
        });

        sqlx::query(
            r#"
            INSERT INTO invoices (id, invoice_number, ref_type, ref_id, ref_code, approved_by_admin, total_amount, currency, payload)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
            ON CONFLICT (id) DO NOTHING;
            "#
        )
        .bind("inv-089")
        .bind("INV/MEPC/2026/0089")
        .bind("BUYER_ORDER")
        .bind("ord-sample-089")
        .bind("ORD-MEPC-2026-089")
        .bind("Direktur Operasional & Ekspor PT. MEPC")
        .bind(224000.0)
        .bind("USD")
        .bind(inv_payload)
        .execute(pool)
        .await?;
    }

    Ok(())
}
