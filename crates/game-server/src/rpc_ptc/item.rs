use super::*;

pub async fn on_rpc_get_weapon_data_arg(
    _: &RpcPtcContext,
    session: &mut PlayerSession,
    _: RpcGetWeaponDataArg,
) -> Result<RpcGetWeaponDataRet, i32> {
    Ok(RpcGetWeaponDataRet {
        retcode: 0,
        weapon_list: protocol::util::build_sync_weapon_info_list(&session.player_info),
    })
}

pub async fn on_rpc_get_equip_data_arg(
    _: &RpcPtcContext,
    session: &mut PlayerSession,
    _: RpcGetEquipDataArg,
) -> Result<RpcGetEquipDataRet, i32> {
    Ok(RpcGetEquipDataRet {
        retcode: 0,
        equip_list: protocol::util::build_sync_equip_info_list(&session.player_info),
    })
}

pub async fn on_rpc_get_resource_data_arg(
    _: &RpcPtcContext,
    session: &mut PlayerSession,
    _: RpcGetResourceDataArg,
) -> Result<RpcGetResourceDataRet, i32> {
    Ok(RpcGetResourceDataRet {
        retcode: 0,
        resource_list: protocol::util::build_sync_resource_info_list(&session.player_info),
        auto_recovery_info: session
            .player_info
            .auto_recovery_info
            .as_ref()
            .unwrap()
            .iter()
            .map(|(id, info)| (*id as u32, info.clone()))
            .collect(),
    })
}

pub async fn on_rpc_get_avatar_data_arg(
    _: &RpcPtcContext,
    session: &mut PlayerSession,
    _: RpcGetAvatarDataArg,
) -> Result<RpcGetAvatarDataRet, i32> {
    Ok(RpcGetAvatarDataRet {
        retcode: 0,
        avatar_list: protocol::util::build_sync_avatar_info_list(&session.player_info),
    })
}

pub async fn on_rpc_get_wishlist_data_arg(
    _: &RpcPtcContext,
    _: &mut PlayerSession,
    _: RpcGetWishlistDataArg,
) -> Result<RpcGetWishlistDataRet, i32> {
    Ok(RpcGetWishlistDataRet {
        retcode: 0,
        wishlist_plan_list: Vec::with_capacity(0),
    })
}

pub async fn on_rpc_get_buddy_data_arg(
    _: &RpcPtcContext,
    _: &mut PlayerSession,
    _: RpcGetBuddyDataArg,
) -> Result<RpcGetBuddyDataRet, i32> {
    Ok(RpcGetBuddyDataRet::default())
}
