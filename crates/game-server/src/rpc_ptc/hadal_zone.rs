use super::*;

pub async fn on_rpc_get_hadal_zone_data_arg(
    _: &RpcPtcContext,
    _: &mut PlayerSession,
    _: RpcGetHadalZoneDataArg,
) -> Result<RpcGetHadalZoneDataRet, i32> {
    Ok(RpcGetHadalZoneDataRet::default())
}
