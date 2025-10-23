use crate::app::server::AppState;
use axum::{Router, routing::get};

pub mod auth;
pub mod discord;
pub mod homepage;
pub mod not_found;

pub fn router() -> Router<AppState> {
    Router::new()
        .merge(homepage::router())
        .merge(auth::router())
        .route("/test", get(test))
        .fallback(not_found::handle_404)
}

pub async fn test() -> impl axum::response::IntoResponse {
    let oob = maud::html! {
        div id="test" hx-swap-oob="true" {
            "Hi there! Replaced via OOB"
        }
    };

    axum::response::Html(oob.into_string())
}
