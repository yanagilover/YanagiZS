use std::collections::HashMap;

use crate::{
    item_info::ItemInfo, player_info::PlayerInfo, scene_info::SceneInfo, AvatarInfo,
    AvatarSkillInfo, AvatarSync, AvatarUnitInfo, EquipInfo, ItemSync, PlayerBasicInfo,
    PtcHallRefreshArg, ResourceInfo, WeaponInfo,
};

pub fn build_player_basic_info(player_info: &PlayerInfo) -> PlayerBasicInfo {
    PlayerBasicInfo {
        last_enter_world_timestamp: player_info.last_enter_world_timestamp.unwrap_or_default()
            as i64,
        avatar_id: player_info.avatar_id.unwrap_or_default(),
        player_avatar_id: player_info.avatar_id.unwrap_or_default(),
        main_city_avatar_id: player_info.main_city_avatar_id.unwrap_or_default(),
        nick_name: player_info.nick_name.clone().unwrap_or_default(),
        level: player_info
            .yorozuya_info
            .as_ref()
            .map(|yi| yi.yorozuya_level)
            .flatten()
            .unwrap_or_default(),
    }
}

pub fn build_client_scene_info(
    player_info: &PlayerInfo,
    scene_uid: u64,
) -> Option<crate::SceneInfo> {
    let dungeon_collection = player_info.dungeon_collection.as_ref().unwrap();
    let Some(scene_info) = dungeon_collection.scenes.as_ref().unwrap().get(&scene_uid) else {
        return None;
    };

    let player_pos_in_main_city = player_info.pos_in_main_city.as_ref().unwrap();
    let initial_transform = player_pos_in_main_city
        .initial_pos_id
        .clone()
        .unwrap_or_default();

    Some(match scene_info {
        SceneInfo::Hall {
            section_id,
            main_city_time_info,
            camera_x,
            camera_y,
            ..
        } => crate::SceneInfo {
            scene_type: 1,
            hall_scene_info: Some(crate::HallSceneInfo {
                section_id: *section_id as u32,
                player_avatar_id: player_info.avatar_id.unwrap_or_default(),
                main_city_avatar_id: player_info.main_city_avatar_id.unwrap_or_default(),
                bgm_id: player_info
                    .bgm_info
                    .as_ref()
                    .map(|bgm| bgm.bgm_id.clone())
                    .flatten()
                    .unwrap_or_default(),
                day_of_week: main_city_time_info.day_of_week as u32,
                time_of_day: main_city_time_info.initial_time,
                camera_x: *camera_x,
                camera_y: *camera_y,
                position: initial_transform.is_empty().then(|| crate::Transform {
                    position: player_pos_in_main_city
                        .position
                        .clone()
                        .unwrap_or_default()
                        .into(),
                    rotation: player_pos_in_main_city
                        .rotation
                        .clone()
                        .unwrap_or_default()
                        .into(),
                }),
                main_city_objects_state: player_info
                    .main_city_objects_state
                    .as_ref()
                    .map(|map| map.iter().map(|(&k, &v)| (k, v)).collect())
                    .unwrap_or_default(),
                scene_unit_list: Vec::new(),
                transform_id: initial_transform,
            }),
            ..Default::default()
        },
        SceneInfo::Fresh { .. } => crate::SceneInfo {
            scene_type: 4,
            fresh_scene_info: Some(crate::FreshSceneInfo {
                beginner_procedure_id: player_info
                    .beginner_procedure_info
                    .as_ref()
                    .unwrap()
                    .procedure_id
                    .unwrap_or_default() as u32,
            }),
            ..Default::default()
        },
        SceneInfo::Fight {
            id,
            local_play_type,
            time,
            weather,
            end_hollow,
            ..
        } => crate::SceneInfo {
            scene_type: 3,
            event_id: *id as u32,
            local_play_type: *local_play_type,
            fight_scene_info: Some(crate::FightSceneInfo {
                end_hollow: *end_hollow,
                level_reward_info: crate::LevelRewardInfo {},
                level_perform_info: crate::LevelPerformInfo {
                    time: time.to_protocol_string(),
                    weather: weather.to_protocol_string(),
                },
            }),
            ..Default::default()
        },
    })
}

pub fn build_client_dungeon_info(
    player_info: &PlayerInfo,
    scene_uid: u64,
) -> Option<crate::DungeonInfo> {
    let dungeon_collection = player_info.dungeon_collection.as_ref().unwrap();
    let Some(scene_info) = dungeon_collection.scenes.as_ref().unwrap().get(&scene_uid) else {
        return None;
    };

    match scene_info {
        SceneInfo::Hall { .. } | SceneInfo::Fresh { .. } => return None,
        _ => (),
    }

    let dungeon_info = dungeon_collection
        .dungeons
        .as_ref()
        .unwrap()
        .get(scene_info.get_dungeon_uid())
        .unwrap();

    Some(crate::DungeonInfo {
        quest_id: dungeon_info.world_quest_id as u32,
        dungeon_equip_info: crate::DungeonEquipInfo::default(),
        avatar_list: dungeon_info
            .avatars
            .iter()
            .map(|(_, unit)| {
                let avatar_info = player_info.items.as_ref().unwrap().get(&unit.uid).unwrap();
                AvatarUnitInfo {
                    avatar_id: *avatar_info.get_id() as u32,
                }
            })
            .collect(),
        ..Default::default()
    })
}

pub fn build_hall_refresh_arg(
    player_info: &PlayerInfo,
    hall_scene_uid: u64,
    refresh_immediately: bool,
) -> Option<PtcHallRefreshArg> {
    let dungeon_collection = player_info.dungeon_collection.as_ref().unwrap();
    let scene_info = dungeon_collection
        .scenes
        .as_ref()
        .unwrap()
        .get(&hall_scene_uid);
    let player_pos_in_main_city = player_info.pos_in_main_city.as_ref().unwrap();

    match scene_info {
        Some(SceneInfo::Hall {
            section_id,
            main_city_time_info,
            camera_x,
            camera_y,
            ..
        }) => Some(PtcHallRefreshArg {
            refresh_immediately,
            section_id: *section_id as u32,
            player_avatar_id: player_info.avatar_id.unwrap_or_default(),
            main_city_avatar_id: player_info.main_city_avatar_id.unwrap_or_default(),
            transform_id: player_pos_in_main_city
                .initial_pos_id
                .clone()
                .unwrap_or_default(),
            bgm_id: player_info
                .bgm_info
                .as_ref()
                .map(|bgm| bgm.bgm_id.clone())
                .flatten()
                .unwrap_or_default(),
            day_of_week: main_city_time_info.day_of_week as u32,
            time_of_day: main_city_time_info.initial_time,
            camera_x: *camera_x,
            camera_y: *camera_y,
            position: crate::Transform {
                position: player_pos_in_main_city
                    .position
                    .clone()
                    .unwrap_or_default()
                    .into(),
                rotation: player_pos_in_main_city
                    .rotation
                    .clone()
                    .unwrap_or_default()
                    .into(),
            },
            main_city_objects_state: player_info
                .main_city_objects_state
                .as_ref()
                .map(|map| map.iter().map(|(&k, &v)| (k, v)).collect())
                .unwrap_or_default(),
            scene_unit_list: Vec::new(),
        }),
        _ => None,
    }
}

pub fn build_sync_avatar_info_list(player_info: &PlayerInfo) -> Vec<AvatarInfo> {
    player_info
        .items
        .as_ref()
        .unwrap()
        .iter()
        .map(|(uid, item)| {
            if let ItemInfo::AvatarInfo {
                id,
                first_get_time,
                star,
                exp,
                level,
                rank,
                unlocked_talent_num,
                talent_switch,
                skills,
                ..
            } = item
            {
                Some(AvatarInfo {
                    template_id: *id as u32,
                    level: *level as u32,
                    exp: *exp as u32,
                    star: *star as u32,
                    rank: *rank as u32,
                    unlocked_talent_num: *unlocked_talent_num as u32,
                    first_get_time: *first_get_time as i64,
                    talent_switch_list: talent_switch.clone(),
                    cur_weapon_uid: player_info
                        .items
                        .as_ref()
                        .unwrap()
                        .iter()
                        .find(|(_, item)| {
                            if let ItemInfo::Weapon { avatar_uid, .. } = item {
                                *avatar_uid == *uid
                            } else {
                                false
                            }
                        })
                        .map(|(uid, _)| (*uid & 0xFFFFFFFF) as u32)
                        .unwrap_or(0),
                    skill_type_level: skills
                        .iter()
                        .map(|(ty, lv)| AvatarSkillInfo {
                            skill_type: *ty as u32,
                            level: *lv as u32,
                        })
                        .collect(),
                })
            } else {
                None
            }
        })
        .flatten()
        .collect()
}

pub fn build_sync_weapon_info_list(player_info: &PlayerInfo) -> Vec<WeaponInfo> {
    player_info
        .items
        .as_ref()
        .unwrap()
        .iter()
        .map(|(_, item)| {
            if let ItemInfo::Weapon {
                uid,
                id,
                star,
                exp,
                level,
                lock,
                refine_level,
                ..
            } = item
            {
                Some(WeaponInfo {
                    template_id: *id as u32,
                    level: *level as u32,
                    exp: *exp,
                    star: *star as u32,
                    refine_level: *refine_level as u32,
                    uid: (*uid & 0xFFFFFFFF) as u32,
                    lock: *lock != 0,
                })
            } else {
                None
            }
        })
        .flatten()
        .collect()
}

pub fn build_sync_equip_info_list(player_info: &PlayerInfo) -> Vec<EquipInfo> {
    player_info
        .items
        .as_ref()
        .unwrap()
        .iter()
        .map(|(_, item)| {
            if let ItemInfo::Equip {
                uid,
                id,
                star,
                exp,
                level,
                lock,
                ..
            } = item
            {
                Some(EquipInfo {
                    template_id: *id as u32,
                    level: *level as u32,
                    exp: *exp,
                    star: *star as u32,
                    uid: (*uid & 0xFFFFFFFF) as u32,
                    lock: *lock != 0,
                })
            } else {
                None
            }
        })
        .flatten()
        .collect()
}

pub fn build_sync_resource_info_list(player_info: &PlayerInfo) -> Vec<ResourceInfo> {
    player_info
        .items
        .as_ref()
        .unwrap()
        .iter()
        .map(|(_, item)| match item {
            ItemInfo::Currency { id, count, .. } => Some(ResourceInfo {
                template_id: *id as u32,
                count: *count,
            }),
            ItemInfo::Resource { id, count, .. } => Some(ResourceInfo {
                template_id: *id as u32,
                count: *count,
            }),
            ItemInfo::Consumable { id, count, .. } => Some(ResourceInfo {
                template_id: *id as u32,
                count: *count,
            }),
            ItemInfo::AvatarPiece { id, count, .. } => Some(ResourceInfo {
                template_id: *id as u32,
                count: *count,
            }),
            _ => None,
        })
        .flatten()
        .collect()
}

pub fn build_sync_auto_recovery_info(
    player_info: &PlayerInfo,
) -> HashMap<u32, crate::AutoRecoveryInfo> {
    player_info
        .auto_recovery_info
        .as_ref()
        .unwrap()
        .iter()
        .map(|(id, info)| (*id as u32, info.clone()))
        .collect()
}

pub fn build_item_sync(player_info: &PlayerInfo) -> ItemSync {
    ItemSync {
        weapon_list: build_sync_weapon_info_list(player_info),
        equip_list: build_sync_equip_info_list(player_info),
        resource_list: build_sync_resource_info_list(player_info),
        auto_recovery_info: build_sync_auto_recovery_info(player_info),
    }
}

pub fn build_avatar_sync(player_info: &PlayerInfo) -> AvatarSync {
    AvatarSync {
        avatar_list: build_sync_avatar_info_list(player_info),
    }
}
