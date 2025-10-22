use axum::response::{Html, IntoResponse};
use maud::html;

use crate::utils::webpage::WebPageBuilder;

pub async fn homepage() -> impl IntoResponse {
    let page = WebPageBuilder::new()
        .body(html! {
            h1."text-red-500 text-center" {
                "Walrus"
            }
        })
        .build();

    Html(page.into_string())
}
