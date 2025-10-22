use axum::response::{Html, IntoResponse};
use maud::html;

use crate::utils::webpage::WebPageBuilder;

pub async fn homepage() -> impl IntoResponse {
    let page = WebPageBuilder::new()
        .body(html! {
            h1 {
                "Walrus"
            }
        })
        .build();

    Html(page.into_string())
}
