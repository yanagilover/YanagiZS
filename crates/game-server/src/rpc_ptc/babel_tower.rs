use super::*;

pub async fn on_rpc_get_babel_tower_data_arg(
    _: &RpcPtcContext,
    _: &mut PlayerSession,
    _: RpcGetBabelTowerDataArg,
) -> Result<RpcGetBabelTowerDataRet, i32> {
    Ok(RpcGetBabelTowerDataRet::default())
}
