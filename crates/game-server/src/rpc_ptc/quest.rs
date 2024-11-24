use super::*;

pub async fn on_rpc_get_quest_data_arg(
    _: &RpcPtcContext,
    _: &mut PlayerSession,
    arg: RpcGetQuestDataArg,
) -> Result<RpcGetQuestDataRet, i32> {
    Ok(RpcGetQuestDataRet {
        retcode: 0,
        quest_type: arg.quest_type,
        quest_data: QuestData::default(),
    })
}

pub async fn on_rpc_get_archive_info_arg(
    _: &RpcPtcContext,
    session: &mut PlayerSession,
    _: RpcGetArchiveInfoArg,
) -> Result<RpcGetArchiveInfoRet, i32> {
    let archive_info = session.player_info.archive_info.as_ref().unwrap();

    Ok(RpcGetArchiveInfoRet {
        retcode: 0,
        archive_info: ArchiveInfo {
            hollow_archive_id_list: archive_info
                .hollow_archive_id
                .as_ref()
                .map(|set| set.iter().map(|id| *id as u32).collect())
                .unwrap_or_default(),
            videotaps_info: archive_info
                .videotaps_info
                .as_ref()
                .unwrap()
                .iter()
                .map(|(id, videotape)| VideotapeInfo {
                    archive_file_id: *id as u32,
                    finished: videotape.finished,
                })
                .collect(),
        },
    })
}

pub async fn on_rpc_get_yorozuya_info_arg(
    _: &RpcPtcContext,
    session: &mut PlayerSession,
    _: RpcGetYorozuyaInfoArg,
) -> Result<RpcGetYorozuyaInfoRet, i32> {
    let yorozuya_info = session.player_info.yorozuya_info.as_ref().unwrap();

    Ok(RpcGetYorozuyaInfoRet {
        retcode: 0,
        yorozuya_info: YorozuyaInfo {
            unlock_hollow_id_list: yorozuya_info
                .unlock_hollow_id
                .as_ref()
                .map(|list| list.iter().map(|id| *id as u32).collect())
                .unwrap_or_default(),
        },
    })
}

pub async fn on_rpc_get_fairy_info_arg(
    _: &RpcPtcContext,
    _: &mut PlayerSession,
    _: RpcGetFairyInfoArg,
) -> Result<RpcGetFairyInfoRet, i32> {
    Ok(RpcGetFairyInfoRet {
        retcode: 0,
        info: FairyInfo::default(),
    })
}

pub async fn on_rpc_check_yorozuya_info_refresh_arg(
    _: &RpcPtcContext,
    _: &mut PlayerSession,
    _: RpcCheckYorozuyaInfoRefreshArg,
) -> Result<RpcCheckYorozuyaInfoRefreshRet, i32> {
    Ok(RpcCheckYorozuyaInfoRefreshRet::default())
}
