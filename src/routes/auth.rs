use axum::{extract::State, http::StatusCode, Json};
use sqlx::PgPool;

use crate::models::auth::{AuthResponse, LoginRequest, RegisterRequest};
use crate::models::user::User;
use crate::responses::api_response::{ApiError, conflict, internal_error, unauthorized};
use crate::services::auth_service::{self, AuthError};

fn auth_error(err: AuthError) -> ApiError
{
    match err
    {
        AuthError::Database(e) =>
        {
            tracing::error!(error = ?e, "Database error");
            internal_error("Database error")
        }
        AuthError::InvalidCredentials => unauthorized("Invalid credentials"),
        AuthError::UserAlreadyExists => conflict("User already exists"),
        AuthError::HashingError => internal_error("Failed to hash password"),
        AuthError::TokenCreationError => internal_error("Failed to create token"),
    }
}

pub async fn register
(
    State(pool): State<PgPool>,
    Json(payload): Json<RegisterRequest>,
) -> Result<(StatusCode, Json<User>), ApiError>
{
    let user = auth_service::register(&pool, payload)
        .await
        .map_err(auth_error)?;

    Ok((StatusCode::CREATED, Json(user)))
}

pub async fn login
(
    State(pool): State<PgPool>,
    Json(payload): Json<LoginRequest>,
) -> Result<Json<AuthResponse>, ApiError>
{
    let response = auth_service::login(&pool, payload)
        .await
        .map_err(auth_error)?;

    Ok(Json(response))
}