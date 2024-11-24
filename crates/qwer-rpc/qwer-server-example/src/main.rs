use std::{
    sync::{Arc, OnceLock},
    time::Duration,
};

use anyhow::Result;
use protocol::{
    PlayerBasicInfo, RpcGetPlayerBasicInfoArg, RpcGetPlayerBasicInfoRet, RpcPlayerLoginArg,
    RpcPlayerLoginRet,
};
use qwer::ProtocolID;
use qwer_rpc::{ProtocolServiceFrontend, RpcPtcContext, RpcPtcPoint, RpcPtcServiceFrontend};
use tokio::time;

static SERVICE: OnceLock<RpcPtcServiceFrontend> = OnceLock::new();
static SERVER_POINT: OnceLock<Arc<RpcPtcPoint>> = OnceLock::new();

#[tokio::main]
async fn main() -> Result<()> {
    common::logging::init(tracing::Level::DEBUG);
    println!("Hello, world!");

    let service =
        SERVICE.get_or_init(|| RpcPtcServiceFrontend::new(ProtocolServiceFrontend::new()));

    let point = service.create_point(Some("0.0.0.0:10101".parse()?)).await?;
    let point = SERVER_POINT.get_or_init(|| point);

    point.register_rpc_recv(RpcPlayerLoginArg::PROTOCOL_ID, handle_login);
    point.register_rpc_recv(RpcGetPlayerBasicInfoArg::PROTOCOL_ID, handle_get_basic_info);

    time::sleep(Duration::from_secs(100000)).await;

    Ok(())
}

async fn handle_login(ctx: RpcPtcContext) {
    let arg = ctx.get_arg::<RpcPlayerLoginArg>().unwrap();
    println!("Login: {arg:?}");

    ctx.send_ret(RpcPlayerLoginRet { retcode: 0 }).await;
}

async fn handle_get_basic_info(ctx: RpcPtcContext) {
    println!("BasicInfo req");

    ctx.send_ret(RpcGetPlayerBasicInfoRet {
        basic_info: PlayerBasicInfo {
            nick_name: String::from("Name"),
            ..Default::default()
        },
        retcode: 1337,
    })
    .await;
}
