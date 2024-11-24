use super::from_hex;
use std::collections::HashMap;

use serde::Deserialize;

pub type EncryptionConfMap = HashMap<u32, EncryptionConfig>;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct EncryptionConfig {
    #[serde(deserialize_with = "from_hex")]
    pub server_private_key: Box<[u8]>,
    #[serde(deserialize_with = "from_hex")]
    pub client_public_key: Box<[u8]>,
}
