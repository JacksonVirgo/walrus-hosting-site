use axum::{Router, routing::get};

use crate::app::server::AppState;

pub mod page;

pub fn router() -> Router<AppState> {
    Router::new().route("/", get(page::homepage))
}
