use super::*;

pub async fn on_rpc_get_embattles_data_arg(
    _: &RpcPtcContext,
    _: &mut PlayerSession,
    _: RpcGetEmbattlesDataArg,
) -> Result<RpcGetEmbattlesDataRet, i32> {
    Ok(RpcGetEmbattlesDataRet {
        retcode: 0,
        embattles_data: EmbattlesData::default(),
    })
}

pub async fn on_rpc_report_embattle_info_arg(
    _: &RpcPtcContext,
    _: &mut PlayerSession,
    _: RpcReportEmbattleInfoArg,
) -> Result<RpcReportEmbattleInfoRet, i32> {
    Ok(RpcReportEmbattleInfoRet::default())
}
