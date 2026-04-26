use axum::{http::StatusCode, Json};
use serde_json::{json, Value};

use crate::services::users_service::ServiceError;

pub type ApiError = (StatusCode, Json<Value>);

pub fn internal_error(message: &str) -> ApiError
{
    (
        StatusCode::INTERNAL_SERVER_ERROR,
        Json
        (
            json!
            (
                {
                    "error": message
                }
            )
        ),
    )
}

pub fn not_found(message: &str) -> ApiError 
{
    (
        StatusCode::NOT_FOUND,
        Json(json!({ "error": message })),
    )
}

pub fn service_error(error: ServiceError, message: &str) -> ApiError
{
    match error
    {
        ServiceError::Database(err) =>
        {
            eprintln!("[DATABASE ERROR] {:?}", err);
            internal_error(message)
        }
        ServiceError::NotFound => not_found("User not found"),
    }
}