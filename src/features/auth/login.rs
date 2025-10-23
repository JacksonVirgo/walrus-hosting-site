use axum::response::{IntoResponse, Redirect};

pub async fn auth_discord() -> impl IntoResponse {
    let client_id = std::env::var("DISCORD_CLIENT_ID").expect("DISCORD_CLIENT_ID was not supplied");
    let redirect_uri =
        std::env::var("DISCORD_OAUTH_REDIRECT").expect("DISCORD_OAUTH_REDIRECT was not supplied");

    let auth_url = format!(
        "https://discord.com/oauth2/authorize?client_id={}&redirect_uri={}&response_type=code&scope=identify",
        client_id,
        urlencoding::encode(&redirect_uri)
    );

    Redirect::to(&auth_url)
}
