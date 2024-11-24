use common::time_util;
use protocol::{
    item_info::ItemInfo,
    player_info::{
        ArchiveInfo, AreasInfo, BGMInfo, BattleEventInfo, BeginnerProcedureInfo, CollectMap,
        DungeonCollection, Embattles, EquipGachaInfo, FairyInfo, GMData, HollowInfo,
        LoadingPageTipsInfo, MUIPData, MainCityQuestData, NewbieInfo, OperationMailReceiveInfo,
        PayInfo, PlayerInfo, PlayerMailExtInfos, PlayerPosInMainCity, PopupWindowInfo, QuestData,
        RamenData, ShopsInfo, SingleDungeonGroup, TipsInfo, Transform, UnlockInfo, VHSStoreData,
        Vector3f, VideotapeInfo, YorozuyaInfo,
    },
    AutoRecoveryInfo,
};
use qwer::{
    pdkhashmap, phashmap, phashset, PropertyDoubleKeyHashMap, PropertyHashMap, PropertyHashSet,
};

use crate::FILECFG;

pub struct UidCounter {
    player_uid: u32,
    counter: u32,
}

impl UidCounter {
    pub fn new(player_uid: u32, last_uid: u32) -> Self {
        Self {
            player_uid,
            counter: last_uid,
        }
    }

    pub fn next(&mut self) -> u64 {
        self.counter += 1;
        ((self.player_uid as u64) << 32) | self.counter as u64
    }

    pub fn last_uid(&self) -> u32 {
        self.counter
    }
}

pub fn create_starting_player_info(uid: u64, nick_name: &str) -> (UidCounter, PlayerInfo) {
    let mut counter = UidCounter::new((uid & 0xFFFFFFFF) as u32, 0);
    let mut player_info = PlayerInfo {
        uid: Some(uid),
        account_name: Some(uid.to_string()),
        last_enter_world_timestamp: Some(0),
        items: Some(phashmap!()),
        dungeon_collection: Some(DungeonCollection {
            dungeons: Some(qwer::phashmap![]),
            scenes: Some(qwer::phashmap![]),
            default_scene_uid: Some(0),
            transform: Some(Transform::default()),
            used_story_mode: Some(true),
            used_manual_qte_mode: Some(true),
        }),
        properties: Some(pdkhashmap![]),
        scene_properties: Some(pdkhashmap![]),
        quest_data: Some(QuestData {
            quests: Some(pdkhashmap![]),
            is_afk: Some(false),
            unlock_condition_progress: Some(pdkhashmap![]),
            world_quest_collection_uid: Some(0),
            world_quest_for_cur_dungeon: Some(0),
            world_quest_for_cur_dungeon_afk: Some(0),
        }),
        joined_chat_rooms: Some(Vec::with_capacity(0)),
        scene_uid: Some(0),
        archive_info: Some(ArchiveInfo {
            videotaps_info: Some(phashmap![(
                1010001,
                VideotapeInfo {
                    finished: true,
                    star_count: phashmap![],
                    awarded_star: phashmap![],
                }
            )]),
            hollow_archive_id: Some(phashset![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]),
        }),
        auto_recovery_info: Some(phashmap![(
            501,
            AutoRecoveryInfo {
                buy_times: 0,
                last_recovery_timestamp: 0,
            }
        )]),
        unlock_info: Some(UnlockInfo {
            condition_progress: Some(pdkhashmap![]),
            unlocked_list: Some(phashset![]),
        }),
        yorozuya_info: Some(YorozuyaInfo {
            yorozuya_level: Some(1),
            yorozuya_rank: Some(1),
            gm_enabled: Some(true),
            gm_quests: Some(phashmap![]),
            finished_hollow_quest_count: Some(0),
            finished_hollow_quest_count_of_type: Some(phashmap![]),
            hollow_quests: Some(pdkhashmap![]),
            urgent_quests_queue: Some(phashmap![]),
            unlock_hollow_id: Some(vec![1001]),
            unlock_hollow_id_progress: Some(pdkhashmap![]),
            last_refresh_timestamp_common: Some(0),
            last_refresh_timestamp_urgent: Some(0),
            next_refresh_timestamp_urgent: Some(0),
        }),
        equip_gacha_info: Some(EquipGachaInfo {
            avatar_level_advance_times: Some(0),
            equip_star_up_times: Some(0),
            security_num_by_lv: Some(phashmap![]),
            smithy_level: Some(0),
            total_gacha_times: Some(0),
        }),
        beginner_procedure_info: Some(BeginnerProcedureInfo {
            procedure_id: Some(0),
        }),
        pos_in_main_city: Some(PlayerPosInMainCity {
            position: Some(Vector3f {
                x: 17.35,
                y: 0.37,
                z: 6.01,
            }),
            rotation: Some(Vector3f {
                x: 0.0,
                y: 216.0,
                z: 0.0,
            }),
            initial_pos_id: Some(String::from("Workshop_PlayerPos_Default")),
        }),
        fairy_info: Some(FairyInfo {
            condition_progress: Some(pdkhashmap![]),
            fairy_groups: Some(phashmap![]),
        }),
        popup_window_info: Some(PopupWindowInfo {
            condition_progress: Some(pdkhashmap![]),
            popup_window_list: Some(Vec::new()),
        }),
        tips_info: Some(TipsInfo {
            tips_list: Some(Vec::new()),
            tips_group: Some(Vec::new()),
            tips_condition_progress: Some(pdkhashmap![]),
            tips_group_condition_progress: Some(pdkhashmap![]),
        }),
        main_city_quest_data: Some(MainCityQuestData {
            in_progress_quests: Some(Vec::new()),
            exicing_finish_script_group: Some(vec![10020001]),
        }),
        embattles: Some(Embattles {
            last_embattles: Some(phashmap![]),
        }),
        day_change_info: Some(protocol::player_info::DayChangeInfo {
            last_daily_refresh_timing: Some(time_util::unix_timestamp()),
        }),
        npcs_info: Some(protocol::player_info::PlayerNPCsInfo {
            npcs_info: Some(phashmap![]),
            destroy_npc_when_leave_section: Some(phashset![]),
        }),
        scripts_to_execute: Some(pdkhashmap![]),
        scripts_to_remove: Some(phashmap![]),
        last_leave_world_timestamp: Some(0),
        muip_data: Some(MUIPData {
            ban_begin_time: Some(String::with_capacity(0)),
            ban_end_time: Some(String::with_capacity(0)),
            tag_value: Some(0),
            dungeon_enter_times: Some(phashmap![]),
            scene_enter_times: Some(phashmap![]),
            dungeon_pass_times: Some(phashmap![]),
            scene_pass_times: Some(phashmap![]),
            alread_cmd_uids: Some(phashset![]),
            game_total_time: Some(0),
            language_type: Some(0),
        }),
        nick_name: Some(nick_name.to_string()),
        ramen_data: Some(RamenData {
            unlock_ramen: Some(phashset![20301, 20401, 20501, 20601, 20201]),
            cur_ramen: Some(0),
            used_times: Some(0),
            unlock_initiative_item: Some(phashset![]),
            unlock_ramen_condition_progress: Some(pdkhashmap![]),
            unlock_item_condition_progress: Some(pdkhashmap![]),
            has_mystical_spice: Some(true),
            unlock_has_mystical_spice_condition_progress: Some(phashmap![]),
            cur_mystical_spice: Some(0),
            unlock_mystical_spice: Some(phashset![
                30101, 30601, 30201, 30501, 30301, 30801, 31201, 30401, 31401, 31001
            ]),
            unlock_mystical_spice_condition_progress: Some(pdkhashmap![]),
            unlock_initiative_item_group: Some(phashset![]),
            hollow_item_history: Some(phashmap![]),
            initial_item_ability: Some(0),
            new_unlock_ramen: Some(Vec::new()),
            eat_ramen_times: Some(0),
            make_hollow_item_times: Some(0),
            new_unlock_initiative_item: Some(phashset![]),
        }),
        shop: Some(ShopsInfo {
            shops: Some(phashmap![]),
            shop_buy_times: Some(0),
            vip_level: Some(0),
        }),
        vhs_store_data: Some(VHSStoreData {
            store_level: Some(0),
            unreceived_reward: Some(0),
            hollow_enter_times: Some(0),
            last_receive_time: Some(0),
            vhs_collection_slot: Some(Vec::new()),
            unlock_vhs_collection: Some(phashset![]),
            already_trending: Some(phashset![]),
            unlock_trending_condition_progress: Some(pdkhashmap![]),
            is_need_refresh: Some(true),
            scripts_id: Some(phashset![]),
            store_exp: Some(0),
            is_level_chg_tips: Some(true),
            vhs_hollow: Some(Vec::new()),
            is_receive_trending_reward: Some(false),
            is_need_first_trending: Some(false),
            last_basic_script: Some(0),
            is_complete_first_trending: Some(false),
            last_basic_npc: Some(0),
            can_random_trending: Some(phashset![]),
            vhs_trending_info: Some(Vec::new()),
            unlock_vhs_trending_info: Some(phashmap![]),
            vhs_flow: Some(0),
            received_reward: Some(0),
            last_reward: Some(0),
            last_exp: Some(0),
            last_flow: Some(0),
            last_vhs_trending_info: Some(Vec::new()),
            new_know_trend: Some(Vec::new()),
            quest_finish_script: Some(pdkhashmap![]),
            quest_finish_scripts_id: Some(phashset![]),
            total_received_reward: Some(phashmap![]),
            last_vhs_npc_info: Some(Vec::new()),
            vhs_npc_info: Some(Vec::new()),
            npc_info: Some(phashset![]),
            total_received_reward_times: Some(0),
        }),
        operation_mail_receive_info: Some(OperationMailReceiveInfo {
            receive_list: Some(phashset![]),
            condition_progress: Some(pdkhashmap![]),
        }),
        second_last_enter_world_timestamp: Some(0),
        login_times: Some(0),
        create_timestamp: Some(time_util::unix_timestamp()),
        gender: Some(0),
        avatar_id: Some(2021),
        prev_scene_uid: Some(0),
        register_cps: Some(String::with_capacity(0)),
        register_platform: Some(3),
        pay_info: Some(PayInfo {
            month_total_pay: Some(0),
        }),
        private_npcs: Some(phashmap![]),
        battle_event_info: Some(BattleEventInfo {
            unlock_battle: Some(phashset![]),
            unlock_battle_condition_progress: Some(pdkhashmap![]),
            alread_rand_battle: Some(pdkhashmap![]),
            alread_battle_stage: Some(Vec::new()),
            rand_battle_type: Some(phashmap![]),
        }),
        gm_data: Some(GMData {
            register_conditions: Some(phashset![]),
            condition_proress: Some(pdkhashmap![]),
            completed_conditions: Some(phashset![]),
        }),
        player_mail_ext_infos: Some(PlayerMailExtInfos {
            player_mail_ext_info: Some(phashmap![]),
        }),
        single_dungeon_group: Some(SingleDungeonGroup {
            dungeons: Some(phashmap![]),
            scenes: Some(pdkhashmap![]),
            npcs: Some(pdkhashmap![]),
            section: Some(pdkhashmap![]),
        }),
        newbie_info: Some(NewbieInfo {
            unlocked_id: Some(phashset![3]),
            condition_progress: Some(pdkhashmap!()),
        }),
        loading_page_tips_info: Some(LoadingPageTipsInfo {
            unlocked_id: Some(phashset![1, 2, 3]),
            condition_progress: Some(pdkhashmap![]),
        }),
        switch_of_story_mode: Some(true),
        switch_of_qte: Some(true),
        collect_map: Some(CollectMap {
            card_map: Some(phashset![]),
            curse_map: Some(phashset![]),
            unlock_cards: Some(phashset![]),
            unlock_curses: Some(phashset![]),
            event_icon_map: Some(phashset![]),
            unlock_events: Some(phashset![]),
            new_card_map: Some(phashset![]),
            new_curse_map: Some(phashset![]),
            new_event_icon_map: Some(phashset![]),
            unlock_event_icon_condition_progress: Some(pdkhashmap![]),
            unlock_card_condition_progress: Some(pdkhashmap![]),
            unlock_curse_condition_progress: Some(pdkhashmap![]),
            unlock_event_condition_progress: Some(pdkhashmap![]),
            unlock_event_icons: Some(phashset![]),
        }),
        areas_info: Some(AreasInfo {
            area_owners_info: Some(pdkhashmap!()),
            sequence: Some(0),
        }),
        bgm_info: Some(BGMInfo { bgm_id: Some(1005) }),
        main_city_objects_state: Some(phashmap!()),
        hollow_info: Some(HollowInfo {
            banned_hollow_event: Some(phashset!()),
        }),
        main_city_avatar_id: Some(1221),
    };

    // Give all avatars
    FILECFG
        .get()
        .unwrap()
        .avatar_base_template_tb
        .data()
        .unwrap_or_default()
        .iter()
        .filter(|tmpl| tmpl.camp() != 0)
        .for_each(|tmpl| {
            let uid = counter.next();
            player_info.items.as_mut().unwrap().insert(
                uid,
                ItemInfo::AvatarInfo {
                    uid,
                    id: tmpl.id(),
                    count: 1,
                    package: 0,
                    first_get_time: time_util::unix_timestamp(),
                    star: 6,
                    exp: 0,
                    level: 60,
                    rank: 6,
                    unlocked_talent_num: 6,
                    talent_switch: (0..6).map(|i| i >= 3).collect(),
                    skills: PropertyHashMap::Base((0..=6).map(|st| (st, 1)).collect()),
                    is_custom_by_dungeon: false,
                    robot_id: 0,
                },
            );
        });

    (counter, player_info)
}
