use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use chrono::{Duration, Utc};
use jsonwebtoken::{encode, EncodingKey, Header};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Claims 
{
    pub sub: String, // ID do usuário (subject)
    pub exp: usize,  // Data de expiração
}

pub fn hash_password(password: &str) 
    -> Result<String, argon2::password_hash::Error> 
{
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    Ok(argon2.hash_password(password.as_bytes(), &salt)?.to_string())
}

pub fn verify_password(password: &str, password_hash: &str) 
    -> bool 
{
    let parsed_hash = match PasswordHash::new(password_hash) 
    {
        Ok(h) => h,
        Err(_) => return false,
    };
    Argon2::default().verify_password(password.as_bytes(), &parsed_hash).is_ok()
}

pub fn create_jwt(user_id: i32) 
    -> Result<String, jsonwebtoken::errors::Error> 
{
    let expiration = Utc::now()
        .checked_add_signed(Duration::hours(24))
        .expect("Timestamp inválido")
        .timestamp() as usize;

    let claims = Claims {
        sub: user_id.to_string(),
        exp: expiration,
    };

    let secret = std::env::var("JWT_SECRET").expect("[ERRO] JWT_SECRET não configurada no .env");
    encode(&Header::default(), &claims, &EncodingKey::from_secret(secret.as_bytes()))
}
