use super::*;

pub async fn on_rpc_get_gacha_data_arg(
    _: &RpcPtcContext,
    _: &mut PlayerSession,
    arg: RpcGetGachaDataArg,
) -> Result<RpcGetGachaDataRet, i32> {
    Ok(RpcGetGachaDataRet {
        gacha_type: arg.gacha_type,
        ..Default::default()
    })
}
