use tokio::net::TcpListener;
use crate::server::tcp::handle_connection;


pub struct Server 
{
    address: String,
}

impl Server 
{
    pub fn new(address: &str) -> Server 
    {
        Server 
        {
            address: address.to_string(),
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
            
            tokio::spawn
            (
                async move 
                {
                    handle_connection(stream).await;
                }
            );
        }
    }
}