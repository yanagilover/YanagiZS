use super::*;

pub async fn on_rpc_get_camp_idle_data_arg(
    _: &RpcPtcContext,
    _: &mut PlayerSession,
    _: RpcGetCampIdleDataArg,
) -> Result<RpcGetCampIdleDataRet, i32> {
    Ok(RpcGetCampIdleDataRet {
        retcode: 0,
        camp_idle_data: CampIdleData::default(),
    })
}
