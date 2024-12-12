use std::{
    net::SocketAddr,
    sync::{Arc, Weak},
    time::Duration,
};

use crate::{
    protocol::protocol_entity::ProtocolEntity,
    protocol::protocol_service::ProtocolServiceFrontendImpl, ProtocolContext,
};

pub struct ProtocolPoint {
    service_backend: Weak<ProtocolServiceFrontendImpl>,
    entity: Arc<ProtocolEntity>,
    channel: u16,
}

impl ProtocolPoint {
    pub fn new(
        service_backend: &Arc<ProtocolServiceFrontendImpl>,
        entity: Arc<ProtocolEntity>,
        channel: u16,
    ) -> Self {
        Self {
            service_backend: Arc::downgrade(service_backend),
            entity,
            channel,
        }
    }

    pub fn register_rpc_call(&self, cb: impl Fn(ProtocolContext) + Send + Sync + 'static) {
        self.entity
            .get_channel_info_mut(self.channel)
            .unwrap()
            .callback = Some(Box::new(cb));
    }

    pub async fn send_rpc(
        &self,
        addr: SocketAddr,
        rpc_arg: Box<[u8]>,
        to_channel: u16,
        arg_uid: u64,
    ) {
        let _ = self
            .service_backend
            .upgrade()
            .unwrap()
            .send_rpc(
                self.entity.clone(),
                self.channel,
                to_channel,
                addr,
                &rpc_arg,
                Duration::ZERO,
                arg_uid == 0,
                arg_uid,
            )
            .await;
    }

    pub async fn call_rpc(
        &self,
        addr: SocketAddr,
        rpc_arg: &[u8],
        timeout: Duration,
    ) -> Option<Box<[u8]>> {
        self.service_backend
            .upgrade()
            .unwrap()
            .send_rpc(
                self.entity.clone(),
                self.channel,
                0,
                addr,
                rpc_arg,
                timeout,
                false,
                0,
            )
            .await
            .inspect_err(|err| tracing::error!("ProtocolPoint::CallRpc -> send_rpc failed: {err}"))
            .ok()
            .flatten()
    }

    pub fn close(&self, addr: SocketAddr) {
        self.service_backend
            .upgrade()
            .unwrap()
            .close_session(&self.entity, addr);
    }

    pub fn get_channel(&self) -> u16 {
        self.channel
    }
}
