use std::{
    collections::HashMap,
    sync::{LazyLock, OnceLock},
    time::Duration,
};

use anyhow::Result;
use common::config::TomlConfig;
use common::config::{AppConfig, EncryptionConfMap, ServerList};
use serde::Deserialize;
use yanagi_http_client::AutopatchClient;

mod data;
mod http_handlers;

#[derive(Deserialize)]
struct DispatchConfig {
    pub http_addr: String,
    pub outer_http_url: String,
    pub design_data_url: String,
}

impl TomlConfig for DispatchConfig {
    const DEFAULT_TOML: &str = include_str!("../dispatch.toml");
}

struct AppState {
    pub dispatch_config: &'static DispatchConfig,
    pub app_config: OnceLock<AppConfig>,
    pub server_list: OnceLock<HashMap<String, ServerList>>,
    pub encryption_config: OnceLock<HashMap<String, EncryptionConfMap>>,
}

#[tokio::main]
async fn main() -> Result<()> {
    static CONFIG: LazyLock<DispatchConfig> =
        LazyLock::new(|| DispatchConfig::load_or_create("dispatch.toml"));
    static STATE: OnceLock<AppState> = OnceLock::new();

    common::print_splash();
    common::logging::init(tracing::Level::DEBUG);

    let state = STATE.get_or_init(|| AppState {
        dispatch_config: &CONFIG,
        app_config: OnceLock::new(),
        server_list: OnceLock::new(),
        encryption_config: OnceLock::new(),
    });

    std::thread::spawn(move || fetch_configuration(state));

    let router = http_handlers::routes().with_state(state);
    axum_server::bind(CONFIG.http_addr.parse()?)
        .serve(router.into_make_service())
        .await?;

    Ok(())
}

fn fetch_configuration(state: &AppState) {
    const RETRY_TIME: Duration = Duration::from_secs(5);

    let design_data_url = &state.dispatch_config.design_data_url;
    let client = AutopatchClient::new(design_data_url).retry_after(RETRY_TIME);

    let app_config: AppConfig = client.fetch_until_success("/config.json");

    let mut server_list_map = HashMap::with_capacity(app_config.version_info_groups.len());
    let mut encryption_conf_map = HashMap::with_capacity(app_config.version_info_groups.len());
    for (version, info) in app_config.version_info_groups.iter() {
        let server_list = client.fetch_until_success(&info.server_list_url);
        let encryption_conf = client.fetch_until_success(&info.encryption_config_url);

        server_list_map.insert(version.clone(), server_list);
        encryption_conf_map.insert(version.clone(), encryption_conf);
    }

    tracing::info!("successfully fetched all remote configuration from autopatch!");

    let _ = state.app_config.set(app_config);
    let _ = state.server_list.set(server_list_map);
    let _ = state.encryption_config.set(encryption_conf_map);
}
