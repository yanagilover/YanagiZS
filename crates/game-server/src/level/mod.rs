use std::{
    collections::{HashMap, VecDeque},
    sync::OnceLock,
};

use event_graph_runner::EventGraphGroup;
use protocol::PtcSyncEventInfoArg;
use qwer_rpc::RpcPtcContext;
use tracing::instrument;
use yanagi_eventgraph::MainCityConfig;

use crate::PlayerSession;

mod event_graph_runner;

static MAINCITY_CONFIG: OnceLock<MainCityConfig> = OnceLock::new();

pub fn load_script_config(main_city_config_data: &str) {
    let _ = MAINCITY_CONFIG.set(
        serde_json::from_str(main_city_config_data).expect("failed to load main city config data"),
    );
}

pub struct BoundInteractInfo {
    pub interact_id: i32,
    pub participators: HashMap<u32, String>,
    pub name: String,
    pub scale_x: f64,
    pub scale_y: f64,
    pub scale_z: f64,
    pub scale_w: f64,
    pub scale_r: f64,
}

#[derive(Default)]
pub struct LevelEventGraphManager {
    pub bound_interact_map: HashMap<u64, (i32, BoundInteractInfo)>,
    pub listen_events: HashMap<i32, HashMap<String, String>>,
    pub scene_uid: u64,
    pub section_id: i32,
    pending_events_info_sync: VecDeque<PtcSyncEventInfoArg>,
    cur_interaction: i32,
    cur_interact_unit_tag: i32,
}

impl LevelEventGraphManager {
    pub fn begin_interact(&mut self, interaction: i32, unit_tag: i32) {
        self.cur_interaction = interaction;
        self.cur_interact_unit_tag = unit_tag;
    }

    pub fn push_protocol_action(&mut self, action_info: protocol::ActionInfo) {
        self.pending_events_info_sync
            .push_back(PtcSyncEventInfoArg {
                owner_id: self.cur_interaction as u32,
                npc_interaction: String::from("OnInteract"),
                tag: self.cur_interact_unit_tag as u32,
                owner_type: 3, // SceneUnit = 3,
                action_list: vec![action_info],
            });
    }

    pub async fn sync_event_info(&mut self, ctx: &RpcPtcContext) {
        while let Some(ptc) = self.pending_events_info_sync.pop_front() {
            ctx.send_ptc(ptc).await;
        }
    }
}

#[instrument(skip(session))]
pub fn on_section_added(session: &mut PlayerSession, scene_uid: u64, section_id: i32) {
    let section_config = MAINCITY_CONFIG
        .get()
        .unwrap()
        .sections
        .get(&section_id)
        .unwrap();

    event_graph_runner::trigger_group(
        session,
        &section_config.section_progress,
        EventGraphGroup::OnAdd,
        scene_uid,
        section_id,
    );
}

#[instrument(skip(session))]
pub fn on_section_enter(session: &mut PlayerSession, scene_uid: u64, section_id: i32) {
    let section_config = MAINCITY_CONFIG
        .get()
        .unwrap()
        .sections
        .get(&section_id)
        .unwrap();

    session.level_event_graph_mgr.scene_uid = scene_uid;
    session.level_event_graph_mgr.section_id = section_id;
    session.level_event_graph_mgr.bound_interact_map.clear();

    event_graph_runner::trigger_group(
        session,
        &section_config.section_progress,
        EventGraphGroup::OnEnter,
        scene_uid,
        section_id,
    );
}

#[instrument(skip(session))]
pub fn fire_event(session: &mut PlayerSession, interact_id: i32, event_name: &str) {
    if let Some(event_graph_name) = session
        .level_event_graph_mgr
        .listen_events
        .get(&interact_id)
        .map(|e| e.get(event_name))
        .flatten()
        .cloned()
    {
        let section_config = MAINCITY_CONFIG
            .get()
            .unwrap()
            .sections
            .get(&session.level_event_graph_mgr.section_id)
            .unwrap();

        event_graph_runner::trigger_event(
            session,
            event_name,
            section_config
                .section_progress
                .events
                .get(&event_graph_name)
                .unwrap(),
            session.level_event_graph_mgr.scene_uid,
            session.level_event_graph_mgr.section_id,
        );
    }
}
