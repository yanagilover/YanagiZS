use protocol_macros::polymorphic;
use qwer::{PropertyDoubleKeyHashMap, PropertyHashMap};

use crate::{player_info::BoundNPCAndInteractInfo, QuestState, QuestStatisticsType};

polymorphic!(
    QuestInfo [
        id: i32,
        finished_count: i32,
        collection_uid: u64,
        progress: u16,
        parent_quest_id: i32,
        state: QuestState,
        finish_condition_progress: PropertyHashMap<i32, i32>,
        progress_time: u32,
        sort_id: u64,
    ]
    ArchiveBattle {
        statistics: PropertyHashMap<QuestStatisticsType, u64>,
        dungeon_uid: u64,
        star: u8,
    } = 7,
    ArchiveFile { } = 1,
    Challenge { } = 6,
    DungeonInner { } = 2,
    Hollow {
        statistics: PropertyHashMap<QuestStatisticsType, u64>,
        dungeon_uid: u64,
        statistics_ext: PropertyDoubleKeyHashMap<QuestStatisticsType, i32, i32>,
        acquired_hollow_challenge_reward: i32,
    } = 3,
    Knowledge { } = 8,
    MainCity {
        bound_npc_and_interact: PropertyHashMap<u64, BoundNPCAndInteractInfo>,
    } = 5,
    Manual { } = 4,
);
