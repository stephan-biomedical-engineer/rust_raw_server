use std::sync::Arc;
use tokio::time::{sleep, Duration};
use sqlx::PgPool;
use tower_governor::{governor::GovernorConfigBuilder, GovernorLayer};
use tower_http::cors::CorsLayer;


use axum::
{
    http::
    {
        header::{AUTHORIZATION, CONTENT_TYPE},
        Method,
    },
    routing::{get, post},
    Router,
};

use crate::config::Config;

use crate::routes::
{
    auth::{login, register},
    health::health,
    users::{delete_user, get_user, list_users, update_user},
};

#[derive(Clone)]
pub struct AppState 
{
    pub pool: PgPool,
    pub config: Config,
}

pub fn build_app(state: AppState) -> Router 
{
    let cors = CorsLayer::new()
        .allow_origin(tower_http::cors::Any)
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers([AUTHORIZATION, CONTENT_TYPE]);

    let governor_conf = Arc::new
    (
        GovernorConfigBuilder::default()
            .per_second(2)
            .burst_size(10)
            .finish()
            .unwrap(),
    );

    let governor_limiter = governor_conf.limiter().clone();

    tokio::spawn(async move 
    {
        loop 
        {
            sleep(Duration::from_secs(300)).await;

            tracing::debug!
            (
                "rate limiting storage size: {}",
                governor_limiter.len()
            );

            governor_limiter.retain_recent();
        }
    });

    let auth_routes = Router::new()
        .route("/register", post(register))
        .route("/login", post(login))
        .layer(GovernorLayer::new(governor_conf));

    Router::new()
        .route("/health", get(health))
        .route("/users", get(list_users))
        .route("/users/{id}", get(get_user).put(update_user).delete(delete_user))
        .nest("/auth", auth_routes)
        .with_state(state)
        .layer(cors)
}