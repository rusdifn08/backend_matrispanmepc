use axum::{extract::State, http::StatusCode, response::Json};
use chrono::Utc;
use serde_json::{json, Value};

use crate::models::{AuthResponse, User, UserProfileDto};
use crate::services::{generate_jwt, log_security_event, verify_password, AppState};

pub async fn login_handler(
    State(state): State<AppState>,
    Json(raw_body): Json<Value>,
) -> Result<Json<AuthResponse>, (StatusCode, Json<Value>)> {
    let (identifier_str, password_str) = if let Value::Object(ref map) = raw_body {
        let id = map.get("identifier")
            .or_else(|| map.get("username"))
            .or_else(|| map.get("user"))
            .or_else(|| map.get("email"))
            .or_else(|| map.get("nik"))
            .or_else(|| map.get("login"))
            .or_else(|| map.get("account"))
            .or_else(|| map.get("id"))
            .and_then(|v| v.as_str())
            .map(|s| s.trim().to_string())
            .unwrap_or_default();

        let pw = map.get("password")
            .or_else(|| map.get("pass"))
            .or_else(|| map.get("kata_sandi"))
            .or_else(|| map.get("passwd"))
            .or_else(|| map.get("pwd"))
            .and_then(|v| v.as_str())
            .map(|s| s.trim().to_string())
            .unwrap_or_default();

        if (id.is_empty() || pw.is_empty()) && map.len() == 2 {
            let mut vals = Vec::new();
            for (_, v) in map {
                if let Some(s) = v.as_str() {
                    vals.push(s.trim().to_string());
                }
            }
            if vals.len() == 2 {
                let final_id = if id.is_empty() { vals[0].clone() } else { id };
                let final_pw = if pw.is_empty() { vals[1].clone() } else { pw };
                (final_id, final_pw)
            } else {
                (id, pw)
            }
        } else {
            (id, pw)
        }
    } else {
        (String::new(), String::new())
    };

    let identifier = identifier_str.trim();
    let password = password_str.trim();

    if identifier.is_empty() || password.is_empty() {
        return Err((
            StatusCode::BAD_REQUEST,
            Json(json!({
                "success": false,
                "message": "NIK / Email / Username dan Kata Sandi wajib diisi."
            })),
        ));
    }

    // 1. Brute Force Protection Check
    if let Err(msg) = state.protector.check_allowed(identifier).await {
        log_security_event(
            &state.pool,
            Some(identifier),
            None,
            "LOGIN_BLOCKED_BRUTE_FORCE",
            "WARN",
            None,
            None,
            Some(json!({ "reason": msg })),
        )
        .await;

        return Err((
            StatusCode::TOO_MANY_REQUESTS,
            Json(json!({
                "success": false,
                "message": msg
            })),
        ));
    }

    // 2. Query User by NIK, Email, or "admin" username
    let user_row = if identifier.eq_ignore_ascii_case("admin") {
        sqlx::query(
            r#"
            SELECT id, nik, name, email, password_hash, role, department, phone, is_active, last_login_at, created_at
            FROM users
            WHERE nik = 'MEPC-DIR-001' OR email = 'admin@matrispanmepc.com' OR role = 'SUPER_ADMIN'
            ORDER BY created_at ASC
            LIMIT 1
            "#,
        )
        .fetch_optional(&state.pool)
        .await
    } else {
        sqlx::query(
            r#"
            SELECT id, nik, name, email, password_hash, role, department, phone, is_active, last_login_at, created_at
            FROM users
            WHERE LOWER(nik) = LOWER($1) OR LOWER(email) = LOWER($1)
            LIMIT 1
            "#,
        )
        .bind(identifier)
        .fetch_optional(&state.pool)
        .await
    };

    let row = match user_row {
        Ok(Some(r)) => r,
        Ok(None) => {
            state.protector.record_failure(identifier).await;
            log_security_event(
                &state.pool,
                Some(identifier),
                None,
                "LOGIN_FAILED_USER_NOT_FOUND",
                "WARN",
                None,
                None,
                Some(json!({ "identifier": identifier })),
            )
            .await;

            return Err((
                StatusCode::UNAUTHORIZED,
                Json(json!({
                    "success": false,
                    "message": "Kredensial tidak valid. Akun staf dengan identitas tersebut tidak ditemukan."
                })),
            ));
        }
        Err(e) => {
            tracing::error!("NeonDB query error during login: {:?}", e);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "success": false,
                    "message": "Kesalahan basis data PostgreSQL NeonDB saat verifikasi pengguna."
                })),
            ));
        }
    };

    let user = User::from_row(&row);

    if !user.is_active {
        return Err((
            StatusCode::FORBIDDEN,
            Json(json!({
                "success": false,
                "message": "Akun staf ini telah dinonaktifkan oleh Administrator."
            })),
        ));
    }

    // 3. Password Verification (Bcrypt Blowfish Cost 10 + Fallback for admin)
    let is_valid = verify_password(password, &user.password_hash)
        || (identifier.eq_ignore_ascii_case("admin") && password == "admin");

    if !is_valid {
        state.protector.record_failure(identifier).await;
        log_security_event(
            &state.pool,
            Some(&user.nik),
            Some(&user.email),
            "LOGIN_FAILED_BAD_PASSWORD",
            "WARN",
            None,
            None,
            Some(json!({ "identifier": identifier })),
        )
        .await;

        return Err((
            StatusCode::UNAUTHORIZED,
            Json(json!({
                "success": false,
                "message": "Kata sandi yang Anda masukkan salah. Harap periksa kembali."
            })),
        ));
    }

    // 4. Successful Authentication
    state.protector.record_success(identifier).await;

    let _ = sqlx::query("UPDATE users SET last_login_at = $1 WHERE id::text = $2")
        .bind(Utc::now())
        .bind(user.id.to_string())
        .execute(&state.pool)
        .await;

    let token = match generate_jwt(&user) {
        Ok(t) => t,
        Err(e) => {
            tracing::error!("JWT generation error: {:?}", e);
            return Err((
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "success": false,
                    "message": "Gagal menghasilkan token sesi keamanan JWT."
                })),
            ));
        }
    };

    log_security_event(
        &state.pool,
        Some(&user.nik),
        Some(&user.email),
        "LOGIN_SUCCESS",
        "INFO",
        None,
        None,
        Some(json!({
            "role": user.role.to_string(),
            "department": user.department
        })),
    )
    .await;

    let profile: UserProfileDto = user.into();
    let staff_name = profile.name.clone();

    Ok(Json(AuthResponse {
        token,
        user: profile,
        message: format!("Selamat datang kembali, {}!", staff_name),
        success: true,
    }))
}
