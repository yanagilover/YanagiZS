use protocol_macros::polymorphic;
use qwer::{OctData, PropertyHashSet};

use crate::player_info::EventGraphsInfo;

polymorphic!(
    SectionInfoExt [
        destroy_npc_when_no_player: PropertyHashSet<u64>,
    ]
    Hall {} = 1,
    /*Hollow {
        hollow_level_info: HollowLevelInfo,
        hollow_grid_map_info: HollowGridMapInfo
    } = 0,*/ // todo!
);

polymorphic!(
    SceneTableExt [
        event_graphs_info: EventGraphsInfo,
    ]
    Fight {} = 3,
    Fresh {} = 4,
    Hall {} = 1,
    Hollow {
        grid_random_seed: i32,
        alter_section_id: i32,
    } = 2,
);

polymorphic!(
    DungeonTableExt []
    Hall {} = 1,
    Hollow {
        avatars: PropertyHashSet<HollowDungeonAvatarInfo>,
        scene_properties_uid: u64,
        buddy: HollowDungeonBuddyInfo,
    } = 2,
);

#[derive(OctData, Clone, Debug, PartialEq, Eq, Hash)]
pub struct HollowDungeonAvatarInfo {
    pub uid: u64,
    pub properties_uid: u64,
}

#[derive(OctData, Clone, Debug, PartialEq, Eq, Hash)]
pub struct HollowDungeonBuddyInfo {
    pub uid: u64,
    pub properties_uid: u64,
}
