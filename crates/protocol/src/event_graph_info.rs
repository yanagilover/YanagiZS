use protocol_macros::polymorphic;
use qwer::{OctData, PropertyHashMap};

use crate::{player_info::EventInfo, InteractInfo};

#[derive(OctData, Clone, Debug)]
pub struct EventListenerInfo {
    pub event_graph_id: i32,
    pub events_to_trigger: Vec<String>,
}

polymorphic!(
    EventGraphInfo [
        events_info: PropertyHashMap<i32, EventInfo>,
        specials: PropertyHashMap<String, u64>,
        is_new: bool,
        finished: bool,
        list_specials: PropertyHashMap<String, Vec<u64>>,
    ]
    Hollow {
        fired_count: u8,
        hollow_event_template_id: i32,
        uid: u64,
        is_created_by_gm: bool,
    } = 3,
    NPC {
        sequence_of_group: u16,
        section_list_events: PropertyHashMap<String, EventListenerInfo>,
        interact_info: InteractInfo,
        hide: bool,
    } = 2,
    Section { } = 1,
);
