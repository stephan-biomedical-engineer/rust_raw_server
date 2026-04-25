use dotenvy::dotenv;
use sqlx::PgPool;

use rust_raw_server::server::server::Server;

#[tokio::main]
async fn main() 
{
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL")
        .expect("[ERROR] DATABASE_URL must be set");

    let pool = PgPool::connect(&database_url)
        .await
        .expect("[ERROR] Failed to connect to PostgreSQL");

    let server = Server::new("127.0.0.1:7878", pool);

    server.run().await;
}