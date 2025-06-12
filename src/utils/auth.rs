use bcrypt::{hash, verify, DEFAULT_COST};
use jsonwebtoken::{encode, Header, EncodingKey};
use serde::{Serialize, Deserialize};
use std::env;
use chrono::Utc;

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: String,
    pub role: String,
    pub exp: usize,
}

pub fn hash_password(password: &str) -> Result<String, bcrypt::BcryptError> {
    hash(password, DEFAULT_COST)
}

pub fn verify_password(password: &str, hash: &str) -> bool {
    verify(password, hash).unwrap_or(false)
}

pub fn generate_token(username: &str, role: &str) -> String {
    let expiration = Utc::now()
        .checked_add_signed(chrono::Duration::hours(24))
        .unwrap()
        .timestamp() as usize;

    let claims = Claims {
        sub: username.to_string(),
        role: role.to_string(),
        exp: expiration,
    };

    let secret = env::var("JWT_SECRET").expect("JWT_SECRET must be set");
    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_bytes()))
        .expect("Failed to encode token")
}