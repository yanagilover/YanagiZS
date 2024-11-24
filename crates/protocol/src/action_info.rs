use protocol_macros::polymorphic;
use qwer::{OctData, PropertyHashMap};

use crate::{player_info::ChoiceInfo, HollowShopCurrency, HollowShopType};

#[derive(OctData, Clone, Debug)]
pub struct ConfigItem {
    pub uid: i32,
    pub item_id: i32,
    pub count: i32,
    pub value: i32,
    pub base_value: i32,
    pub discount: i32,
}

#[derive(OctData, Clone, Debug)]
pub struct ConfigShopInfo {
    pub goods: Vec<ConfigItem>,
    pub currency: HollowShopCurrency,
}

polymorphic!(
    ActionInfo []
    ServerChoices {
        choices: Vec<ChoiceInfo>,
        finished: bool,
    } = 52,
    DropHollowItem {
        drop_item: i32,
    } = 162,
    FinishBlackout {
        finished: bool,
        show_tips: bool,
    } = 133,
    Loop {
        loop_times: u16,
    } = 141,
    Perform {
        step: u8,
        r#return: PropertyHashMap<String, i32>,
    } = 23,
    /*PrepareNextHollow {
        section_id: i32,
        finished: bool,
        show_other: bool,
        main_map: HollowGridMapProtocolInfo,
    } = 130,*/ // TODO!
    ActionRandomChallenge {
        choices: Vec<i32>,
        choice_result: i32,
        finished: bool,
    } = 109,
    RemoveCurse {
        curse_can_remove: Vec<u64>,
        to_remove_num: u8,
        choosed: bool,
    } = 105,
    SetHollowSystemState {
        finished: bool,
    } = 134,
    Shop {
        shop_info: PropertyHashMap<HollowShopType, ConfigShopInfo>,
        finished: bool,
    } = 62,
    SlotMachine {
        indexes: Vec<i32>,
        index: i32,
        finished: bool,
    } = 131,
    TriggerBattle {
        next_action_id: i32,
        finished: bool,
    } = 56,
);
