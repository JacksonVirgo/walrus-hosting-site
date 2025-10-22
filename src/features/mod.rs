use axum::{
    Router,
    http::StatusCode,
    response::{Html, IntoResponse},
};
use maud::html;

use crate::{app::server::AppState, utils::WebPageBuilder};

pub fn router() -> Router<AppState> {
    Router::new().fallback(handle_404)
}

async fn handle_404() -> impl IntoResponse {
    let page = WebPageBuilder::new()
        .body(html! {
            h1 {
                "Not Found"
            }
            div {
                "Requested page does not exist"
            }
        })
        .build();

    (StatusCode::NOT_FOUND, Html(page.into_string()))
}
