use std::sync::Arc;
use tokio::net::TcpListener;
use tokio::sync::Mutex;

use crate::server::tcp::handle_connection;
use crate::state::AppState;


pub struct Server 
{
    address: String,
    state: Arc<Mutex<AppState>>,
}

impl Server 
{
    pub fn new(address: &str, state: Arc<Mutex<AppState>>) -> Server 
    {
        Server 
        {
            address: address.to_string(),
            state,
        }
    }

    pub async fn run(&self) 
    {
        let listener = TcpListener::bind(&self.address)
          .await
          .expect("[ERROR] failed to bind address");

        println!("[SUCCESS] Async server running at http://{}", self.address);

        loop
        {
            let (stream, _) = listener
              .accept()
              .await
              .expect("[ERROR] Failed to accept connection");
            
            let state = Arc::clone(&self.state);

            tokio::spawn
            (
                async move 
                {
                    handle_connection(stream, state).await;
                }
            );
        }
    }
}