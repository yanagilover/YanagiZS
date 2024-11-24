use std::collections::HashMap;

use protocol::{
    player_info::{EventGraphsInfo, SectionInfo},
    scene_ext::SectionInfoExt,
    PtcHallRefreshArg, SceneUnitProtocolInfo,
};
use qwer::{phashmap, phashset, PropertyHashMap, PropertyHashSet};

use crate::{level, PlayerSession};

pub fn init_hall_scene_section(session: &mut PlayerSession, scene_uid: u64, section_id: i32) {
    let single_dungeon_group = session.player_info.single_dungeon_group.as_mut().unwrap();
    if single_dungeon_group
        .section
        .as_ref()
        .unwrap()
        .contains(&scene_uid, &section_id)
    {
        return;
    }

    let section_map = single_dungeon_group.section.as_mut().unwrap();
    section_map.insert(
        scene_uid,
        section_id,
        SectionInfo {
            scene_uid,
            id: section_id,
            event_graphs_info: EventGraphsInfo {
                event_graphs_info: phashmap![],
                default_event_graph_id: -1,
            },
            section_info_ext: SectionInfoExt::Hall {
                destroy_npc_when_no_player: phashset![],
            },
        },
    );

    level::on_section_added(session, scene_uid, section_id);
}

pub fn add_scene_units_to_scene_info(
    session: &mut PlayerSession,
    scene_uid: u64,
    scene_info: &mut protocol::SceneInfo,
) {
    let Some(hall_scene_info) = scene_info.hall_scene_info.as_mut() else {
        return;
    };

    hall_scene_info.scene_unit_list =
        build_scene_unit_protocol_info(session, scene_uid, hall_scene_info.section_id);
}

pub fn add_scene_units_to_hall_refresh_arg(
    session: &mut PlayerSession,
    scene_uid: u64,
    refresh_arg: &mut PtcHallRefreshArg,
) {
    refresh_arg.scene_unit_list =
        build_scene_unit_protocol_info(session, scene_uid, refresh_arg.section_id);
}

fn build_scene_unit_protocol_info(
    session: &mut PlayerSession,
    scene_uid: u64,
    section_id: u32,
) -> Vec<SceneUnitProtocolInfo> {
    let sdg = session.player_info.single_dungeon_group.as_ref().unwrap();
    sdg.npcs
        .as_ref()
        .unwrap()
        .iter()
        .filter(|(s_uid, _, npc)| {
            **s_uid == scene_uid && npc.scene_data.section_id == section_id as i32
        })
        .map(|(_, uid, npc)| SceneUnitProtocolInfo {
            npc_id: npc.tag_value as u32,
            is_interactable: true,
            interacts_info: session
                .level_event_graph_mgr
                .bound_interact_map
                .get(uid)
                .map(|(_, interact)| {
                    HashMap::from([(
                        interact.interact_id as u32,
                        protocol::InteractInfo {
                            name: interact.name.clone(),
                            participators: interact.participators.clone(),
                            scale_x: interact.scale_x,
                            scale_y: interact.scale_y,
                            scale_z: interact.scale_z,
                            scale_w: interact.scale_w,
                            scale_r: interact.scale_r,
                            interact_id: npc.tag_value,
                            interact_target_list: vec![2],
                        },
                    )])
                })
                .unwrap_or_default(),
        })
        .collect()
}
