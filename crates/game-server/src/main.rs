use std::{
    sync::{LazyLock, OnceLock},
    time::Duration,
};

use anyhow::Result;
use common::{
    config::{DatabaseSettings, TomlConfig},
    time_util,
};
use dashmap::DashMap;
use database::DbContext;
use level::LevelEventGraphManager;
use player_info::PlayerInfo;
use player_util::UidCounter;
use qwer::ProtocolID;
use qwer_rpc::{
    middleware::MiddlewareModel, ProtocolServiceFrontend, RpcPtcContext, RpcPtcServiceFrontend,
};

use protocol::*;
use serde::Deserialize;
use tracing::{error, info, warn};
use yanagi_data::{ArchiveFile, NapFileCfg};

mod database;
mod level;
mod player_util;
mod remote_config;
mod rpc_ptc;
mod scene_section_util;

#[derive(Deserialize)]
pub struct GameServerConfig {
    pub server_name: String,
    pub bind_client_version: String,
    pub design_data_url: String,
    pub database: DatabaseSettings,
}

impl TomlConfig for GameServerConfig {
    const DEFAULT_TOML: &str = include_str!("../gameserver.toml");
}

struct PlayerSession {
    pub player_uid: u64,
    pub uid_counter: UidCounter,
    pub player_info: PlayerInfo,
    pub last_save_time: u64,
    pub level_event_graph_mgr: LevelEventGraphManager,
}

static FILECFG: OnceLock<NapFileCfg> = OnceLock::new();

static PLAYER_MAP: LazyLock<DashMap<u64, PlayerSession>> = LazyLock::new(|| DashMap::new());
static DB_CONTEXT: OnceLock<DbContext> = OnceLock::new();

#[tokio::main]
async fn main() -> Result<()> {
    static CONFIG: LazyLock<GameServerConfig> =
        LazyLock::new(|| GameServerConfig::load_or_create("gameserver.toml"));
    static DESIGN_DATA: OnceLock<ArchiveFile> = OnceLock::new();

    common::print_splash();
    common::logging::init(tracing::Level::DEBUG);
    let remote_cfg = remote_config::download(&CONFIG);
    let design_data_blk = remote_config::download_design_data_blk(&remote_cfg.version_info);
    let main_city_script =
        remote_config::download_main_city_script_config(&remote_cfg.version_info);

    let _ = DESIGN_DATA.set(yanagi_data::read_archive_file(std::io::Cursor::new(
        &design_data_blk,
    ))?);

    level::load_script_config(&main_city_script);

    let nap_cfg = NapFileCfg::new(&DESIGN_DATA.get().unwrap());
    FILECFG.get_or_init(|| nap_cfg);

    let db_context = DbContext::connect(&CONFIG.database).await?;
    DB_CONTEXT.get_or_init(|| db_context);

    let service = RpcPtcServiceFrontend::new(ProtocolServiceFrontend::new());
    let listen_point = service.create_point(Some("0.0.0.0:10101".parse()?)).await?;

    listen_point.register_rpc_recv(RpcPlayerLoginArg::PROTOCOL_ID, on_rpc_player_login_arg);
    rpc_ptc::register_handlers(&listen_point);

    // sleep, service stuff is running in separate task
    tokio::time::sleep(Duration::from_secs(u64::MAX)).await;
    Ok(())
}

pub async fn on_rpc_player_login_arg(ctx: RpcPtcContext) {
    let _arg: RpcPlayerLoginArg = ctx.get_arg().unwrap();

    let Some(MiddlewareModel::Account(account_mw)) = ctx
        .middleware_list
        .iter()
        .find(|&mw| matches!(mw, MiddlewareModel::Account(_)))
    else {
        warn!("login failed: account middleware is missing");
        return;
    };

    let Ok((uid_counter, player_info)) = DB_CONTEXT
        .get()
        .unwrap()
        .get_or_create_player_data(account_mw.player_uid)
        .await
        .inspect_err(|err| error!("login failed: get_or_create_player_data failed: {err}"))
    else {
        ctx.send_ret(RpcPlayerLoginRet { retcode: 1 }).await;
        return;
    };

    PLAYER_MAP.insert(
        account_mw.player_uid,
        PlayerSession {
            player_uid: account_mw.player_uid,
            uid_counter,
            player_info,
            level_event_graph_mgr: LevelEventGraphManager::default(),
            last_save_time: time_util::unix_timestamp(),
        },
    );

    info!("player with uid {} is logging in!", account_mw.player_uid);
    ctx.send_ret(RpcPlayerLoginRet { retcode: 0 }).await;
}

async fn post_rpc_handle(session: &mut PlayerSession) {
    let timestamp = time_util::unix_timestamp();

    if (timestamp - session.last_save_time) >= 30 {
        session.last_save_time = timestamp;
        DB_CONTEXT
            .get()
            .unwrap()
            .save_player_data(session.uid_counter.last_uid(), &session.player_info)
            .await
            .expect("failed to save player data");

        info!(
            "successfully saved player data (uid: {})",
            session.player_uid
        );
    }
}
