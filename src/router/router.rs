use std::sync::Arc;
use tokio::sync::Mutex;

use crate::http::request::Request;
use crate::http::response::Response;
use crate::routes::health::health;
use crate::routes::home::{about, home};
use crate::routes::users::{list_users, create_user};
use crate::state::AppState;

pub struct Router;

impl Router 
{
    pub async fn handle
    (
        request: &Request,
        state: Arc<Mutex<AppState>>,
    ) -> Response 
    {
        match (request.method.as_str(), request.path.as_str()) 
        {
            ("GET", "/") => home(),
            ("GET", "/sobre") => about(),
            ("GET", "/health") => health(),
            ("GET", "/users") => list_users(state).await,
            ("POST", "/users") => create_user(request, state).await,
            _ => Response::not_found(),
        }
    }
}