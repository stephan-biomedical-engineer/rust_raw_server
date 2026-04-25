use sqlx::PgPool;
use serde::{Deserialize, Serialize};

use crate::http::request::Request;
use crate::http::response::Response;

#[derive(Serialize, sqlx::FromRow)]
pub struct User
{
    pub id: i32,
    pub name: String,
}

#[derive(Deserialize)]
struct CreateUserRequest
{
    name: String,
}

pub async fn list_users(pool: PgPool) -> Response
{
    let users = match sqlx::query_as::<_, User>
        (
            "SELECT id, name FROM users ORDER BY id"
        )
        .fetch_all(&pool)
        .await
    {
        Ok(users) => users,
        Err(_) =>
        {
            return Response::json
            (
                500,
                "INTERNAL SERVER ERROR",
                "{\"error\":\"Failed to fetch users\"}",
            )
        }
    };    


    let json = serde_json::to_string(&users)
        .expect("[ERROR] Failed to serialize users");

    Response::json(200, "OK", &json)
}

pub async fn create_user
(
    request: &Request,
    pool: PgPool,
) -> Response
{
    let payload: CreateUserRequest = match serde_json::from_str(&request.body)
    {
        Ok(payload) => payload,
        Err(_) =>
        {
            return Response::json
            (
                400,
                "BAD REQUEST",
                "{\"error\":\"Invalid JSON payload\"}",
            );
        }
    };

    let user = match sqlx::query_as::<_, User>
        (
            "INSERT INTO users (name) VALUES ($1) RETURNING id, name"
        )
        .bind(&payload.name)
        .fetch_one(&pool)
        .await
    {
        Ok(user) => user,
        Err(_) =>
        {
            return Response::json
            (
                500,
                "INTERNAL SERVER ERROR",
                "{\"error\":\"Failed to create user\"}",
            );
        }
    };

    let json = serde_json::to_string(&user)
        .expect("[ERROR] Failed to serialize created user");

    Response::json(201, "CREATED", &json)
}