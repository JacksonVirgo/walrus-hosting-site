use serde::Deserialize;

pub const ACCESS_EXPIRY: i64 = 60 * 30;
pub const REFRESH_EXPIRY: i64 = 60 * 60 * 24 * 14;

#[derive(Deserialize)]
pub struct AuthQuery {
    pub code: String,
}

#[derive(Deserialize)]
pub struct DiscordTokenResponse {
    pub access_token: String,
    pub token_type: String,
    pub expires_in: u64,
    pub refresh_token: String,
    pub scope: String,
}
