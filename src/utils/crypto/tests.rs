#[cfg(test)]
mod tests {
    use crate::utils::crypto::{decrypt, encrypt};
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
