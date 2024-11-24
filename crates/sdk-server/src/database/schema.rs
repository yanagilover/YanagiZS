use std::sync::LazyLock;

use rand::distributions::{Alphanumeric, DistString};
use regex::Regex;
use serde::{Deserialize, Serialize};
use surrealdb::RecordId;

use crate::util;

#[derive(Debug, Serialize, Deserialize)]
pub struct Account {
    pub id: Option<RecordId>,
    pub username: Username,
    pub password: Password,
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(transparent)]
pub struct Username(String);

#[derive(Debug, Serialize, Deserialize)]
pub struct Password(String);

impl Account {
    pub fn new(username: Username, password: Password) -> Self {
        const TOKEN_LENGTH: usize = 64;

        Self {
            id: None,
            username,
            password,
            token: Alphanumeric.sample_string(&mut rand::thread_rng(), TOKEN_LENGTH),
        }
    }

    pub fn uid(&self) -> String {
        let mut uid = self.id.clone().unwrap().key().to_string();
        uid.retain(|c| !['⟨', '⟩'].contains(&c));

        uid
    }
}

impl Username {
    pub fn parse(username: String) -> Option<Self> {
        static ALLOWED_PATTERN: LazyLock<Regex> =
            LazyLock::new(|| Regex::new("^[a-zA-Z0-9._@-]{6,25}$").unwrap());

        ALLOWED_PATTERN
            .is_match(&username)
            .then_some(Self(username))
    }

    pub fn as_str(&self) -> &str {
        self.0.as_str()
    }
}

impl Password {
    const PASSWORD_LENGTH: std::ops::Range<usize> = 8..30;

    pub fn new(password: String, password_v2: String) -> Result<Self, PasswordError> {
        if password != password_v2 {
            return Err(PasswordError::PairMismatch);
        }

        if !Self::PASSWORD_LENGTH.contains(&password.len()) {
            return Err(PasswordError::LengthMismatch);
        }

        let hash = util::hash_string(&password).map_err(PasswordError::HashFailed)?;
        Ok(Self(hash))
    }

    pub fn verify(&self, password: &str) -> bool {
        util::verify_hash(password, &self.0).is_some()
    }
}

#[derive(thiserror::Error, Debug)]
pub enum PasswordError {
    #[error("password pair mismatch")]
    PairMismatch,
    #[error("password length mismatch")]
    LengthMismatch,
    #[error("failed to generate password hash: {0}")]
    HashFailed(pbkdf2::password_hash::Error),
}
