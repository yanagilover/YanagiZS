use std::sync::{LazyLock, OnceLock};

use anyhow::Result;
use common::config::{DatabaseSettings, TomlConfig};
use database::DbContext;
use remote_config::RemoteConfiguration;
use serde::Deserialize;
use tracing::Level;
use udp_server::UdpServer;

mod database;
mod net;
mod packet;
mod remote_config;
mod session;
mod udp_server;

#[derive(Deserialize)]
pub struct GateServerConfig {
    pub server_name: String,
    pub bind_client_version: String,
    pub design_data_url: String,
    pub database: DatabaseSettings,
}

struct AppState {
    remote_config: RemoteConfiguration,
    db_context: DbContext,
}

impl TomlConfig for GateServerConfig {
    const DEFAULT_TOML: &str = include_str!("../gateserver.toml");
}

#[tokio::main]
async fn main() -> Result<()> {
    static CONFIG: LazyLock<GateServerConfig> =
        LazyLock::new(|| GateServerConfig::load_or_create("gateserver.toml"));
    static STATE: OnceLock<AppState> = OnceLock::new();

    common::print_splash();
    common::logging::init(Level::DEBUG);
    let config = remote_config::download(&CONFIG);
    let gateway_port = config.port;

    let db_context = DbContext::connect(&CONFIG.database).await?;
    let state = STATE.get_or_init(|| AppState {
        remote_config: config,
        db_context,
    });

    let server = UdpServer::new(&format!("0.0.0.0:{gateway_port}"), state)?;
    tokio::task::spawn_blocking(|| server.serve()).await?;

    Ok(())
}
