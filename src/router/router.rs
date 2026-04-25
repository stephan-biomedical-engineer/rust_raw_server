use sqlx::PgPool;

use crate::http::request::Request;
use crate::http::response::Response;
use crate::routes::health::health;
use crate::routes::home::{about, home};
use crate::routes::users::{list_users, create_user};

pub struct Router;

impl Router 
{
    pub async fn handle
    (
        request: &Request,
        pool: PgPool,
    ) -> Response 
    {
        match (request.method.as_str(), request.path.as_str()) 
        {
            ("GET", "/") => home(),
            ("GET", "/about") => about(),
            ("GET", "/health") => health(),
            ("GET", "/users") => list_users(pool).await,
            ("POST", "/users") => create_user(request, pool).await,
            _ => Response::not_found(),
        }
    }
}