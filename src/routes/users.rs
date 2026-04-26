use axum::{extract::{Path, State}, http::StatusCode, Json};
use validator::Validate;

use crate::models::user::{UpdateUserRequest, User};
use crate::responses::api_response::{service_error, unauthorized, validation_error, ApiError};
use crate::services::users_service;
use crate::auth::extractor::AuthUser;
use crate::app::AppState;

pub async fn list_users
    (
        State(state): State<AppState>,
    ) -> Result<Json<Vec<User>>, ApiError>
{
    let users = users_service::list_users(&state.pool)
        .await
        .map_err(|err| service_error(err, "Failed to fetch users"))?;

    Ok(Json(users))
}

pub async fn get_user
    (
        State(state): State<AppState>,
        Path(id): Path<i32>,
    ) -> Result<Json<User>, ApiError>
{
    let user = users_service::get_user(&state.pool, id)
        .await
        .map_err(|err| service_error(err, "Failed to fetch user"))?;

    Ok(Json(user))
}

pub async fn update_user
    (
        auth_user: AuthUser,
        State(state): State<AppState>,
        Path(id): Path<i32>,
        Json(payload): Json<UpdateUserRequest>,
    ) -> Result<Json<User>, ApiError>
{
    payload.validate()
        .map_err(|_| validation_error("Invalid update payload"))?;
        
    if auth_user.user_id != id
    {
        return Err(unauthorized("You can only update your own account"));
    }
    let user = users_service::update_user(&state.pool, id, payload.name)
        .await
        .map_err(|err| service_error(err, "Failed to update user"))?;

    Ok(Json(user))
}

pub async fn delete_user
    (
        auth_user: AuthUser,
        State(state): State<AppState>,
        Path(id): Path<i32>,
    ) -> Result<StatusCode, ApiError>
{
    if auth_user.user_id != id
    {
        return Err(unauthorized("You can only delete your own account"));
    }
    users_service::delete_user(&state.pool, id)
        .await
        .map_err(|err| service_error(err, "Failed to delete user"))?;

    Ok(StatusCode::NO_CONTENT)
}