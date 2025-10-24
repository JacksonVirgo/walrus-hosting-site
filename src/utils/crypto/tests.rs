#[cfg(test)]
mod tests {
    use crate::utils::crypto::{decrypt, encrypt, hash, verify_hash};
    use anyhow::Result;
    use base64::Engine;
    use base64::prelude::BASE64_STANDARD;

    const TEST_KEY: &[u8] = b"supersecretkey123456789012345678";

    #[test]
    fn encrypt_roundtrip() -> Result<()> {
        let plaintext = "hello, this is a secret";
        let cipher_b64 = encrypt(plaintext, TEST_KEY)?;
        let recovered = decrypt(&cipher_b64, TEST_KEY)?;
        assert_eq!(recovered, plaintext);
        Ok(())
    }

    #[test]
    fn decrypt_invalid_key() -> Result<()> {
        let plaintext = "top secret";
        let cipher_b64 = encrypt(plaintext, TEST_KEY)?;

        let other_key = b"aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa";
        assert_ne!(other_key, TEST_KEY);

        let res = decrypt(&cipher_b64, other_key);
        assert!(res.is_err(), "decrypt should fail with wrong key");
        Ok(())
    }

    #[test]
    fn tampered_encrypt_ciphertext() -> Result<()> {
        let plaintext = "integrity test";
        let cipher_b64 = encrypt(plaintext, TEST_KEY)?;

        let mut raw = BASE64_STANDARD.decode(&cipher_b64)?;
        const NONCE_LEN: usize = 12;
        assert!(
            raw.len() > NONCE_LEN,
            "ciphertext too short for tamper test"
        );

        let idx = NONCE_LEN;
        raw[idx] ^= 0xFF;
        let tampered = BASE64_STANDARD.encode(&raw);

        let res = decrypt(&tampered, TEST_KEY);
        assert!(res.is_err(), "decrypt should fail on tampered ciphertext");
        Ok(())
    }

    #[test]
    fn hash_roundtrip() -> Result<()> {
        let plaintext = "this is a token".to_string();
        let hashed = hash(&plaintext)?;
        let verify = verify_hash(&plaintext, &hashed);
        assert_eq!(verify, true);
        Ok(())
    }

    #[test]
    fn invalid_hash_verify() -> Result<()> {
        let plaintext = "this is the correct token".to_string();
        let hashed = hash(&plaintext)?;
        let verify = verify_hash(&"this is an invalid token".to_string(), &hashed);
        assert_eq!(verify, false);
        Ok(())
    }
}
