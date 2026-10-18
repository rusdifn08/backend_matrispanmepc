use sqlx::PgPool;

pub async fn seed_messages(pool: &PgPool) -> Result<(), sqlx::Error> {
    let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM messages")
        .fetch_one(pool)
        .await?;

    if count.0 == 0 {
        tracing::info!("Seeding official inquiry messages into NeonDB...");
        let msgs = vec![
            (
                "msg-inq-101", "Hans Dietrich", Some("Global Spice GmbH"),
                "+49 152 9841 2300", Some("purchasing@globalspice.de"),
                "WHATS_APP", "Inquiry: COA & Phytosanitary Certificate for Nutmeg ABCD",
                "Good day PT. MEPC team. Following our order ORD-MEPC-2026-089, could you please confirm the lab report for aflatoxin levels and expected shipment schedule from Port of Belawan?",
                Some("ORD-MEPC-2026-089"), "UNREAD"
            ),
            (
                "msg-inq-102", "Tariq Al-Mansoor", Some("Gulf Spices Trading LLC"),
                "+971 50 123 4567", Some("tariq@gulfspices.ae"),
                "WEB_INQUIRY", "Request for Quotation: White Copra Edible Grade 25 MT CIF Jebel Ali",
                "Dear Export Division, We submitted order ORD-MEPC-2026-092 via your portal. Kindly provide official CIF Jebel Ali quotation and confirm 20ft container packing specs.",
                Some("ORD-MEPC-2026-092"), "UNREAD"
            ),
            (
                "msg-inq-103", "Bapak Tumpal Sembiring", Some("Kelompok Tani Rempah Dairi Makmur"),
                "+62 812-6345-9981", None,
                "WHATS_APP", "Konfirmasi Sampel Biji Pala Dairi Sudah Dikirim ke Medan",
                "Selamat siang admin PT. MEPC, sampel biji pala super 2 kg sudah kami kirim melalui ekspedisi ke kantor Medan untuk uji lab moisture. Mohon kabari jika sudah tiba.",
                Some("SUP-MEPC-2026-042"), "READ"
            )
        ];

        for (id, name, comp, phone, email, chan, subj, cont, ref_code, stat) in msgs {
            sqlx::query(
                r#"
                INSERT INTO messages (id, sender_name, sender_company, sender_phone, sender_email, channel, subject, content, ref_code, status)
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
                ON CONFLICT (id) DO NOTHING;
                "#
            )
            .bind(id)
            .bind(name)
            .bind(comp)
            .bind(phone)
            .bind(email)
            .bind(chan)
            .bind(subj)
            .bind(cont)
            .bind(ref_code)
            .bind(stat)
            .execute(pool)
            .await?;
        }
    }

    Ok(())
}
