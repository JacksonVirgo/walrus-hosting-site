use crate::features::router;
use anyhow::Result;
use axum::Router;
use tower_http::services::ServeDir;
use tracing::info;

#[derive(Clone)]
pub struct AppState {}

pub async fn start_server() -> Result<()> {
    let address = std::env::var("ADDRESS").unwrap_or("0.0.0.0".into());
    let port = std::env::var("PORT").unwrap_or("3000".into());
    let target_url = format!("{}:{}", address, port);

    info!("Starting server at: http://{}", target_url);

    let state = AppState {};

    let app = generate_router(state);
    let listener = tokio::net::TcpListener::bind(target_url).await?;
    axum::serve(listener, app).await?;

    Ok(())
}

pub fn generate_router(state: AppState) -> Router {
    let public = ServeDir::new("/app/public");
    Router::new()
        .merge(router())
        .with_state(state)
        .nest_service("/public", public)
}
