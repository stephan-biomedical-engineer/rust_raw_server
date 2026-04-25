use axum::{extract::State, http::StatusCode, Json};
use serde_json::Value;
use sqlx::{PgPool, Postgres};

use crate::models::user::{CreateUserRequest, User};

pub async fn list_users(
    State(pool): State<PgPool>,
) -> Result<Json<Vec<User>>, (StatusCode, Json<Value>)>
{
    let users = sqlx::query_as::<Postgres, User>(
        "SELECT id, name FROM users ORDER BY id"
    )
    .fetch_all(&pool)
    .await
    .map_err(|err| {
        eprintln!("[DATABASE ERROR] {:?}", err);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({
                "error": "Failed to fetch users"
            })),
        )
    })?;

    Ok(Json(users))
}

pub async fn create_user(
    State(pool): State<PgPool>,
    Json(payload): Json<CreateUserRequest>,
) -> Result<(StatusCode, Json<User>), (StatusCode, Json<Value>)>
{
    let user = sqlx::query_as::<Postgres, User>(
        "INSERT INTO users (name) VALUES ($1) RETURNING id, name"
    )
    .bind(payload.name)
    .fetch_one(&pool)
    .await
    .map_err(|err| {
        eprintln!("[DATABASE ERROR] {:?}", err);
        (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({
                "error": "Failed to create user"
            })),
        )
    })?;

    Ok((StatusCode::CREATED, Json(user)))
}