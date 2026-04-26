use sqlx::PgPool;

use crate::auth::jwt::{create_jwt, hash_password, verify_password};
use crate::models::auth::{AuthResponse, LoginRequest, RegisterRequest};
use crate::models::user::User;
use crate::repositories::users_repository;

pub enum AuthError 
{
    Database(sqlx::Error),
    InvalidCredentials,
    UserAlreadyExists,
    HashingError,
    TokenCreationError,
}

pub async fn register(pool: &PgPool, req: RegisterRequest) -> Result<User, AuthError> 
{
    if users_repository::find_by_email(pool, &req.email)
        .await
        .map_err(AuthError::Database)
        ?.is_some() 
    {
        return Err(AuthError::UserAlreadyExists);
    }

    let hash = hash_password(&req.password).map_err(|_| AuthError::HashingError)?;

    let user = users_repository::create(pool, req.name, req.email, hash)
        .await
        .map_err(AuthError::Database)?;

    Ok(user)
}

pub async fn login(pool: &PgPool, req: LoginRequest) -> Result<AuthResponse, AuthError> 
{
    let user = users_repository::find_by_email(pool, &req.email)
        .await
        .map_err(AuthError::Database)?
        .ok_or(AuthError::InvalidCredentials)?; 

    if !verify_password(&req.password, &user.password_hash) 
    {
        return Err(AuthError::InvalidCredentials); 
    }

    let token = create_jwt(user.id).map_err(|_| AuthError::TokenCreationError)?;

    Ok(AuthResponse 
    {
        token,
        token_type: "Bearer".to_string(),
    })
}
