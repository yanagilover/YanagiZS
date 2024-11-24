use super::*;

pub async fn on_rpc_get_activity_data_arg(
    _: &RpcPtcContext,
    _: &mut PlayerSession,
    _: RpcGetActivityDataArg,
) -> Result<RpcGetActivityDataRet, i32> {
    Ok(RpcGetActivityDataRet {
        retcode: 0,
        ..Default::default()
    })
}

pub async fn on_rpc_get_web_activity_data_arg(
    _: &RpcPtcContext,
    _: &mut PlayerSession,
    _: RpcGetWebActivityDataArg,
) -> Result<RpcGetWebActivityDataRet, i32> {
    Ok(RpcGetWebActivityDataRet::default())
}
