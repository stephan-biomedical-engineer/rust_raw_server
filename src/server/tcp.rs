use std::sync::Arc;
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpStream;
use tokio::sync::Mutex;

use crate::http::request::Request;
use crate::http::response::Response;
use crate::router::router::Router;
use crate::state::AppState;

pub async fn handle_connection
    (
        mut stream: TcpStream,
        state: Arc<Mutex<AppState>>,
    )   
{
    let mut buffer = [0; 1024];
    
    stream
      .read(&mut buffer)
      .await
      .expect("[ERROR] Failed to read request");

    let response = match Request::from_buffer(&buffer)
    {
        Some(request) => Router::handle(&request, state).await,
        None => Response::new
        (
            400,
            "BAD REQUEST",
            "{\"error\":\"Invalid request\"}",
        ),
    };

    stream
      .write_all(response.to_http_string().as_bytes())
      .await
      .expect("[ERROR] Failed to write response");
}