use protocol::player_info::{NpcInfo, NpcSceneData, Transform};
use qwer::{phashset, PropertyHashSet};
use tracing::debug;
use yanagi_eventgraph::{action_pb, ConfigEvent, ConfigEventAction, SectionEventGraphConfig};

use crate::{level::BoundInteractInfo, PlayerSession};

#[derive(Debug)]
pub enum EventGraphGroup {
    OnAdd,
    OnEnter,
}

pub fn trigger_group(
    session: &mut PlayerSession,
    config: &SectionEventGraphConfig,
    group: EventGraphGroup,
    scene_uid: u64,
    section_id: i32,
) {
    debug!(
        "[EventGraph] player {} triggered event group {group:?}",
        session.player_uid
    );

    match group {
        EventGraphGroup::OnAdd => config.on_add.iter().for_each(|event_name| {
            trigger_event(
                session,
                event_name,
                config.events.get(event_name).unwrap(),
                scene_uid,
                section_id,
            )
        }),
        EventGraphGroup::OnEnter => config.on_enter.iter().for_each(|event_name| {
            trigger_event(
                session,
                event_name,
                config.events.get(event_name).unwrap(),
                scene_uid,
                section_id,
            )
        }),
    }
}

pub fn trigger_event(
    session: &mut PlayerSession,
    event_name: &str,
    config: &ConfigEvent,
    scene_uid: u64,
    section_id: i32,
) {
    debug!(
        "[EventGraph] player {} triggered event {event_name} (id: {})",
        session.player_uid, config.id,
    );

    config
        .actions
        .iter()
        .for_each(|action| execute_action(session, action, scene_uid, section_id));
}

pub fn execute_action(
    session: &mut PlayerSession,
    action: &ConfigEventAction,
    scene_uid: u64,
    section_id: i32,
) {
    use ConfigEventAction::*;
    match action {
        ActionCreateNpcCfg { id, tag_id } => {
            let uid = session.uid_counter.next();
            let sdg = session.player_info.single_dungeon_group_mut();
            sdg.npcs_mut().insert(
                scene_uid,
                uid,
                NpcInfo {
                    uid,
                    id: *id,
                    tag_value: *tag_id,
                    scene_uid,
                    parent_uid: 0,
                    owner_uid: 0,
                    scene_data: NpcSceneData {
                        section_id,
                        transform: Transform::default(),
                    },
                    references: phashset![],
                },
            );
        }
        ActionChangeInteractCfg {
            interact_id,
            tag_ids,
            participators,
            interact_scale,
            section_listen_events,
            ..
        } => {
            let sdg = session.player_info.single_dungeon_group_mut();
            sdg.npcs()
                .iter()
                .filter(|(s_uid, _, _)| **s_uid == scene_uid)
                .for_each(|(_, &uid, npc)| {
                    if tag_ids.contains(&npc.tag_value) {
                        session.level_event_graph_mgr.bound_interact_map.insert(
                            uid,
                            (
                                *interact_id,
                                BoundInteractInfo {
                                    participators: participators.clone(),
                                    scale_x: interact_scale.x,
                                    scale_y: interact_scale.y,
                                    scale_z: interact_scale.z,
                                    scale_w: 0.0,
                                    scale_r: 0.0,
                                    name: String::from("A"),
                                    interact_id: *interact_id,
                                },
                            ),
                        );

                        session
                            .level_event_graph_mgr
                            .listen_events
                            .insert(*interact_id, section_listen_events.clone());
                    }
                });
        }
        ActionSetMainCityObjectState { object_state, .. } => {
            let main_city_objects_state = session.player_info.main_city_objects_state_mut();
            object_state
                .iter()
                .for_each(|(&obj, &state)| main_city_objects_state.insert(obj, state));
        }
        ActionOpenUI {
            ui,
            args,
            store_template_id,
        } => {
            use yanagi_eventgraph::Message;
            session
                .level_event_graph_mgr
                .push_protocol_action(protocol::ActionInfo {
                    action_type: 5,
                    body: action_pb::ActionOpenUi {
                        ui: ui.clone(),
                        args: *args,
                        npc_id: 0,
                        store_template_id: *store_template_id,
                    }
                    .encode_to_vec(),
                });
        }
        ActionSwitchSection {
            section_id,
            transform,
            camera_x,
            camera_y,
        } => {
            use yanagi_eventgraph::Message;
            session
                .level_event_graph_mgr
                .push_protocol_action(protocol::ActionInfo {
                    action_type: 6,
                    body: action_pb::ActionSwitchSection {
                        section: *section_id,
                        transform_id: transform.clone(),
                        camera_x: *camera_x,
                        camera_y: *camera_y,
                    }
                    .encode_to_vec(),
                });
        }
    }
}
