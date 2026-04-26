use std::net::SocketAddr;

use dotenvy::dotenv;
use sqlx::PgPool;
use tokio::net::TcpListener;
use tracing::info;
use tracing_subscriber::EnvFilter;

use rust_raw_server::app::build_app;

#[tokio::main]
async fn main() 
{
    dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::new("info"))
        .init();

    let database_url = std::env::var("DATABASE_URL")
        .expect("[ERROR] DATABASE_URL must be set");

    let pool = PgPool::connect(&database_url)
        .await
        .expect("[ERROR] Failed to connect to PostgreSQL");

    let app = build_app(pool);

    let listener = TcpListener::bind("0.0.0.0:7878")
        .await
        .expect("[ERROR] Failed to bind address");

    info!("Server running on http://{}", listener.local_addr().unwrap());

    axum::serve
    (
        listener,
        app.into_make_service_with_connect_info::<SocketAddr>(),
    )
        .await
        .expect("[ERROR] Server failed");
}