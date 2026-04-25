use std::sync::Arc;

use serde::{Deserialize, Serialize};
use tokio::sync::Mutex;

use crate::http::request::Request;
use crate::http::response::Response;
use crate::state::AppState;

#[derive(Serialize, Clone, Debug)]
pub struct User
{
    pub id: u32,
    pub name: String,
}

#[derive(Deserialize)]
struct CreateUserRequest
{
    name: String,
}

pub async fn list_users(state: Arc<Mutex<AppState>>) -> Response
{
    let state = state.lock().await;

    let json = serde_json::to_string(&state.users)
        .expect("[ERROR] Failed to serialize users");

    Response::json(200, "OK", &json)
}

pub async fn create_user
(
    request: &Request,
    state: Arc<Mutex<AppState>>,
) -> Response
{
    let payload: CreateUserRequest = match serde_json::from_str(&request.body)
    {
        Ok(payload) => payload,
        Err(_) =>
        {
            return Response::json(
                400,
                "BAD REQUEST",
                "{\"error\":\"Invalid JSON payload\"}",
            );
        }
    };

    let mut state = state.lock().await;

    let user = User
    {
        id: state.next_user_id,
        name: payload.name,
    };

    state.next_user_id += 1;
    state.users.push(user.clone());

    let json = serde_json::to_string(&user)
        .expect("[ERROR] Failed to serialize created user");

    Response::json(201, "CREATED", &json)
}