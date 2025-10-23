use axum::{
    extract::State,
    response::{Html, IntoResponse},
};
use maud::html;

use crate::{app::server::AppState, utils::webpage::WebPageBuilder};

pub async fn homepage(State(_): State<AppState>) -> impl IntoResponse {
    let page = WebPageBuilder::new()
        .body(html! {
            h1."text-red-500 text-center" {
                "Walrus"
            }
        })
        .build();

    Html(page.into_string())
}
