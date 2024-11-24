use qwer_rpc::{middleware::MiddlewareModel, RpcPtcContext, RpcPtcPoint};

use crate::PlayerSession;
use paste::paste;
use protocol::*;
use qwer::*;

mod abyss;
mod activity;
mod arcade;
mod babel_tower;
mod battle_pass;
mod camp_idle;
mod daily_challenge;
mod embattles;
mod gacha;
mod hadal_zone;
mod interact;
mod item;
mod main_city;
mod player;
mod quest;
mod shop;
mod social;
mod unlock;
mod vhs_store;
mod world;

use abyss::*;
use activity::*;
use arcade::*;
use babel_tower::*;
use battle_pass::*;
use camp_idle::*;
use daily_challenge::*;
use embattles::*;
use gacha::*;
use hadal_zone::*;
use interact::*;
use item::*;
use main_city::*;
use player::*;
use quest::*;
use shop::*;
use social::*;
use unlock::*;
use vhs_store::*;
use world::*;

macro_rules! rpc_handlers {
    (($rpc_ptc_point:ident) $($name:ident;)*) => {
        paste! {
            $(
                async fn [<_on_ $name:snake _arg>](ctx: ::qwer_rpc::RpcPtcContext) {
                    let Ok(arg) = ctx.get_arg::<::protocol::[<$name Arg>]>() else {
                        ::tracing::warn!("failed to unmarshal arg {}", stringify!($name));
                        return;
                    };

                    let Some(MiddlewareModel::Account(account_mw)) = ctx
                        .middleware_list
                        .iter()
                        .find(|&mw| matches!(mw, MiddlewareModel::Account(_)))
                    else {
                        ::tracing::warn!("failed to handle {}: account middleware is missing", stringify!($name));
                        return;
                    };

                    let Some(mut session) = crate::PLAYER_MAP.get_mut(&account_mw.player_uid) else {
                        ::tracing::warn!("failed to handle {}: player session with uid {} is not active", stringify!($name), account_mw.player_uid);
                        return;
                    };

                    match [<on_ $name:snake _arg>](&ctx, &mut session, arg).await {
                        Ok(ret) => {
                            ctx.send_ret(ret).await;
                            ::tracing::info!("successfully handled {}Arg", stringify!($name));
                        },
                        Err(retcode) => {
                            ::tracing::warn!("on_{}_arg returned error code: {}", stringify!([<$name:snake>]), retcode);
                            ctx.send_ret([<$name Ret>] {
                                retcode,
                                ..Default::default()
                            }).await;
                        }
                    }

                    crate::post_rpc_handle(&mut session).await;
                }
            )*
        }

        $(
            paste!($rpc_ptc_point.register_rpc_recv(::protocol::[<$name Arg>]::PROTOCOL_ID, [<_on_ $name:snake _arg>]));
        )*
    };
}

pub fn register_handlers(listen_point: &RpcPtcPoint) {
    rpc_handlers! {
        (listen_point)
        RpcGetPlayerBasicInfo;
        RpcGetWeaponData;
        RpcGetEquipData;
        RpcGetResourceData;
        RpcGetAvatarData;
        RpcGetWishlistData; // new 1.4
        RpcGetQuestData;
        RpcGetArchiveInfo;
        RpcGetYorozuyaInfo;
        RpcGetAbyssInfo;
        RpcGetBuddyData;
        RpcGetAbyssArpeggioData;
        RpcGetServerTimestamp;
        RpcGetVideoUsmKeyData;
        RpcGetAuthkey;
        RpcGetGachaData;
        RpcGetCampIdleData;
        RpcSavePlayerSystemSetting;
        RpcGetRamenData;
        RpcGetCafeData;
        RpcGetRewardBuffData;
        RpcGetPlayerMails;
        RpcGetFairyInfo;
        RpcGetTipsInfo;
        RpcGetClientSystemsInfo;
        RpcGetPrivateMessageData;
        RpcGetCollectMap;
        RpcGetWorkbenchInfo;
        RpcGetAbyssRewardData;
        RpcGetVhsStoreInfo;
        RpcGetActivityData;
        RpcGetWebActivityData;
        RpcGetEmbattlesData;
        RpcGetNewsStandData;
        RpcGetTrashbinHermitData;
        RpcGetMainCityRevivalData;
        RpcGetArcadeData;
        RpcGetBattlePassData;
        RpcGetHadalZoneData;
        RpcGetBabelTowerData;
        RpcGetDailyChallengeInfo;
        RpcGetRoleCardData;
        RpcGetChatEmojiList;
        RpcGetFriendList;
        RpcGetCharacterQuestList;
        RpcGetExplorationData;
        RpcGetFashionStoreInfo;
        RpcGetShoppingMallInfo;

        // new 1.4
        RpcGetMiniscapeEntrustData;
        RpcGetJourneyInfo;

        RpcGetOnlineFriendsList;
        RpcEnterWorld;

        RpcSceneTransition;
        RpcEnterSectionComplete;
        RpcReportEmbattleInfo;
        RpcGetMonthCardRewardList;
        RpcGetDisplayCaseData;
        RpcGetPhotoWallData;
        RpcSavePosInMainCity;
        RpcReportUiLayoutPlatform;
        RpcPlayerOperation;
        RpcPlayerTransaction;
        RpcGetRechargeItemList;

        RpcModTime;
        RpcModMainCityAvatar;
        RpcInteractWithClientEntity;
        RpcInteractWithUnit;
        RpcRunEventGraph;
        RpcEnterSection;
        RpcRefreshSection;

        RpcCheckYorozuyaInfoRefresh;
        RpcBeginTrainingCourseBattle;
        RpcBattleReport;
        RpcEndBattle;
        RpcLeaveCurDungeon;
    };
}
