use qwer::{OctData, PropertyDoubleKeyHashMap, PropertyHashMap};

use crate::{DungeonContentDropPoolType, PropertyType, ReportType};

#[derive(OctData, Clone, Debug)]
pub struct AvatarPropertyChgInHollow {
    pub hp_lost: i32,
    pub hp_add: i32,
}

#[derive(OctData, Clone, Debug)]
pub struct AvatarUnitInfo {
    pub uid: u64,
    pub properties_uid: u64,
    pub is_banned: bool,
    pub modified_property: PropertyDoubleKeyHashMap<u64, PropertyType, i32>,
    pub hp_lost_hollow: i32,
    pub hp_add_hollow: i32,
    pub layer_property_change: PropertyHashMap<i32, AvatarPropertyChgInHollow>,
}

#[derive(OctData, Clone, Debug)]
pub struct BuddyUnitInfo {
    pub uid: u64,
    pub properties: u64,
}

#[derive(OctData, Clone, Debug)]
pub struct DungeonDropPollInfo {
    pub action_card_mask: PropertyHashMap<i32, i32>,
}

#[derive(OctData, Clone, Debug)]
pub struct BattleReport {
    pub index: i32,
    pub report_type: ReportType,
    pub id: i32,
}

#[derive(OctData, Clone, Debug)]
pub struct DungeonInfo {
    pub uid: u64,
    pub id: i32,
    pub default_scene_uid: u64,
    pub start_timestamp: u64,
    pub to_be_destroyed: bool,
    pub back_scene_uid: u64,
    pub quest_collection_uid: u64,
    pub avatars: PropertyHashMap<u64, AvatarUnitInfo>,
    pub buddy: BuddyUnitInfo,
    pub world_quest_id: i32,
    pub scene_properties_uid: u64,
    pub drop_poll_chg_infos: PropertyHashMap<DungeonContentDropPoolType, DungeonDropPollInfo>,
    pub is_in_dungeon: bool,
    pub initiative_item: i32,
    pub initiative_item_used_times: i32,
    pub avatar_map: PropertyHashMap<i8, AvatarUnitInfo>,
    pub battle_report: Vec<BattleReport>,
    pub dungeon_group_uid: u64,
    pub entered_times: u16,
    pub is_preset_avatar: bool,
    pub hollow_event_version: i32,
}
