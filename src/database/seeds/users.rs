use sqlx::PgPool;
use uuid::Uuid;
use crate::security::hash_password;

pub async fn seed_users(pool: &PgPool) -> Result<(), sqlx::Error> {
    let count: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM users")
        .fetch_one(pool)
        .await?;

    if count.0 == 0 {
        tracing::info!("Seeding initial corporate staff accounts with bcrypt password hashes...");
        let admin_hash = hash_password("admin").unwrap_or_else(|_| "$2b$10$92IXUNpkjO0rOQ5byMi.Ye4oKoEa3Ro9llC/.og/at2.uheWG/igi".to_string());

        let users = vec![
            ("MEPC-DIR-001", "H. Rusdi Matrispan", "admin@matrispanmepc.com", "SUPER_ADMIN", "Direksi & Manajemen Eksekutif", "+62 822-7777-1234"),
            ("MEPC-EXP-002", "Bambang Sudirman", "bambang.sudirman@matrispanmepc.com", "EXPORT_MANAGER", "Divisi Pemasaran & Operasional Ekspor", "+62 812-3344-5566"),
            ("MEPC-LOG-003", "Zulham Efendi", "zulham.logistik@matrispanmepc.com", "LOGISTICS_OFFICER", "Divisi Logistik, Surveyor & Muatan Pelabuhan", "+62 813-9876-5432"),
            ("MEPC-SOU-004", "Siti Aminah Hasibuan", "siti.aminah@matrispanmepc.com", "SOURCING_OFFICER", "Divisi Sourcing & Kemitraan Petani", "+62 852-6123-4567"),
            ("MEPC-FIN-005", "Rahmat Hidayat", "rahmat.finance@matrispanmepc.com", "FINANCE_OFFICER", "Divisi Keuangan & Devisa Kargo", "+62 878-1122-3344"),
        ];

        for (nik, name, email, role, dept, phone) in users {
            let id = Uuid::new_v4().to_string();
            sqlx::query(
                r#"
                INSERT INTO users (id, nik, name, email, password_hash, role, department, phone, is_active)
                VALUES ($1, $2, $3, $4, $5, $6, $7, $8, TRUE)
                ON CONFLICT (nik) DO NOTHING;
                "#
            )
            .bind(id)
            .bind(nik)
            .bind(name)
            .bind(email)
            .bind(&admin_hash)
            .bind(role)
            .bind(dept)
            .bind(phone)
            .execute(pool)
            .await?;
        }

        // Insert initial system audit log
        let audit_id = Uuid::new_v4().to_string();
        sqlx::query(
            r#"
            INSERT INTO security_audit_logs (id, user_nik, user_email, action, severity, details)
            VALUES ($1, 'SYSTEM', 'system@matrispanmepc.com', 'SYSTEM_INIT_SEED', 'INFO', '{"seeded_users": 5}'::jsonb)
            "#
        )
        .bind(audit_id)
        .execute(pool)
        .await?;
    }

    Ok(())
}
