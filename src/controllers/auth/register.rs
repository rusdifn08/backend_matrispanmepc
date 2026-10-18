use axum::{extract::State, http::StatusCode, response::Json};
use serde_json::{json, Value};
use uuid::Uuid;

use crate::models::{AuthResponse, RegisterPayload, User, UserProfileDto, UserRole};
use crate::services::{
    generate_jwt, hash_password, log_security_event, validate_nik, validate_password_strength, AppState,
};

pub async fn register_handler(
    State(state): State<AppState>,
    Json(payload): Json<RegisterPayload>,
) -> Result<Json<AuthResponse>, (StatusCode, Json<Value>)> {
    // 1. Validations
    if let Err(msg) = validate_nik(&payload.nik) {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({ "success": false, "message": msg })),
        ));
    }

    if payload.name.trim().is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({ "success": false, "message": "Nama lengkap staf wajib diisi." })),
        ));
    }

    if payload.email.trim().is_empty() || !payload.email.contains('@') {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({ "success": false, "message": "Format alamat email tidak valid." })),
        ));
    }

    if let Err(msg) = validate_password_strength(&payload.password) {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({ "success": false, "message": msg })),
        ));
    }

    let nik_clean = payload.nik.trim().to_uppercase();
    let email_clean = payload.email.trim().to_lowercase();

    // 2. Check for duplicate NIK or Email in NeonDB
    let duplicate_check: (i64,) = match sqlx::query_as(
        "SELECT COUNT(*) FROM users WHERE UPPER(nik) = $1 OR LOWER(email) = $2",
    )
    .bind(&nik_clean)
    .bind(&email_clean)
    .fetch_one(&state.pool)
    .await
    {
        Ok(c) => c,
        Err(e) => {
            tracing::error!("NeonDB duplicate check error: {:?}", e);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "success": false,
                    "message": "Gagal memeriksa keunikan data staf di basis data NeonDB."
                })),
            ));
        }
    };

    if duplicate_check.0 > 0 {
        return Err((
            StatusCode::CONFLICT,
            Json(json!({
                "success": false,
                "message": "NIK atau Email ini telah terdaftar di database kepegawaian PT. MEPC."
            })),
        ));
    }

    // 3. Cryptographic Password Hashing (Bcrypt)
    let password_hash = match hash_password(&payload.password) {
        Ok(h) => h,
        Err(e) => {
            tracing::error!("Bcrypt hashing error: {:?}", e);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "success": false,
                    "message": "Kesalahan enkripsi kata sandi."
                })),
            ));
        }
    };

    let user_id = Uuid::new_v4();
    let role: UserRole = payload.role.as_deref().unwrap_or("EXPORT_STAFF").into();
    let dept = if payload.department.trim().is_empty() {
        "Divisi Operasional & Ekspor".to_string()
    } else {
        payload.department.trim().to_string()
    };

    let new_user = User {
        id: user_id,
        nik: nik_clean.clone(),
        name: payload.name.trim().to_string(),
        email: email_clean.clone(),
        password_hash,
        role,
        department: dept.clone(),
        phone: payload.phone.map(|p| p.trim().to_string()),
        is_active: true,
        last_login_at: None,
        created_at: chrono::Utc::now(),
        updated_at: chrono::Utc::now(),
    };

    // 4. Insert into NeonDB
    let insert_result = sqlx::query(
        r#"
        INSERT INTO users (id, nik, name, email, password_hash, role, department, phone, is_active)
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
        "#,
    )
    .bind(new_user.id.to_string())
    .bind(&new_user.nik)
    .bind(&new_user.name)
    .bind(&new_user.email)
    .bind(&new_user.password_hash)
    .bind(new_user.role.to_string())
    .bind(&new_user.department)
    .bind(&new_user.phone)
    .bind(new_user.is_active)
    .execute(&state.pool)
    .await;

    if let Err(e) = insert_result {
        tracing::error!("NeonDB insert user error: {:?}", e);
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "success": false,
                "message": "Gagal menyimpan data karyawan baru ke database NeonDB."
            })),
        ));
    }

    // 5. Generate Token and Audit
    let token = match generate_jwt(&new_user) {
        Ok(t) => t,
        Err(e) => {
            tracing::error!("JWT generation error on register: {:?}", e);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "success": false,
                    "message": "Gagal menghasilkan token otentikasi pendaftaran."
                })),
            ));
        }
    };

    log_security_event(
        &state.pool,
        Some(&new_user.nik),
        Some(&new_user.email),
        "SIGNUP_SUCCESS",
        "INFO",
        None,
        None,
        Some(json!({
            "nik": new_user.nik,
            "role": new_user.role.to_string(),
            "department": new_user.department
        })),
    )
    .await;

    let profile: UserProfileDto = new_user.into();

    Ok(Json(AuthResponse {
        token,
        user: profile.clone(),
        message: format!(
            "Akun staf atas nama {} (NIK: {}) berhasil didaftarkan!",
            profile.name, profile.nik
        ),
        success: true,
    }))
}
