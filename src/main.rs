use std::sync::Arc;
use tokio::sync::Mutex;

use mini_servidor::server::server::Server;
use mini_servidor::state::AppState;

#[tokio::main]
async fn main() 
{
    let state = Arc::new(Mutex::new(AppState::new()));

    let server = Server::new("127.0.0.1:7878", state);

    server.run().await;
}