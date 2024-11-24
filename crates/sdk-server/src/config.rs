use common::config::{DatabaseSettings, TomlConfig};
use serde::Deserialize;

#[derive(Deserialize)]
pub struct SdkConfig {
    pub http_addr: String,
    pub database: DatabaseSettings,
}

impl TomlConfig for SdkConfig {
    const DEFAULT_TOML: &str = include_str!("../sdk_server.toml");
}
