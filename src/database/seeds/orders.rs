use sqlx::PgPool;
use serde_json::json;

pub async fn seed_orders(pool: &PgPool) -> Result<(), sqlx::Error> {
    let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM buyer_orders")
        .fetch_one(pool)
        .await?;

    if count.0 == 0 {
        tracing::info!("Seeding initial verified buyer orders into NeonDB...");
        let orders = vec![
            (
                "ord-sample-089", "ORD-MEPC-2026-089", "Hans Dietrich", "Global Spice GmbH",
                "purchasing@globalspice.de", "+49 152 9841 2300", "Germany",
                "Port of Hamburg (DEHAM)", "FOB Belawan",
                json!([
                    {
                        "product_id": "prod-spices-01", "product_name": "Nutmeg Whole ABCD Grade (Biji Pala Utuh)",
                        "hs_code": "0908.11.00", "quantity": 15.0, "unit": "MT",
                        "unit_price_usd": 8400.0, "subtotal_usd": 126000.0
                    },
                    {
                        "product_id": "prod-spices-02", "product_name": "Cloves Lal Pari Quality (Cengkeh Lal Pari)",
                        "hs_code": "0907.10.00", "quantity": 10.0, "unit": "MT",
                        "unit_price_usd": 9800.0, "subtotal_usd": 98000.0
                    }
                ]),
                "APPROVED", 224000.0, "USD",
                "Please prepare phytosanitary certificate and certificate of origin (Form A) for European Customs clearance."
            ),
            (
                "ord-sample-092", "ORD-MEPC-2026-092", "Tariq Al-Mansoor", "Gulf Spices Trading LLC",
                "tariq@gulfspices.ae", "+971 50 123 4567", "United Arab Emirates",
                "Port of Jebel Ali (AEJEA)", "CIF Jebel Ali",
                json!([
                    {
                        "product_id": "prod-coconut-02", "product_name": "White Copra Edible Grade (Kopra Putih)",
                        "hs_code": "1203.00.00", "quantity": 25.0, "unit": "MT",
                        "unit_price_usd": 1450.0, "subtotal_usd": 36250.0
                    }
                ]),
                "PENDING", 36250.0, "USD",
                "Requires Certificate of Analysis (COA) for moisture < 5% and free fatty acid < 0.5%."
            ),
            (
                "ord-sample-095", "ORD-MEPC-2026-095", "Kenji Takahashi", "Nippon Marine Foods Corp",
                "k.takahashi@nipponmarine.co.jp", "+81 3 5555 0199", "Japan",
                "Port of Yokohama (JPYOK)", "FOB Belawan",
                json!([
                    {
                        "product_id": "prod-seafood-01", "product_name": "Frozen Black Tiger Shrimp HLSO (Udang Windu Beku)",
                        "hs_code": "0306.17.10", "quantity": 18.0, "unit": "MT",
                        "unit_price_usd": 15500.0, "subtotal_usd": 279000.0
                    }
                ]),
                "CONTACTED", 279000.0, "USD",
                "HACCP and EU/Japan approved cold storage processing with blast freeze -40C."
            )
        ];

        for (id, code, buyer, comp, email, wa, country, port, inco, items, status, total, curr, notes) in orders {
            sqlx::query(
                r#"
                INSERT INTO buyer_orders (id, order_code, buyer_name, company_name, email, whatsapp_number, destination_country, destination_port, incoterms, items, status, total_amount, currency, notes)
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14)
                ON CONFLICT (id) DO NOTHING;
                "#
            )
            .bind(id)
            .bind(code)
            .bind(buyer)
            .bind(comp)
            .bind(email)
            .bind(wa)
            .bind(country)
            .bind(port)
            .bind(inco)
            .bind(items)
            .bind(status)
            .bind(total)
            .bind(curr)
            .bind(notes)
            .execute(pool)
            .await?;
        }
    }

    Ok(())
}
