use protocol_macros::polymorphic;
use qwer::{OctData, PropertyHashMap};

use crate::PropertyType;

#[derive(OctData, Clone, Debug, PartialEq, Eq)]
pub struct PropertyKeyValue {
    pub key: PropertyType,
    pub value: i32,
}

polymorphic!(
    ItemInfo [
        uid: u64,
        id: i32,
        count: i32,
        package: u16,
        first_get_time: u64
    ]
    Arcana {
        affix_list: Vec<i32>,
        dress_index: u8,
    } = 33,
    AvatarInfo {
        star: u8,
        exp: u32,
        level: u8,
        rank: u8,
        unlocked_talent_num: u8,
        talent_switch: Vec<bool>,
        skills: PropertyHashMap<u8, u8>,
        is_custom_by_dungeon: bool,
        robot_id: i32
    } = 3,
    AvatarLevelUpMaterial {} = 12,
    AvatarPiece {} = 4,
    Bless {
        remain_time: i32,
        get_time: u64,
        ban_character: Vec<i32>,
        specials: PropertyHashMap<String, i32>,
        slot: u8,
        is_super_curse: bool,
    } = 32,
    Buddy {} = 8,
    Consumable {} = 10,
    Currency {} = 1,
    Equip {
        avatar_uid: u64,
        avatar_dressed_index: u8,
        rand_properties: Vec<PropertyKeyValue>,
        star: u8,
        exp: u32,
        level: u8,
        lock: u8,
        base_rand_properties: Vec<PropertyKeyValue>,
        rand_properties_lv: Vec<i32>,
    } = 7,
    EquipLevelUpMaterial { } = 14,
    Gift { } = 51,
    HollowItem { } = 15,
    OptionalGift { } = 52,
    Resource { } = 2,
    TarotCard {
        is_mute: bool,
        specials: PropertyHashMap<String, i32>,
    } = 31,
    Useable { } = 11,
    Weapon {
        avatar_uid: u64,
        star: u8,
        exp: u32,
        level: u8,
        lock: u8,
        refine_level: u8,
    } = 5,
    WeaponLevelUpMaterial { } = 13,
);
