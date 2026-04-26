use axum::
{
    extract::FromRequestParts,
    http::{request::Parts, StatusCode},
};

use crate::auth::jwt::decode_jwt;
use crate::responses::api_response::{ApiError, internal_error};

pub struct AuthUser
{
    pub user_id: i32,
}

impl<S> FromRequestParts<S> for AuthUser
where
    S: Send + Sync,
{
    type Rejection = ApiError;

    async fn from_request_parts(
        parts: &mut Parts,
        _state: &S,
    ) -> Result<Self, Self::Rejection>
    {
        let auth_header = parts.headers
            .get("Authorization")
            .and_then(|h| h.to_str().ok());

        let token = match auth_header
        {
            Some(value) if value.starts_with("Bearer ") =>
            {
                value.trim_start_matches("Bearer ").to_string()
            }
            _ =>
            {
                return Err((
                    StatusCode::UNAUTHORIZED,
                    axum::Json(serde_json::json!({
                        "error": "Missing or invalid Authorization header"
                    })),
                ));
            }
        };

        let claims = decode_jwt(&token).map_err(|_| {
            (
                StatusCode::UNAUTHORIZED,
                axum::Json(serde_json::json!({
                    "error": "Invalid token"
                })),
            )
        })?;

        let user_id = claims.sub.parse::<i32>().map_err(|_| {
            internal_error("Invalid token payload")
        })?;

        Ok(AuthUser { user_id })
    }
}