use tracing::{debug, instrument};

use crate::level;

use super::*;

#[instrument(skip_all)]
pub async fn on_rpc_interact_with_client_entity_arg(
    _: &RpcPtcContext,
    _: &mut PlayerSession,
    arg: RpcInteractWithClientEntityArg,
) -> Result<RpcInteractWithClientEntityRet, i32> {
    debug!("{arg:?}");
    Ok(RpcInteractWithClientEntityRet::default())
}

#[instrument(skip_all)]
pub async fn on_rpc_interact_with_unit_arg(
    ctx: &RpcPtcContext,
    session: &mut PlayerSession,
    arg: RpcInteractWithUnitArg,
) -> Result<RpcInteractWithUnitRet, i32> {
    debug!("{arg:?}");

    session
        .level_event_graph_mgr
        .begin_interact(arg.interaction, arg.unit_tag);

    level::fire_event(session, arg.interaction, "OnInteract");
    session.level_event_graph_mgr.sync_event_info(ctx).await;

    Ok(RpcInteractWithUnitRet::default())
}

pub async fn on_rpc_run_event_graph_arg(
    ctx: &RpcPtcContext,
    _: &mut PlayerSession,
    arg: RpcRunEventGraphArg,
) -> Result<RpcRunEventGraphRet, i32> {
    ctx.send_ptc(PtcUpdateEventGraphArg {
        owner_type: arg.owner_type,
        tag: arg.tag,
        event_graph_uid: arg.event_graph_uid,
        npc_interaction: String::from("OnInteract"),
        is_event_success: true,
        event_graph_owner_uid: arg.owner_id,
    })
    .await;

    Ok(RpcRunEventGraphRet::default())
}
