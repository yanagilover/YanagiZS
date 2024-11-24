use std::time::Duration;

use common::config::*;
use yanagi_encryption::xor::MhyXorpad;
use yanagi_http_client::AutopatchClient;

use crate::GateServerConfig;

pub struct RemoteConfiguration {
    pub xorpad: MhyXorpad,
    pub encryption_conf: EncryptionConfig,
    pub port: u16,
}

pub fn download(config: &'static GateServerConfig) -> RemoteConfiguration {
    const RETRY_TIME: Duration = Duration::from_secs(5);

    let client = AutopatchClient::new(&config.design_data_url).retry_after(RETRY_TIME);
    let app_config: AppConfig = client.fetch_until_success("/config.json");
    let version_info = app_config
        .version_info_groups
        .get(&config.bind_client_version)
        .expect(
            "Fatal: remote config doesn't contain configuration for specified bind_client_version",
        );

    let server_list: ServerList = client.fetch_until_success(&version_info.server_list_url);

    let server_info = server_list
        .into_iter()
        .find(|info| info.notice_region == config.server_name)
        .expect("Fatal: remote config doesn't contain configuration with specified server_name");

    let mut encryption_conf: EncryptionConfMap =
        client.fetch_until_success(&version_info.encryption_config_url);

    let Some(encryption_conf) = encryption_conf.remove(&server_info.rsa_ver) else {
        panic!(
            "Fatal: remote config doesn't contain encryption config with rsa_ver: {}",
            server_info.rsa_ver,
        )
    };

    let xorpad = MhyXorpad::from_ec2b(&server_info.client_secret_key).unwrap();

    RemoteConfiguration {
        xorpad,
        encryption_conf,
        port: server_info.port,
    }
}
