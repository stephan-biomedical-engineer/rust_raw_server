use std::{net::SocketAddr, sync::Arc};
use dotenvy::dotenv;
use sqlx::PgPool;
use tokio::net::TcpListener;
use tracing::info;
use tracing_subscriber::EnvFilter;
use tower_http::cors::CorsLayer;

use routes::
{
    auth::{login, register},
    health::health,
    users::{list_users, get_user, update_user, delete_user},
};

use axum::
{
    routing::{get, post}, 
    Router,
    http::{header::{AUTHORIZATION, CONTENT_TYPE}, Method,}
};

use tower_governor::
{
    governor::GovernorConfigBuilder,
    GovernorLayer,
};

mod models;
mod repositories;
mod responses;
mod routes;
mod services;
mod auth;


#[tokio::main]
async fn main() 
{
    dotenv().ok();

    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::new("info"))
        .init();

    let governor_conf = Arc::new
    (
        GovernorConfigBuilder::default()
            .per_second(2)
            .burst_size(10)
            .finish()
            .unwrap(),
    );

    let governor_limiter = governor_conf.limiter().clone();

    std::thread::spawn(move || 
    {
        loop 
        {
            std::thread::sleep(std::time::Duration::from_secs(300));

            tracing::debug!
            (
                "rate limiting storage size: {}",
                governor_limiter.len()
            );

            governor_limiter.retain_recent();
        }
    });

    let database_url = std::env::var("DATABASE_URL")
        .expect("[ERROR] DATABASE_URL must be set");

    let pool = PgPool::connect(&database_url)
        .await
        .expect("[ERROR] Failed to connect to PostgreSQL");

    let cors = CorsLayer::new()
        // .allow_origin("http://localhost:5173".parse::<HeaderValue>().unwrap())
        .allow_origin(tower_http::cors::Any)
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers([AUTHORIZATION, CONTENT_TYPE]);

    let auth_routes = Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .layer(GovernorLayer::new(governor_conf));

    let app = Router::new()
        .route("/health", get(health))
        .route("/users", get(list_users))
        .route("/users/{id}", get(get_user).put(update_user).delete(delete_user))
        .nest("/auth", auth_routes)
        .with_state(pool)
        .layer(cors);

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