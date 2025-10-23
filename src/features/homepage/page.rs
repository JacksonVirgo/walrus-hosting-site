use axum::{
    extract::State,
    response::{Html, IntoResponse},
};
use axum_extra::extract::{CookieJar, cookie::Cookie};
use maud::html;

use crate::{app::server::AppState, utils::webpage::WebPageBuilder};

pub async fn homepage(State(_): State<AppState>, jar: CookieJar) -> impl IntoResponse {
    let count: u32 = jar
        .get("count")
        .and_then(|c| c.value().parse::<u32>().ok())
        .unwrap_or(0)
        + 1;
    let mut count_cookie = Cookie::new("count", count.to_string());
    count_cookie.set_path("/"); // make it valid for whole site
    count_cookie.set_http_only(true); // optional, hides from JS
    count_cookie.set_secure(false); // for localhost dev

    println!("Count: {}", count);

    let page = WebPageBuilder::new()
        .body(html! {
            h1."text-red-500 text-center" {
                "Walrus"
            }
            div {
                (count)
            }
        })
        .build();

    let jar = jar.add(count_cookie);
    (jar, Html(page.into_string()))
}
