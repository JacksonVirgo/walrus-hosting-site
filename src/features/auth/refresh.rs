use axum::{extract::State, response::IntoResponse};
use axum_extra::extract::CookieJar;
use reqwest::StatusCode;

use crate::{app::server::AppState, data::sessions::Session, utils::snowflake::Snowflake};

pub async fn token_refresh_endpoint(
    State(ctx): State<AppState>,
    jar: CookieJar,
) -> impl IntoResponse {
    let Some(id) = jar.get("access_id") else {
        return StatusCode::UNAUTHORIZED;
    };

    let Some(refresh) = jar.get("refresh_token") else {
        return StatusCode::UNAUTHORIZED;
    };

    let Ok(id) = id.value().parse::<Snowflake>() else {
        return StatusCode::UNAUTHORIZED;
    };

    let session = sqlx::query!(
        r#"
        SELECT (id, refresh_token, refresh_expires_at)
        FROM sessions
        WHERE id = $1
    "#,
        id
    )
    .fetch_optional(&ctx.db)
    .await;

    StatusCode::ACCEPTED
}
