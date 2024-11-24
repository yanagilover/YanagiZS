use std::{
    io::Cursor,
    net::{IpAddr, Ipv4Addr, SocketAddr},
    sync::{Arc, Weak},
    time::Duration,
};

use dashmap::{
    mapref::one::{Ref, RefMut},
    DashMap,
};
use qwer::ProtocolStream;
use tokio::{sync::OnceCell, time::timeout};

use crate::{
    object_res_mini_mgr::ResObj, protocol::protocol_helper,
    protocol::protocol_point::ProtocolPoint,
    protocol::protocol_service::ProtocolServiceFrontendImpl, ProtocolContext, ProtocolLinker,
    ProtocolListener, ProtocolSessionImpl,
};

pub struct ProtocolEntity {
    service_backend: OnceCell<Weak<ProtocolServiceFrontendImpl>>,
    local_addr: Option<SocketAddr>,
    protocol_listener: Option<ProtocolListener>,
    used_channels: DashMap<u16, ProtocolChannelInfo>,
    pub sessions: DashMap<SocketAddr, Arc<ProtocolSessionImpl>>,
    pub session_uid_map: DashMap<u64, SocketAddr>,
}

pub struct ProtocolChannelInfo {
    pub callback: Option<Box<dyn Fn(ProtocolContext) + Send + Sync>>,
}

#[derive(thiserror::Error, Debug)]
enum AcceptError {
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),
    #[error("Invalid addr len")]
    InvalidAddrLen,
    #[error("Invalid addr format")]
    InvalidAddrFormat,
}

impl ProtocolEntity {
    pub fn new(local_addr: Option<SocketAddr>, listener: Option<ProtocolListener>) -> Self {
        Self {
            service_backend: OnceCell::new(),
            local_addr,
            protocol_listener: listener,
            used_channels: DashMap::new(),
            sessions: DashMap::new(),
            session_uid_map: DashMap::new(),
        }
    }

    pub async fn connect(
        self: Arc<Self>,
        addr: SocketAddr,
        no_reverse: bool,
    ) -> std::io::Result<u64> {
        if let Some(session_impl) = self.sessions.get(&addr) {
            return Ok(session_impl.get_uid());
        }

        let linker = protocol_helper::connect(addr).await?;
        let protocol_session = Arc::new(ProtocolSessionImpl::new(true, linker, self.clone()));
        let uid = self
            .service_backend()
            .session_mgr
            .insert(protocol_session.clone());

        let mut buf: Vec<u8> = Vec::new();
        let mut ps = ProtocolStream::new(Cursor::new(&mut buf));
        if let Some(local_addr) = self.local_addr {
            let IpAddr::V4(ip) = local_addr.ip() else {
                unreachable!();
            };

            ps.push_boolean(true)?;
            let octets = ip.octets();
            ps.push_u8(octets.len() as u8)?;

            if no_reverse {
                ps.push(&octets)?;
            } else {
                // it's hell
                todo!()
            }

            ps.push_u16(local_addr.port())?;
            ps.push_boolean(no_reverse)?;
        } else {
            ps.push_boolean(false)?;
            ps.push_u8(0)?;
        }

        protocol_session.linker.send(&buf).await;

        self.sessions.insert(addr, protocol_session.clone());
        self.session_uid_map
            .insert(protocol_session.get_uid(), addr);

        tokio::spawn(async move { self.service_backend().begin_recv(protocol_session).await });
        Ok(uid)
    }

    pub fn get_channel_info(&self, channel_id: u16) -> Option<Ref<'_, u16, ProtocolChannelInfo>> {
        self.used_channels.get(&channel_id)
    }

    pub fn get_channel_info_mut(
        &self,
        channel_id: u16,
    ) -> Option<RefMut<'_, u16, ProtocolChannelInfo>> {
        self.used_channels.get_mut(&channel_id)
    }

    pub fn set_backend(&self, backend: Arc<ProtocolServiceFrontendImpl>) {
        let _ = self.service_backend.set(Arc::downgrade(&backend));
    }

    pub fn create_point(self: Arc<Self>, channel: u16) -> Option<ProtocolPoint> {
        if self.used_channels.contains_key(&channel) {
            return None;
        }

        self.used_channels
            .insert(channel, ProtocolChannelInfo { callback: None });

        Some(ProtocolPoint::new(
            &self.service_backend(),
            self.clone(),
            channel,
        ))
    }

    fn service_backend(&self) -> Arc<ProtocolServiceFrontendImpl> {
        self.service_backend
            .get()
            .unwrap()
            .upgrade()
            .expect("backend gone?")
    }

    pub fn has_listener(&self) -> bool {
        self.protocol_listener.is_some()
    }

    pub async fn accept(self: &Arc<Self>) -> std::io::Result<Arc<ProtocolSessionImpl>> {
        loop {
            let mut linker = self.protocol_listener.as_ref().unwrap().accept().await?;
            let Ok(result) = timeout(
                Duration::from_millis(200),
                Self::proceed_accept(&mut linker),
            )
            .await
            else {
                continue;
            };

            let remote_addr = match result {
                Ok(r) => r,
                Err(err) => {
                    tracing::warn!("accept error, skipping client: {err}");
                    continue;
                }
            };

            let session = Arc::new(ProtocolSessionImpl::new(false, linker, self.clone()));

            self.sessions
                .insert(session.linker.remote_addr, session.clone());
            if let Some(addr) = remote_addr {
                self.sessions.insert(addr, session.clone());
            }

            return Ok(session);
        }
    }

    async fn proceed_accept(
        linker: &mut ProtocolLinker,
    ) -> Result<Option<SocketAddr>, AcceptError> {
        let mut buf = [0u8; 9];
        let mut n = 0;

        while n < 2 {
            n += linker.recv_some(&mut buf[..2]).await?;
        }

        let mut ps = ProtocolStream::new(Cursor::new(&buf));
        let has_addr = ps.pop_boolean()?;
        let addr_size = ps.pop_u8()? as usize;

        if has_addr {
            if addr_size != 4 {
                return Err(AcceptError::InvalidAddrLen);
            }

            while n < 5 + addr_size {
                n += linker.recv_some(&mut buf[n..]).await?;
            }

            let mut ps = ProtocolStream::new(Cursor::new(&buf[2..]));
            let octets = ps.pop(addr_size)?;
            let port = ps.pop_u16()?;
            let no_reverse = ps.pop_boolean()?;
            if !no_reverse {
                return Err(AcceptError::InvalidAddrFormat); // not supported yet
            }

            Ok(Some(SocketAddr::new(
                IpAddr::V4(Ipv4Addr::new(octets[0], octets[1], octets[2], octets[3])),
                port,
            )))
        } else {
            if addr_size != 0 {
                return Err(AcceptError::InvalidAddrLen);
            }

            Ok(None)
        }
    }
}
