use std::io::{Read, Write};
use std::net::{TcpStream};

use crate::http::request::Request;
use crate::http::response::Response;
use crate::router::router::Router;

pub fn handle_connection(mut stream: TcpStream)
{
    let mut buffer = [0; 1024];
    
    stream
      .read(&mut buffer)
      .expect("[ERROR] Failed to read request");

    let response = match Request::from_buffer(&buffer)
    {
        Some(request) => Router::handle(&request),
        None => Response::new(
            400,
            "BAD REQUEST",
            "<h1>400</h1><p>Requisição inválida.</p>",
        ),
    };

    stream
      .write_all(response.to_http_string().as_bytes())
      .expect("[ERROR] Failed to write response");
}