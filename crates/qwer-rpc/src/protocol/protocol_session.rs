use std::{
    fmt,
    net::SocketAddr,
    sync::{atomic::AtomicU64, Arc},
};

use common::time_util;
use tokio::sync::OnceCell;

use crate::{
    object_res_mini_mgr::ResObj, protocol::protocol_entity::ProtocolEntity, ProtocolLinker,
};

#[derive(Clone)]
pub struct ProtocolSession {
    pub session_id: u64,
    pub local_channel: u16,
    pub remote_channel: u16,
    pub remote_addr: SocketAddr,
}

pub struct ProtocolSessionImpl {
    pub is_connector: bool,
    pub linker: ProtocolLinker,
    pub entity: Arc<ProtocolEntity>,
    pub last_active_time: AtomicU64,
    uid: OnceCell<u64>,
}

pub struct ProtocolContext {
    pub session: ProtocolSession,
    pub body: Box<[u8]>,
    pub rpc_arg_uid: u64,
    pub local_channel: u16,
    pub remote_channel: u16,
}

impl fmt::Display for ProtocolSession {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}/{}/{}",
            self.session_id, self.local_channel, self.remote_channel
        )
    }
}

impl ProtocolSessionImpl {
    pub fn new(is_connector: bool, linker: ProtocolLinker, entity: Arc<ProtocolEntity>) -> Self {
        ProtocolSessionImpl {
            is_connector,
            linker,
            entity,
            last_active_time: AtomicU64::new(time_util::unix_timestamp()),
            uid: OnceCell::new(),
        }
    }
}

impl ResObj for Arc<ProtocolSessionImpl> {
    fn set_uid(&self, uid: u64) {
        let _ = self.uid.set(uid);
    }

    fn get_uid(&self) -> u64 {
        self.uid.get().copied().unwrap_or_default()
    }
}
