use reqwest::Client;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct DiscordUserData {
    pub id: String,
    pub username: String,
    pub discriminator: String,
    pub avatar: Option<String>,
}

pub struct DiscordUser {
    pub client: Option<Client>,
    pub access_token: String,
}

impl DiscordUser {
    pub fn from_token(access_token: String) -> Self {
        Self {
            client: None,
            access_token,
        }
    }

    pub fn with_client(&mut self, client: &Client) -> &mut Self {
        self.client = Some(client.clone());
        self
    }

    pub async fn fetch(&self) -> anyhow::Result<DiscordUserData> {
        let client = self.client.clone().unwrap_or(Client::new());
        let data = client
            .get("https://discord.com/api/users/@me")
            .bearer_auth(&self.access_token)
            .send()
            .await?
            .json()
            .await?;
        Ok(data)
    }
}
