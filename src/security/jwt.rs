use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};

pub const JWT_SECRET: &[u8] = b"pt_matrispan_exim_phoenix_corp_super_secure_enterprise_key_2026";

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

pub fn create_jwt_token(
    user_id: &str,
    nik: &str,
    name: &str,
    email: &str,
    role: &str,
    department: &str,
) -> Result<String, jsonwebtoken::errors::Error> {
    let expiration = Utc::now()
        .checked_add_signed(Duration::days(7))
        .expect("Valid timestamp")
        .timestamp() as usize;

    let claims = Claims {
        sub: user_id.to_string(),
        nik: nik.to_string(),
        name: name.to_string(),
        email: email.to_string(),
        role: role.to_string(),
        department: department.to_string(),
        exp: expiration,
    };

    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(JWT_SECRET),
    )
}

pub fn verify_jwt_token(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let token_data = decode::<Claims>(
        token,
        &DecodingKey::from_secret(JWT_SECRET),
        &Validation::default(),
    )?;
    Ok(token_data.claims)
}
