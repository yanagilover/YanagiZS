use protocol_macros::polymorphic;
use qwer::{OctData, PropertyDoubleKeyHashMap, PropertyHashMap, PropertyHashSet};

use crate::{player_info::BoundNPCAndInteractInfo, TimePeriodType, UIType, WeatherType};

#[derive(OctData, Clone, Debug)]
pub struct TimeEventInfo {
    pub executed_count: i32,
}

#[derive(OctData, Clone, Debug)]
pub struct TimeEventGroupInfo {
    pub group_id: i32,
    pub executing_scripts: PropertyHashSet<i32>,
    pub complete_time: u64,
    pub time_events_info: PropertyHashMap<i32, TimeEventInfo>,
    pub bound_npc_and_interact: PropertyHashMap<u64, BoundNPCAndInteractInfo>,
    pub executing_time_event: PropertyHashSet<i32>,
}

#[derive(OctData, Clone, Debug)]
pub struct MainCityTimeInfo {
    pub initial_time: u32,
    pub day_of_week: u8,
    pub passed_milliseconds: u64,
    pub executing_event_groups: PropertyHashSet<i32>,
    pub unlocked_time_events: PropertyHashSet<i32>,
    #[server_only]
    pub time_event_groups_info: PropertyHashMap<i32, TimeEventGroupInfo>,
    #[server_only]
    pub condition_progress_of_unlock: PropertyDoubleKeyHashMap<i32, i32, i32>,
    #[server_only]
    pub condition_progress_of_end: PropertyDoubleKeyHashMap<i32, i32, i32>,
    pub ended_time_events: PropertyHashSet<i32>,
    pub leave_time: u64,
}

polymorphic!(
    SceneInfo [
        uid: u64,
        id: i32,
        dungeon_uid: u64,
        end_timestamp: u64,
        back_scene_uid: u64,
        entered_times: u16,
        section_id: i32,
        open_ui: UIType,
        to_be_destroyed: bool,
        camera_x: u32,
        camera_y: u32,
    ]
    Hall {
        main_city_time_info: MainCityTimeInfo,
    } = 1,
    // Hollow {} = 2,
    Fight {
        end_hollow: bool,
        local_play_type: u32,
        time: TimePeriodType,
        weather: WeatherType,
    } = 3,
    Fresh {} = 4,
);
