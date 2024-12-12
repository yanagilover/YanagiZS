use common::time_util;

use super::*;

pub async fn on_rpc_get_player_basic_info_arg(
    _ctx: &RpcPtcContext,
    session: &mut PlayerSession,
    _arg: RpcGetPlayerBasicInfoArg,
) -> Result<RpcGetPlayerBasicInfoRet, i32> {
    Ok(RpcGetPlayerBasicInfoRet {
        retcode: 0,
        basic_info: protocol::util::build_player_basic_info(&session.player_info),
    })
}

pub async fn on_rpc_get_server_timestamp_arg(
    _: &RpcPtcContext,
    _: &mut PlayerSession,
    _: RpcGetServerTimestampArg,
) -> Result<RpcGetServerTimestampRet, i32> {
    Ok(RpcGetServerTimestampRet {
        retcode: 0,
        utc_offset: 3,
        timestamp: time_util::unix_timestamp(),
    })
}

pub async fn on_rpc_get_video_usm_key_data_arg(
    _: &RpcPtcContext,
    _: &mut PlayerSession,
    _: RpcGetVideoUsmKeyDataArg,
) -> Result<RpcGetVideoUsmKeyDataRet, i32> {
    Ok(RpcGetVideoUsmKeyDataRet { retcode: 0 })
}

pub async fn on_rpc_get_authkey_arg(
    _: &RpcPtcContext,
    _: &mut PlayerSession,
    _: RpcGetAuthkeyArg,
) -> Result<RpcGetAuthkeyRet, i32> {
    Ok(RpcGetAuthkeyRet::default())
}

pub async fn on_rpc_save_player_system_setting_arg(
    _: &RpcPtcContext,
    _: &mut PlayerSession,
    arg: RpcSavePlayerSystemSettingArg,
) -> Result<RpcSavePlayerSystemSettingRet, i32> {
    tracing::info!("save_player_system_setting: {arg:?}");

    Ok(RpcSavePlayerSystemSettingRet { retcode: 0 })
}

pub async fn on_rpc_get_player_mails_arg(
    _: &RpcPtcContext,
    _: &mut PlayerSession,
    _: RpcGetPlayerMailsArg,
) -> Result<RpcGetPlayerMailsRet, i32> {
    Ok(RpcGetPlayerMailsRet::default())
}

pub async fn on_rpc_get_role_card_data_arg(
    _: &RpcPtcContext,
    _: &mut PlayerSession,
    _: RpcGetRoleCardDataArg,
) -> Result<RpcGetRoleCardDataRet, i32> {
    Ok(RpcGetRoleCardDataRet {
        retcode: 0,
        role_card_data: RoleCardData::default(),
    })
}

pub async fn on_rpc_get_month_card_reward_list_arg(
    _: &RpcPtcContext,
    _: &mut PlayerSession,
    _: RpcGetMonthCardRewardListArg,
) -> Result<RpcGetMonthCardRewardListRet, i32> {
    Ok(RpcGetMonthCardRewardListRet::default())
}

pub async fn on_rpc_get_display_case_data_arg(
    _: &RpcPtcContext,
    _: &mut PlayerSession,
    _: RpcGetDisplayCaseDataArg,
) -> Result<RpcGetDisplayCaseDataRet, i32> {
    Ok(RpcGetDisplayCaseDataRet::default())
}

pub async fn on_rpc_player_operation_arg(
    _: &RpcPtcContext,
    _: &mut PlayerSession,
    _: RpcPlayerOperationArg,
) -> Result<RpcPlayerOperationRet, i32> {
    Ok(RpcPlayerOperationRet::default())
}

pub async fn on_rpc_player_transaction_arg(
    _: &RpcPtcContext,
    session: &mut PlayerSession,
    _: RpcPlayerTransactionArg,
) -> Result<RpcPlayerTransactionRet, i32> {
    let player_uid = session.player_info.uid();
    let scene_uid = session.player_info.scene_uid();

    Ok(RpcPlayerTransactionRet {
        retcode: 0,
        transaction: format!("{player_uid}-{scene_uid}"),
    })
}

pub async fn on_rpc_get_player_network_data_arg(
    _: &RpcPtcContext,
    _: &mut PlayerSession,
    _: RpcGetPlayerNetworkDataArg,
) -> Result<RpcGetPlayerNetworkDataRet, i32> {
    Ok(RpcGetPlayerNetworkDataRet {
        retcode: 0,
        player_network_data: Some(PlayerNetworkData::default()),
    })
}
