use std::collections::{HashMap, HashSet};

use protocol_macros::property_accessors;
use qwer::{OctData, PropertyDoubleKeyHashMap, PropertyHashMap, PropertyHashSet};

use crate::action_info::ActionInfo;
use crate::dungeon_info::DungeonInfo;
use crate::event_graph_info::EventGraphInfo;
use crate::item_info::ItemInfo;
use crate::quest_info::QuestInfo;
use crate::scene_ext::{DungeonTableExt, SceneTableExt, SectionInfoExt};
use crate::scene_info::SceneInfo;
use crate::{
    ActionState, AutoRecoveryInfo, EventState, FairyState, HollowBattleEventType, HollowQuestType,
    InteractInfo, MailState, QuestType,
};

#[derive(OctData, Copy, Clone, Debug, Default)]
pub struct Vector3f {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

#[derive(thiserror::Error, Debug)]
#[error("vector length mismatch, expected 3, got: {0}")]
pub struct VectorLengthError(usize);

impl TryFrom<Vec<f64>> for Vector3f {
    type Error = VectorLengthError;

    fn try_from(value: Vec<f64>) -> Result<Self, Self::Error> {
        (value.len() == 3)
            .then_some(Self {
                x: value[0],
                y: value[1],
                z: value[2],
            })
            .ok_or(VectorLengthError(value.len()))
    }
}

impl From<Vector3f> for Vec<f64> {
    fn from(value: Vector3f) -> Self {
        vec![value.x, value.y, value.z]
    }
}

#[derive(OctData, Clone, Debug, Default)]
pub struct Transform {
    pub position: Vector3f,
    pub rotation: Vector3f,
}

#[derive(OctData, Clone, Debug)]
pub struct EventStackFrame {
    pub action_info: ActionInfo,
    pub action_id: i32,
}

#[derive(OctData, Clone, Debug)]
pub struct EventInfo {
    pub id: i32,
    pub cur_action_id: i32,
    pub action_move_path: Vec<i32>,
    pub state: EventState,
    pub prev_state: EventState,
    pub cur_action_info: ActionInfo,
    pub cur_action_state: ActionState,
    pub predicated_failed_actions: PropertyHashSet<i32>,
    pub stack_frames: Vec<EventStackFrame>,
}

#[derive(OctData, Clone, Debug)]
pub struct ChoiceInfo {
    pub id: i32,
    pub hide_info: bool,
    pub forbidden: bool,
}

#[derive(OctData, Clone, Debug)]
pub struct EventGraphsInfo {
    pub event_graphs_info: PropertyHashMap<i32, EventGraphInfo>,
    pub default_event_graph_id: i32,
}

#[derive(OctData, Clone, Debug, Default)]
#[property_object(u16, 0x01)]
#[root]
#[property_accessors]
pub struct PlayerInfo {
    #[tag = 1]
    pub uid: Option<u64>,
    #[tag = 2]
    pub account_name: Option<String>,
    #[tag = 3]
    pub last_enter_world_timestamp: Option<u64>,
    #[tag = 4]
    pub items: Option<PropertyHashMap<u64, ItemInfo>>,
    #[tag = 5]
    pub dungeon_collection: Option<DungeonCollection>,
    #[tag = 6]
    #[server_only]
    pub properties: Option<PropertyDoubleKeyHashMap<u64, u16, i32>>,
    #[tag = 7]
    pub scene_properties: Option<PropertyDoubleKeyHashMap<u64, u16, i32>>,
    #[tag = 8]
    pub quest_data: Option<QuestData>,
    #[tag = 9]
    pub joined_chat_rooms: Option<Vec<u64>>,
    #[tag = 10]
    pub scene_uid: Option<u64>,
    #[tag = 11]
    pub archive_info: Option<ArchiveInfo>,
    #[tag = 12]
    pub auto_recovery_info: Option<PropertyHashMap<i32, AutoRecoveryInfo>>,
    #[tag = 13]
    pub unlock_info: Option<UnlockInfo>,
    #[tag = 14]
    pub yorozuya_info: Option<YorozuyaInfo>,
    #[tag = 15]
    pub equip_gacha_info: Option<EquipGachaInfo>,
    #[tag = 16]
    pub beginner_procedure_info: Option<BeginnerProcedureInfo>,
    #[tag = 17]
    pub pos_in_main_city: Option<PlayerPosInMainCity>,
    #[tag = 18]
    pub fairy_info: Option<FairyInfo>,
    #[tag = 19]
    pub popup_window_info: Option<PopupWindowInfo>,
    #[tag = 20]
    pub tips_info: Option<TipsInfo>,
    #[tag = 21]
    pub main_city_quest_data: Option<MainCityQuestData>,
    #[tag = 22]
    pub embattles: Option<Embattles>,
    #[tag = 23]
    #[server_only]
    pub day_change_info: Option<DayChangeInfo>,
    #[tag = 24]
    #[server_only]
    pub npcs_info: Option<PlayerNPCsInfo>,
    #[tag = 25]
    #[server_only]
    pub scripts_to_execute: Option<PropertyDoubleKeyHashMap<i32, i32, ToExecuteScriptInfo>>,
    #[tag = 26]
    #[server_only]
    pub scripts_to_remove: Option<PropertyHashMap<i32, PropertyHashSet<i32>>>,
    #[tag = 27]
    pub last_leave_world_timestamp: Option<u64>,
    #[tag = 28]
    #[server_only]
    pub muip_data: Option<MUIPData>,
    #[tag = 29]
    pub nick_name: Option<String>,
    #[tag = 30]
    pub ramen_data: Option<RamenData>,
    #[tag = 31]
    pub shop: Option<ShopsInfo>,
    #[tag = 32]
    pub vhs_store_data: Option<VHSStoreData>,
    #[tag = 33]
    #[server_only]
    pub operation_mail_receive_info: Option<OperationMailReceiveInfo>,
    #[tag = 34]
    pub second_last_enter_world_timestamp: Option<u64>,
    #[tag = 35]
    pub login_times: Option<u32>,
    #[tag = 36]
    pub create_timestamp: Option<u64>,
    #[tag = 37]
    pub gender: Option<u8>,
    #[tag = 38]
    pub avatar_id: Option<u32>,
    #[tag = 39]
    pub prev_scene_uid: Option<u64>,
    #[tag = 40]
    pub register_cps: Option<String>,
    #[tag = 41]
    pub register_platform: Option<u32>,
    #[tag = 42]
    pub pay_info: Option<PayInfo>,
    #[tag = 43]
    #[server_only]
    pub private_npcs: Option<PropertyHashMap<u64, NpcInfo>>,
    #[tag = 44]
    pub battle_event_info: Option<BattleEventInfo>,
    #[tag = 45]
    pub gm_data: Option<GMData>,
    #[tag = 46]
    #[server_only]
    pub player_mail_ext_infos: Option<PlayerMailExtInfos>,
    #[tag = 47]
    #[server_only]
    pub single_dungeon_group: Option<SingleDungeonGroup>,
    #[tag = 48]
    pub newbie_info: Option<NewbieInfo>,
    #[tag = 49]
    pub loading_page_tips_info: Option<LoadingPageTipsInfo>,
    #[tag = 50]
    pub switch_of_story_mode: Option<bool>,
    #[tag = 51]
    pub switch_of_qte: Option<bool>,
    #[tag = 52]
    pub collect_map: Option<CollectMap>,
    #[tag = 53]
    pub areas_info: Option<AreasInfo>,
    #[tag = 54]
    pub bgm_info: Option<BGMInfo>,
    #[tag = 55]
    pub main_city_objects_state: Option<PropertyHashMap<i32, i32>>,
    #[tag = 56]
    pub hollow_info: Option<HollowInfo>,
    #[tag = 57]
    pub main_city_avatar_id: Option<u32>,
}

#[derive(OctData, Clone, Debug, Default)]
#[property_object]
#[property_accessors]
pub struct DungeonCollection {
    #[tag = 1]
    pub dungeons: Option<PropertyHashMap<u64, DungeonInfo>>,
    #[tag = 2]
    pub scenes: Option<PropertyHashMap<u64, SceneInfo>>,
    #[tag = 3]
    pub default_scene_uid: Option<u64>,
    #[tag = 4]
    pub transform: Option<Transform>,
    #[tag = 5]
    pub used_story_mode: Option<bool>,
    #[tag = 6]
    pub used_manual_qte_mode: Option<bool>,
}

#[derive(OctData, Clone, Debug)]
pub struct BoundNPCAndInteractInfo {
    pub is_bound_npc: bool,
    pub interacts: PropertyHashSet<i32>,
    pub npc_reference_uid: u64,
}

#[derive(OctData, Clone, Debug, Default)]
#[property_object]
pub struct QuestData {
    #[tag = 1]
    pub quests: Option<PropertyDoubleKeyHashMap<u64, i32, QuestInfo>>,
    #[tag = 2]
    pub world_quest_for_cur_dungeon: Option<i32>,
    #[tag = 3]
    pub world_quest_collection_uid: Option<u64>,
    #[tag = 4]
    #[server_only]
    pub unlock_condition_progress: Option<PropertyDoubleKeyHashMap<i32, i32, i32>>,
    #[tag = 5]
    pub is_afk: Option<bool>,
    #[tag = 6]
    pub world_quest_for_cur_dungeon_afk: Option<i32>,
}

#[derive(OctData, Clone, Debug)]
pub struct VideotapeInfo {
    pub star_count: PropertyHashMap<u8, u16>,
    pub finished: bool,
    pub awarded_star: PropertyHashMap<u8, HashSet<u16>>,
}

#[derive(OctData, Clone, Debug, Default)]
#[property_object]
#[property_accessors]
pub struct ArchiveInfo {
    #[tag = 1]
    pub videotaps_info: Option<PropertyHashMap<i32, VideotapeInfo>>,
    #[tag = 2]
    pub hollow_archive_id: Option<PropertyHashSet<i32>>,
}

#[derive(OctData, Clone, Debug)]
#[property_object]
pub struct UnlockInfo {
    #[tag = 1]
    pub unlocked_list: Option<PropertyHashSet<i32>>,
    #[tag = 2]
    #[server_only]
    pub condition_progress: Option<PropertyDoubleKeyHashMap<i32, i32, i32>>,
}

#[derive(OctData, Clone, Debug, Default)]
#[property_object]
#[property_accessors]
pub struct YorozuyaInfo {
    #[tag = 1]
    pub last_refresh_timestamp_common: Option<u64>,
    #[tag = 2]
    pub yorozuya_level: Option<u32>,
    #[tag = 3]
    pub yorozuya_rank: Option<u32>,
    #[tag = 4]
    pub gm_quests: Option<PropertyHashMap<HollowQuestType, Vec<i32>>>,
    #[tag = 5]
    pub gm_enabled: Option<bool>,
    #[tag = 6]
    pub hollow_quests: Option<PropertyDoubleKeyHashMap<i32, HollowQuestType, PropertyHashSet<i32>>>,
    #[tag = 7]
    pub urgent_quests_queue: Option<PropertyHashMap<i32, Vec<i32>>>,
    #[tag = 8]
    pub last_refresh_timestamp_urgent: Option<u64>,
    #[tag = 9]
    pub next_refresh_timestamp_urgent: Option<u64>,
    #[tag = 10]
    pub finished_hollow_quest_count: Option<u32>,
    #[tag = 11]
    pub finished_hollow_quest_count_of_type: Option<PropertyHashMap<i16, u32>>,
    #[tag = 12]
    pub unlock_hollow_id: Option<Vec<i32>>,
    #[tag = 13]
    #[server_only]
    pub unlock_hollow_id_progress: Option<PropertyDoubleKeyHashMap<i32, i32, i32>>,
}

#[derive(OctData, Clone, Debug, Default)]
#[property_object]
pub struct EquipGachaInfo {
    #[tag = 1]
    pub smithy_level: Option<i32>,
    #[tag = 2]
    pub security_num_by_lv: Option<PropertyHashMap<i32, i32>>,
    #[tag = 3]
    #[server_only]
    pub total_gacha_times: Option<i32>,
    #[tag = 4]
    #[server_only]
    pub equip_star_up_times: Option<i32>,
    #[tag = 5]
    #[server_only]
    pub avatar_level_advance_times: Option<i32>,
}

#[derive(OctData, Clone, Debug)]
#[property_object]
#[property_accessors]
pub struct BeginnerProcedureInfo {
    #[tag = 1]
    pub procedure_id: Option<i32>,
}

#[derive(OctData, Clone, Debug, Default)]
#[property_object]
#[property_accessors]
pub struct PlayerPosInMainCity {
    #[tag = 1]
    pub position: Option<Vector3f>,
    #[tag = 2]
    pub rotation: Option<Vector3f>,
    #[tag = 3]
    pub initial_pos_id: Option<String>,
}

#[derive(OctData, Clone, Debug, Default)]
#[property_object]
pub struct FairyInfo {
    #[tag = 1]
    pub fairy_groups: Option<PropertyHashMap<i32, FairyState>>,
    #[tag = 2]
    #[server_only]
    pub condition_progress: Option<PropertyDoubleKeyHashMap<i32, i32, i32>>,
}

#[derive(OctData, Clone, Debug)]
#[property_object]
pub struct PopupWindowInfo {
    #[tag = 1]
    pub popup_window_list: Option<Vec<i32>>,
    #[tag = 2]
    #[server_only]
    pub condition_progress: Option<PropertyDoubleKeyHashMap<i32, i32, i32>>,
}

#[derive(OctData, Clone, Debug, Default)]
#[property_object]
pub struct TipsInfo {
    #[tag = 1]
    pub tips_list: Option<Vec<i32>>,
    #[tag = 2]
    #[server_only]
    pub tips_condition_progress: Option<PropertyDoubleKeyHashMap<i32, i32, i32>>,
    #[tag = 3]
    pub tips_group: Option<Vec<i32>>,
    #[tag = 4]
    #[server_only]
    pub tips_group_condition_progress: Option<PropertyDoubleKeyHashMap<i32, i32, i32>>,
}

#[derive(OctData, Clone, Debug, Default)]
#[property_object]
pub struct MainCityQuestData {
    #[tag = 1]
    pub exicing_finish_script_group: Option<Vec<i32>>,
    #[tag = 2]
    pub in_progress_quests: Option<Vec<i32>>,
}

#[derive(OctData, Clone, Debug)]
pub struct EmbattleInfo {
    pub avatars: Vec<i32>,
    pub buddy: i32,
}

#[derive(OctData, Clone, Debug)]
#[property_object]
pub struct Embattles {
    #[tag = 1]
    pub last_embattles: Option<PropertyHashMap<QuestType, EmbattleInfo>>,
}

#[derive(OctData, Clone, Debug)]
#[property_object]
pub struct DayChangeInfo {
    #[tag = 1]
    pub last_daily_refresh_timing: Option<u64>,
}

#[derive(OctData, Clone, Debug)]
pub struct PlayerNPCInfo {
    pub interact_info: InteractInfo,
    pub npc_uid: u64,
    pub event_graphs_info: EventGraphsInfo,
    pub npc_tag_id: i32,
    pub vhs_trending_id: i32,
    pub visible: bool,
    pub invisible_by_quest: PropertyHashSet<i32>,
    pub look_ik: bool,
}

#[derive(OctData, Clone, Debug)]
#[property_object]
pub struct PlayerNPCsInfo {
    #[tag = 1]
    pub npcs_info: Option<PropertyHashMap<u64, PlayerNPCInfo>>,
    #[tag = 2]
    pub destroy_npc_when_leave_section: Option<PropertyHashSet<u64>>,
}

#[derive(OctData, Clone, Debug)]
pub struct ToExecuteScriptInfo {
    pub remove_after_finish: bool,
    pub specials: PropertyHashMap<String, i64>,
    pub event_graphs: PropertyHashSet<i32>,
}

#[derive(OctData, Clone, Debug, Default)]
#[property_object]
pub struct MUIPData {
    #[tag = 1]
    pub ban_begin_time: Option<String>,
    #[tag = 2]
    pub ban_end_time: Option<String>,
    #[tag = 3]
    pub tag_value: Option<u64>,
    #[tag = 4]
    pub dungeon_enter_times: Option<PropertyHashMap<i32, i32>>,
    #[tag = 5]
    pub scene_enter_times: Option<PropertyHashMap<i32, i32>>,
    #[tag = 6]
    pub dungeon_pass_times: Option<PropertyHashMap<i32, i32>>,
    #[tag = 7]
    pub scene_pass_times: Option<PropertyHashMap<i32, i32>>,
    #[tag = 8]
    pub alread_cmd_uids: Option<PropertyHashSet<u64>>,
    #[tag = 9]
    pub game_total_time: Option<u64>,
    #[tag = 10]
    pub language_type: Option<u16>,
}

#[derive(OctData, Clone, Debug, Default)]
#[property_object]
pub struct RamenData {
    #[tag = 1]
    pub unlock_ramen: Option<PropertyHashSet<i32>>,
    #[tag = 2]
    pub cur_ramen: Option<i32>,
    #[tag = 3]
    pub used_times: Option<i32>,
    #[tag = 4]
    pub unlock_initiative_item: Option<PropertyHashSet<i32>>,
    #[tag = 5]
    #[server_only]
    pub unlock_ramen_condition_progress: Option<PropertyDoubleKeyHashMap<i32, i32, i32>>,
    #[tag = 6]
    #[server_only]
    pub unlock_item_condition_progress: Option<PropertyDoubleKeyHashMap<i32, i32, i32>>,
    #[tag = 7]
    pub has_mystical_spice: Option<bool>,
    #[tag = 8]
    #[server_only]
    pub unlock_has_mystical_spice_condition_progress: Option<PropertyHashMap<i32, i32>>,
    #[tag = 9]
    pub cur_mystical_spice: Option<i32>,
    #[tag = 10]
    pub unlock_mystical_spice: Option<PropertyHashSet<i32>>,
    #[tag = 11]
    #[server_only]
    pub unlock_mystical_spice_condition_progress: Option<PropertyDoubleKeyHashMap<i32, i32, i32>>,
    #[tag = 12]
    pub unlock_initiative_item_group: Option<PropertyHashSet<i32>>,
    #[tag = 13]
    pub hollow_item_history: Option<PropertyHashMap<i32, i32>>,
    #[tag = 14]
    pub initial_item_ability: Option<u64>,
    #[tag = 15]
    #[property_object(u8, 0x01)]
    pub new_unlock_ramen: Option<Vec<i32>>,
    #[tag = 16]
    #[server_only]
    pub eat_ramen_times: Option<i32>,
    #[tag = 17]
    #[server_only]
    pub make_hollow_item_times: Option<i32>,
    #[tag = 18]
    pub new_unlock_initiative_item: Option<PropertyHashSet<i32>>,
}

#[derive(OctData, Clone, Debug)]
pub struct GoodsInfo {
    pub id: i32,
    pub purchased_num: u32,
    pub last_refresh_time: u64,
    pub discount: u16,
}

#[derive(OctData, Clone, Debug)]
pub struct ShelfInfo {
    pub id: i32,
    pub custom_goods_in_shelf: PropertyHashSet<i32>,
    pub goods_info: PropertyHashMap<i32, GoodsInfo>,
}

#[derive(OctData, Clone, Debug)]
pub struct ShopInfo {
    pub id: i32,
    pub shelf_info: PropertyHashMap<i32, ShelfInfo>,
    pub refreshed_count: i32,
    pub last_refresh_time: u64,
}

#[derive(OctData, Clone, Debug, Default)]
#[property_object]
pub struct ShopsInfo {
    #[tag = 1]
    pub vip_level: Option<u8>,
    #[tag = 2]
    #[server_only]
    pub shops: Option<PropertyHashMap<i32, ShopInfo>>,
    #[tag = 3]
    #[server_only]
    pub shop_buy_times: Option<i32>,
}

#[derive(OctData, Clone, Debug)]
pub struct VHSTrendingInfo {
    pub trend_id: i32,
    pub state: u16,
    pub match_level: u16,
    pub is_accept: bool,
}

#[derive(OctData, Clone, Debug)]
pub struct VHSTrendingCfgInfo {
    pub trend_id: i32,
    pub complete_level: i16,
    pub know_state: i16,
}

#[derive(OctData, Clone, Debug)]
pub struct VHSNpcInfo {
    pub npc_id: i32,
    pub state: i16,
    pub new_know: bool,
}

#[derive(OctData, Clone, Debug, Default)]
#[property_object]
pub struct VHSStoreData {
    #[tag = 1]
    pub store_level: Option<i32>,
    #[tag = 2]
    pub unreceived_reward: Option<i32>,
    #[tag = 3]
    #[server_only]
    pub hollow_enter_times: Option<i32>,
    #[tag = 4]
    pub last_receive_time: Option<i32>,
    #[tag = 5]
    #[property_object(u8, 0x01)]
    pub vhs_collection_slot: Option<Vec<i32>>,
    #[tag = 6]
    pub unlock_vhs_collection: Option<PropertyHashSet<i32>>,
    #[tag = 7]
    #[server_only]
    pub already_trending: Option<PropertyHashSet<i32>>,
    #[tag = 8]
    #[server_only]
    pub unlock_trending_condition_progress: Option<PropertyDoubleKeyHashMap<i32, i32, i32>>,
    #[tag = 9]
    pub is_need_refresh: Option<bool>,
    #[tag = 10]
    #[server_only]
    pub scripts_id: Option<PropertyHashSet<i32>>,
    #[tag = 11]
    pub store_exp: Option<i32>,
    #[tag = 12]
    pub is_level_chg_tips: Option<bool>,
    #[tag = 13]
    #[server_only]
    pub vhs_hollow: Option<Vec<i32>>,
    #[tag = 14]
    #[server_only]
    pub is_receive_trending_reward: Option<bool>,
    #[tag = 15]
    #[server_only]
    pub is_need_first_trending: Option<bool>,
    #[tag = 16]
    #[server_only]
    pub last_basic_script: Option<i32>,
    #[tag = 17]
    #[server_only]
    pub is_complete_first_trending: Option<bool>,
    #[tag = 18]
    #[server_only]
    pub last_basic_npc: Option<u64>,
    #[tag = 19]
    #[server_only]
    pub can_random_trending: Option<PropertyHashSet<i32>>,
    #[tag = 20]
    #[property_object(u8, 0x01)]
    pub vhs_trending_info: Option<Vec<VHSTrendingInfo>>,
    #[tag = 21]
    pub unlock_vhs_trending_info: Option<PropertyHashMap<i32, VHSTrendingCfgInfo>>,
    #[tag = 22]
    pub vhs_flow: Option<i32>,
    #[tag = 23]
    pub received_reward: Option<i32>,
    #[tag = 24]
    pub last_reward: Option<i32>,
    #[tag = 25]
    pub last_exp: Option<i32>,
    #[tag = 26]
    pub last_flow: Option<i32>,
    #[tag = 27]
    #[property_object(u8, 0x01)]
    pub last_vhs_trending_info: Option<Vec<VHSTrendingInfo>>,
    #[tag = 28]
    #[property_object(u8, 0x01)]
    pub new_know_trend: Option<Vec<i32>>,
    #[tag = 29]
    #[server_only]
    pub quest_finish_script: Option<PropertyDoubleKeyHashMap<i32, i32, HashMap<String, u64>>>,
    #[tag = 30]
    #[server_only]
    pub quest_finish_scripts_id: Option<PropertyHashSet<i32>>,
    #[tag = 31]
    #[server_only]
    pub total_received_reward: Option<PropertyHashMap<i32, i32>>,
    #[tag = 32]
    #[property_object(u8, 0x01)]
    pub last_vhs_npc_info: Option<Vec<VHSNpcInfo>>,
    #[tag = 33]
    #[server_only]
    pub vhs_npc_info: Option<Vec<VHSNpcInfo>>,
    #[tag = 34]
    #[server_only]
    pub npc_info: Option<PropertyHashSet<i32>>,
    #[tag = 35]
    #[server_only]
    pub total_received_reward_times: Option<i32>,
}

#[derive(OctData, Clone, Debug, Default)]
#[property_object]
pub struct OperationMailReceiveInfo {
    #[tag = 1]
    pub receive_list: Option<PropertyHashSet<i32>>,
    #[tag = 2]
    pub condition_progress: Option<PropertyDoubleKeyHashMap<i32, i32, i32>>,
}

#[derive(OctData, Clone, Debug)]
#[property_object]
pub struct PayInfo {
    #[tag = 1]
    pub month_total_pay: Option<i32>,
}

#[derive(OctData, Clone, Debug)]
pub struct NpcSceneData {
    pub section_id: i32,
    pub transform: Transform,
}

#[derive(OctData, Clone, Debug)]
pub struct NpcInfo {
    pub uid: u64,
    pub id: i32,
    pub tag_value: i32,
    pub scene_uid: u64,
    pub parent_uid: u64,
    pub owner_uid: u64,
    pub scene_data: NpcSceneData,
    pub references: PropertyHashSet<u64>,
}

#[derive(OctData, Clone, Debug)]
#[property_object]
pub struct BattleEventInfo {
    #[tag = 1]
    #[server_only]
    pub unlock_battle: Option<PropertyHashSet<i32>>,
    #[tag = 2]
    #[server_only]
    pub unlock_battle_condition_progress: Option<PropertyDoubleKeyHashMap<i32, i32, i32>>,
    #[tag = 3]
    #[server_only]
    pub alread_rand_battle: Option<PropertyDoubleKeyHashMap<i32, i32, HashSet<i32>>>,
    #[tag = 4]
    pub rand_battle_type: Option<PropertyHashMap<i32, HollowBattleEventType>>,
    #[tag = 5]
    #[property_object(u8, 0x01)]
    pub alread_battle_stage: Option<Vec<String>>,
}

#[derive(OctData, Clone, Debug)]
#[property_object]
pub struct GMData {
    #[tag = 1]
    #[server_only]
    pub condition_proress: Option<PropertyDoubleKeyHashMap<String, i32, i32>>,
    #[tag = 2]
    #[server_only]
    pub completed_conditions: Option<PropertyHashSet<String>>,
    #[tag = 3]
    #[server_only]
    pub register_conditions: Option<PropertyHashSet<String>>,
}

#[derive(OctData, Clone, Debug)]
pub struct PlayerMailExtInfo {
    pub timestamp: u64,
    pub mail_state: MailState,
}

#[derive(OctData, Clone, Debug)]
#[property_object]
pub struct PlayerMailExtInfos {
    #[tag = 1]
    pub player_mail_ext_info: Option<PropertyHashMap<String, PlayerMailExtInfo>>,
}

#[derive(OctData, Clone, Debug)]
pub struct DungeonTable {
    pub uid: u64,
    pub id: i32,
    pub begin_timestamp: u64,
    pub dungeon_ext: DungeonTableExt,
    pub to_be_destroyed: bool,
}

#[derive(OctData, Clone, Debug)]
pub struct SceneTable {
    pub uid: u64,
    pub id: i32,
    pub begin_timestamp: u64,
    pub scene_ext: SceneTableExt,
    pub to_be_destroyed: bool,
}

#[derive(OctData, Clone, Debug)]
pub struct SectionInfo {
    pub id: i32,
    pub scene_uid: u64,
    pub event_graphs_info: EventGraphsInfo,
    pub section_info_ext: SectionInfoExt,
}

#[derive(OctData, Clone, Debug, Default)]
#[property_object]
#[property_accessors]
pub struct SingleDungeonGroup {
    #[tag = 1]
    pub dungeons: Option<PropertyHashMap<u64, DungeonTable>>,
    #[tag = 2]
    pub scenes: Option<PropertyDoubleKeyHashMap<u64, u64, SceneTable>>,
    #[tag = 3]
    pub section: Option<PropertyDoubleKeyHashMap<u64, i32, SectionInfo>>,
    #[tag = 4]
    pub npcs: Option<PropertyDoubleKeyHashMap<u64, u64, NpcInfo>>,
}

#[derive(OctData, Clone, Debug)]
#[property_object]
pub struct NewbieInfo {
    #[tag = 1]
    pub unlocked_id: Option<PropertyHashSet<i32>>,
    #[tag = 2]
    #[server_only]
    pub condition_progress: Option<PropertyDoubleKeyHashMap<i32, i32, i32>>,
}

#[derive(OctData, Clone, Debug)]
#[property_object]
pub struct LoadingPageTipsInfo {
    #[tag = 1]
    pub unlocked_id: Option<PropertyHashSet<i32>>,
    #[tag = 2]
    #[server_only]
    pub condition_progress: Option<PropertyDoubleKeyHashMap<i32, i32, i32>>,
}

#[derive(OctData, Clone, Debug, Default)]
#[property_object]
pub struct CollectMap {
    #[tag = 1]
    pub card_map: Option<PropertyHashSet<i32>>,
    #[tag = 2]
    pub curse_map: Option<PropertyHashSet<i32>>,
    #[tag = 3]
    pub event_icon_map: Option<PropertyHashSet<i32>>,
    #[tag = 4]
    #[server_only]
    pub unlock_cards: Option<PropertyHashSet<i32>>,
    #[tag = 5]
    #[server_only]
    pub unlock_card_condition_progress: Option<PropertyDoubleKeyHashMap<i32, i32, i32>>,
    #[tag = 6]
    #[server_only]
    pub unlock_curses: Option<PropertyHashSet<i32>>,
    #[tag = 7]
    #[server_only]
    pub unlock_curse_condition_progress: Option<PropertyDoubleKeyHashMap<i32, i32, i32>>,
    #[tag = 8]
    #[server_only]
    pub unlock_events: Option<PropertyHashSet<i32>>,
    #[tag = 9]
    #[server_only]
    pub unlock_event_condition_progress: Option<PropertyDoubleKeyHashMap<i32, i32, i32>>,
    #[tag = 10]
    #[server_only]
    pub unlock_event_icons: Option<PropertyHashSet<i32>>,
    #[tag = 11]
    #[server_only]
    pub unlock_event_icon_condition_progress: Option<PropertyDoubleKeyHashMap<i32, i32, i32>>,
    #[tag = 12]
    pub new_card_map: Option<PropertyHashSet<i32>>,
    #[tag = 13]
    pub new_curse_map: Option<PropertyHashSet<i32>>,
    #[tag = 14]
    pub new_event_icon_map: Option<PropertyHashSet<i32>>,
}

#[derive(OctData, Clone, Debug)]
pub struct AreaNPCInfo {
    pub tag_id: i32,
    pub interacts: PropertyHashSet<i32>,
}

#[derive(OctData, Clone, Debug)]
pub struct AreaOwnerInfo {
    pub owner_type: u16,
    pub owner_id: i32,
    pub npcs: PropertyHashMap<u64, AreaNPCInfo>,
    pub sequence: u32,
}

#[derive(OctData, Clone, Debug)]
#[property_object]
pub struct AreasInfo {
    #[tag = 1]
    pub area_owners_info: Option<PropertyDoubleKeyHashMap<u16, i32, AreaOwnerInfo>>,
    #[tag = 2]
    pub sequence: Option<u32>,
}

#[derive(OctData, Clone, Debug)]
#[property_object]
#[property_accessors]
pub struct BGMInfo {
    #[tag = 1]
    pub bgm_id: Option<u32>,
}

#[derive(OctData, Clone, Debug)]
#[property_object]
pub struct HollowInfo {
    #[tag = 1]
    pub banned_hollow_event: Option<PropertyHashSet<i32>>,
}
