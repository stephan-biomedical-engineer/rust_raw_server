use axum::{routing::get, Router};
use dotenvy::dotenv;
use sqlx::PgPool;
use tokio::net::TcpListener;

mod models;
mod routes;

use routes::health::health;
use routes::users::{create_user, list_users};

#[tokio::main]
async fn main() 
{
    dotenv().ok();

    let database_url = std::env::var("DATABASE_URL")
        .expect("[ERROR] DATABASE_URL must be set");

    let pool = PgPool::connect(&database_url)
        .await
        .expect("[ERROR] Failed to connect to PostgreSQL");
    
    let app = Router::new()
        .route("/health", get(health))
        .route("/users", get(list_users).post(create_user))
        .with_state(pool);

    let listener = TcpListener::bind("127.0.0.1:7878")
        .await
        .expect("[ERROR] Failed to bind address");

    println!("Server running on http://{}", listener.local_addr().unwrap());

    axum::serve(listener, app)
        .await
        .expect("[ERROR] Server failed");
}