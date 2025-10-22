use axum::{
    http::StatusCode,
    response::{Html, IntoResponse},
};
use maud::html;

use crate::utils::webpage::WebPageBuilder;

pub async fn handle_404() -> impl IntoResponse {
    let page = WebPageBuilder::new()
        .body(html! {
            h1 {
                "Not Found!"
            }
            div hx-on="click" hx-get="/test"  {
                "Requested page does not exist"
            }

            div id="test" hx-swap-oob="true" {
                "Testtt"
            }
        })
        .build();

    (StatusCode::NOT_FOUND, Html(page.into_string()))
}
