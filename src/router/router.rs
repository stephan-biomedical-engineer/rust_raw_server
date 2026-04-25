use crate::http::request::Request;
use crate::http::response::Response;
use crate::routes::health::health;
use crate::routes::home::{about, home};
use crate::routes::users::{list_users, create_user};

pub struct Router;

impl Router 
{
    pub fn handle(request: &Request) -> Response 
    {
        match (request.method.as_str(), request.path.as_str()) 
        {
            ("GET", "/") => home(),
            ("GET", "/sobre") => about(),
            ("GET", "/health") => health(),
            ("GET", "/users") => list_users(),
            ("POST", "/users") => create_user(request),
            _ => Response::not_found(),
        }
    }
}