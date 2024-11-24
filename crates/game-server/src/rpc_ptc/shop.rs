use super::*;

pub async fn on_rpc_get_fashion_store_info_arg(
    _: &RpcPtcContext,
    _: &mut PlayerSession,
    _: RpcGetFashionStoreInfoArg,
) -> Result<RpcGetFashionStoreInfoRet, i32> {
    Ok(RpcGetFashionStoreInfoRet {
        retcode: 0,
        info: FashionStoreInfo::default(),
    })
}

pub async fn on_rpc_get_shopping_mall_info_arg(
    _: &RpcPtcContext,
    _: &mut PlayerSession,
    _: RpcGetShoppingMallInfoArg,
) -> Result<RpcGetShoppingMallInfoRet, i32> {
    Ok(RpcGetShoppingMallInfoRet {
        retcode: 0,
        shopping_mall_info: ShoppingMallInfo::default(),
    })
}

pub async fn on_rpc_get_recharge_item_list_arg(
    _: &RpcPtcContext,
    _: &mut PlayerSession,
    _: RpcGetRechargeItemListArg,
) -> Result<RpcGetRechargeItemListRet, i32> {
    Ok(RpcGetRechargeItemListRet::default())
}
