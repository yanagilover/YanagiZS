use super::*;

pub async fn on_rpc_get_abyss_info_arg(
    _: &RpcPtcContext,
    _: &mut PlayerSession,
    _: RpcGetAbyssInfoArg,
) -> Result<RpcGetAbyssInfoRet, i32> {
    Ok(RpcGetAbyssInfoRet {
        retcode: 0,
        abyss_info: AbyssInfo::default(),
    })
}

pub async fn on_rpc_get_abyss_arpeggio_data_arg(
    _: &RpcPtcContext,
    _: &mut PlayerSession,
    _: RpcGetAbyssArpeggioDataArg,
) -> Result<RpcGetAbyssArpeggioDataRet, i32> {
    Ok(RpcGetAbyssArpeggioDataRet::default())
}

pub async fn on_rpc_get_abyss_reward_data_arg(
    _: &RpcPtcContext,
    _: &mut PlayerSession,
    _: RpcGetAbyssRewardDataArg,
) -> Result<RpcGetAbyssRewardDataRet, i32> {
    Ok(RpcGetAbyssRewardDataRet {
        retcode: 0,
        abyss_reward_data: AbyssRewardData::default(),
    })
}
