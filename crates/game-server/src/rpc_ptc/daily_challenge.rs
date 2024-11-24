use super::*;

pub async fn on_rpc_get_daily_challenge_info_arg(
    _: &RpcPtcContext,
    _: &mut PlayerSession,
    _: RpcGetDailyChallengeInfoArg,
) -> Result<RpcGetDailyChallengeInfoRet, i32> {
    Ok(RpcGetDailyChallengeInfoRet {
        retcode: 0,
        info: DailyChallengeInfo {
            ..Default::default()
        },
    })
}
