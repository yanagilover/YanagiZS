use std::collections::HashMap;

use serde::Deserialize;

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct AppConfig {
    pub version_info_groups: HashMap<String, ConfigurationInfo>,
}

#[derive(Deserialize)]
#[serde(rename_all = "PascalCase")]
pub struct ConfigurationInfo {
    pub seed: String,
    pub server_list_url: String,
    pub platform: String,
    pub environment: String,
    pub encryption_config_url: String,
    pub design_data_url: String,
    pub cdn_check_url: String,
}
