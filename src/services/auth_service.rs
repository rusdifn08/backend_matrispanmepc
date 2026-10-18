use chrono::{Duration as ChronoDuration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use crate::models::User;


const JWT_SECRET: &[u8] = b"MEPC_JWT_SECRET_2026_ENTERPRISE_RUST_KEY_AXUM_BELAWAN_EXPORT";

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub nik: String,
    pub name: String,
    pub email: String,
    pub role: String,
    pub department: String,
    pub exp: usize,
}

// In-memory brute-force protection tracker
// Identifier -> (failed_count, lock_until)
#[derive(Clone)]
pub struct BruteForceProtector {
    attempts: Arc<Mutex<HashMap<String, (u32, chrono::DateTime<Utc>)>>>,
}

impl BruteForceProtector {
    pub fn new() -> Self {
        Self {
            attempts: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    pub async fn check_allowed(&self, identifier: &str) -> Result<(), String> {
        let attempts = self.attempts.lock().await;
        let id_clean = identifier.trim().to_lowercase();
        if let Some((count, lock_until)) = attempts.get(&id_clean) {
            let now = Utc::now();
            if *count >= 5 && now < *lock_until {
                let remaining = (*lock_until - now).num_seconds();
                return Err(format!(
                    "Akun terkunci sementara demi keamanan karena 5 kali percobaan login gagal. Silakan coba kembali dalam {} detik.",
                    remaining
                ));
            }
        }
        Ok(())
    }

    pub async fn record_failure(&self, identifier: &str) {
        let mut attempts = self.attempts.lock().await;
        let id_clean = identifier.trim().to_lowercase();
        let now = Utc::now();
        let entry = attempts.entry(id_clean).or_insert((0, now));
        entry.0 += 1;
        if entry.0 >= 5 {
            // Lock out for 5 minutes
            entry.1 = now + ChronoDuration::minutes(5);
        }
    }

    pub async fn record_success(&self, identifier: &str) {
        let mut attempts = self.attempts.lock().await;
        let id_clean = identifier.trim().to_lowercase();
        attempts.remove(&id_clean);
    }
}

pub fn hash_password(password: &str) -> Result<String, bcrypt::BcryptError> {
    bcrypt::hash(password, 10)
}

pub fn verify_password(password: &str, hash: &str) -> bool {
    bcrypt::verify(password, hash).unwrap_or(false)
}

pub fn generate_jwt(user: &User) -> Result<String, jsonwebtoken::errors::Error> {
    let expiration = Utc::now()
        .checked_add_signed(ChronoDuration::hours(24))
        .expect("Valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: user.id.to_string(),
        nik: user.nik.clone(),
        name: user.name.clone(),
        email: user.email.clone(),
        role: user.role.to_string(),
        department: user.department.clone(),
        exp: expiration,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(JWT_SECRET),
    )
}

pub fn validate_jwt(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let token_clean = token.trim_start_matches("Bearer ").trim();
    let token_data = decode::<Claims>(
        token_clean,
        &DecodingKey::from_secret(JWT_SECRET),
        &Validation::default(),
    )?;
    Ok(token_data.claims)
}

pub fn validate_nik(nik: &str) -> Result<(), &'static str> {
    let clean = nik.trim();
    if clean.len() < 4 {
        return Err("NIK harus memiliki minimal 4 karakter.");
    }
    if clean.len() > 30 {
        return Err("NIK maksimal 30 karakter.");
    }
    if !clean.chars().all(|c| c.is_alphanumeric() || c == '-' || c == '_') {
        return Err("NIK hanya boleh mengandung huruf, angka, strip (-), atau garis bawah (_).");
    }
    Ok(())
}

pub fn validate_password_strength(password: &str) -> Result<(), &'static str> {
    if password.len() < 5 {
        return Err("Kata sandi harus minimal 5 karakter.");
    }
    Ok(())
}

pub async fn log_security_event(
    pool: &PgPool,
    user_nik: Option<&str>,
    user_email: Option<&str>,
    action: &str,
    severity: &str,
    ip_address: Option<&str>,
    user_agent: Option<&str>,
    details: Option<serde_json::Value>,
) {
    let details_val = details.unwrap_or(serde_json::json!({}));
    let _ = sqlx::query(
        r#"
        INSERT INTO security_audit_logs (user_nik, user_email, action, severity, ip_address, user_agent, details)
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        "#,
    )
    .bind(user_nik)
    .bind(user_email)
    .bind(action)
    .bind(severity)
    .bind(ip_address)
    .bind(user_agent)
    .bind(details_val)
    .execute(pool)
    .await;
}
