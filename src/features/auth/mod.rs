use crate::app::server::AppState;
use axum::{Router, routing::get};

pub mod callback;
pub mod cookies;
pub mod data;
pub mod login;
pub mod refresh;

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/auth/login", get(login::auth_discord))
        .route("/auth/callback", get(callback::auth_callback))
}
