use super::*;

pub async fn on_rpc_get_tips_info_arg(
    _: &RpcPtcContext,
    _: &mut PlayerSession,
    _: RpcGetTipsInfoArg,
) -> Result<RpcGetTipsInfoRet, i32> {
    Ok(RpcGetTipsInfoRet {
        retcode: 0,
        tips_info: TipsInfo::default(),
        ..Default::default()
    })
}

pub async fn on_rpc_get_client_systems_info_arg(
    _: &RpcPtcContext,
    _: &mut PlayerSession,
    _: RpcGetClientSystemsInfoArg,
) -> Result<RpcGetClientSystemsInfoRet, i32> {
    Ok(RpcGetClientSystemsInfoRet {
        retcode: 0,
        info: ClientSystemsInfo::default(),
    })
}

pub async fn on_rpc_get_private_message_data_arg(
    _: &RpcPtcContext,
    _: &mut PlayerSession,
    _: RpcGetPrivateMessageDataArg,
) -> Result<RpcGetPrivateMessageDataRet, i32> {
    Ok(RpcGetPrivateMessageDataRet {
        retcode: 0,
        private_message_data: PrivateMessageData::default(),
    })
}

pub async fn on_rpc_get_collect_map_arg(
    _: &RpcPtcContext,
    _: &mut PlayerSession,
    _: RpcGetCollectMapArg,
) -> Result<RpcGetCollectMapRet, i32> {
    Ok(RpcGetCollectMapRet {
        retcode: 0,
        collect_map: CollectMap::default(),
    })
}

pub async fn on_rpc_get_workbench_info_arg(
    _: &RpcPtcContext,
    _: &mut PlayerSession,
    _: RpcGetWorkbenchInfoArg,
) -> Result<RpcGetWorkbenchInfoRet, i32> {
    Ok(RpcGetWorkbenchInfoRet {
        retcode: 0,
        workbench_info: WorkbenchInfo::default(),
    })
}

pub async fn on_rpc_report_ui_layout_platform_arg(
    _: &RpcPtcContext,
    _: &mut PlayerSession,
    _: RpcReportUiLayoutPlatformArg,
) -> Result<RpcReportUiLayoutPlatformRet, i32> {
    Ok(RpcReportUiLayoutPlatformRet::default())
}
