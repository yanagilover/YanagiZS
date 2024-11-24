use super::*;

pub async fn on_rpc_get_vhs_store_info_arg(
    _: &RpcPtcContext,
    _: &mut PlayerSession,
    _: RpcGetVhsStoreInfoArg,
) -> Result<RpcGetVhsStoreInfoRet, i32> {
    Ok(RpcGetVhsStoreInfoRet {
        retcode: 0,
        info: VhsStoreInfo::default(),
    })
}
