use axum::{http::StatusCode, Json};
use serde_json::{json, Value};

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