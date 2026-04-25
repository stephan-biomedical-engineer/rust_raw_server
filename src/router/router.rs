use crate::http::request::Request;
use crate::http::response::Response;
use crate::routes::home::{about, home};

pub struct Router;

impl Router 
{
    pub fn handle(request: &Request) -> Response 
    {
        match (request.method.as_str(), request.path.as_str()) 
        {
            ("GET", "/") => home(),
            ("GET", "/sobre") => about(),
            _ => Response::not_found(),
        }
    }
}