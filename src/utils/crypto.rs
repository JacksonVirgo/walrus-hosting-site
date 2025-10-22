use anyhow::anyhow;
use chacha20poly1305::{
    ChaCha20Poly1305, Key, Nonce,
    aead::{Aead, AeadCore, KeyInit, OsRng},
};

use base64::{Engine, prelude::*};

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

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;
    use base64::Engine;
    use base64::prelude::BASE64_STANDARD;

    const TEST_KEY_STR: &str = "supersecretkey123456789012345678";
    const TEST_KEY_BYTES: &[u8] = TEST_KEY_STR.as_bytes();

    #[test]
    fn roundtrip() -> Result<()> {
        let plaintext = "hello, this is a secret";
        let cipher_b64 = encrypt(plaintext, TEST_KEY_BYTES)?;
        let recovered = decrypt(&cipher_b64, TEST_KEY_BYTES)?;
        assert_eq!(recovered, plaintext);
        Ok(())
    }

    #[test]
    fn decrypt_invalid_key() -> Result<()> {
        let plaintext = "top secret";
        let cipher_b64 = encrypt(plaintext, TEST_KEY_BYTES)?;

        let other_key = b"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
        assert_ne!(other_key, TEST_KEY_BYTES);

        let res = decrypt(&cipher_b64, other_key);
        assert!(res.is_err(), "decrypt should fail with wrong key");
        Ok(())
    }

    #[test]
    fn tampered_ciphertext() -> Result<()> {
        let plaintext = "integrity test";
        let cipher_b64 = encrypt(plaintext, TEST_KEY_BYTES)?;

        let mut raw = BASE64_STANDARD.decode(&cipher_b64)?;
        const NONCE_LEN: usize = 12;
        assert!(
            raw.len() > NONCE_LEN,
            "ciphertext too short for tamper test"
        );

        let idx = NONCE_LEN;
        raw[idx] ^= 0xFF;
        let tampered = BASE64_STANDARD.encode(&raw);

        let res = decrypt(&tampered, TEST_KEY_BYTES);
        assert!(res.is_err(), "decrypt should fail on tampered ciphertext");
        Ok(())
    }
}
