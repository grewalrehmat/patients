use argon2::{Argon2, PasswordHash, PasswordHasher, PasswordVerifier};
use argon2::password_hash::SaltString;
use rand::rngs::OsRng;

pub fn hash_password(password: &str) -> Result<String, argon2::password_hash::Error> {
    // Generate a random salt
    let salt = SaltString::generate(&mut OsRng);

    // Hash password using Argon2 with default config
    let argon2 = Argon2::default();
    let password_hash = argon2.hash_password(password.as_bytes(), &salt)?;

    // Return PHC string
    Ok(password_hash.to_string())
}

pub fn verify_password(hash: &str, password: &str) -> Result<bool, argon2::password_hash::Error> {
    let parsed_hash = PasswordHash::new(hash)?;
    Ok(Argon2::default().verify_password(password.as_bytes(), &parsed_hash).is_ok())
}