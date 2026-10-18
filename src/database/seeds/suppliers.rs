use sqlx::PgPool;

pub async fn seed_suppliers(pool: &PgPool) -> Result<(), sqlx::Error> {
    let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM supplier_requests")
        .fetch_one(pool)
        .await?;

    if count.0 == 0 {
        tracing::info!("Seeding authentic farmer supply submissions into NeonDB...");
        let sups = vec![
            (
                "sup-sample-042", "SUP-MEPC-2026-042", "Bapak Tumpal Sembiring",
                "Kelompok Tani Rempah Dairi Makmur", "Kabupaten Dairi", "Sumatera Utara",
                "+62 812-6345-9981", "Spices", "Biji Pala Asalan & ABCD Grade",
                20.0, "Ton", "Kadar Air < 11%, Mutu Super ABCD", 95000.0,
                "Ready Stock", "Panen raya musim ini melimpah, siap kirim ke gudang PT. MEPC di Medan/Binjai setiap dua minggu.",
                "PENDING"
            ),
            (
                "sup-sample-045", "SUP-MEPC-2026-045", "Ibu Ratna Saragih",
                "Koperasi Produsen Kelapa Asahan", "Kabupaten Asahan", "Sumatera Utara",
                "+62 813-7721-0044", "Coconut", "Briket Arang Kelapa Shisha & Barbeque",
                35.0, "Ton", "Abu Putih < 2.2%, Kalori 7300 Kcal", 16500.0,
                "Ready Stock", "Pabrik briket kami memiliki 4 oven rotary dan kapasitas continue untuk ekspor Timur Tengah.",
                "PENDING"
            )
        ];

        for (id, code, sup, group, reg, prov, wa, cat, name, cap, unit, grade, price, readi, notes, status) in sups {
            sqlx::query(
                r#"
                INSERT INTO supplier_requests (id, req_code, supplier_name, group_or_cv_name, location_regency, location_province, whatsapp_number, commodity_category, commodity_name, qty_monthly_capacity, unit, moisture_or_grade, offered_price_idr, readiness_status, notes, status)
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16)
                ON CONFLICT (id) DO NOTHING;
                "#
            )
            .bind(id)
            .bind(code)
            .bind(sup)
            .bind(group)
            .bind(reg)
            .bind(prov)
            .bind(wa)
            .bind(cat)
            .bind(name)
            .bind(cap)
            .bind(unit)
            .bind(grade)
            .bind(price)
            .bind(readi)
            .bind(notes)
            .bind(status)
            .execute(pool)
            .await?;
        }
    }

    Ok(())
}
