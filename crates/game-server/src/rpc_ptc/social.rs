use super::*;

pub async fn on_rpc_get_friend_list_arg(
    _: &RpcPtcContext,
    _: &mut PlayerSession,
    _: RpcGetFriendListArg,
) -> Result<RpcGetFriendListRet, i32> {
    Ok(RpcGetFriendListRet::default())
}

pub async fn on_rpc_get_chat_emoji_list_arg(
    _: &RpcPtcContext,
    _: &mut PlayerSession,
    _: RpcGetChatEmojiListArg,
) -> Result<RpcGetChatEmojiListRet, i32> {
    Ok(RpcGetChatEmojiListRet::default())
}

pub async fn on_rpc_get_online_friends_list_arg(
    _: &RpcPtcContext,
    _: &mut PlayerSession,
    _: RpcGetOnlineFriendsListArg,
) -> Result<RpcGetOnlineFriendsListRet, i32> {
    Ok(RpcGetOnlineFriendsListRet::default())
}
