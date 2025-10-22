use axum::{Router, http::StatusCode, response::IntoResponse};

use crate::app::server::AppState;

pub fn router() -> Router<AppState> {
    Router::new().fallback(handle_404)
}

async fn handle_404() -> impl IntoResponse {
    (StatusCode::NOT_FOUND, "Not Found")
}
