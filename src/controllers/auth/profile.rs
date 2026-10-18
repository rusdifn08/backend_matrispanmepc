use axum::{
    extract::State,
    http::{HeaderMap, StatusCode},
    response::Json,
};
use serde_json::{json, Value};

use crate::models::{User, UserProfileDto};
use crate::services::{validate_jwt, AppState};

pub async fn get_current_user_handler(
    State(state): State<AppState>,
    headers: HeaderMap,
) -> Result<Json<UserProfileDto>, (StatusCode, Json<Value>)> {
    let auth_header = headers
        .get("authorization")
        .and_then(|h| h.to_str().ok())
        .ok_or_else(|| {
            (
                StatusCode::UNAUTHORIZED,
                Json(json!({
                    "success": false,
                    "message": "Token otentikasi Bearer wajib disertakan pada header."
                })),
            )
        })?;

    let claims = validate_jwt(auth_header).map_err(|_| {
        (
            StatusCode::UNAUTHORIZED,
            Json(json!({
                "success": false,
                "message": "Sesi otentikasi telah berakhir atau token tidak valid. Silakan login kembali."
            })),
        )
    })?;

    let row = sqlx::query(
        r#"
        SELECT id, nik, name, email, password_hash, role, department, phone, is_active, last_login_at, created_at, updated_at
        FROM users
        WHERE id = $1 AND is_active = TRUE
        "#,
    )
    .bind(&claims.sub)
    .fetch_optional(&state.pool)
    .await
    .map_err(|e| {
        tracing::error!("NeonDB query user by id error: {:?}", e);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "success": false,
                "message": "Kesalahan basis data saat mengambil profil staf."
            })),
        )
    })?
    .ok_or_else(|| {
        (
            StatusCode::NOT_FOUND,
            Json(json!({
                "success": false,
                "message": "Profil pengguna staf tidak ditemukan atau telah dinonaktifkan."
            })),
        )
    })?;

    let user = User::from_row(&row);
    let profile: UserProfileDto = user.into();

    Ok(Json(profile))
}
