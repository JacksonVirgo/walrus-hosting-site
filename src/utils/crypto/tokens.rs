use base64::{Engine, engine::general_purpose};
use rand::TryRngCore;

pub fn generate_token() -> anyhow::Result<String> {
    Ok(generate_token_custom(32)?)
}

pub fn generate_token_custom(len: usize) -> anyhow::Result<String> {
    let mut bytes = vec![0u8; len];
    let mut rng = rand::rng();
    rng.try_fill_bytes(&mut bytes)?;
    Ok(general_purpose::URL_SAFE_NO_PAD.encode(&bytes))
}
