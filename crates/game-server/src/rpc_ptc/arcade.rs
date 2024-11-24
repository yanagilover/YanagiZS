use super::*;

pub async fn on_rpc_get_arcade_data_arg(
    _: &RpcPtcContext,
    _: &mut PlayerSession,
    _: RpcGetArcadeDataArg,
) -> Result<RpcGetArcadeDataRet, i32> {
    Ok(RpcGetArcadeDataRet::default())
}
