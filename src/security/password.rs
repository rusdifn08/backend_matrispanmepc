pub fn hash_password(password: &str) -> Result<String, bcrypt::BcryptError> {
    bcrypt::hash(password, 10)
}

pub fn verify_password(password: &str, hash: &str) -> bool {
    bcrypt::verify(password, hash).unwrap_or(false)
}

pub fn validate_password_strength(password: &str) -> Result<(), String> {
    if password.len() < 8 {
        return Err("Password minimal harus 8 karakter".to_string());
    }
    if !password.chars().any(|c| c.is_ascii_uppercase()) {
        return Err("Password harus mengandung minimal satu huruf besar (A-Z)".to_string());
    }
    if !password.chars().any(|c| c.is_ascii_digit()) {
        return Err("Password harus mengandung minimal satu angka (0-9)".to_string());
    }
    Ok(())
}
