use rsa::{Pkcs1v15Encrypt, RsaPrivateKey};
use thiserror::Error;

use password_hash::{PasswordHash, PasswordHasher, SaltString};
use pbkdf2::Pbkdf2;
const SDK_PRIVATE_KEY: &[u8] = include_bytes!("../rsa/private_key.der");

#[derive(Error, Debug)]
pub enum CryptoError {
    #[error("failed to decrypt: {0}")]
    Decrypt(#[from] rsa::Error),
    #[error("failed to decode base64 string")]
    FromBase64,
    #[error("from_utf8 failed: {0}")]
    FromUtf8(#[from] std::string::FromUtf8Error),
}

pub fn rsa_decrypt(cipher: &str) -> Result<String, CryptoError> {
    let private_key: RsaPrivateKey = rsa::pkcs8::DecodePrivateKey::from_pkcs8_der(SDK_PRIVATE_KEY)
        .expect("failed to decode private key");
    let payload = private_key.decrypt(
        Pkcs1v15Encrypt,
        &rbase64::decode(cipher).map_err(|_| CryptoError::FromBase64)?,
    )?;

    Ok(String::from_utf8(payload)?)
}

pub fn hash_string(content: &str) -> Result<String, pbkdf2::password_hash::Error> {
    const HASH_PARAMS: pbkdf2::Params = pbkdf2::Params {
        rounds: 10_000,
        output_length: 32,
    };

    let salt = SaltString::generate(rand::thread_rng());
    let hash =
        Pbkdf2.hash_password_customized(content.as_bytes(), None, None, HASH_PARAMS, &salt)?;

    Ok(hash.serialize().to_string())
}

#[must_use]
pub fn verify_hash(content: &str, hash_str: &str) -> Option<()> {
    let hash = PasswordHash::new(hash_str).ok()?;
    hash.verify_password(&[&Pbkdf2], content).ok()
}
