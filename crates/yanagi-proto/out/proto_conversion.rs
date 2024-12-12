#[allow(unused)]
impl From<::protocol::RpcGetAbyssRewardDataArg> for GetAbyssRewardDataCsReq {
    fn from(value: ::protocol::RpcGetAbyssRewardDataArg) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<GetAbyssRewardDataCsReq> for ::protocol::RpcGetAbyssRewardDataArg {
    fn from(value: GetAbyssRewardDataCsReq) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::CampIdleData> for CampIdleData {
    fn from(value: ::protocol::CampIdleData) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<CampIdleData> for ::protocol::CampIdleData {
    fn from(value: CampIdleData) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetExplorationDataArg> for GetExplorationDataCsReq {
    fn from(value: ::protocol::RpcGetExplorationDataArg) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<GetExplorationDataCsReq> for ::protocol::RpcGetExplorationDataArg {
    fn from(value: GetExplorationDataCsReq) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::BattleRewardInfo> for BattleRewardInfo {
    fn from(value: ::protocol::BattleRewardInfo) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<BattleRewardInfo> for ::protocol::BattleRewardInfo {
    fn from(value: BattleRewardInfo) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::RpcBeginTrainingCourseBattleArg>
for BeginTrainingCourseBattleCsReq {
    fn from(value: ::protocol::RpcBeginTrainingCourseBattleArg) -> Self {
        Self {
            quest_id: value.quest_id.into(),
            avatars: value.avatars.into_iter().map(|v| v.into()).collect(),
            buddy_id: value.buddy_id.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<BeginTrainingCourseBattleCsReq>
for ::protocol::RpcBeginTrainingCourseBattleArg {
    fn from(value: BeginTrainingCourseBattleCsReq) -> Self {
        Self {
            quest_id: value.quest_id.into(),
            avatars: value.avatars.into_iter().map(|v| v.into()).collect(),
            buddy_id: value.buddy_id.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::RpcSavePosInMainCityArg> for SavePosInMainCityCsReq {
    fn from(value: ::protocol::RpcSavePosInMainCityArg) -> Self {
        Self {
            position: Some(value.position.into()),
            section_id: value.section_id.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<SavePosInMainCityCsReq> for ::protocol::RpcSavePosInMainCityArg {
    fn from(value: SavePosInMainCityCsReq) -> Self {
        Self {
            position: value.position.map(|v| v.into()).unwrap_or_default(),
            section_id: value.section_id.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::EquipWishlistPlan> for EquipWishlistPlan {
    fn from(value: ::protocol::EquipWishlistPlan) -> Self {
        Self {
            plan_type: value.plan_type.into(),
            wish_weapon_id_list: value
                .wish_weapon_id_list
                .into_iter()
                .map(|v| v.into())
                .collect(),
            wish_equip_id_list: value
                .wish_equip_id_list
                .into_iter()
                .map(|v| v.into())
                .collect(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<EquipWishlistPlan> for ::protocol::EquipWishlistPlan {
    fn from(value: EquipWishlistPlan) -> Self {
        Self {
            plan_type: value.plan_type.into(),
            wish_weapon_id_list: value
                .wish_weapon_id_list
                .into_iter()
                .map(|v| v.into())
                .collect(),
            wish_equip_id_list: value
                .wish_equip_id_list
                .into_iter()
                .map(|v| v.into())
                .collect(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetVhsStoreInfoArg> for GetVhsStoreInfoCsReq {
    fn from(value: ::protocol::RpcGetVhsStoreInfoArg) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<GetVhsStoreInfoCsReq> for ::protocol::RpcGetVhsStoreInfoArg {
    fn from(value: GetVhsStoreInfoCsReq) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetVideoUsmKeyDataRet> for GetVideoUsmKeyDataScRsp {
    fn from(value: ::protocol::RpcGetVideoUsmKeyDataRet) -> Self {
        Self {
            retcode: value.retcode.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<GetVideoUsmKeyDataScRsp> for ::protocol::RpcGetVideoUsmKeyDataRet {
    fn from(value: GetVideoUsmKeyDataScRsp) -> Self {
        Self {
            retcode: value.retcode.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetPlayerMailsRet> for GetPlayerMailsScRsp {
    fn from(value: ::protocol::RpcGetPlayerMailsRet) -> Self {
        Self {
            retcode: value.retcode.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<GetPlayerMailsScRsp> for ::protocol::RpcGetPlayerMailsRet {
    fn from(value: GetPlayerMailsScRsp) -> Self {
        Self {
            retcode: value.retcode.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetAbyssInfoArg> for GetAbyssInfoCsReq {
    fn from(value: ::protocol::RpcGetAbyssInfoArg) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<GetAbyssInfoCsReq> for ::protocol::RpcGetAbyssInfoArg {
    fn from(value: GetAbyssInfoCsReq) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetPrivateMessageDataArg> for GetPrivateMessageDataCsReq {
    fn from(value: ::protocol::RpcGetPrivateMessageDataArg) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<GetPrivateMessageDataCsReq> for ::protocol::RpcGetPrivateMessageDataArg {
    fn from(value: GetPrivateMessageDataCsReq) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::NewsStandData> for NewsStandData {
    fn from(value: ::protocol::NewsStandData) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<NewsStandData> for ::protocol::NewsStandData {
    fn from(value: NewsStandData) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetNewsStandDataArg> for GetNewsStandDataCsReq {
    fn from(value: ::protocol::RpcGetNewsStandDataArg) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<GetNewsStandDataCsReq> for ::protocol::RpcGetNewsStandDataArg {
    fn from(value: GetNewsStandDataCsReq) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetVideoUsmKeyDataArg> for GetVideoUsmKeyDataCsReq {
    fn from(value: ::protocol::RpcGetVideoUsmKeyDataArg) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<GetVideoUsmKeyDataCsReq> for ::protocol::RpcGetVideoUsmKeyDataArg {
    fn from(value: GetVideoUsmKeyDataCsReq) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetPlayerBasicInfoArg> for GetPlayerBasicInfoCsReq {
    fn from(value: ::protocol::RpcGetPlayerBasicInfoArg) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<GetPlayerBasicInfoCsReq> for ::protocol::RpcGetPlayerBasicInfoArg {
    fn from(value: GetPlayerBasicInfoCsReq) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetServerTimestampRet> for GetServerTimestampScRsp {
    fn from(value: ::protocol::RpcGetServerTimestampRet) -> Self {
        Self {
            retcode: value.retcode.into(),
            timestamp: value.timestamp.into(),
            utc_offset: value.utc_offset.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<GetServerTimestampScRsp> for ::protocol::RpcGetServerTimestampRet {
    fn from(value: GetServerTimestampScRsp) -> Self {
        Self {
            retcode: value.retcode.into(),
            timestamp: value.timestamp.into(),
            utc_offset: value.utc_offset.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetAuthkeyArg> for GetAuthkeyCsReq {
    fn from(value: ::protocol::RpcGetAuthkeyArg) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<GetAuthkeyCsReq> for ::protocol::RpcGetAuthkeyArg {
    fn from(value: GetAuthkeyCsReq) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetWishlistDataRet> for GetWishlistDataScRsp {
    fn from(value: ::protocol::RpcGetWishlistDataRet) -> Self {
        Self {
            retcode: value.retcode.into(),
            wishlist_plan_list: value
                .wishlist_plan_list
                .into_iter()
                .map(|v| v.into())
                .collect(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<GetWishlistDataScRsp> for ::protocol::RpcGetWishlistDataRet {
    fn from(value: GetWishlistDataScRsp) -> Self {
        Self {
            retcode: value.retcode.into(),
            wishlist_plan_list: value
                .wishlist_plan_list
                .into_iter()
                .map(|v| v.into())
                .collect(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetRamenDataArg> for GetRamenDataCsReq {
    fn from(value: ::protocol::RpcGetRamenDataArg) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<GetRamenDataCsReq> for ::protocol::RpcGetRamenDataArg {
    fn from(value: GetRamenDataCsReq) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetQuestDataRet> for GetQuestDataScRsp {
    fn from(value: ::protocol::RpcGetQuestDataRet) -> Self {
        Self {
            quest_type: value.quest_type.into(),
            quest_data: Some(value.quest_data.into()),
            retcode: value.retcode.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<GetQuestDataScRsp> for ::protocol::RpcGetQuestDataRet {
    fn from(value: GetQuestDataScRsp) -> Self {
        Self {
            quest_type: value.quest_type.into(),
            quest_data: value.quest_data.map(|v| v.into()).unwrap_or_default(),
            retcode: value.retcode.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetTrashbinHermitDataRet> for GetTrashbinHermitDataScRsp {
    fn from(value: ::protocol::RpcGetTrashbinHermitDataRet) -> Self {
        Self {
            trashbin_hermit_data: Some(value.trashbin_hermit_data.into()),
            retcode: value.retcode.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<GetTrashbinHermitDataScRsp> for ::protocol::RpcGetTrashbinHermitDataRet {
    fn from(value: GetTrashbinHermitDataScRsp) -> Self {
        Self {
            trashbin_hermit_data: value
                .trashbin_hermit_data
                .map(|v| v.into())
                .unwrap_or_default(),
            retcode: value.retcode.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetFairyInfoArg> for GetFairyInfoCsReq {
    fn from(value: ::protocol::RpcGetFairyInfoArg) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<GetFairyInfoCsReq> for ::protocol::RpcGetFairyInfoArg {
    fn from(value: GetFairyInfoCsReq) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetCampIdleDataRet> for GetCampIdleDataScRsp {
    fn from(value: ::protocol::RpcGetCampIdleDataRet) -> Self {
        Self {
            retcode: value.retcode.into(),
            camp_idle_data: Some(value.camp_idle_data.into()),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<GetCampIdleDataScRsp> for ::protocol::RpcGetCampIdleDataRet {
    fn from(value: GetCampIdleDataScRsp) -> Self {
        Self {
            retcode: value.retcode.into(),
            camp_idle_data: value.camp_idle_data.map(|v| v.into()).unwrap_or_default(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::RpcInteractWithUnitRet> for InteractWithUnitScRsp {
    fn from(value: ::protocol::RpcInteractWithUnitRet) -> Self {
        Self {
            retcode: value.retcode.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<InteractWithUnitScRsp> for ::protocol::RpcInteractWithUnitRet {
    fn from(value: InteractWithUnitScRsp) -> Self {
        Self {
            retcode: value.retcode.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetCollectMapArg> for GetCollectMapCsReq {
    fn from(value: ::protocol::RpcGetCollectMapArg) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<GetCollectMapCsReq> for ::protocol::RpcGetCollectMapArg {
    fn from(value: GetCollectMapCsReq) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::RpcInteractWithClientEntityArg> for InteractWithClientEntityCsReq {
    fn from(value: ::protocol::RpcInteractWithClientEntityArg) -> Self {
        Self {
            interaction: value.interaction.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<InteractWithClientEntityCsReq> for ::protocol::RpcInteractWithClientEntityArg {
    fn from(value: InteractWithClientEntityCsReq) -> Self {
        Self {
            interaction: value.interaction.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::RpcModMainCityAvatarRet> for ModMainCityAvatarScRsp {
    fn from(value: ::protocol::RpcModMainCityAvatarRet) -> Self {
        Self {
            retcode: value.retcode.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<ModMainCityAvatarScRsp> for ::protocol::RpcModMainCityAvatarRet {
    fn from(value: ModMainCityAvatarScRsp) -> Self {
        Self {
            retcode: value.retcode.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::RamenData> for RamenData {
    fn from(value: ::protocol::RamenData) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<RamenData> for ::protocol::RamenData {
    fn from(value: RamenData) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetShoppingMallInfoArg> for GetShoppingMallInfoCsReq {
    fn from(value: ::protocol::RpcGetShoppingMallInfoArg) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<GetShoppingMallInfoCsReq> for ::protocol::RpcGetShoppingMallInfoArg {
    fn from(value: GetShoppingMallInfoCsReq) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetTipsInfoArg> for GetTipsInfoCsReq {
    fn from(value: ::protocol::RpcGetTipsInfoArg) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<GetTipsInfoCsReq> for ::protocol::RpcGetTipsInfoArg {
    fn from(value: GetTipsInfoCsReq) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::ClientSystemsInfo> for ClientSystemsInfo {
    fn from(value: ::protocol::ClientSystemsInfo) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<ClientSystemsInfo> for ::protocol::ClientSystemsInfo {
    fn from(value: ClientSystemsInfo) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetActivityDataRet> for GetActivityDataScRsp {
    fn from(value: ::protocol::RpcGetActivityDataRet) -> Self {
        Self {
            retcode: value.retcode.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<GetActivityDataScRsp> for ::protocol::RpcGetActivityDataRet {
    fn from(value: GetActivityDataScRsp) -> Self {
        Self {
            retcode: value.retcode.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetYorozuyaInfoArg> for GetYorozuyaInfoCsReq {
    fn from(value: ::protocol::RpcGetYorozuyaInfoArg) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<GetYorozuyaInfoCsReq> for ::protocol::RpcGetYorozuyaInfoArg {
    fn from(value: GetYorozuyaInfoCsReq) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::RpcReportUiLayoutPlatformArg> for ReportUiLayoutPlatformCsReq {
    fn from(value: ::protocol::RpcReportUiLayoutPlatformArg) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<ReportUiLayoutPlatformCsReq> for ::protocol::RpcReportUiLayoutPlatformArg {
    fn from(value: ReportUiLayoutPlatformCsReq) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::ItemSync> for ItemSync {
    fn from(value: ::protocol::ItemSync) -> Self {
        Self {
            weapon_list: value.weapon_list.into_iter().map(|v| v.into()).collect(),
            auto_recovery_info: value
                .auto_recovery_info
                .into_iter()
                .map(|(k, v)| (k.into(), v.into()))
                .collect(),
            resource_list: value.resource_list.into_iter().map(|v| v.into()).collect(),
            equip_list: value.equip_list.into_iter().map(|v| v.into()).collect(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<ItemSync> for ::protocol::ItemSync {
    fn from(value: ItemSync) -> Self {
        Self {
            weapon_list: value.weapon_list.into_iter().map(|v| v.into()).collect(),
            auto_recovery_info: value
                .auto_recovery_info
                .into_iter()
                .map(|(k, v)| (k.into(), v.into()))
                .collect(),
            resource_list: value.resource_list.into_iter().map(|v| v.into()).collect(),
            equip_list: value.equip_list.into_iter().map(|v| v.into()).collect(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::RpcBattleReportArg> for BattleReportCsReq {
    fn from(value: ::protocol::RpcBattleReportArg) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<BattleReportCsReq> for ::protocol::RpcBattleReportArg {
    fn from(value: BattleReportCsReq) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetDisplayCaseDataArg> for GetDisplayCaseDataCsReq {
    fn from(value: ::protocol::RpcGetDisplayCaseDataArg) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<GetDisplayCaseDataCsReq> for ::protocol::RpcGetDisplayCaseDataArg {
    fn from(value: GetDisplayCaseDataCsReq) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::RpcRunEventGraphRet> for RunEventGraphScRsp {
    fn from(value: ::protocol::RpcRunEventGraphRet) -> Self {
        Self {
            retcode: value.retcode.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<RunEventGraphScRsp> for ::protocol::RpcRunEventGraphRet {
    fn from(value: RunEventGraphScRsp) -> Self {
        Self {
            retcode: value.retcode.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::RpcLeaveCurDungeonArg> for LeaveCurDungeonCsReq {
    fn from(value: ::protocol::RpcLeaveCurDungeonArg) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<LeaveCurDungeonCsReq> for ::protocol::RpcLeaveCurDungeonArg {
    fn from(value: LeaveCurDungeonCsReq) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::PtcUpdateEventGraphArg> for UpdateEventGraphScNotify {
    fn from(value: ::protocol::PtcUpdateEventGraphArg) -> Self {
        Self {
            is_event_success: value.is_event_success.into(),
            event_graph_uid: value.event_graph_uid.into(),
            event_graph_owner_uid: value.event_graph_owner_uid.into(),
            tag: value.tag.into(),
            npc_interaction: value.npc_interaction.into(),
            owner_type: value.owner_type.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<UpdateEventGraphScNotify> for ::protocol::PtcUpdateEventGraphArg {
    fn from(value: UpdateEventGraphScNotify) -> Self {
        Self {
            is_event_success: value.is_event_success.into(),
            event_graph_uid: value.event_graph_uid.into(),
            event_graph_owner_uid: value.event_graph_owner_uid.into(),
            tag: value.tag.into(),
            npc_interaction: value.npc_interaction.into(),
            owner_type: value.owner_type.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetBattlePassDataArg> for GetBattlePassDataCsReq {
    fn from(value: ::protocol::RpcGetBattlePassDataArg) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<GetBattlePassDataCsReq> for ::protocol::RpcGetBattlePassDataArg {
    fn from(value: GetBattlePassDataCsReq) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetClientSystemsInfoRet> for GetClientSystemsInfoScRsp {
    fn from(value: ::protocol::RpcGetClientSystemsInfoRet) -> Self {
        Self {
            info: Some(value.info.into()),
            retcode: value.retcode.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<GetClientSystemsInfoScRsp> for ::protocol::RpcGetClientSystemsInfoRet {
    fn from(value: GetClientSystemsInfoScRsp) -> Self {
        Self {
            info: value.info.map(|v| v.into()).unwrap_or_default(),
            retcode: value.retcode.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetMiniscapeEntrustDataRet> for GetMiniscapeEntrustDataScRsp {
    fn from(value: ::protocol::RpcGetMiniscapeEntrustDataRet) -> Self {
        Self {
            retcode: value.retcode.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<GetMiniscapeEntrustDataScRsp> for ::protocol::RpcGetMiniscapeEntrustDataRet {
    fn from(value: GetMiniscapeEntrustDataScRsp) -> Self {
        Self {
            retcode: value.retcode.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::RpcEndBattleRet> for EndBattleScRsp {
    fn from(value: ::protocol::RpcEndBattleRet) -> Self {
        Self {
            battle_reward: Some(value.battle_reward.into()),
            retcode: value.retcode.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<EndBattleScRsp> for ::protocol::RpcEndBattleRet {
    fn from(value: EndBattleScRsp) -> Self {
        Self {
            battle_reward: value.battle_reward.map(|v| v.into()).unwrap_or_default(),
            retcode: value.retcode.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetPlayerNetworkDataRet> for GetPlayerNetworkDataScRsp {
    fn from(value: ::protocol::RpcGetPlayerNetworkDataRet) -> Self {
        Self {
            retcode: value.retcode.into(),
            player_network_data: value.player_network_data.map(|v| v.into()),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<GetPlayerNetworkDataScRsp> for ::protocol::RpcGetPlayerNetworkDataRet {
    fn from(value: GetPlayerNetworkDataScRsp) -> Self {
        Self {
            retcode: value.retcode.into(),
            player_network_data: value.player_network_data.map(|v| v.into()),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetArchiveInfoArg> for GetArchiveInfoCsReq {
    fn from(value: ::protocol::RpcGetArchiveInfoArg) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<GetArchiveInfoCsReq> for ::protocol::RpcGetArchiveInfoArg {
    fn from(value: GetArchiveInfoCsReq) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetRamenDataRet> for GetRamenDataScRsp {
    fn from(value: ::protocol::RpcGetRamenDataRet) -> Self {
        Self {
            ramen_data: Some(value.ramen_data.into()),
            retcode: value.retcode.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<GetRamenDataScRsp> for ::protocol::RpcGetRamenDataRet {
    fn from(value: GetRamenDataScRsp) -> Self {
        Self {
            ramen_data: value.ramen_data.map(|v| v.into()).unwrap_or_default(),
            retcode: value.retcode.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetDailyChallengeInfoArg> for GetDailyChallengeInfoCsReq {
    fn from(value: ::protocol::RpcGetDailyChallengeInfoArg) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<GetDailyChallengeInfoCsReq> for ::protocol::RpcGetDailyChallengeInfoArg {
    fn from(value: GetDailyChallengeInfoCsReq) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetBattlePassDataRet> for GetBattlePassDataScRsp {
    fn from(value: ::protocol::RpcGetBattlePassDataRet) -> Self {
        Self {
            retcode: value.retcode.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<GetBattlePassDataScRsp> for ::protocol::RpcGetBattlePassDataRet {
    fn from(value: GetBattlePassDataScRsp) -> Self {
        Self {
            retcode: value.retcode.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::RpcRunEventGraphArg> for RunEventGraphCsReq {
    fn from(value: ::protocol::RpcRunEventGraphArg) -> Self {
        Self {
            owner_type: value.owner_type.into(),
            section_id: value.section_id.into(),
            event_graph_uid: value.event_graph_uid.into(),
            owner_id: value.owner_id.into(),
            tag: value.tag.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<RunEventGraphCsReq> for ::protocol::RpcRunEventGraphArg {
    fn from(value: RunEventGraphCsReq) -> Self {
        Self {
            owner_type: value.owner_type.into(),
            section_id: value.section_id.into(),
            event_graph_uid: value.event_graph_uid.into(),
            owner_id: value.owner_id.into(),
            tag: value.tag.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetJourneyInfoArg> for GetJourneyInfoCsReq {
    fn from(value: ::protocol::RpcGetJourneyInfoArg) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<GetJourneyInfoCsReq> for ::protocol::RpcGetJourneyInfoArg {
    fn from(value: GetJourneyInfoCsReq) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::Transform> for Transform {
    fn from(value: ::protocol::Transform) -> Self {
        Self {
            position: value.position.into_iter().map(|v| v.into()).collect(),
            rotation: value.rotation.into_iter().map(|v| v.into()).collect(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<Transform> for ::protocol::Transform {
    fn from(value: Transform) -> Self {
        Self {
            position: value.position.into_iter().map(|v| v.into()).collect(),
            rotation: value.rotation.into_iter().map(|v| v.into()).collect(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetTipsInfoRet> for GetTipsInfoScRsp {
    fn from(value: ::protocol::RpcGetTipsInfoRet) -> Self {
        Self {
            retcode: value.retcode.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<GetTipsInfoScRsp> for ::protocol::RpcGetTipsInfoRet {
    fn from(value: GetTipsInfoScRsp) -> Self {
        Self {
            retcode: value.retcode.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetMainCityRevivalDataArg> for GetMainCityRevivalDataCsReq {
    fn from(value: ::protocol::RpcGetMainCityRevivalDataArg) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<GetMainCityRevivalDataCsReq> for ::protocol::RpcGetMainCityRevivalDataArg {
    fn from(value: GetMainCityRevivalDataCsReq) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetDisplayCaseDataRet> for GetDisplayCaseDataScRsp {
    fn from(value: ::protocol::RpcGetDisplayCaseDataRet) -> Self {
        Self {
            retcode: value.retcode.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<GetDisplayCaseDataScRsp> for ::protocol::RpcGetDisplayCaseDataRet {
    fn from(value: GetDisplayCaseDataScRsp) -> Self {
        Self {
            retcode: value.retcode.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetWishlistDataArg> for GetWishlistDataCsReq {
    fn from(value: ::protocol::RpcGetWishlistDataArg) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<GetWishlistDataCsReq> for ::protocol::RpcGetWishlistDataArg {
    fn from(value: GetWishlistDataCsReq) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::FightSceneInfo> for FightSceneInfo {
    fn from(value: ::protocol::FightSceneInfo) -> Self {
        Self {
            level_perform_info: Some(value.level_perform_info.into()),
            end_hollow: value.end_hollow.into(),
            level_reward_info: Some(value.level_reward_info.into()),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<FightSceneInfo> for ::protocol::FightSceneInfo {
    fn from(value: FightSceneInfo) -> Self {
        Self {
            level_perform_info: value
                .level_perform_info
                .map(|v| v.into())
                .unwrap_or_default(),
            end_hollow: value.end_hollow.into(),
            level_reward_info: value
                .level_reward_info
                .map(|v| v.into())
                .unwrap_or_default(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::RpcCheckYorozuyaInfoRefreshArg> for CheckYorozuyaInfoRefreshCsReq {
    fn from(value: ::protocol::RpcCheckYorozuyaInfoRefreshArg) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<CheckYorozuyaInfoRefreshCsReq> for ::protocol::RpcCheckYorozuyaInfoRefreshArg {
    fn from(value: CheckYorozuyaInfoRefreshCsReq) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::AvatarInfo> for AvatarInfo {
    fn from(value: ::protocol::AvatarInfo) -> Self {
        Self {
            template_id: value.template_id.into(),
            level: value.level.into(),
            first_get_time: value.first_get_time.into(),
            exp: value.exp.into(),
            cur_weapon_uid: value.cur_weapon_uid.into(),
            skill_type_level: value
                .skill_type_level
                .into_iter()
                .map(|v| v.into())
                .collect(),
            rank: value.rank.into(),
            talent_switch_list: value
                .talent_switch_list
                .into_iter()
                .map(|v| v.into())
                .collect(),
            star: value.star.into(),
            unlocked_talent_num: value.unlocked_talent_num.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<AvatarInfo> for ::protocol::AvatarInfo {
    fn from(value: AvatarInfo) -> Self {
        Self {
            template_id: value.template_id.into(),
            level: value.level.into(),
            first_get_time: value.first_get_time.into(),
            exp: value.exp.into(),
            cur_weapon_uid: value.cur_weapon_uid.into(),
            skill_type_level: value
                .skill_type_level
                .into_iter()
                .map(|v| v.into())
                .collect(),
            rank: value.rank.into(),
            talent_switch_list: value
                .talent_switch_list
                .into_iter()
                .map(|v| v.into())
                .collect(),
            star: value.star.into(),
            unlocked_talent_num: value.unlocked_talent_num.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::AvatarUnitInfo> for AvatarUnitInfo {
    fn from(value: ::protocol::AvatarUnitInfo) -> Self {
        Self {
            avatar_id: value.avatar_id.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<AvatarUnitInfo> for ::protocol::AvatarUnitInfo {
    fn from(value: AvatarUnitInfo) -> Self {
        Self {
            avatar_id: value.avatar_id.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetWeaponDataArg> for GetWeaponDataCsReq {
    fn from(value: ::protocol::RpcGetWeaponDataArg) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<GetWeaponDataCsReq> for ::protocol::RpcGetWeaponDataArg {
    fn from(value: GetWeaponDataCsReq) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetChatEmojiListArg> for GetChatEmojiListCsReq {
    fn from(value: ::protocol::RpcGetChatEmojiListArg) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<GetChatEmojiListCsReq> for ::protocol::RpcGetChatEmojiListArg {
    fn from(value: GetChatEmojiListCsReq) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::JourneyInfo> for JourneyInfo {
    fn from(value: ::protocol::JourneyInfo) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<JourneyInfo> for ::protocol::JourneyInfo {
    fn from(value: JourneyInfo) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::RpcWeaponDressArg> for WeaponDressCsReq {
    fn from(value: ::protocol::RpcWeaponDressArg) -> Self {
        Self {
            weapon_uid: value.weapon_uid.into(),
            avatar_id: value.avatar_id.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<WeaponDressCsReq> for ::protocol::RpcWeaponDressArg {
    fn from(value: WeaponDressCsReq) -> Self {
        Self {
            weapon_uid: value.weapon_uid.into(),
            avatar_id: value.avatar_id.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::RpcModMainCityAvatarArg> for ModMainCityAvatarCsReq {
    fn from(value: ::protocol::RpcModMainCityAvatarArg) -> Self {
        Self {
            main_city_avatar_id: value.main_city_avatar_id.into(),
            player_avatar_id: value.player_avatar_id.into(),
            avatar_id: value.avatar_id.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<ModMainCityAvatarCsReq> for ::protocol::RpcModMainCityAvatarArg {
    fn from(value: ModMainCityAvatarCsReq) -> Self {
        Self {
            main_city_avatar_id: value.main_city_avatar_id.into(),
            player_avatar_id: value.player_avatar_id.into(),
            avatar_id: value.avatar_id.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetCafeDataArg> for GetCafeDataCsReq {
    fn from(value: ::protocol::RpcGetCafeDataArg) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<GetCafeDataCsReq> for ::protocol::RpcGetCafeDataArg {
    fn from(value: GetCafeDataCsReq) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetAvatarDataArg> for GetAvatarDataCsReq {
    fn from(value: ::protocol::RpcGetAvatarDataArg) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<GetAvatarDataCsReq> for ::protocol::RpcGetAvatarDataArg {
    fn from(value: GetAvatarDataCsReq) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetBabelTowerDataArg> for GetBabelTowerDataCsReq {
    fn from(value: ::protocol::RpcGetBabelTowerDataArg) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<GetBabelTowerDataCsReq> for ::protocol::RpcGetBabelTowerDataArg {
    fn from(value: GetBabelTowerDataCsReq) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::PlayerNetworkData> for PlayerNetworkData {
    fn from(value: ::protocol::PlayerNetworkData) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<PlayerNetworkData> for ::protocol::PlayerNetworkData {
    fn from(value: PlayerNetworkData) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::PtcEnterSceneArg> for EnterSceneScNotify {
    fn from(value: ::protocol::PtcEnterSceneArg) -> Self {
        Self {
            scene_info: Some(value.scene_info.into()),
            dungeon_info: value.dungeon_info.map(|v| v.into()),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<EnterSceneScNotify> for ::protocol::PtcEnterSceneArg {
    fn from(value: EnterSceneScNotify) -> Self {
        Self {
            scene_info: value.scene_info.map(|v| v.into()).unwrap_or_default(),
            dungeon_info: value.dungeon_info.map(|v| v.into()),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetBuddyDataArg> for GetBuddyDataCsReq {
    fn from(value: ::protocol::RpcGetBuddyDataArg) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<GetBuddyDataCsReq> for ::protocol::RpcGetBuddyDataArg {
    fn from(value: GetBuddyDataCsReq) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::RpcPlayerOperationArg> for PlayerOperationCsReq {
    fn from(value: ::protocol::RpcPlayerOperationArg) -> Self {
        Self {
            param: value.param.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<PlayerOperationCsReq> for ::protocol::RpcPlayerOperationArg {
    fn from(value: PlayerOperationCsReq) -> Self {
        Self {
            param: value.param.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::PtcPlayerSyncArg> for PlayerSyncScNotify {
    fn from(value: ::protocol::PtcPlayerSyncArg) -> Self {
        Self {
            avatar: value.avatar.map(|v| v.into()),
            item: value.item.map(|v| v.into()),
            basic_info: value.basic_info.map(|v| v.into()),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<PlayerSyncScNotify> for ::protocol::PtcPlayerSyncArg {
    fn from(value: PlayerSyncScNotify) -> Self {
        Self {
            avatar: value.avatar.map(|v| v.into()),
            item: value.item.map(|v| v.into()),
            basic_info: value.basic_info.map(|v| v.into()),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::PtcHallRefreshArg> for HallRefreshScNotify {
    fn from(value: ::protocol::PtcHallRefreshArg) -> Self {
        Self {
            time_of_day: value.time_of_day.into(),
            main_city_objects_state: value
                .main_city_objects_state
                .into_iter()
                .map(|(k, v)| (k.into(), v.into()))
                .collect(),
            bgm_id: value.bgm_id.into(),
            section_id: value.section_id.into(),
            refresh_immediately: value.refresh_immediately.into(),
            player_avatar_id: value.player_avatar_id.into(),
            main_city_avatar_id: value.main_city_avatar_id.into(),
            day_of_week: value.day_of_week.into(),
            scene_unit_list: value
                .scene_unit_list
                .into_iter()
                .map(|v| v.into())
                .collect(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<HallRefreshScNotify> for ::protocol::PtcHallRefreshArg {
    fn from(value: HallRefreshScNotify) -> Self {
        Self {
            time_of_day: value.time_of_day.into(),
            main_city_objects_state: value
                .main_city_objects_state
                .into_iter()
                .map(|(k, v)| (k.into(), v.into()))
                .collect(),
            bgm_id: value.bgm_id.into(),
            section_id: value.section_id.into(),
            refresh_immediately: value.refresh_immediately.into(),
            player_avatar_id: value.player_avatar_id.into(),
            main_city_avatar_id: value.main_city_avatar_id.into(),
            day_of_week: value.day_of_week.into(),
            scene_unit_list: value
                .scene_unit_list
                .into_iter()
                .map(|v| v.into())
                .collect(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetArchiveInfoRet> for GetArchiveInfoScRsp {
    fn from(value: ::protocol::RpcGetArchiveInfoRet) -> Self {
        Self {
            archive_info: Some(value.archive_info.into()),
            retcode: value.retcode.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<GetArchiveInfoScRsp> for ::protocol::RpcGetArchiveInfoRet {
    fn from(value: GetArchiveInfoScRsp) -> Self {
        Self {
            archive_info: value.archive_info.map(|v| v.into()).unwrap_or_default(),
            retcode: value.retcode.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::RpcPlayerLoginRet> for PlayerLoginScRsp {
    fn from(value: ::protocol::RpcPlayerLoginRet) -> Self {
        Self {
            retcode: value.retcode.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<PlayerLoginScRsp> for ::protocol::RpcPlayerLoginRet {
    fn from(value: PlayerLoginScRsp) -> Self {
        Self {
            retcode: value.retcode.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::RpcPlayerLoginArg> for PlayerLoginCsReq {
    fn from(value: ::protocol::RpcPlayerLoginArg) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<PlayerLoginCsReq> for ::protocol::RpcPlayerLoginArg {
    fn from(value: PlayerLoginCsReq) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetYorozuyaInfoRet> for GetYorozuyaInfoScRsp {
    fn from(value: ::protocol::RpcGetYorozuyaInfoRet) -> Self {
        Self {
            retcode: value.retcode.into(),
            yorozuya_info: Some(value.yorozuya_info.into()),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<GetYorozuyaInfoScRsp> for ::protocol::RpcGetYorozuyaInfoRet {
    fn from(value: GetYorozuyaInfoScRsp) -> Self {
        Self {
            retcode: value.retcode.into(),
            yorozuya_info: value.yorozuya_info.map(|v| v.into()).unwrap_or_default(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetJourneyInfoRet> for GetJourneyInfoScRsp {
    fn from(value: ::protocol::RpcGetJourneyInfoRet) -> Self {
        Self {
            retcode: value.retcode.into(),
            info: Some(value.info.into()),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<GetJourneyInfoScRsp> for ::protocol::RpcGetJourneyInfoRet {
    fn from(value: GetJourneyInfoScRsp) -> Self {
        Self {
            retcode: value.retcode.into(),
            info: value.info.map(|v| v.into()).unwrap_or_default(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetClientSystemsInfoArg> for GetClientSystemsInfoCsReq {
    fn from(value: ::protocol::RpcGetClientSystemsInfoArg) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<GetClientSystemsInfoCsReq> for ::protocol::RpcGetClientSystemsInfoArg {
    fn from(value: GetClientSystemsInfoCsReq) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetAbyssInfoRet> for GetAbyssInfoScRsp {
    fn from(value: ::protocol::RpcGetAbyssInfoRet) -> Self {
        Self {
            retcode: value.retcode.into(),
            abyss_info: Some(value.abyss_info.into()),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<GetAbyssInfoScRsp> for ::protocol::RpcGetAbyssInfoRet {
    fn from(value: GetAbyssInfoScRsp) -> Self {
        Self {
            retcode: value.retcode.into(),
            abyss_info: value.abyss_info.map(|v| v.into()).unwrap_or_default(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetPlayerNetworkDataArg> for GetPlayerNetworkDataCsReq {
    fn from(value: ::protocol::RpcGetPlayerNetworkDataArg) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<GetPlayerNetworkDataCsReq> for ::protocol::RpcGetPlayerNetworkDataArg {
    fn from(value: GetPlayerNetworkDataCsReq) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetNewsStandDataRet> for GetNewsStandDataScRsp {
    fn from(value: ::protocol::RpcGetNewsStandDataRet) -> Self {
        Self {
            retcode: value.retcode.into(),
            news_stand_data: Some(value.news_stand_data.into()),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<GetNewsStandDataScRsp> for ::protocol::RpcGetNewsStandDataRet {
    fn from(value: GetNewsStandDataScRsp) -> Self {
        Self {
            retcode: value.retcode.into(),
            news_stand_data: value.news_stand_data.map(|v| v.into()).unwrap_or_default(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::RpcRefreshSectionRet> for RefreshSectionScRsp {
    fn from(value: ::protocol::RpcRefreshSectionRet) -> Self {
        Self {
            retcode: value.retcode.into(),
            refresh_status: value.refresh_status.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<RefreshSectionScRsp> for ::protocol::RpcRefreshSectionRet {
    fn from(value: RefreshSectionScRsp) -> Self {
        Self {
            retcode: value.retcode.into(),
            refresh_status: value.refresh_status.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::RpcWeaponUnDressArg> for WeaponUnDressCsReq {
    fn from(value: ::protocol::RpcWeaponUnDressArg) -> Self {
        Self {
            avatar_id: value.avatar_id.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<WeaponUnDressCsReq> for ::protocol::RpcWeaponUnDressArg {
    fn from(value: WeaponUnDressCsReq) -> Self {
        Self {
            avatar_id: value.avatar_id.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::LevelRewardInfo> for LevelRewardInfo {
    fn from(value: ::protocol::LevelRewardInfo) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<LevelRewardInfo> for ::protocol::LevelRewardInfo {
    fn from(value: LevelRewardInfo) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::RpcRefreshSectionArg> for RefreshSectionCsReq {
    fn from(value: ::protocol::RpcRefreshSectionArg) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<RefreshSectionCsReq> for ::protocol::RpcRefreshSectionArg {
    fn from(value: RefreshSectionCsReq) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::FairyInfo> for FairyInfo {
    fn from(value: ::protocol::FairyInfo) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<FairyInfo> for ::protocol::FairyInfo {
    fn from(value: FairyInfo) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::TrashbinHermitData> for TrashbinHermitData {
    fn from(value: ::protocol::TrashbinHermitData) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<TrashbinHermitData> for ::protocol::TrashbinHermitData {
    fn from(value: TrashbinHermitData) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::RpcPlayerTransactionArg> for PlayerTransactionCsReq {
    fn from(value: ::protocol::RpcPlayerTransactionArg) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<PlayerTransactionCsReq> for ::protocol::RpcPlayerTransactionArg {
    fn from(value: PlayerTransactionCsReq) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::ActionInfo> for ActionInfo {
    fn from(value: ::protocol::ActionInfo) -> Self {
        Self {
            body: value.body.into_iter().map(|v| v.into()).collect(),
            action_type: value.action_type.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<ActionInfo> for ::protocol::ActionInfo {
    fn from(value: ActionInfo) -> Self {
        Self {
            body: value.body.into_iter().map(|v| v.into()).collect(),
            action_type: value.action_type.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetAbyssArpeggioDataArg> for GetAbyssArpeggioDataCsReq {
    fn from(value: ::protocol::RpcGetAbyssArpeggioDataArg) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<GetAbyssArpeggioDataCsReq> for ::protocol::RpcGetAbyssArpeggioDataArg {
    fn from(value: GetAbyssArpeggioDataCsReq) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetEmbattlesDataArg> for GetEmbattlesDataCsReq {
    fn from(value: ::protocol::RpcGetEmbattlesDataArg) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<GetEmbattlesDataCsReq> for ::protocol::RpcGetEmbattlesDataArg {
    fn from(value: GetEmbattlesDataCsReq) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::SceneUnitProtocolInfo> for SceneUnitProtocolInfo {
    fn from(value: ::protocol::SceneUnitProtocolInfo) -> Self {
        Self {
            npc_id: value.npc_id.into(),
            is_interactable: value.is_interactable.into(),
            interacts_info: value
                .interacts_info
                .into_iter()
                .map(|(k, v)| (k.into(), v.into()))
                .collect(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<SceneUnitProtocolInfo> for ::protocol::SceneUnitProtocolInfo {
    fn from(value: SceneUnitProtocolInfo) -> Self {
        Self {
            npc_id: value.npc_id.into(),
            is_interactable: value.is_interactable.into(),
            interacts_info: value
                .interacts_info
                .into_iter()
                .map(|(k, v)| (k.into(), v.into()))
                .collect(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::AutoRecoveryInfo> for AutoRecoveryInfo {
    fn from(value: ::protocol::AutoRecoveryInfo) -> Self {
        Self {
            last_recovery_timestamp: value.last_recovery_timestamp.into(),
            buy_times: value.buy_times.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<AutoRecoveryInfo> for ::protocol::AutoRecoveryInfo {
    fn from(value: AutoRecoveryInfo) -> Self {
        Self {
            last_recovery_timestamp: value.last_recovery_timestamp.into(),
            buy_times: value.buy_times.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetCharacterQuestListArg> for GetCharacterQuestListCsReq {
    fn from(value: ::protocol::RpcGetCharacterQuestListArg) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<GetCharacterQuestListCsReq> for ::protocol::RpcGetCharacterQuestListArg {
    fn from(value: GetCharacterQuestListCsReq) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::EquipInfo> for EquipInfo {
    fn from(value: ::protocol::EquipInfo) -> Self {
        Self {
            uid: value.uid.into(),
            template_id: value.template_id.into(),
            exp: value.exp.into(),
            lock: value.lock.into(),
            level: value.level.into(),
            star: value.star.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<EquipInfo> for ::protocol::EquipInfo {
    fn from(value: EquipInfo) -> Self {
        Self {
            uid: value.uid.into(),
            template_id: value.template_id.into(),
            exp: value.exp.into(),
            lock: value.lock.into(),
            level: value.level.into(),
            star: value.star.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::GachaData> for GachaData {
    fn from(value: ::protocol::GachaData) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<GachaData> for ::protocol::GachaData {
    fn from(value: GachaData) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetMonthCardRewardListArg> for GetMonthCardRewardListCsReq {
    fn from(value: ::protocol::RpcGetMonthCardRewardListArg) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<GetMonthCardRewardListCsReq> for ::protocol::RpcGetMonthCardRewardListArg {
    fn from(value: GetMonthCardRewardListCsReq) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::RpcEnterSectionCompleteRet> for EnterSectionCompleteScRsp {
    fn from(value: ::protocol::RpcEnterSectionCompleteRet) -> Self {
        Self {
            retcode: value.retcode.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<EnterSectionCompleteScRsp> for ::protocol::RpcEnterSectionCompleteRet {
    fn from(value: EnterSectionCompleteScRsp) -> Self {
        Self {
            retcode: value.retcode.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::LevelPerformInfo> for LevelPerformInfo {
    fn from(value: ::protocol::LevelPerformInfo) -> Self {
        Self {
            weather: value.weather.into(),
            time: value.time.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<LevelPerformInfo> for ::protocol::LevelPerformInfo {
    fn from(value: LevelPerformInfo) -> Self {
        Self {
            weather: value.weather.into(),
            time: value.time.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetHadalZoneDataArg> for GetHadalZoneDataCsReq {
    fn from(value: ::protocol::RpcGetHadalZoneDataArg) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<GetHadalZoneDataCsReq> for ::protocol::RpcGetHadalZoneDataArg {
    fn from(value: GetHadalZoneDataCsReq) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetAbyssArpeggioDataRet> for GetAbyssArpeggioDataScRsp {
    fn from(value: ::protocol::RpcGetAbyssArpeggioDataRet) -> Self {
        Self {
            retcode: value.retcode.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<GetAbyssArpeggioDataScRsp> for ::protocol::RpcGetAbyssArpeggioDataRet {
    fn from(value: GetAbyssArpeggioDataScRsp) -> Self {
        Self {
            retcode: value.retcode.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetGachaDataRet> for GetGachaDataScRsp {
    fn from(value: ::protocol::RpcGetGachaDataRet) -> Self {
        Self {
            gacha_data: Some(value.gacha_data.into()),
            retcode: value.retcode.into(),
            gacha_type: value.gacha_type.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<GetGachaDataScRsp> for ::protocol::RpcGetGachaDataRet {
    fn from(value: GetGachaDataScRsp) -> Self {
        Self {
            gacha_data: value.gacha_data.map(|v| v.into()).unwrap_or_default(),
            retcode: value.retcode.into(),
            gacha_type: value.gacha_type.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::PlayerBasicInfo> for PlayerBasicInfo {
    fn from(value: ::protocol::PlayerBasicInfo) -> Self {
        Self {
            level: value.level.into(),
            last_enter_world_timestamp: value.last_enter_world_timestamp.into(),
            player_avatar_id: value.player_avatar_id.into(),
            main_city_avatar_id: value.main_city_avatar_id.into(),
            avatar_id: value.avatar_id.into(),
            nick_name: value.nick_name.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<PlayerBasicInfo> for ::protocol::PlayerBasicInfo {
    fn from(value: PlayerBasicInfo) -> Self {
        Self {
            level: value.level.into(),
            last_enter_world_timestamp: value.last_enter_world_timestamp.into(),
            player_avatar_id: value.player_avatar_id.into(),
            main_city_avatar_id: value.main_city_avatar_id.into(),
            avatar_id: value.avatar_id.into(),
            nick_name: value.nick_name.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::AvatarWishlistPlan> for AvatarWishlistPlan {
    fn from(value: ::protocol::AvatarWishlistPlan) -> Self {
        Self {
            plan_type: value.plan_type.into(),
            avatar_plan_param1: value.avatar_plan_param1.into(),
            avatar_plan_param2: value.avatar_plan_param2.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<AvatarWishlistPlan> for ::protocol::AvatarWishlistPlan {
    fn from(value: AvatarWishlistPlan) -> Self {
        Self {
            plan_type: value.plan_type.into(),
            avatar_plan_param1: value.avatar_plan_param1.into(),
            avatar_plan_param2: value.avatar_plan_param2.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetWorkbenchInfoArg> for GetWorkbenchInfoCsReq {
    fn from(value: ::protocol::RpcGetWorkbenchInfoArg) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<GetWorkbenchInfoCsReq> for ::protocol::RpcGetWorkbenchInfoArg {
    fn from(value: GetWorkbenchInfoCsReq) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::WishlistPlanInfo> for WishlistPlanInfo {
    fn from(value: ::protocol::WishlistPlanInfo) -> Self {
        Self {
            avatar_id: value.avatar_id.into(),
            avatar_wishlist_plan: value.avatar_wishlist_plan.map(|v| v.into()),
            equip_wishlist_plan: value.equip_wishlist_plan.map(|v| v.into()),
            skill_wishlist_plan: value.skill_wishlist_plan.map(|v| v.into()),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<WishlistPlanInfo> for ::protocol::WishlistPlanInfo {
    fn from(value: WishlistPlanInfo) -> Self {
        Self {
            avatar_id: value.avatar_id.into(),
            avatar_wishlist_plan: value.avatar_wishlist_plan.map(|v| v.into()),
            equip_wishlist_plan: value.equip_wishlist_plan.map(|v| v.into()),
            skill_wishlist_plan: value.skill_wishlist_plan.map(|v| v.into()),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetTrashbinHermitDataArg> for GetTrashbinHermitDataCsReq {
    fn from(value: ::protocol::RpcGetTrashbinHermitDataArg) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<GetTrashbinHermitDataCsReq> for ::protocol::RpcGetTrashbinHermitDataArg {
    fn from(value: GetTrashbinHermitDataCsReq) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::CafeData> for CafeData {
    fn from(value: ::protocol::CafeData) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<CafeData> for ::protocol::CafeData {
    fn from(value: CafeData) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::RpcReportEmbattleInfoArg> for ReportEmbattleInfoCsReq {
    fn from(value: ::protocol::RpcReportEmbattleInfoArg) -> Self {
        Self {
            avatar_list: value.avatar_list.into_iter().map(|v| v.into()).collect(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<ReportEmbattleInfoCsReq> for ::protocol::RpcReportEmbattleInfoArg {
    fn from(value: ReportEmbattleInfoCsReq) -> Self {
        Self {
            avatar_list: value.avatar_list.into_iter().map(|v| v.into()).collect(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetMiniscapeEntrustDataArg> for GetMiniscapeEntrustDataCsReq {
    fn from(value: ::protocol::RpcGetMiniscapeEntrustDataArg) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<GetMiniscapeEntrustDataCsReq> for ::protocol::RpcGetMiniscapeEntrustDataArg {
    fn from(value: GetMiniscapeEntrustDataCsReq) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::RpcEnterSectionArg> for EnterSectionCsReq {
    fn from(value: ::protocol::RpcEnterSectionArg) -> Self {
        Self {
            section_id: value.section_id.into(),
            transform_id: value.transform_id.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<EnterSectionCsReq> for ::protocol::RpcEnterSectionArg {
    fn from(value: EnterSectionCsReq) -> Self {
        Self {
            section_id: value.section_id.into(),
            transform_id: value.transform_id.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::AvatarSkillInfo> for AvatarSkillInfo {
    fn from(value: ::protocol::AvatarSkillInfo) -> Self {
        Self {
            skill_type: value.skill_type.into(),
            level: value.level.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<AvatarSkillInfo> for ::protocol::AvatarSkillInfo {
    fn from(value: AvatarSkillInfo) -> Self {
        Self {
            skill_type: value.skill_type.into(),
            level: value.level.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetResourceDataArg> for GetResourceDataCsReq {
    fn from(value: ::protocol::RpcGetResourceDataArg) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<GetResourceDataCsReq> for ::protocol::RpcGetResourceDataArg {
    fn from(value: GetResourceDataCsReq) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetRoleCardDataArg> for GetRoleCardDataCsReq {
    fn from(value: ::protocol::RpcGetRoleCardDataArg) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<GetRoleCardDataCsReq> for ::protocol::RpcGetRoleCardDataArg {
    fn from(value: GetRoleCardDataCsReq) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetWeaponDataRet> for GetWeaponDataScRsp {
    fn from(value: ::protocol::RpcGetWeaponDataRet) -> Self {
        Self {
            retcode: value.retcode.into(),
            weapon_list: value.weapon_list.into_iter().map(|v| v.into()).collect(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<GetWeaponDataScRsp> for ::protocol::RpcGetWeaponDataRet {
    fn from(value: GetWeaponDataScRsp) -> Self {
        Self {
            retcode: value.retcode.into(),
            weapon_list: value.weapon_list.into_iter().map(|v| v.into()).collect(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetPhotoWallDataArg> for GetPhotoWallDataCsReq {
    fn from(value: ::protocol::RpcGetPhotoWallDataArg) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<GetPhotoWallDataCsReq> for ::protocol::RpcGetPhotoWallDataArg {
    fn from(value: GetPhotoWallDataCsReq) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::PtcSyncEventInfoArg> for SyncEventInfoScNotify {
    fn from(value: ::protocol::PtcSyncEventInfoArg) -> Self {
        Self {
            npc_interaction: value.npc_interaction.into(),
            action_list: value.action_list.into_iter().map(|v| v.into()).collect(),
            tag: value.tag.into(),
            owner_id: value.owner_id.into(),
            owner_type: value.owner_type.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<SyncEventInfoScNotify> for ::protocol::PtcSyncEventInfoArg {
    fn from(value: SyncEventInfoScNotify) -> Self {
        Self {
            npc_interaction: value.npc_interaction.into(),
            action_list: value.action_list.into_iter().map(|v| v.into()).collect(),
            tag: value.tag.into(),
            owner_id: value.owner_id.into(),
            owner_type: value.owner_type.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::DungeonInfo> for DungeonInfo {
    fn from(value: ::protocol::DungeonInfo) -> Self {
        Self {
            dungeon_equip_info: Some(value.dungeon_equip_info.into()),
            quest_id: value.quest_id.into(),
            avatar_list: value.avatar_list.into_iter().map(|v| v.into()).collect(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<DungeonInfo> for ::protocol::DungeonInfo {
    fn from(value: DungeonInfo) -> Self {
        Self {
            dungeon_equip_info: value
                .dungeon_equip_info
                .map(|v| v.into())
                .unwrap_or_default(),
            quest_id: value.quest_id.into(),
            avatar_list: value.avatar_list.into_iter().map(|v| v.into()).collect(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetPlayerBasicInfoRet> for GetPlayerBasicInfoScRsp {
    fn from(value: ::protocol::RpcGetPlayerBasicInfoRet) -> Self {
        Self {
            retcode: value.retcode.into(),
            basic_info: Some(value.basic_info.into()),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<GetPlayerBasicInfoScRsp> for ::protocol::RpcGetPlayerBasicInfoRet {
    fn from(value: GetPlayerBasicInfoScRsp) -> Self {
        Self {
            retcode: value.retcode.into(),
            basic_info: value.basic_info.map(|v| v.into()).unwrap_or_default(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetPhotoWallDataRet> for GetPhotoWallDataScRsp {
    fn from(value: ::protocol::RpcGetPhotoWallDataRet) -> Self {
        Self {
            retcode: value.retcode.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<GetPhotoWallDataScRsp> for ::protocol::RpcGetPhotoWallDataRet {
    fn from(value: GetPhotoWallDataScRsp) -> Self {
        Self {
            retcode: value.retcode.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::DungeonEquipInfo> for DungeonEquipInfo {
    fn from(value: ::protocol::DungeonEquipInfo) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<DungeonEquipInfo> for ::protocol::DungeonEquipInfo {
    fn from(value: DungeonEquipInfo) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::ResourceInfo> for ResourceInfo {
    fn from(value: ::protocol::ResourceInfo) -> Self {
        Self {
            template_id: value.template_id.into(),
            count: value.count.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<ResourceInfo> for ::protocol::ResourceInfo {
    fn from(value: ResourceInfo) -> Self {
        Self {
            template_id: value.template_id.into(),
            count: value.count.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetGachaDataArg> for GetGachaDataCsReq {
    fn from(value: ::protocol::RpcGetGachaDataArg) -> Self {
        Self {
            gacha_type: value.gacha_type.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<GetGachaDataCsReq> for ::protocol::RpcGetGachaDataArg {
    fn from(value: GetGachaDataCsReq) -> Self {
        Self {
            gacha_type: value.gacha_type.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::RewardBuffData> for RewardBuffData {
    fn from(value: ::protocol::RewardBuffData) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<RewardBuffData> for ::protocol::RewardBuffData {
    fn from(value: RewardBuffData) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetAuthkeyRet> for GetAuthkeyScRsp {
    fn from(value: ::protocol::RpcGetAuthkeyRet) -> Self {
        Self {
            retcode: value.retcode.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<GetAuthkeyScRsp> for ::protocol::RpcGetAuthkeyRet {
    fn from(value: GetAuthkeyScRsp) -> Self {
        Self {
            retcode: value.retcode.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::VideotapeInfo> for VideotapeInfo {
    fn from(value: ::protocol::VideotapeInfo) -> Self {
        Self {
            finished: value.finished.into(),
            archive_file_id: value.archive_file_id.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<VideotapeInfo> for ::protocol::VideotapeInfo {
    fn from(value: VideotapeInfo) -> Self {
        Self {
            finished: value.finished.into(),
            archive_file_id: value.archive_file_id.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetArcadeDataArg> for GetArcadeDataCsReq {
    fn from(value: ::protocol::RpcGetArcadeDataArg) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<GetArcadeDataCsReq> for ::protocol::RpcGetArcadeDataArg {
    fn from(value: GetArcadeDataCsReq) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetMainCityRevivalDataRet> for GetMainCityRevivalDataScRsp {
    fn from(value: ::protocol::RpcGetMainCityRevivalDataRet) -> Self {
        Self {
            retcode: value.retcode.into(),
            main_city_revival: Some(value.main_city_revival.into()),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<GetMainCityRevivalDataScRsp> for ::protocol::RpcGetMainCityRevivalDataRet {
    fn from(value: GetMainCityRevivalDataScRsp) -> Self {
        Self {
            retcode: value.retcode.into(),
            main_city_revival: value
                .main_city_revival
                .map(|v| v.into())
                .unwrap_or_default(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetCafeDataRet> for GetCafeDataScRsp {
    fn from(value: ::protocol::RpcGetCafeDataRet) -> Self {
        Self {
            cafe_data: Some(value.cafe_data.into()),
            retcode: value.retcode.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<GetCafeDataScRsp> for ::protocol::RpcGetCafeDataRet {
    fn from(value: GetCafeDataScRsp) -> Self {
        Self {
            cafe_data: value.cafe_data.map(|v| v.into()).unwrap_or_default(),
            retcode: value.retcode.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetFairyInfoRet> for GetFairyInfoScRsp {
    fn from(value: ::protocol::RpcGetFairyInfoRet) -> Self {
        Self {
            info: Some(value.info.into()),
            retcode: value.retcode.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<GetFairyInfoScRsp> for ::protocol::RpcGetFairyInfoRet {
    fn from(value: GetFairyInfoScRsp) -> Self {
        Self {
            info: value.info.map(|v| v.into()).unwrap_or_default(),
            retcode: value.retcode.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::RpcEnterSectionCompleteArg> for EnterSectionCompleteCsReq {
    fn from(value: ::protocol::RpcEnterSectionCompleteArg) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<EnterSectionCompleteCsReq> for ::protocol::RpcEnterSectionCompleteArg {
    fn from(value: EnterSectionCompleteCsReq) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetCampIdleDataArg> for GetCampIdleDataCsReq {
    fn from(value: ::protocol::RpcGetCampIdleDataArg) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<GetCampIdleDataCsReq> for ::protocol::RpcGetCampIdleDataArg {
    fn from(value: GetCampIdleDataCsReq) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetPlayerMailsArg> for GetPlayerMailsCsReq {
    fn from(value: ::protocol::RpcGetPlayerMailsArg) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<GetPlayerMailsCsReq> for ::protocol::RpcGetPlayerMailsArg {
    fn from(value: GetPlayerMailsCsReq) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::AvatarSync> for AvatarSync {
    fn from(value: ::protocol::AvatarSync) -> Self {
        Self {
            avatar_list: value.avatar_list.into_iter().map(|v| v.into()).collect(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<AvatarSync> for ::protocol::AvatarSync {
    fn from(value: AvatarSync) -> Self {
        Self {
            avatar_list: value.avatar_list.into_iter().map(|v| v.into()).collect(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::RpcEndBattleArg> for EndBattleCsReq {
    fn from(value: ::protocol::RpcEndBattleArg) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<EndBattleCsReq> for ::protocol::RpcEndBattleArg {
    fn from(value: EndBattleCsReq) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::SkillWishlistPlan> for SkillWishlistPlan {
    fn from(value: ::protocol::SkillWishlistPlan) -> Self {
        Self {
            plan_type: value.plan_type.into(),
            wish_skill_id_list: value
                .wish_skill_id_list
                .into_iter()
                .map(|v| v.into())
                .collect(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<SkillWishlistPlan> for ::protocol::SkillWishlistPlan {
    fn from(value: SkillWishlistPlan) -> Self {
        Self {
            plan_type: value.plan_type.into(),
            wish_skill_id_list: value
                .wish_skill_id_list
                .into_iter()
                .map(|v| v.into())
                .collect(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::RpcInteractWithUnitArg> for InteractWithUnitCsReq {
    fn from(value: ::protocol::RpcInteractWithUnitArg) -> Self {
        Self {
            unit_tag: value.unit_tag.into(),
            r#type: value.r#type.into(),
            interaction: value.interaction.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<InteractWithUnitCsReq> for ::protocol::RpcInteractWithUnitArg {
    fn from(value: InteractWithUnitCsReq) -> Self {
        Self {
            unit_tag: value.unit_tag.into(),
            r#type: value.r#type.into(),
            interaction: value.interaction.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::HallSceneInfo> for HallSceneInfo {
    fn from(value: ::protocol::HallSceneInfo) -> Self {
        Self {
            time_of_day: value.time_of_day.into(),
            section_id: value.section_id.into(),
            position: value.position.map(|v| v.into()),
            bgm_id: value.bgm_id.into(),
            camera_y: value.camera_y.into(),
            main_city_avatar_id: value.main_city_avatar_id.into(),
            day_of_week: value.day_of_week.into(),
            main_city_objects_state: value
                .main_city_objects_state
                .into_iter()
                .map(|(k, v)| (k.into(), v.into()))
                .collect(),
            scene_unit_list: value
                .scene_unit_list
                .into_iter()
                .map(|v| v.into())
                .collect(),
            transform_id: value.transform_id.into(),
            player_avatar_id: value.player_avatar_id.into(),
            camera_x: value.camera_x.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<HallSceneInfo> for ::protocol::HallSceneInfo {
    fn from(value: HallSceneInfo) -> Self {
        Self {
            time_of_day: value.time_of_day.into(),
            section_id: value.section_id.into(),
            position: value.position.map(|v| v.into()),
            bgm_id: value.bgm_id.into(),
            camera_y: value.camera_y.into(),
            main_city_avatar_id: value.main_city_avatar_id.into(),
            day_of_week: value.day_of_week.into(),
            main_city_objects_state: value
                .main_city_objects_state
                .into_iter()
                .map(|(k, v)| (k.into(), v.into()))
                .collect(),
            scene_unit_list: value
                .scene_unit_list
                .into_iter()
                .map(|v| v.into())
                .collect(),
            transform_id: value.transform_id.into(),
            player_avatar_id: value.player_avatar_id.into(),
            camera_x: value.camera_x.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetFashionStoreInfoArg> for GetFashionStoreInfoCsReq {
    fn from(value: ::protocol::RpcGetFashionStoreInfoArg) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<GetFashionStoreInfoCsReq> for ::protocol::RpcGetFashionStoreInfoArg {
    fn from(value: GetFashionStoreInfoCsReq) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::AbyssInfo> for AbyssInfo {
    fn from(value: ::protocol::AbyssInfo) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<AbyssInfo> for ::protocol::AbyssInfo {
    fn from(value: AbyssInfo) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetFriendListArg> for GetFriendListCsReq {
    fn from(value: ::protocol::RpcGetFriendListArg) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<GetFriendListCsReq> for ::protocol::RpcGetFriendListArg {
    fn from(value: GetFriendListCsReq) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::QuestData> for QuestData {
    fn from(value: ::protocol::QuestData) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<QuestData> for ::protocol::QuestData {
    fn from(value: QuestData) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetActivityDataArg> for GetActivityDataCsReq {
    fn from(value: ::protocol::RpcGetActivityDataArg) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<GetActivityDataCsReq> for ::protocol::RpcGetActivityDataArg {
    fn from(value: GetActivityDataCsReq) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetRechargeItemListArg> for GetRechargeItemListCsReq {
    fn from(value: ::protocol::RpcGetRechargeItemListArg) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<GetRechargeItemListCsReq> for ::protocol::RpcGetRechargeItemListArg {
    fn from(value: GetRechargeItemListCsReq) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetServerTimestampArg> for GetServerTimestampCsReq {
    fn from(value: ::protocol::RpcGetServerTimestampArg) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<GetServerTimestampCsReq> for ::protocol::RpcGetServerTimestampArg {
    fn from(value: GetServerTimestampCsReq) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetRewardBuffDataRet> for GetRewardBuffDataScRsp {
    fn from(value: ::protocol::RpcGetRewardBuffDataRet) -> Self {
        Self {
            info: Some(value.info.into()),
            retcode: value.retcode.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<GetRewardBuffDataScRsp> for ::protocol::RpcGetRewardBuffDataRet {
    fn from(value: GetRewardBuffDataScRsp) -> Self {
        Self {
            info: value.info.map(|v| v.into()).unwrap_or_default(),
            retcode: value.retcode.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::MainCityRevivalData> for MainCityRevivalData {
    fn from(value: ::protocol::MainCityRevivalData) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<MainCityRevivalData> for ::protocol::MainCityRevivalData {
    fn from(value: MainCityRevivalData) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::RpcSceneTransitionArg> for SceneTransitionCsReq {
    fn from(value: ::protocol::RpcSceneTransitionArg) -> Self {
        Self {
            section_id: value.section_id.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<SceneTransitionCsReq> for ::protocol::RpcSceneTransitionArg {
    fn from(value: SceneTransitionCsReq) -> Self {
        Self {
            section_id: value.section_id.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::RpcModTimeArg> for ModTimeCsReq {
    fn from(value: ::protocol::RpcModTimeArg) -> Self {
        Self {
            time_period: value.time_period.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<ModTimeCsReq> for ::protocol::RpcModTimeArg {
    fn from(value: ModTimeCsReq) -> Self {
        Self {
            time_period: value.time_period.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::InteractInfo> for InteractInfo {
    fn from(value: ::protocol::InteractInfo) -> Self {
        Self {
            interact_id: value.interact_id.into(),
            participators: value
                .participators
                .into_iter()
                .map(|(k, v)| (k.into(), v.into()))
                .collect(),
            interact_target_list: value
                .interact_target_list
                .into_iter()
                .map(|v| v.into())
                .collect(),
            scale_w: value.scale_w.into(),
            scale_x: value.scale_x.into(),
            scale_r: value.scale_r.into(),
            scale_y: value.scale_y.into(),
            name: value.name.into(),
            scale_z: value.scale_z.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<InteractInfo> for ::protocol::InteractInfo {
    fn from(value: InteractInfo) -> Self {
        Self {
            interact_id: value.interact_id.into(),
            participators: value
                .participators
                .into_iter()
                .map(|(k, v)| (k.into(), v.into()))
                .collect(),
            interact_target_list: value
                .interact_target_list
                .into_iter()
                .map(|v| v.into())
                .collect(),
            scale_w: value.scale_w.into(),
            scale_x: value.scale_x.into(),
            scale_r: value.scale_r.into(),
            scale_y: value.scale_y.into(),
            name: value.name.into(),
            scale_z: value.scale_z.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetEquipDataRet> for GetEquipDataScRsp {
    fn from(value: ::protocol::RpcGetEquipDataRet) -> Self {
        Self {
            equip_list: value.equip_list.into_iter().map(|v| v.into()).collect(),
            retcode: value.retcode.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<GetEquipDataScRsp> for ::protocol::RpcGetEquipDataRet {
    fn from(value: GetEquipDataScRsp) -> Self {
        Self {
            equip_list: value.equip_list.into_iter().map(|v| v.into()).collect(),
            retcode: value.retcode.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::PtcAddAvatarArg> for AddAvatarScNotify {
    fn from(value: ::protocol::PtcAddAvatarArg) -> Self {
        Self {
            perform_type: value.perform_type.into(),
            avatar_id: value.avatar_id.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<AddAvatarScNotify> for ::protocol::PtcAddAvatarArg {
    fn from(value: AddAvatarScNotify) -> Self {
        Self {
            perform_type: value.perform_type.into(),
            avatar_id: value.avatar_id.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetEquipDataArg> for GetEquipDataCsReq {
    fn from(value: ::protocol::RpcGetEquipDataArg) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<GetEquipDataCsReq> for ::protocol::RpcGetEquipDataArg {
    fn from(value: GetEquipDataCsReq) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::RpcSavePlayerSystemSettingArg> for SavePlayerSystemSettingCsReq {
    fn from(value: ::protocol::RpcSavePlayerSystemSettingArg) -> Self {
        Self {
            r#type: value.r#type.into(),
            params: value.params.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<SavePlayerSystemSettingCsReq> for ::protocol::RpcSavePlayerSystemSettingArg {
    fn from(value: SavePlayerSystemSettingCsReq) -> Self {
        Self {
            r#type: value.r#type.into(),
            params: value.params.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::YorozuyaInfo> for YorozuyaInfo {
    fn from(value: ::protocol::YorozuyaInfo) -> Self {
        Self {
            unlock_hollow_id_list: value
                .unlock_hollow_id_list
                .into_iter()
                .map(|v| v.into())
                .collect(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<YorozuyaInfo> for ::protocol::YorozuyaInfo {
    fn from(value: YorozuyaInfo) -> Self {
        Self {
            unlock_hollow_id_list: value
                .unlock_hollow_id_list
                .into_iter()
                .map(|v| v.into())
                .collect(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetAvatarDataRet> for GetAvatarDataScRsp {
    fn from(value: ::protocol::RpcGetAvatarDataRet) -> Self {
        Self {
            retcode: value.retcode.into(),
            avatar_list: value.avatar_list.into_iter().map(|v| v.into()).collect(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<GetAvatarDataScRsp> for ::protocol::RpcGetAvatarDataRet {
    fn from(value: GetAvatarDataScRsp) -> Self {
        Self {
            retcode: value.retcode.into(),
            avatar_list: value.avatar_list.into_iter().map(|v| v.into()).collect(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::WeaponInfo> for WeaponInfo {
    fn from(value: ::protocol::WeaponInfo) -> Self {
        Self {
            level: value.level.into(),
            refine_level: value.refine_level.into(),
            star: value.star.into(),
            template_id: value.template_id.into(),
            exp: value.exp.into(),
            uid: value.uid.into(),
            lock: value.lock.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<WeaponInfo> for ::protocol::WeaponInfo {
    fn from(value: WeaponInfo) -> Self {
        Self {
            level: value.level.into(),
            refine_level: value.refine_level.into(),
            star: value.star.into(),
            template_id: value.template_id.into(),
            exp: value.exp.into(),
            uid: value.uid.into(),
            lock: value.lock.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetBuddyDataRet> for GetBuddyDataScRsp {
    fn from(value: ::protocol::RpcGetBuddyDataRet) -> Self {
        Self {
            retcode: value.retcode.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<GetBuddyDataScRsp> for ::protocol::RpcGetBuddyDataRet {
    fn from(value: GetBuddyDataScRsp) -> Self {
        Self {
            retcode: value.retcode.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::RpcEnterWorldRet> for EnterWorldScRsp {
    fn from(value: ::protocol::RpcEnterWorldRet) -> Self {
        Self {
            retcode: value.retcode.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<EnterWorldScRsp> for ::protocol::RpcEnterWorldRet {
    fn from(value: EnterWorldScRsp) -> Self {
        Self {
            retcode: value.retcode.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetRewardBuffDataArg> for GetRewardBuffDataCsReq {
    fn from(value: ::protocol::RpcGetRewardBuffDataArg) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<GetRewardBuffDataCsReq> for ::protocol::RpcGetRewardBuffDataArg {
    fn from(value: GetRewardBuffDataCsReq) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetWebActivityDataRet> for GetWebActivityDataScRsp {
    fn from(value: ::protocol::RpcGetWebActivityDataRet) -> Self {
        Self {
            retcode: value.retcode.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<GetWebActivityDataScRsp> for ::protocol::RpcGetWebActivityDataRet {
    fn from(value: GetWebActivityDataScRsp) -> Self {
        Self {
            retcode: value.retcode.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::SceneInfo> for SceneInfo {
    fn from(value: ::protocol::SceneInfo) -> Self {
        Self {
            hall_scene_info: value.hall_scene_info.map(|v| v.into()),
            fight_scene_info: value.fight_scene_info.map(|v| v.into()),
            event_id: value.event_id.into(),
            local_play_type: value.local_play_type.into(),
            scene_type: value.scene_type.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<SceneInfo> for ::protocol::SceneInfo {
    fn from(value: SceneInfo) -> Self {
        Self {
            hall_scene_info: value.hall_scene_info.map(|v| v.into()),
            fight_scene_info: value.fight_scene_info.map(|v| v.into()),
            event_id: value.event_id.into(),
            local_play_type: value.local_play_type.into(),
            scene_type: value.scene_type.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::RpcEnterWorldArg> for EnterWorldCsReq {
    fn from(value: ::protocol::RpcEnterWorldArg) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<EnterWorldCsReq> for ::protocol::RpcEnterWorldArg {
    fn from(value: EnterWorldCsReq) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetQuestDataArg> for GetQuestDataCsReq {
    fn from(value: ::protocol::RpcGetQuestDataArg) -> Self {
        Self {
            quest_type: value.quest_type.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<GetQuestDataCsReq> for ::protocol::RpcGetQuestDataArg {
    fn from(value: GetQuestDataCsReq) -> Self {
        Self {
            quest_type: value.quest_type.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetResourceDataRet> for GetResourceDataScRsp {
    fn from(value: ::protocol::RpcGetResourceDataRet) -> Self {
        Self {
            auto_recovery_info: value
                .auto_recovery_info
                .into_iter()
                .map(|(k, v)| (k.into(), v.into()))
                .collect(),
            resource_list: value.resource_list.into_iter().map(|v| v.into()).collect(),
            retcode: value.retcode.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<GetResourceDataScRsp> for ::protocol::RpcGetResourceDataRet {
    fn from(value: GetResourceDataScRsp) -> Self {
        Self {
            auto_recovery_info: value
                .auto_recovery_info
                .into_iter()
                .map(|(k, v)| (k.into(), v.into()))
                .collect(),
            resource_list: value.resource_list.into_iter().map(|v| v.into()).collect(),
            retcode: value.retcode.into(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<::protocol::RpcGetWebActivityDataArg> for GetWebActivityDataCsReq {
    fn from(value: ::protocol::RpcGetWebActivityDataArg) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<GetWebActivityDataCsReq> for ::protocol::RpcGetWebActivityDataArg {
    fn from(value: GetWebActivityDataCsReq) -> Self {
        Self { ..Default::default() }
    }
}
#[allow(unused)]
impl From<::protocol::ArchiveInfo> for ArchiveInfo {
    fn from(value: ::protocol::ArchiveInfo) -> Self {
        Self {
            hollow_archive_id_list: value
                .hollow_archive_id_list
                .into_iter()
                .map(|v| v.into())
                .collect(),
            videotaps_info: value.videotaps_info.into_iter().map(|v| v.into()).collect(),
            ..Default::default()
        }
    }
}
#[allow(unused)]
impl From<ArchiveInfo> for ::protocol::ArchiveInfo {
    fn from(value: ArchiveInfo) -> Self {
        Self {
            hollow_archive_id_list: value
                .hollow_archive_id_list
                .into_iter()
                .map(|v| v.into())
                .collect(),
            videotaps_info: value.videotaps_info.into_iter().map(|v| v.into()).collect(),
            ..Default::default()
        }
    }
}
#[macro_export]
macro_rules! decode_and_forward_proto {
    (
        $cmd_id:expr, $buf:expr, $session:expr, $point:expr, $addr:expr,
        $middlewares:expr, $timeout:expr
    ) => {
        match $cmd_id { GetAbyssRewardDataCsReq::CMD_ID => { let packet = NetPacket:: <
        ::yanagi_proto::GetAbyssRewardDataCsReq > ::decode($buf) ?; let rpc_arg =
        ::protocol::RpcGetAbyssRewardDataArg::from(packet.body); let rpc_ret :
        ::protocol::RpcGetAbyssRewardDataRet = $point .call_rpc($addr, rpc_arg,
        $middlewares, $timeout). await ?; $session .send_null_rsp(packet.head.packet_id);
        }, GetExplorationDataCsReq::CMD_ID => { let packet = NetPacket:: <
        ::yanagi_proto::GetExplorationDataCsReq > ::decode($buf) ?; let rpc_arg =
        ::protocol::RpcGetExplorationDataArg::from(packet.body); let rpc_ret :
        ::protocol::RpcGetExplorationDataRet = $point .call_rpc($addr, rpc_arg,
        $middlewares, $timeout). await ?; $session .send_null_rsp(packet.head.packet_id);
        }, BeginTrainingCourseBattleCsReq::CMD_ID => { let packet = NetPacket:: <
        ::yanagi_proto::BeginTrainingCourseBattleCsReq > ::decode($buf) ?; let rpc_arg =
        ::protocol::RpcBeginTrainingCourseBattleArg::from(packet.body); let rpc_ret :
        ::protocol::RpcBeginTrainingCourseBattleRet = $point .call_rpc($addr, rpc_arg,
        $middlewares, $timeout). await ?; $session .send_null_rsp(packet.head.packet_id);
        }, SavePosInMainCityCsReq::CMD_ID => { let packet = NetPacket:: <
        ::yanagi_proto::SavePosInMainCityCsReq > ::decode($buf) ?; let rpc_arg =
        ::protocol::RpcSavePosInMainCityArg::from(packet.body); let rpc_ret :
        ::protocol::RpcSavePosInMainCityRet = $point .call_rpc($addr, rpc_arg,
        $middlewares, $timeout). await ?; $session .send_null_rsp(packet.head.packet_id);
        }, GetVhsStoreInfoCsReq::CMD_ID => { let packet = NetPacket:: <
        ::yanagi_proto::GetVhsStoreInfoCsReq > ::decode($buf) ?; let rpc_arg =
        ::protocol::RpcGetVhsStoreInfoArg::from(packet.body); let rpc_ret :
        ::protocol::RpcGetVhsStoreInfoRet = $point .call_rpc($addr, rpc_arg,
        $middlewares, $timeout). await ?; $session .send_null_rsp(packet.head.packet_id);
        }, GetAbyssInfoCsReq::CMD_ID => { let packet = NetPacket:: <
        ::yanagi_proto::GetAbyssInfoCsReq > ::decode($buf) ?; let rpc_arg =
        ::protocol::RpcGetAbyssInfoArg::from(packet.body); let rpc_ret :
        ::protocol::RpcGetAbyssInfoRet = $point .call_rpc($addr, rpc_arg, $middlewares,
        $timeout). await ?; let proto_rsp =
        ::yanagi_proto::GetAbyssInfoScRsp::from(rpc_ret); $session .send_rsp(packet.head
        .packet_id, proto_rsp); }, GetPrivateMessageDataCsReq::CMD_ID => { let packet =
        NetPacket:: < ::yanagi_proto::GetPrivateMessageDataCsReq > ::decode($buf) ?; let
        rpc_arg = ::protocol::RpcGetPrivateMessageDataArg::from(packet.body); let rpc_ret
        : ::protocol::RpcGetPrivateMessageDataRet = $point .call_rpc($addr, rpc_arg,
        $middlewares, $timeout). await ?; $session .send_null_rsp(packet.head.packet_id);
        }, GetNewsStandDataCsReq::CMD_ID => { let packet = NetPacket:: <
        ::yanagi_proto::GetNewsStandDataCsReq > ::decode($buf) ?; let rpc_arg =
        ::protocol::RpcGetNewsStandDataArg::from(packet.body); let rpc_ret :
        ::protocol::RpcGetNewsStandDataRet = $point .call_rpc($addr, rpc_arg,
        $middlewares, $timeout). await ?; let proto_rsp =
        ::yanagi_proto::GetNewsStandDataScRsp::from(rpc_ret); $session .send_rsp(packet
        .head.packet_id, proto_rsp); }, GetVideoUsmKeyDataCsReq::CMD_ID => { let packet =
        NetPacket:: < ::yanagi_proto::GetVideoUsmKeyDataCsReq > ::decode($buf) ?; let
        rpc_arg = ::protocol::RpcGetVideoUsmKeyDataArg::from(packet.body); let rpc_ret :
        ::protocol::RpcGetVideoUsmKeyDataRet = $point .call_rpc($addr, rpc_arg,
        $middlewares, $timeout). await ?; let proto_rsp =
        ::yanagi_proto::GetVideoUsmKeyDataScRsp::from(rpc_ret); $session .send_rsp(packet
        .head.packet_id, proto_rsp); }, GetPlayerBasicInfoCsReq::CMD_ID => { let packet =
        NetPacket:: < ::yanagi_proto::GetPlayerBasicInfoCsReq > ::decode($buf) ?; let
        rpc_arg = ::protocol::RpcGetPlayerBasicInfoArg::from(packet.body); let rpc_ret :
        ::protocol::RpcGetPlayerBasicInfoRet = $point .call_rpc($addr, rpc_arg,
        $middlewares, $timeout). await ?; let proto_rsp =
        ::yanagi_proto::GetPlayerBasicInfoScRsp::from(rpc_ret); $session .send_rsp(packet
        .head.packet_id, proto_rsp); }, GetAuthkeyCsReq::CMD_ID => { let packet =
        NetPacket:: < ::yanagi_proto::GetAuthkeyCsReq > ::decode($buf) ?; let rpc_arg =
        ::protocol::RpcGetAuthkeyArg::from(packet.body); let rpc_ret :
        ::protocol::RpcGetAuthkeyRet = $point .call_rpc($addr, rpc_arg, $middlewares,
        $timeout). await ?; let proto_rsp =
        ::yanagi_proto::GetAuthkeyScRsp::from(rpc_ret); $session .send_rsp(packet.head
        .packet_id, proto_rsp); }, GetRamenDataCsReq::CMD_ID => { let packet =
        NetPacket:: < ::yanagi_proto::GetRamenDataCsReq > ::decode($buf) ?; let rpc_arg =
        ::protocol::RpcGetRamenDataArg::from(packet.body); let rpc_ret :
        ::protocol::RpcGetRamenDataRet = $point .call_rpc($addr, rpc_arg, $middlewares,
        $timeout). await ?; let proto_rsp =
        ::yanagi_proto::GetRamenDataScRsp::from(rpc_ret); $session .send_rsp(packet.head
        .packet_id, proto_rsp); }, GetFairyInfoCsReq::CMD_ID => { let packet =
        NetPacket:: < ::yanagi_proto::GetFairyInfoCsReq > ::decode($buf) ?; let rpc_arg =
        ::protocol::RpcGetFairyInfoArg::from(packet.body); let rpc_ret :
        ::protocol::RpcGetFairyInfoRet = $point .call_rpc($addr, rpc_arg, $middlewares,
        $timeout). await ?; let proto_rsp =
        ::yanagi_proto::GetFairyInfoScRsp::from(rpc_ret); $session .send_rsp(packet.head
        .packet_id, proto_rsp); }, GetCollectMapCsReq::CMD_ID => { let packet =
        NetPacket:: < ::yanagi_proto::GetCollectMapCsReq > ::decode($buf) ?; let rpc_arg
        = ::protocol::RpcGetCollectMapArg::from(packet.body); let rpc_ret :
        ::protocol::RpcGetCollectMapRet = $point .call_rpc($addr, rpc_arg, $middlewares,
        $timeout). await ?; $session .send_null_rsp(packet.head.packet_id); },
        InteractWithClientEntityCsReq::CMD_ID => { let packet = NetPacket:: <
        ::yanagi_proto::InteractWithClientEntityCsReq > ::decode($buf) ?; let rpc_arg =
        ::protocol::RpcInteractWithClientEntityArg::from(packet.body); let rpc_ret :
        ::protocol::RpcInteractWithClientEntityRet = $point .call_rpc($addr, rpc_arg,
        $middlewares, $timeout). await ?; $session .send_null_rsp(packet.head.packet_id);
        }, GetShoppingMallInfoCsReq::CMD_ID => { let packet = NetPacket:: <
        ::yanagi_proto::GetShoppingMallInfoCsReq > ::decode($buf) ?; let rpc_arg =
        ::protocol::RpcGetShoppingMallInfoArg::from(packet.body); let rpc_ret :
        ::protocol::RpcGetShoppingMallInfoRet = $point .call_rpc($addr, rpc_arg,
        $middlewares, $timeout). await ?; $session .send_null_rsp(packet.head.packet_id);
        }, GetTipsInfoCsReq::CMD_ID => { let packet = NetPacket:: <
        ::yanagi_proto::GetTipsInfoCsReq > ::decode($buf) ?; let rpc_arg =
        ::protocol::RpcGetTipsInfoArg::from(packet.body); let rpc_ret :
        ::protocol::RpcGetTipsInfoRet = $point .call_rpc($addr, rpc_arg, $middlewares,
        $timeout). await ?; let proto_rsp =
        ::yanagi_proto::GetTipsInfoScRsp::from(rpc_ret); $session .send_rsp(packet.head
        .packet_id, proto_rsp); }, GetYorozuyaInfoCsReq::CMD_ID => { let packet =
        NetPacket:: < ::yanagi_proto::GetYorozuyaInfoCsReq > ::decode($buf) ?; let
        rpc_arg = ::protocol::RpcGetYorozuyaInfoArg::from(packet.body); let rpc_ret :
        ::protocol::RpcGetYorozuyaInfoRet = $point .call_rpc($addr, rpc_arg,
        $middlewares, $timeout). await ?; let proto_rsp =
        ::yanagi_proto::GetYorozuyaInfoScRsp::from(rpc_ret); $session .send_rsp(packet
        .head.packet_id, proto_rsp); }, ReportUiLayoutPlatformCsReq::CMD_ID => { let
        packet = NetPacket:: < ::yanagi_proto::ReportUiLayoutPlatformCsReq >
        ::decode($buf) ?; let rpc_arg =
        ::protocol::RpcReportUiLayoutPlatformArg::from(packet.body); let rpc_ret :
        ::protocol::RpcReportUiLayoutPlatformRet = $point .call_rpc($addr, rpc_arg,
        $middlewares, $timeout). await ?; $session .send_null_rsp(packet.head.packet_id);
        }, BattleReportCsReq::CMD_ID => { let packet = NetPacket:: <
        ::yanagi_proto::BattleReportCsReq > ::decode($buf) ?; let rpc_arg =
        ::protocol::RpcBattleReportArg::from(packet.body); let rpc_ret :
        ::protocol::RpcBattleReportRet = $point .call_rpc($addr, rpc_arg, $middlewares,
        $timeout). await ?; $session .send_null_rsp(packet.head.packet_id); },
        GetDisplayCaseDataCsReq::CMD_ID => { let packet = NetPacket:: <
        ::yanagi_proto::GetDisplayCaseDataCsReq > ::decode($buf) ?; let rpc_arg =
        ::protocol::RpcGetDisplayCaseDataArg::from(packet.body); let rpc_ret :
        ::protocol::RpcGetDisplayCaseDataRet = $point .call_rpc($addr, rpc_arg,
        $middlewares, $timeout). await ?; let proto_rsp =
        ::yanagi_proto::GetDisplayCaseDataScRsp::from(rpc_ret); $session .send_rsp(packet
        .head.packet_id, proto_rsp); }, LeaveCurDungeonCsReq::CMD_ID => { let packet =
        NetPacket:: < ::yanagi_proto::LeaveCurDungeonCsReq > ::decode($buf) ?; let
        rpc_arg = ::protocol::RpcLeaveCurDungeonArg::from(packet.body); let rpc_ret :
        ::protocol::RpcLeaveCurDungeonRet = $point .call_rpc($addr, rpc_arg,
        $middlewares, $timeout). await ?; $session .send_null_rsp(packet.head.packet_id);
        }, GetBattlePassDataCsReq::CMD_ID => { let packet = NetPacket:: <
        ::yanagi_proto::GetBattlePassDataCsReq > ::decode($buf) ?; let rpc_arg =
        ::protocol::RpcGetBattlePassDataArg::from(packet.body); let rpc_ret :
        ::protocol::RpcGetBattlePassDataRet = $point .call_rpc($addr, rpc_arg,
        $middlewares, $timeout). await ?; let proto_rsp =
        ::yanagi_proto::GetBattlePassDataScRsp::from(rpc_ret); $session .send_rsp(packet
        .head.packet_id, proto_rsp); }, GetArchiveInfoCsReq::CMD_ID => { let packet =
        NetPacket:: < ::yanagi_proto::GetArchiveInfoCsReq > ::decode($buf) ?; let rpc_arg
        = ::protocol::RpcGetArchiveInfoArg::from(packet.body); let rpc_ret :
        ::protocol::RpcGetArchiveInfoRet = $point .call_rpc($addr, rpc_arg, $middlewares,
        $timeout). await ?; let proto_rsp =
        ::yanagi_proto::GetArchiveInfoScRsp::from(rpc_ret); $session .send_rsp(packet
        .head.packet_id, proto_rsp); }, GetDailyChallengeInfoCsReq::CMD_ID => { let
        packet = NetPacket:: < ::yanagi_proto::GetDailyChallengeInfoCsReq >
        ::decode($buf) ?; let rpc_arg =
        ::protocol::RpcGetDailyChallengeInfoArg::from(packet.body); let rpc_ret :
        ::protocol::RpcGetDailyChallengeInfoRet = $point .call_rpc($addr, rpc_arg,
        $middlewares, $timeout). await ?; $session .send_null_rsp(packet.head.packet_id);
        }, RunEventGraphCsReq::CMD_ID => { let packet = NetPacket:: <
        ::yanagi_proto::RunEventGraphCsReq > ::decode($buf) ?; let rpc_arg =
        ::protocol::RpcRunEventGraphArg::from(packet.body); let rpc_ret :
        ::protocol::RpcRunEventGraphRet = $point .call_rpc($addr, rpc_arg, $middlewares,
        $timeout). await ?; let proto_rsp =
        ::yanagi_proto::RunEventGraphScRsp::from(rpc_ret); $session .send_rsp(packet.head
        .packet_id, proto_rsp); }, GetJourneyInfoCsReq::CMD_ID => { let packet =
        NetPacket:: < ::yanagi_proto::GetJourneyInfoCsReq > ::decode($buf) ?; let rpc_arg
        = ::protocol::RpcGetJourneyInfoArg::from(packet.body); let rpc_ret :
        ::protocol::RpcGetJourneyInfoRet = $point .call_rpc($addr, rpc_arg, $middlewares,
        $timeout). await ?; let proto_rsp =
        ::yanagi_proto::GetJourneyInfoScRsp::from(rpc_ret); $session .send_rsp(packet
        .head.packet_id, proto_rsp); }, GetMainCityRevivalDataCsReq::CMD_ID => { let
        packet = NetPacket:: < ::yanagi_proto::GetMainCityRevivalDataCsReq >
        ::decode($buf) ?; let rpc_arg =
        ::protocol::RpcGetMainCityRevivalDataArg::from(packet.body); let rpc_ret :
        ::protocol::RpcGetMainCityRevivalDataRet = $point .call_rpc($addr, rpc_arg,
        $middlewares, $timeout). await ?; let proto_rsp =
        ::yanagi_proto::GetMainCityRevivalDataScRsp::from(rpc_ret); $session
        .send_rsp(packet.head.packet_id, proto_rsp); }, GetWishlistDataCsReq::CMD_ID => {
        let packet = NetPacket:: < ::yanagi_proto::GetWishlistDataCsReq > ::decode($buf)
        ?; let rpc_arg = ::protocol::RpcGetWishlistDataArg::from(packet.body); let
        rpc_ret : ::protocol::RpcGetWishlistDataRet = $point .call_rpc($addr, rpc_arg,
        $middlewares, $timeout). await ?; let proto_rsp =
        ::yanagi_proto::GetWishlistDataScRsp::from(rpc_ret); $session .send_rsp(packet
        .head.packet_id, proto_rsp); }, CheckYorozuyaInfoRefreshCsReq::CMD_ID => { let
        packet = NetPacket:: < ::yanagi_proto::CheckYorozuyaInfoRefreshCsReq >
        ::decode($buf) ?; let rpc_arg =
        ::protocol::RpcCheckYorozuyaInfoRefreshArg::from(packet.body); let rpc_ret :
        ::protocol::RpcCheckYorozuyaInfoRefreshRet = $point .call_rpc($addr, rpc_arg,
        $middlewares, $timeout). await ?; $session .send_null_rsp(packet.head.packet_id);
        }, GetWeaponDataCsReq::CMD_ID => { let packet = NetPacket:: <
        ::yanagi_proto::GetWeaponDataCsReq > ::decode($buf) ?; let rpc_arg =
        ::protocol::RpcGetWeaponDataArg::from(packet.body); let rpc_ret :
        ::protocol::RpcGetWeaponDataRet = $point .call_rpc($addr, rpc_arg, $middlewares,
        $timeout). await ?; let proto_rsp =
        ::yanagi_proto::GetWeaponDataScRsp::from(rpc_ret); $session .send_rsp(packet.head
        .packet_id, proto_rsp); }, GetChatEmojiListCsReq::CMD_ID => { let packet =
        NetPacket:: < ::yanagi_proto::GetChatEmojiListCsReq > ::decode($buf) ?; let
        rpc_arg = ::protocol::RpcGetChatEmojiListArg::from(packet.body); let rpc_ret :
        ::protocol::RpcGetChatEmojiListRet = $point .call_rpc($addr, rpc_arg,
        $middlewares, $timeout). await ?; $session .send_null_rsp(packet.head.packet_id);
        }, WeaponDressCsReq::CMD_ID => { let packet = NetPacket:: <
        ::yanagi_proto::WeaponDressCsReq > ::decode($buf) ?; let rpc_arg =
        ::protocol::RpcWeaponDressArg::from(packet.body); let rpc_ret :
        ::protocol::RpcWeaponDressRet = $point .call_rpc($addr, rpc_arg, $middlewares,
        $timeout). await ?; $session .send_null_rsp(packet.head.packet_id); },
        ModMainCityAvatarCsReq::CMD_ID => { let packet = NetPacket:: <
        ::yanagi_proto::ModMainCityAvatarCsReq > ::decode($buf) ?; let rpc_arg =
        ::protocol::RpcModMainCityAvatarArg::from(packet.body); let rpc_ret :
        ::protocol::RpcModMainCityAvatarRet = $point .call_rpc($addr, rpc_arg,
        $middlewares, $timeout). await ?; let proto_rsp =
        ::yanagi_proto::ModMainCityAvatarScRsp::from(rpc_ret); $session .send_rsp(packet
        .head.packet_id, proto_rsp); }, GetCafeDataCsReq::CMD_ID => { let packet =
        NetPacket:: < ::yanagi_proto::GetCafeDataCsReq > ::decode($buf) ?; let rpc_arg =
        ::protocol::RpcGetCafeDataArg::from(packet.body); let rpc_ret :
        ::protocol::RpcGetCafeDataRet = $point .call_rpc($addr, rpc_arg, $middlewares,
        $timeout). await ?; let proto_rsp =
        ::yanagi_proto::GetCafeDataScRsp::from(rpc_ret); $session .send_rsp(packet.head
        .packet_id, proto_rsp); }, GetAvatarDataCsReq::CMD_ID => { let packet =
        NetPacket:: < ::yanagi_proto::GetAvatarDataCsReq > ::decode($buf) ?; let rpc_arg
        = ::protocol::RpcGetAvatarDataArg::from(packet.body); let rpc_ret :
        ::protocol::RpcGetAvatarDataRet = $point .call_rpc($addr, rpc_arg, $middlewares,
        $timeout). await ?; let proto_rsp =
        ::yanagi_proto::GetAvatarDataScRsp::from(rpc_ret); $session .send_rsp(packet.head
        .packet_id, proto_rsp); }, GetBabelTowerDataCsReq::CMD_ID => { let packet =
        NetPacket:: < ::yanagi_proto::GetBabelTowerDataCsReq > ::decode($buf) ?; let
        rpc_arg = ::protocol::RpcGetBabelTowerDataArg::from(packet.body); let rpc_ret :
        ::protocol::RpcGetBabelTowerDataRet = $point .call_rpc($addr, rpc_arg,
        $middlewares, $timeout). await ?; $session .send_null_rsp(packet.head.packet_id);
        }, GetBuddyDataCsReq::CMD_ID => { let packet = NetPacket:: <
        ::yanagi_proto::GetBuddyDataCsReq > ::decode($buf) ?; let rpc_arg =
        ::protocol::RpcGetBuddyDataArg::from(packet.body); let rpc_ret :
        ::protocol::RpcGetBuddyDataRet = $point .call_rpc($addr, rpc_arg, $middlewares,
        $timeout). await ?; let proto_rsp =
        ::yanagi_proto::GetBuddyDataScRsp::from(rpc_ret); $session .send_rsp(packet.head
        .packet_id, proto_rsp); }, PlayerOperationCsReq::CMD_ID => { let packet =
        NetPacket:: < ::yanagi_proto::PlayerOperationCsReq > ::decode($buf) ?; let
        rpc_arg = ::protocol::RpcPlayerOperationArg::from(packet.body); let rpc_ret :
        ::protocol::RpcPlayerOperationRet = $point .call_rpc($addr, rpc_arg,
        $middlewares, $timeout). await ?; $session .send_null_rsp(packet.head.packet_id);
        }, PlayerLoginCsReq::CMD_ID => { let packet = NetPacket:: <
        ::yanagi_proto::PlayerLoginCsReq > ::decode($buf) ?; let rpc_arg =
        ::protocol::RpcPlayerLoginArg::from(packet.body); let rpc_ret :
        ::protocol::RpcPlayerLoginRet = $point .call_rpc($addr, rpc_arg, $middlewares,
        $timeout). await ?; let proto_rsp =
        ::yanagi_proto::PlayerLoginScRsp::from(rpc_ret); $session .send_rsp(packet.head
        .packet_id, proto_rsp); }, GetClientSystemsInfoCsReq::CMD_ID => { let packet =
        NetPacket:: < ::yanagi_proto::GetClientSystemsInfoCsReq > ::decode($buf) ?; let
        rpc_arg = ::protocol::RpcGetClientSystemsInfoArg::from(packet.body); let rpc_ret
        : ::protocol::RpcGetClientSystemsInfoRet = $point .call_rpc($addr, rpc_arg,
        $middlewares, $timeout). await ?; let proto_rsp =
        ::yanagi_proto::GetClientSystemsInfoScRsp::from(rpc_ret); $session
        .send_rsp(packet.head.packet_id, proto_rsp); }, GetPlayerNetworkDataCsReq::CMD_ID
        => { let packet = NetPacket:: < ::yanagi_proto::GetPlayerNetworkDataCsReq >
        ::decode($buf) ?; let rpc_arg =
        ::protocol::RpcGetPlayerNetworkDataArg::from(packet.body); let rpc_ret :
        ::protocol::RpcGetPlayerNetworkDataRet = $point .call_rpc($addr, rpc_arg,
        $middlewares, $timeout). await ?; let proto_rsp =
        ::yanagi_proto::GetPlayerNetworkDataScRsp::from(rpc_ret); $session
        .send_rsp(packet.head.packet_id, proto_rsp); }, WeaponUnDressCsReq::CMD_ID => {
        let packet = NetPacket:: < ::yanagi_proto::WeaponUnDressCsReq > ::decode($buf) ?;
        let rpc_arg = ::protocol::RpcWeaponUnDressArg::from(packet.body); let rpc_ret :
        ::protocol::RpcWeaponUnDressRet = $point .call_rpc($addr, rpc_arg, $middlewares,
        $timeout). await ?; $session .send_null_rsp(packet.head.packet_id); },
        RefreshSectionCsReq::CMD_ID => { let packet = NetPacket:: <
        ::yanagi_proto::RefreshSectionCsReq > ::decode($buf) ?; let rpc_arg =
        ::protocol::RpcRefreshSectionArg::from(packet.body); let rpc_ret :
        ::protocol::RpcRefreshSectionRet = $point .call_rpc($addr, rpc_arg, $middlewares,
        $timeout). await ?; let proto_rsp =
        ::yanagi_proto::RefreshSectionScRsp::from(rpc_ret); $session .send_rsp(packet
        .head.packet_id, proto_rsp); }, PlayerTransactionCsReq::CMD_ID => { let packet =
        NetPacket:: < ::yanagi_proto::PlayerTransactionCsReq > ::decode($buf) ?; let
        rpc_arg = ::protocol::RpcPlayerTransactionArg::from(packet.body); let rpc_ret :
        ::protocol::RpcPlayerTransactionRet = $point .call_rpc($addr, rpc_arg,
        $middlewares, $timeout). await ?; $session .send_null_rsp(packet.head.packet_id);
        }, GetAbyssArpeggioDataCsReq::CMD_ID => { let packet = NetPacket:: <
        ::yanagi_proto::GetAbyssArpeggioDataCsReq > ::decode($buf) ?; let rpc_arg =
        ::protocol::RpcGetAbyssArpeggioDataArg::from(packet.body); let rpc_ret :
        ::protocol::RpcGetAbyssArpeggioDataRet = $point .call_rpc($addr, rpc_arg,
        $middlewares, $timeout). await ?; let proto_rsp =
        ::yanagi_proto::GetAbyssArpeggioDataScRsp::from(rpc_ret); $session
        .send_rsp(packet.head.packet_id, proto_rsp); }, GetEmbattlesDataCsReq::CMD_ID =>
        { let packet = NetPacket:: < ::yanagi_proto::GetEmbattlesDataCsReq >
        ::decode($buf) ?; let rpc_arg = ::protocol::RpcGetEmbattlesDataArg::from(packet
        .body); let rpc_ret : ::protocol::RpcGetEmbattlesDataRet = $point
        .call_rpc($addr, rpc_arg, $middlewares, $timeout). await ?; $session
        .send_null_rsp(packet.head.packet_id); }, GetCharacterQuestListCsReq::CMD_ID => {
        let packet = NetPacket:: < ::yanagi_proto::GetCharacterQuestListCsReq >
        ::decode($buf) ?; let rpc_arg =
        ::protocol::RpcGetCharacterQuestListArg::from(packet.body); let rpc_ret :
        ::protocol::RpcGetCharacterQuestListRet = $point .call_rpc($addr, rpc_arg,
        $middlewares, $timeout). await ?; $session .send_null_rsp(packet.head.packet_id);
        }, GetMonthCardRewardListCsReq::CMD_ID => { let packet = NetPacket:: <
        ::yanagi_proto::GetMonthCardRewardListCsReq > ::decode($buf) ?; let rpc_arg =
        ::protocol::RpcGetMonthCardRewardListArg::from(packet.body); let rpc_ret :
        ::protocol::RpcGetMonthCardRewardListRet = $point .call_rpc($addr, rpc_arg,
        $middlewares, $timeout). await ?; $session .send_null_rsp(packet.head.packet_id);
        }, GetHadalZoneDataCsReq::CMD_ID => { let packet = NetPacket:: <
        ::yanagi_proto::GetHadalZoneDataCsReq > ::decode($buf) ?; let rpc_arg =
        ::protocol::RpcGetHadalZoneDataArg::from(packet.body); let rpc_ret :
        ::protocol::RpcGetHadalZoneDataRet = $point .call_rpc($addr, rpc_arg,
        $middlewares, $timeout). await ?; $session .send_null_rsp(packet.head.packet_id);
        }, GetWorkbenchInfoCsReq::CMD_ID => { let packet = NetPacket:: <
        ::yanagi_proto::GetWorkbenchInfoCsReq > ::decode($buf) ?; let rpc_arg =
        ::protocol::RpcGetWorkbenchInfoArg::from(packet.body); let rpc_ret :
        ::protocol::RpcGetWorkbenchInfoRet = $point .call_rpc($addr, rpc_arg,
        $middlewares, $timeout). await ?; $session .send_null_rsp(packet.head.packet_id);
        }, GetTrashbinHermitDataCsReq::CMD_ID => { let packet = NetPacket:: <
        ::yanagi_proto::GetTrashbinHermitDataCsReq > ::decode($buf) ?; let rpc_arg =
        ::protocol::RpcGetTrashbinHermitDataArg::from(packet.body); let rpc_ret :
        ::protocol::RpcGetTrashbinHermitDataRet = $point .call_rpc($addr, rpc_arg,
        $middlewares, $timeout). await ?; let proto_rsp =
        ::yanagi_proto::GetTrashbinHermitDataScRsp::from(rpc_ret); $session
        .send_rsp(packet.head.packet_id, proto_rsp); }, ReportEmbattleInfoCsReq::CMD_ID
        => { let packet = NetPacket:: < ::yanagi_proto::ReportEmbattleInfoCsReq >
        ::decode($buf) ?; let rpc_arg = ::protocol::RpcReportEmbattleInfoArg::from(packet
        .body); let rpc_ret : ::protocol::RpcReportEmbattleInfoRet = $point
        .call_rpc($addr, rpc_arg, $middlewares, $timeout). await ?; $session
        .send_null_rsp(packet.head.packet_id); }, GetMiniscapeEntrustDataCsReq::CMD_ID =>
        { let packet = NetPacket:: < ::yanagi_proto::GetMiniscapeEntrustDataCsReq >
        ::decode($buf) ?; let rpc_arg =
        ::protocol::RpcGetMiniscapeEntrustDataArg::from(packet.body); let rpc_ret :
        ::protocol::RpcGetMiniscapeEntrustDataRet = $point .call_rpc($addr, rpc_arg,
        $middlewares, $timeout). await ?; let proto_rsp =
        ::yanagi_proto::GetMiniscapeEntrustDataScRsp::from(rpc_ret); $session
        .send_rsp(packet.head.packet_id, proto_rsp); }, EnterSectionCsReq::CMD_ID => {
        let packet = NetPacket:: < ::yanagi_proto::EnterSectionCsReq > ::decode($buf) ?;
        let rpc_arg = ::protocol::RpcEnterSectionArg::from(packet.body); let rpc_ret :
        ::protocol::RpcEnterSectionRet = $point .call_rpc($addr, rpc_arg, $middlewares,
        $timeout). await ?; $session .send_null_rsp(packet.head.packet_id); },
        GetResourceDataCsReq::CMD_ID => { let packet = NetPacket:: <
        ::yanagi_proto::GetResourceDataCsReq > ::decode($buf) ?; let rpc_arg =
        ::protocol::RpcGetResourceDataArg::from(packet.body); let rpc_ret :
        ::protocol::RpcGetResourceDataRet = $point .call_rpc($addr, rpc_arg,
        $middlewares, $timeout). await ?; let proto_rsp =
        ::yanagi_proto::GetResourceDataScRsp::from(rpc_ret); $session .send_rsp(packet
        .head.packet_id, proto_rsp); }, GetRoleCardDataCsReq::CMD_ID => { let packet =
        NetPacket:: < ::yanagi_proto::GetRoleCardDataCsReq > ::decode($buf) ?; let
        rpc_arg = ::protocol::RpcGetRoleCardDataArg::from(packet.body); let rpc_ret :
        ::protocol::RpcGetRoleCardDataRet = $point .call_rpc($addr, rpc_arg,
        $middlewares, $timeout). await ?; $session .send_null_rsp(packet.head.packet_id);
        }, GetPhotoWallDataCsReq::CMD_ID => { let packet = NetPacket:: <
        ::yanagi_proto::GetPhotoWallDataCsReq > ::decode($buf) ?; let rpc_arg =
        ::protocol::RpcGetPhotoWallDataArg::from(packet.body); let rpc_ret :
        ::protocol::RpcGetPhotoWallDataRet = $point .call_rpc($addr, rpc_arg,
        $middlewares, $timeout). await ?; let proto_rsp =
        ::yanagi_proto::GetPhotoWallDataScRsp::from(rpc_ret); $session .send_rsp(packet
        .head.packet_id, proto_rsp); }, GetGachaDataCsReq::CMD_ID => { let packet =
        NetPacket:: < ::yanagi_proto::GetGachaDataCsReq > ::decode($buf) ?; let rpc_arg =
        ::protocol::RpcGetGachaDataArg::from(packet.body); let rpc_ret :
        ::protocol::RpcGetGachaDataRet = $point .call_rpc($addr, rpc_arg, $middlewares,
        $timeout). await ?; let proto_rsp =
        ::yanagi_proto::GetGachaDataScRsp::from(rpc_ret); $session .send_rsp(packet.head
        .packet_id, proto_rsp); }, GetArcadeDataCsReq::CMD_ID => { let packet =
        NetPacket:: < ::yanagi_proto::GetArcadeDataCsReq > ::decode($buf) ?; let rpc_arg
        = ::protocol::RpcGetArcadeDataArg::from(packet.body); let rpc_ret :
        ::protocol::RpcGetArcadeDataRet = $point .call_rpc($addr, rpc_arg, $middlewares,
        $timeout). await ?; $session .send_null_rsp(packet.head.packet_id); },
        EnterSectionCompleteCsReq::CMD_ID => { let packet = NetPacket:: <
        ::yanagi_proto::EnterSectionCompleteCsReq > ::decode($buf) ?; let rpc_arg =
        ::protocol::RpcEnterSectionCompleteArg::from(packet.body); let rpc_ret :
        ::protocol::RpcEnterSectionCompleteRet = $point .call_rpc($addr, rpc_arg,
        $middlewares, $timeout). await ?; let proto_rsp =
        ::yanagi_proto::EnterSectionCompleteScRsp::from(rpc_ret); $session
        .send_rsp(packet.head.packet_id, proto_rsp); }, GetCampIdleDataCsReq::CMD_ID => {
        let packet = NetPacket:: < ::yanagi_proto::GetCampIdleDataCsReq > ::decode($buf)
        ?; let rpc_arg = ::protocol::RpcGetCampIdleDataArg::from(packet.body); let
        rpc_ret : ::protocol::RpcGetCampIdleDataRet = $point .call_rpc($addr, rpc_arg,
        $middlewares, $timeout). await ?; let proto_rsp =
        ::yanagi_proto::GetCampIdleDataScRsp::from(rpc_ret); $session .send_rsp(packet
        .head.packet_id, proto_rsp); }, GetPlayerMailsCsReq::CMD_ID => { let packet =
        NetPacket:: < ::yanagi_proto::GetPlayerMailsCsReq > ::decode($buf) ?; let rpc_arg
        = ::protocol::RpcGetPlayerMailsArg::from(packet.body); let rpc_ret :
        ::protocol::RpcGetPlayerMailsRet = $point .call_rpc($addr, rpc_arg, $middlewares,
        $timeout). await ?; let proto_rsp =
        ::yanagi_proto::GetPlayerMailsScRsp::from(rpc_ret); $session .send_rsp(packet
        .head.packet_id, proto_rsp); }, EndBattleCsReq::CMD_ID => { let packet =
        NetPacket:: < ::yanagi_proto::EndBattleCsReq > ::decode($buf) ?; let rpc_arg =
        ::protocol::RpcEndBattleArg::from(packet.body); let rpc_ret :
        ::protocol::RpcEndBattleRet = $point .call_rpc($addr, rpc_arg, $middlewares,
        $timeout). await ?; let proto_rsp =
        ::yanagi_proto::EndBattleScRsp::from(rpc_ret); $session .send_rsp(packet.head
        .packet_id, proto_rsp); }, InteractWithUnitCsReq::CMD_ID => { let packet =
        NetPacket:: < ::yanagi_proto::InteractWithUnitCsReq > ::decode($buf) ?; let
        rpc_arg = ::protocol::RpcInteractWithUnitArg::from(packet.body); let rpc_ret :
        ::protocol::RpcInteractWithUnitRet = $point .call_rpc($addr, rpc_arg,
        $middlewares, $timeout). await ?; let proto_rsp =
        ::yanagi_proto::InteractWithUnitScRsp::from(rpc_ret); $session .send_rsp(packet
        .head.packet_id, proto_rsp); }, GetFashionStoreInfoCsReq::CMD_ID => { let packet
        = NetPacket:: < ::yanagi_proto::GetFashionStoreInfoCsReq > ::decode($buf) ?; let
        rpc_arg = ::protocol::RpcGetFashionStoreInfoArg::from(packet.body); let rpc_ret :
        ::protocol::RpcGetFashionStoreInfoRet = $point .call_rpc($addr, rpc_arg,
        $middlewares, $timeout). await ?; $session .send_null_rsp(packet.head.packet_id);
        }, GetFriendListCsReq::CMD_ID => { let packet = NetPacket:: <
        ::yanagi_proto::GetFriendListCsReq > ::decode($buf) ?; let rpc_arg =
        ::protocol::RpcGetFriendListArg::from(packet.body); let rpc_ret :
        ::protocol::RpcGetFriendListRet = $point .call_rpc($addr, rpc_arg, $middlewares,
        $timeout). await ?; $session .send_null_rsp(packet.head.packet_id); },
        GetActivityDataCsReq::CMD_ID => { let packet = NetPacket:: <
        ::yanagi_proto::GetActivityDataCsReq > ::decode($buf) ?; let rpc_arg =
        ::protocol::RpcGetActivityDataArg::from(packet.body); let rpc_ret :
        ::protocol::RpcGetActivityDataRet = $point .call_rpc($addr, rpc_arg,
        $middlewares, $timeout). await ?; let proto_rsp =
        ::yanagi_proto::GetActivityDataScRsp::from(rpc_ret); $session .send_rsp(packet
        .head.packet_id, proto_rsp); }, GetRechargeItemListCsReq::CMD_ID => { let packet
        = NetPacket:: < ::yanagi_proto::GetRechargeItemListCsReq > ::decode($buf) ?; let
        rpc_arg = ::protocol::RpcGetRechargeItemListArg::from(packet.body); let rpc_ret :
        ::protocol::RpcGetRechargeItemListRet = $point .call_rpc($addr, rpc_arg,
        $middlewares, $timeout). await ?; $session .send_null_rsp(packet.head.packet_id);
        }, GetServerTimestampCsReq::CMD_ID => { let packet = NetPacket:: <
        ::yanagi_proto::GetServerTimestampCsReq > ::decode($buf) ?; let rpc_arg =
        ::protocol::RpcGetServerTimestampArg::from(packet.body); let rpc_ret :
        ::protocol::RpcGetServerTimestampRet = $point .call_rpc($addr, rpc_arg,
        $middlewares, $timeout). await ?; let proto_rsp =
        ::yanagi_proto::GetServerTimestampScRsp::from(rpc_ret); $session .send_rsp(packet
        .head.packet_id, proto_rsp); }, SceneTransitionCsReq::CMD_ID => { let packet =
        NetPacket:: < ::yanagi_proto::SceneTransitionCsReq > ::decode($buf) ?; let
        rpc_arg = ::protocol::RpcSceneTransitionArg::from(packet.body); let rpc_ret :
        ::protocol::RpcSceneTransitionRet = $point .call_rpc($addr, rpc_arg,
        $middlewares, $timeout). await ?; $session .send_null_rsp(packet.head.packet_id);
        }, ModTimeCsReq::CMD_ID => { let packet = NetPacket:: <
        ::yanagi_proto::ModTimeCsReq > ::decode($buf) ?; let rpc_arg =
        ::protocol::RpcModTimeArg::from(packet.body); let rpc_ret :
        ::protocol::RpcModTimeRet = $point .call_rpc($addr, rpc_arg, $middlewares,
        $timeout). await ?; $session .send_null_rsp(packet.head.packet_id); },
        GetEquipDataCsReq::CMD_ID => { let packet = NetPacket:: <
        ::yanagi_proto::GetEquipDataCsReq > ::decode($buf) ?; let rpc_arg =
        ::protocol::RpcGetEquipDataArg::from(packet.body); let rpc_ret :
        ::protocol::RpcGetEquipDataRet = $point .call_rpc($addr, rpc_arg, $middlewares,
        $timeout). await ?; let proto_rsp =
        ::yanagi_proto::GetEquipDataScRsp::from(rpc_ret); $session .send_rsp(packet.head
        .packet_id, proto_rsp); }, SavePlayerSystemSettingCsReq::CMD_ID => { let packet =
        NetPacket:: < ::yanagi_proto::SavePlayerSystemSettingCsReq > ::decode($buf) ?;
        let rpc_arg = ::protocol::RpcSavePlayerSystemSettingArg::from(packet.body); let
        rpc_ret : ::protocol::RpcSavePlayerSystemSettingRet = $point .call_rpc($addr,
        rpc_arg, $middlewares, $timeout). await ?; $session .send_null_rsp(packet.head
        .packet_id); }, GetRewardBuffDataCsReq::CMD_ID => { let packet = NetPacket:: <
        ::yanagi_proto::GetRewardBuffDataCsReq > ::decode($buf) ?; let rpc_arg =
        ::protocol::RpcGetRewardBuffDataArg::from(packet.body); let rpc_ret :
        ::protocol::RpcGetRewardBuffDataRet = $point .call_rpc($addr, rpc_arg,
        $middlewares, $timeout). await ?; let proto_rsp =
        ::yanagi_proto::GetRewardBuffDataScRsp::from(rpc_ret); $session .send_rsp(packet
        .head.packet_id, proto_rsp); }, EnterWorldCsReq::CMD_ID => { let packet =
        NetPacket:: < ::yanagi_proto::EnterWorldCsReq > ::decode($buf) ?; let rpc_arg =
        ::protocol::RpcEnterWorldArg::from(packet.body); let rpc_ret :
        ::protocol::RpcEnterWorldRet = $point .call_rpc($addr, rpc_arg, $middlewares,
        $timeout). await ?; let proto_rsp =
        ::yanagi_proto::EnterWorldScRsp::from(rpc_ret); $session .send_rsp(packet.head
        .packet_id, proto_rsp); }, GetQuestDataCsReq::CMD_ID => { let packet =
        NetPacket:: < ::yanagi_proto::GetQuestDataCsReq > ::decode($buf) ?; let rpc_arg =
        ::protocol::RpcGetQuestDataArg::from(packet.body); let rpc_ret :
        ::protocol::RpcGetQuestDataRet = $point .call_rpc($addr, rpc_arg, $middlewares,
        $timeout). await ?; let proto_rsp =
        ::yanagi_proto::GetQuestDataScRsp::from(rpc_ret); $session .send_rsp(packet.head
        .packet_id, proto_rsp); }, GetWebActivityDataCsReq::CMD_ID => { let packet =
        NetPacket:: < ::yanagi_proto::GetWebActivityDataCsReq > ::decode($buf) ?; let
        rpc_arg = ::protocol::RpcGetWebActivityDataArg::from(packet.body); let rpc_ret :
        ::protocol::RpcGetWebActivityDataRet = $point .call_rpc($addr, rpc_arg,
        $middlewares, $timeout). await ?; let proto_rsp =
        ::yanagi_proto::GetWebActivityDataScRsp::from(rpc_ret); $session .send_rsp(packet
        .head.packet_id, proto_rsp); }, _ => ::tracing::warn!("unknown cmd_id: {}",
        $cmd_id), }
    };
}
#[macro_export]
macro_rules! impl_qwer_to_proto_match {
    ($process_proto_message:ident) => {
        match qwer.get_protocol_id() { ::protocol::RpcGetAbyssRewardDataArg::PROTOCOL_ID
        => $process_proto_message (GetAbyssRewardDataCsReq::from(qwer)),
        ::protocol::RpcGetExplorationDataArg::PROTOCOL_ID => $process_proto_message
        (GetExplorationDataCsReq::from(qwer)),
        ::protocol::RpcBeginTrainingCourseBattleArg::PROTOCOL_ID =>
        $process_proto_message (BeginTrainingCourseBattleCsReq::from(qwer)),
        ::protocol::RpcSavePosInMainCityArg::PROTOCOL_ID => $process_proto_message
        (SavePosInMainCityCsReq::from(qwer)),
        ::protocol::RpcGetVhsStoreInfoArg::PROTOCOL_ID => $process_proto_message
        (GetVhsStoreInfoCsReq::from(qwer)), ::protocol::RpcGetAbyssInfoArg::PROTOCOL_ID
        => $process_proto_message (GetAbyssInfoCsReq::from(qwer)),
        ::protocol::RpcGetPrivateMessageDataArg::PROTOCOL_ID => $process_proto_message
        (GetPrivateMessageDataCsReq::from(qwer)),
        ::protocol::RpcGetNewsStandDataArg::PROTOCOL_ID => $process_proto_message
        (GetNewsStandDataCsReq::from(qwer)),
        ::protocol::RpcGetVideoUsmKeyDataArg::PROTOCOL_ID => $process_proto_message
        (GetVideoUsmKeyDataCsReq::from(qwer)),
        ::protocol::RpcGetPlayerBasicInfoArg::PROTOCOL_ID => $process_proto_message
        (GetPlayerBasicInfoCsReq::from(qwer)), ::protocol::RpcGetAuthkeyArg::PROTOCOL_ID
        => $process_proto_message (GetAuthkeyCsReq::from(qwer)),
        ::protocol::RpcGetRamenDataArg::PROTOCOL_ID => $process_proto_message
        (GetRamenDataCsReq::from(qwer)), ::protocol::RpcGetFairyInfoArg::PROTOCOL_ID =>
        $process_proto_message (GetFairyInfoCsReq::from(qwer)),
        ::protocol::RpcGetCollectMapArg::PROTOCOL_ID => $process_proto_message
        (GetCollectMapCsReq::from(qwer)),
        ::protocol::RpcInteractWithClientEntityArg::PROTOCOL_ID => $process_proto_message
        (InteractWithClientEntityCsReq::from(qwer)),
        ::protocol::RpcGetShoppingMallInfoArg::PROTOCOL_ID => $process_proto_message
        (GetShoppingMallInfoCsReq::from(qwer)),
        ::protocol::RpcGetTipsInfoArg::PROTOCOL_ID => $process_proto_message
        (GetTipsInfoCsReq::from(qwer)), ::protocol::RpcGetYorozuyaInfoArg::PROTOCOL_ID =>
        $process_proto_message (GetYorozuyaInfoCsReq::from(qwer)),
        ::protocol::RpcReportUiLayoutPlatformArg::PROTOCOL_ID => $process_proto_message
        (ReportUiLayoutPlatformCsReq::from(qwer)),
        ::protocol::RpcBattleReportArg::PROTOCOL_ID => $process_proto_message
        (BattleReportCsReq::from(qwer)),
        ::protocol::RpcGetDisplayCaseDataArg::PROTOCOL_ID => $process_proto_message
        (GetDisplayCaseDataCsReq::from(qwer)),
        ::protocol::RpcLeaveCurDungeonArg::PROTOCOL_ID => $process_proto_message
        (LeaveCurDungeonCsReq::from(qwer)),
        ::protocol::PtcUpdateEventGraphArg::PROTOCOL_ID => $process_proto_message
        (UpdateEventGraphScNotify::from(qwer)),
        ::protocol::RpcGetBattlePassDataArg::PROTOCOL_ID => $process_proto_message
        (GetBattlePassDataCsReq::from(qwer)),
        ::protocol::RpcGetArchiveInfoArg::PROTOCOL_ID => $process_proto_message
        (GetArchiveInfoCsReq::from(qwer)),
        ::protocol::RpcGetDailyChallengeInfoArg::PROTOCOL_ID => $process_proto_message
        (GetDailyChallengeInfoCsReq::from(qwer)),
        ::protocol::RpcRunEventGraphArg::PROTOCOL_ID => $process_proto_message
        (RunEventGraphCsReq::from(qwer)), ::protocol::RpcGetJourneyInfoArg::PROTOCOL_ID
        => $process_proto_message (GetJourneyInfoCsReq::from(qwer)),
        ::protocol::RpcGetMainCityRevivalDataArg::PROTOCOL_ID => $process_proto_message
        (GetMainCityRevivalDataCsReq::from(qwer)),
        ::protocol::RpcGetWishlistDataArg::PROTOCOL_ID => $process_proto_message
        (GetWishlistDataCsReq::from(qwer)),
        ::protocol::RpcCheckYorozuyaInfoRefreshArg::PROTOCOL_ID => $process_proto_message
        (CheckYorozuyaInfoRefreshCsReq::from(qwer)),
        ::protocol::RpcGetWeaponDataArg::PROTOCOL_ID => $process_proto_message
        (GetWeaponDataCsReq::from(qwer)), ::protocol::RpcGetChatEmojiListArg::PROTOCOL_ID
        => $process_proto_message (GetChatEmojiListCsReq::from(qwer)),
        ::protocol::RpcWeaponDressArg::PROTOCOL_ID => $process_proto_message
        (WeaponDressCsReq::from(qwer)), ::protocol::RpcModMainCityAvatarArg::PROTOCOL_ID
        => $process_proto_message (ModMainCityAvatarCsReq::from(qwer)),
        ::protocol::RpcGetCafeDataArg::PROTOCOL_ID => $process_proto_message
        (GetCafeDataCsReq::from(qwer)), ::protocol::RpcGetAvatarDataArg::PROTOCOL_ID =>
        $process_proto_message (GetAvatarDataCsReq::from(qwer)),
        ::protocol::RpcGetBabelTowerDataArg::PROTOCOL_ID => $process_proto_message
        (GetBabelTowerDataCsReq::from(qwer)), ::protocol::PtcEnterSceneArg::PROTOCOL_ID
        => $process_proto_message (EnterSceneScNotify::from(qwer)),
        ::protocol::RpcGetBuddyDataArg::PROTOCOL_ID => $process_proto_message
        (GetBuddyDataCsReq::from(qwer)), ::protocol::RpcPlayerOperationArg::PROTOCOL_ID
        => $process_proto_message (PlayerOperationCsReq::from(qwer)),
        ::protocol::PtcPlayerSyncArg::PROTOCOL_ID => $process_proto_message
        (PlayerSyncScNotify::from(qwer)), ::protocol::PtcHallRefreshArg::PROTOCOL_ID =>
        $process_proto_message (HallRefreshScNotify::from(qwer)),
        ::protocol::RpcPlayerLoginArg::PROTOCOL_ID => $process_proto_message
        (PlayerLoginCsReq::from(qwer)),
        ::protocol::RpcGetClientSystemsInfoArg::PROTOCOL_ID => $process_proto_message
        (GetClientSystemsInfoCsReq::from(qwer)),
        ::protocol::RpcGetPlayerNetworkDataArg::PROTOCOL_ID => $process_proto_message
        (GetPlayerNetworkDataCsReq::from(qwer)),
        ::protocol::RpcWeaponUnDressArg::PROTOCOL_ID => $process_proto_message
        (WeaponUnDressCsReq::from(qwer)), ::protocol::RpcRefreshSectionArg::PROTOCOL_ID
        => $process_proto_message (RefreshSectionCsReq::from(qwer)),
        ::protocol::RpcPlayerTransactionArg::PROTOCOL_ID => $process_proto_message
        (PlayerTransactionCsReq::from(qwer)),
        ::protocol::RpcGetAbyssArpeggioDataArg::PROTOCOL_ID => $process_proto_message
        (GetAbyssArpeggioDataCsReq::from(qwer)),
        ::protocol::RpcGetEmbattlesDataArg::PROTOCOL_ID => $process_proto_message
        (GetEmbattlesDataCsReq::from(qwer)),
        ::protocol::RpcGetCharacterQuestListArg::PROTOCOL_ID => $process_proto_message
        (GetCharacterQuestListCsReq::from(qwer)),
        ::protocol::RpcGetMonthCardRewardListArg::PROTOCOL_ID => $process_proto_message
        (GetMonthCardRewardListCsReq::from(qwer)),
        ::protocol::RpcGetHadalZoneDataArg::PROTOCOL_ID => $process_proto_message
        (GetHadalZoneDataCsReq::from(qwer)),
        ::protocol::RpcGetWorkbenchInfoArg::PROTOCOL_ID => $process_proto_message
        (GetWorkbenchInfoCsReq::from(qwer)),
        ::protocol::RpcGetTrashbinHermitDataArg::PROTOCOL_ID => $process_proto_message
        (GetTrashbinHermitDataCsReq::from(qwer)),
        ::protocol::RpcReportEmbattleInfoArg::PROTOCOL_ID => $process_proto_message
        (ReportEmbattleInfoCsReq::from(qwer)),
        ::protocol::RpcGetMiniscapeEntrustDataArg::PROTOCOL_ID => $process_proto_message
        (GetMiniscapeEntrustDataCsReq::from(qwer)),
        ::protocol::RpcEnterSectionArg::PROTOCOL_ID => $process_proto_message
        (EnterSectionCsReq::from(qwer)), ::protocol::RpcGetResourceDataArg::PROTOCOL_ID
        => $process_proto_message (GetResourceDataCsReq::from(qwer)),
        ::protocol::RpcGetRoleCardDataArg::PROTOCOL_ID => $process_proto_message
        (GetRoleCardDataCsReq::from(qwer)),
        ::protocol::RpcGetPhotoWallDataArg::PROTOCOL_ID => $process_proto_message
        (GetPhotoWallDataCsReq::from(qwer)), ::protocol::PtcSyncEventInfoArg::PROTOCOL_ID
        => $process_proto_message (SyncEventInfoScNotify::from(qwer)),
        ::protocol::RpcGetGachaDataArg::PROTOCOL_ID => $process_proto_message
        (GetGachaDataCsReq::from(qwer)), ::protocol::RpcGetArcadeDataArg::PROTOCOL_ID =>
        $process_proto_message (GetArcadeDataCsReq::from(qwer)),
        ::protocol::RpcEnterSectionCompleteArg::PROTOCOL_ID => $process_proto_message
        (EnterSectionCompleteCsReq::from(qwer)),
        ::protocol::RpcGetCampIdleDataArg::PROTOCOL_ID => $process_proto_message
        (GetCampIdleDataCsReq::from(qwer)), ::protocol::RpcGetPlayerMailsArg::PROTOCOL_ID
        => $process_proto_message (GetPlayerMailsCsReq::from(qwer)),
        ::protocol::RpcEndBattleArg::PROTOCOL_ID => $process_proto_message
        (EndBattleCsReq::from(qwer)), ::protocol::RpcInteractWithUnitArg::PROTOCOL_ID =>
        $process_proto_message (InteractWithUnitCsReq::from(qwer)),
        ::protocol::RpcGetFashionStoreInfoArg::PROTOCOL_ID => $process_proto_message
        (GetFashionStoreInfoCsReq::from(qwer)),
        ::protocol::RpcGetFriendListArg::PROTOCOL_ID => $process_proto_message
        (GetFriendListCsReq::from(qwer)), ::protocol::RpcGetActivityDataArg::PROTOCOL_ID
        => $process_proto_message (GetActivityDataCsReq::from(qwer)),
        ::protocol::RpcGetRechargeItemListArg::PROTOCOL_ID => $process_proto_message
        (GetRechargeItemListCsReq::from(qwer)),
        ::protocol::RpcGetServerTimestampArg::PROTOCOL_ID => $process_proto_message
        (GetServerTimestampCsReq::from(qwer)),
        ::protocol::RpcSceneTransitionArg::PROTOCOL_ID => $process_proto_message
        (SceneTransitionCsReq::from(qwer)), ::protocol::RpcModTimeArg::PROTOCOL_ID =>
        $process_proto_message (ModTimeCsReq::from(qwer)),
        ::protocol::PtcAddAvatarArg::PROTOCOL_ID => $process_proto_message
        (AddAvatarScNotify::from(qwer)), ::protocol::RpcGetEquipDataArg::PROTOCOL_ID =>
        $process_proto_message (GetEquipDataCsReq::from(qwer)),
        ::protocol::RpcSavePlayerSystemSettingArg::PROTOCOL_ID => $process_proto_message
        (SavePlayerSystemSettingCsReq::from(qwer)),
        ::protocol::RpcGetRewardBuffDataArg::PROTOCOL_ID => $process_proto_message
        (GetRewardBuffDataCsReq::from(qwer)), ::protocol::RpcEnterWorldArg::PROTOCOL_ID
        => $process_proto_message (EnterWorldCsReq::from(qwer)),
        ::protocol::RpcGetQuestDataArg::PROTOCOL_ID => $process_proto_message
        (GetQuestDataCsReq::from(qwer)),
        ::protocol::RpcGetWebActivityDataArg::PROTOCOL_ID => $process_proto_message
        (GetWebActivityDataCsReq::from(qwer)), _ => (), }
    };
}
#[macro_export]
macro_rules! register_ptc_handlers {
    ($point:ident, $conv:ident, $tx:ident) => {
        $point .register_rpc_recv(::protocol::PtcUpdateEventGraphArg::PROTOCOL_ID, move |
        ctx | async move { let _ = $tx .get().unwrap().send(Input::Notify($conv, ctx)).
        await; }); $point .register_rpc_recv(::protocol::PtcEnterSceneArg::PROTOCOL_ID,
        move | ctx | async move { let _ = $tx .get().unwrap().send(Input::Notify($conv,
        ctx)). await; }); $point
        .register_rpc_recv(::protocol::PtcPlayerSyncArg::PROTOCOL_ID, move | ctx | async
        move { let _ = $tx .get().unwrap().send(Input::Notify($conv, ctx)). await; });
        $point .register_rpc_recv(::protocol::PtcHallRefreshArg::PROTOCOL_ID, move | ctx
        | async move { let _ = $tx .get().unwrap().send(Input::Notify($conv, ctx)).
        await; }); $point
        .register_rpc_recv(::protocol::PtcSyncEventInfoArg::PROTOCOL_ID, move | ctx |
        async move { let _ = $tx .get().unwrap().send(Input::Notify($conv, ctx)). await;
        }); $point .register_rpc_recv(::protocol::PtcAddAvatarArg::PROTOCOL_ID, move |
        ctx | async move { let _ = $tx .get().unwrap().send(Input::Notify($conv, ctx)).
        await; });
    };
}
#[macro_export]
macro_rules! forward_as_notify {
    ($session:ident, $ctx:ident) => {
        match $ctx .protocol_id { ::protocol::PtcUpdateEventGraphArg::PROTOCOL_ID => {
        $session .notify(::yanagi_proto::UpdateEventGraphScNotify::from($ctx .get_arg:: <
        ::protocol::PtcUpdateEventGraphArg > ().unwrap(),)); },
        ::protocol::PtcEnterSceneArg::PROTOCOL_ID => { $session
        .notify(::yanagi_proto::EnterSceneScNotify::from($ctx .get_arg:: <
        ::protocol::PtcEnterSceneArg > ().unwrap(),)); },
        ::protocol::PtcPlayerSyncArg::PROTOCOL_ID => { $session
        .notify(::yanagi_proto::PlayerSyncScNotify::from($ctx .get_arg:: <
        ::protocol::PtcPlayerSyncArg > ().unwrap(),)); },
        ::protocol::PtcHallRefreshArg::PROTOCOL_ID => { $session
        .notify(::yanagi_proto::HallRefreshScNotify::from($ctx .get_arg:: <
        ::protocol::PtcHallRefreshArg > ().unwrap(),)); },
        ::protocol::PtcSyncEventInfoArg::PROTOCOL_ID => { $session
        .notify(::yanagi_proto::SyncEventInfoScNotify::from($ctx .get_arg:: <
        ::protocol::PtcSyncEventInfoArg > ().unwrap(),)); },
        ::protocol::PtcAddAvatarArg::PROTOCOL_ID => { $session
        .notify(::yanagi_proto::AddAvatarScNotify::from($ctx .get_arg:: <
        ::protocol::PtcAddAvatarArg > ().unwrap(),)); }, _ => (), }
    };
}
