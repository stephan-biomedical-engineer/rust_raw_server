use axum::{routing::get, Router};
use dotenvy::dotenv;
use sqlx::PgPool;
use tokio::net::TcpListener;
use tracing::info;
use tracing_subscriber::EnvFilter;

mod models;
mod repositories;
mod responses;
mod routes;
mod services;

use routes::health::health;
use routes::users::{create_user, list_users, get_user, update_user, delete_user};

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
    
    let app = Router::new()
        .route("/health", get(health))
        .route("/users", get(list_users).post(create_user))
        .route("/users/{id}", get(get_user).put(update_user).delete(delete_user))
        .with_state(pool);

    let listener = TcpListener::bind("0.0.0.0:7878")
        .await
        .expect("[ERROR] Failed to bind address");

    info!("Server running on http://{}", listener.local_addr().unwrap());

    axum::serve(listener, app)
        .await
        .expect("[ERROR] Server failed");
}