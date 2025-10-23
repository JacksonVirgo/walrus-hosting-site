use std::sync::Arc;

use crate::features::router;
use anyhow::Result;
use axum::{Router, http::HeaderValue};
use reqwest::Method;
use sqlx::{Pool, Postgres};
use tower_http::{cors::CorsLayer, services::ServeDir};
use tracing::info;

#[derive(Clone)]
pub struct Context {
    pub db: Pool<Postgres>,
}

pub type AppState = Arc<Context>;

pub async fn start_server(db: Pool<Postgres>) -> Result<()> {
    let address = std::env::var("ADDRESS").unwrap_or("0.0.0.0".into());
    let port = std::env::var("PORT").unwrap_or("3000".into());
    let target_url = format!("{}:{}", address, port);

    info!("Starting server at: http://{}", target_url);

    let state = Arc::new(Context { db });

    let app = generate_router(state);
    let listener = tokio::net::TcpListener::bind(target_url).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

pub fn generate_router(state: AppState) -> Router {
    let public = ServeDir::new("/app/public");

    let origin: HeaderValue = std::env::var("APP_DOMAIN")
        .expect("APP_DOMAIN must be set")
        .parse()
        .expect("APP_DOMAIN must be a valid URL");

    let cors = CorsLayer::new()
        .allow_origin(origin)
        .allow_credentials(true)
        .allow_methods([Method::GET, Method::POST, Method::PUT, Method::DELETE])
        .allow_headers(vec![]);

    Router::new()
        .merge(router())
        .with_state(state)
        .nest_service("/public", public)
        .layer(cors)
}
