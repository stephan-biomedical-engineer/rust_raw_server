use sqlx::PgPool;
use tokio::net::TcpListener;

use crate::server::tcp::handle_connection;

pub struct Server 
{
    address: String,
    pool: PgPool,
}

impl Server 
{
    pub fn new(address: &str, pool: PgPool) -> Server 
    {
        Server 
        {
            address: address.to_string(),
            pool,
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
            
            let pool = self.pool.clone();

            tokio::spawn
            (
                async move 
                {
                    handle_connection(stream, pool).await;
                }
            );
        }
    }
}