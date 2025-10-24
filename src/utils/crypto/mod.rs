use anyhow::anyhow;
use argon2::{
    Argon2,
    password_hash::{PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
};
use base64::{Engine, prelude::*};
use chacha20poly1305::{
    ChaCha20Poly1305, Key, Nonce,
    aead::{Aead, AeadCore, KeyInit, OsRng},
};

pub mod tests;
pub mod tokens;

pub fn key_from_env() -> anyhow::Result<Vec<u8>> {
    let raw =
        std::env::var("ENCRYPTION_KEY").map_err(|_| anyhow::anyhow!("ENCRYPTION_KEY not set"))?;

    let val = raw.as_bytes().to_owned();

    if val.len() != 32 {
        return Err(anyhow::anyhow!("ENCRYPTION_KEY must be 32 bytes"));
    }

    Ok(val)
}

pub fn encrypt(data: &str, key_bytes: &[u8]) -> anyhow::Result<String> {
    #[allow(deprecated)]
    let key = Key::from_slice(key_bytes);
    let cipher = ChaCha20Poly1305::new(&key);
    let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng);
    let ciphertext = cipher
        .encrypt(&nonce, data.as_bytes())
        .map_err(|e| anyhow::anyhow!(e.to_string()))?;

    let mut out = nonce.to_vec();
    out.extend_from_slice(&ciphertext);

    Ok(BASE64_STANDARD.encode(out))
}

pub fn decrypt(encoded: &str, key_bytes: &[u8]) -> anyhow::Result<String> {
    #[allow(deprecated)]
    let key = Key::from_slice(key_bytes);
    let cipher = ChaCha20Poly1305::new(&key);

    let mut data = BASE64_STANDARD
        .decode(encoded)
        .map_err(|e| anyhow::anyhow!(e.to_string()))?;

    const NONCE_LEN: usize = 12;
    if data.len() < NONCE_LEN {
        return Err(anyhow!("ciphertext too short"));
    }

    let nonce_bytes = data.drain(..NONCE_LEN).collect::<Vec<u8>>();
    #[allow(deprecated)]
    let nonce = Nonce::from_slice(&nonce_bytes);

    let plaintext = cipher
        .decrypt(nonce, data.as_ref())
        .map_err(|e| anyhow::anyhow!(e.to_string()))?;

    Ok(String::from_utf8(plaintext)?)
}

pub fn hash(str: &String) -> anyhow::Result<String> {
    let bytes = str.as_bytes();
    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let hash = argon2
        .hash_password(bytes, &salt)
        .map_err(|e| anyhow::anyhow!(e.to_string()))?
        .to_string();
    Ok(hash)
}

pub fn verify_hash(data: &String, hash: &String) -> bool {
    let Ok(parsed_hash) = PasswordHash::new(&hash) else {
        return false;
    };
    Argon2::default()
        .verify_password(data.as_bytes(), &parsed_hash)
        .is_ok()
}
