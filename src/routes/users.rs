use axum::{extract::{State, Path}, http::StatusCode, Json};
use sqlx::PgPool;

use crate::models::user::{CreateUserRequest, User};
use crate::repositories::users_repository;
use crate::responses::api_response::{internal_error, ApiError, not_found};

pub async fn list_users
    (
        State(pool): State<PgPool>,
    ) -> Result<Json<Vec<User>>, ApiError>
{
    let users = users_repository::find_all(&pool)
        .await
        .map_err(|err| 
        {
            eprintln!("[DATABASE ERROR] {:?}", err);
            internal_error("Failed to fetch users")
        })?;

    Ok(Json(users))
}

pub async fn create_user
    (
        State(pool): State<PgPool>,
        Json(payload): Json<CreateUserRequest>,
    ) -> Result<(StatusCode, Json<User>), ApiError>
{
    let user = users_repository::create(&pool, payload.name)
        .await
        .map_err(|err| 
        {
            eprintln!("[DATABASE ERROR] {:?}", err);
            internal_error("Failed to create user")
        })?;

    Ok((StatusCode::CREATED, Json(user)))
}

pub async fn update_user
    (
        State(pool): State<PgPool>,
        Path(id): Path<i32>,
        Json(payload): Json<CreateUserRequest>,
    ) -> Result<Json<User>, ApiError> 
{
    let user = users_repository::update(&pool, id, payload.name)
        .await
        .map_err(|err| {
            eprintln!("[DATABASE ERROR] {:?}", err);
            internal_error("Failed to update user")
        })?;

    match user {
        Some(u) => Ok(Json(u)),
        None => Err(not_found("User not found")),
    }
}

pub async fn get_user
    (
        State(pool): State<PgPool>,
        Path(id): Path<i32>,
    ) -> Result<Json<User>, ApiError> 
{
    let user = users_repository::find_by_id(&pool, id)
        .await
        .map_err(|err| 
        {
            eprintln!("[DATABASE ERROR] {:?}", err);
            internal_error("Failed to fetch user")
        })?;

    match user 
    {
        Some(u) => Ok(Json(u)),
        None => Err(not_found("User not found")),
    }
}

pub async fn delete_user
    (
        State(pool): State<PgPool>,
        Path(id): Path<i32>,
    ) -> Result<StatusCode, ApiError> 
{
    let rows_affected = users_repository::delete(&pool, id)
        .await
        .map_err(|err| 
        {
            eprintln!("[DATABASE ERROR] {:?}", err);
            internal_error("Failed to delete user")
        })?;

    if rows_affected == 0 
    {
        return Err(not_found("User not found"));
    }

    Ok(StatusCode::NO_CONTENT)
}