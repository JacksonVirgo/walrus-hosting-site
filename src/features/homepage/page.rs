use axum::{
    extract::State,
    response::{Html, IntoResponse},
};
use maud::html;

use crate::{app::server::AppState, data::users::get_users, utils::webpage::WebPageBuilder};

pub async fn homepage(State(state): State<AppState>) -> impl IntoResponse {
    let users = format!(
        "[{}]",
        get_users(&state.db)
            .await
            .iter()
            .map(|v| v.id.to_string())
            .collect::<String>()
    );

    let page = WebPageBuilder::new()
        .body(html! {
            h1."text-red-500 text-center" {
                "Walrus"
            }

            div {
                (users)
            }
        })
        .build();

    Html(page.into_string())
}
