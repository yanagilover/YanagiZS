use std::{
    io::Cursor,
    net::SocketAddr,
    sync::{
        atomic::{self, AtomicU16},
        Arc,
    },
    time::Duration,
};

use common::time_util;
use dashmap::DashMap;
use qwer::{ProtocolHeader, ProtocolStream};
use tokio::{
    sync::{oneshot, OnceCell},
    time,
};

use crate::{
    object_res_mini_mgr::{ObjectResMiniMgr, ResObj},
    protocol::protocol_entity::{ProtocolChannelInfo, ProtocolEntity},
    protocol::protocol_helper,
    protocol::protocol_point::ProtocolPoint,
    ProtocolContext, ProtocolSession, ProtocolSessionImpl,
};

pub struct RpcArgInfo {
    #[expect(unused)]
    pub session: ProtocolSession,
    pub sender: oneshot::Sender<Box<[u8]>>,
    uid: OnceCell<u64>,
}

impl RpcArgInfo {
    pub fn new(session: ProtocolSession, sender: oneshot::Sender<Box<[u8]>>) -> Self {
        Self {
            session,
            sender,
            uid: OnceCell::new(),
        }
    }
}

impl ResObj for RpcArgInfo {
    fn set_uid(&self, uid: u64) {
        let _ = self.uid.set(uid);
    }

    fn get_uid(&self) -> u64 {
        self.uid.get().copied().unwrap_or_default()
    }
}

pub struct ProtocolServiceFrontend {
    backend: Arc<ProtocolServiceFrontendImpl>,
}

impl ProtocolServiceFrontend {
    pub fn new() -> Self {
        Self {
            backend: ProtocolServiceFrontendImpl::new(),
        }
    }

    pub async fn create_point(
        &self,
        local_addr: Option<SocketAddr>,
    ) -> Result<ProtocolPoint, std::io::Error> {
        self.backend.create_point(local_addr).await
    }
}

pub struct ProtocolServiceFrontendImpl {
    protocol_entity_map: DashMap<Option<SocketAddr>, Arc<ProtocolEntity>>,
    pub session_mgr: ObjectResMiniMgr<Arc<ProtocolSessionImpl>>,
    rpc_arg_mgr: ObjectResMiniMgr<RpcArgInfo>,
    channel_counter: AtomicU16,
}

#[derive(thiserror::Error, Debug)]
pub enum ProtocolError {
    #[error("I/O Error: {0}")]
    Io(#[from] std::io::Error),
    #[error("header size doesn't match, expected: {0}, received: {1}")]
    HeaderSize(usize, usize),
}

#[derive(thiserror::Error, Debug)]
pub enum SendRpcError {
    #[error("I/O Error: {0}")]
    Io(#[from] std::io::Error),
    #[error("RPC timeout reached")]
    Timeout,
}

impl ProtocolServiceFrontendImpl {
    pub fn new() -> Arc<Self> {
        let inst = Arc::new(Self {
            protocol_entity_map: DashMap::new(),
            session_mgr: ObjectResMiniMgr::new(),
            rpc_arg_mgr: ObjectResMiniMgr::new(),
            channel_counter: AtomicU16::new(0),
        });

        let inst_clone = inst.clone();
        tokio::spawn(async move { inst_clone.check_active_sessions() });

        inst
    }

    pub async fn send_rpc(
        &self,
        entity: Arc<ProtocolEntity>,
        from_channel: u16,
        to_channel: u16,
        to_addr: SocketAddr,
        body: &[u8],
        timeout: Duration,
        is_ptc: bool,
        arg_uid: u64,
    ) -> Result<Option<Box<[u8]>>, SendRpcError> {
        let session_uid = entity.connect(to_addr, true).await?;

        let Some(session_impl) = self.session_mgr.get(session_uid).map(|s| s.clone()) else {
            tracing::error!("self.session_mgr.get({session_uid}) = None");
            return Ok(None);
        };

        let (rpc_arg_uid, receiver) = if !is_ptc && arg_uid == 0 {
            let (sender, receiver) = oneshot::channel();
            let arg_info = RpcArgInfo::new(
                ProtocolSession {
                    remote_addr: session_impl.linker.remote_addr,
                    session_id: session_impl.get_uid(),
                    local_channel: from_channel,
                    remote_channel: to_channel,
                },
                sender,
            );

            (self.rpc_arg_mgr.insert(arg_info), Some(receiver))
        } else {
            (arg_uid, None)
        };

        let header = ProtocolHeader {
            from_channel,
            to_channel: 0,
            is_rpc_ret: arg_uid != 0,
            rpc_arg_uid,
        };

        let mut buf = Vec::new();
        let mut ps = ProtocolStream::new(Cursor::new(&mut buf));
        ps.push_u16(to_channel)?;
        ps.push_u32(body.len() as u32)?;
        ps.push_u16(ProtocolHeader::SIZE as u16)?;
        header.marshal_to(&mut ps)?;
        ps.push(&body)?;

        session_impl.linker.send(&buf).await;

        match receiver {
            Some(r) => Ok(time::timeout(timeout, r)
                .await
                .map_err(|_| SendRpcError::Timeout)?
                .ok()),
            None => Ok(None),
        }
    }

    pub async fn begin_accept(self: Arc<Self>, protocol_entity: Arc<ProtocolEntity>) {
        loop {
            let protocol_session = protocol_entity.accept().await.unwrap(); // TODO: handle err
            let uid = self.session_mgr.insert(protocol_session.clone());

            tracing::info!("[SERVICE] new ProtocolSession, uid: {uid}");

            let s = self.clone();
            tokio::spawn(async { s.begin_recv(protocol_session).await });
        }
    }

    pub async fn begin_recv(self: Arc<Self>, session: Arc<ProtocolSessionImpl>) {
        let uid = session.get_uid();
        if let Err(err) = self.do_recv(session).await {
            tracing::error!("ProtocolServiceFrontendImpl: session: {uid} recv failed: {err}",);
        }

        self.release_session(uid);
    }

    async fn do_recv(&self, session: Arc<ProtocolSessionImpl>) -> Result<(), ProtocolError> {
        let mut buf = [0u8; 16384];
        let mut n = 0;
        loop {
            while n < 8 {
                match session.linker.recv_some(&mut buf[n..]).await? {
                    r if r > 0 => n += r,
                    _ => return Ok(()),
                }
            }

            let mut ps = ProtocolStream::new(Cursor::new(&mut buf));

            let to_channel = ps.pop_u16()?;
            let body_len = ps.pop_u32()? as usize;
            let header_len = ps.pop_u16()? as usize;

            if header_len != ProtocolHeader::SIZE {
                return Err(ProtocolError::HeaderSize(ProtocolHeader::SIZE, header_len));
            }

            while n < 8 + header_len + body_len {
                match session.linker.recv_some(&mut buf[n..]).await? {
                    r if r > 0 => n += r,
                    _ => return Ok(()),
                }
            }

            let mut ps = ProtocolStream::new(Cursor::new(&mut buf[8..]));
            let header = ProtocolHeader::unmarshal_from(&mut ps)?;

            if let Some(channel_info) = session.entity.get_channel_info(to_channel) {
                let body = ps.pop(body_len)?;
                self.handle_received_rpc(&session, &channel_info, header, body)
                    .await;
            }

            buf.copy_within(8 + header_len + body_len..n, 0);
            n -= 8 + header_len + body_len;
        }
    }

    async fn handle_received_rpc(
        &self,
        session: &Arc<ProtocolSessionImpl>,
        info: &ProtocolChannelInfo,
        header: ProtocolHeader,
        body: Vec<u8>,
    ) {
        session
            .last_active_time
            .store(time_util::unix_timestamp(), atomic::Ordering::SeqCst);

        if header.is_rpc_ret {
            if let Some((_, rpc_arg_info)) = self.rpc_arg_mgr.release(header.rpc_arg_uid) {
                let _ = rpc_arg_info.sender.send(body.into_boxed_slice());
            }
        } else {
            if let Some(cb) = info.callback.as_ref() {
                cb(ProtocolContext {
                    session: ProtocolSession {
                        remote_addr: session.linker.remote_addr,
                        session_id: session.get_uid(),
                        local_channel: header.to_channel,
                        remote_channel: header.from_channel,
                    },
                    rpc_arg_uid: header.rpc_arg_uid,
                    body: body.into_boxed_slice(),
                    local_channel: header.to_channel,
                    remote_channel: header.from_channel,
                })
            }
        }
    }

    async fn check_active_sessions(self: Arc<Self>) {
        loop {
            time::sleep(Duration::from_millis(2000)).await;
            let cur_time = time_util::unix_timestamp();

            let mut remove_session_vec = Vec::new();
            for session in self.session_mgr.iter() {
                if cur_time - session.last_active_time.load(atomic::Ordering::SeqCst) > 30000 {
                    remove_session_vec.push(session.get_uid());
                }
            }

            remove_session_vec
                .into_iter()
                .for_each(|uid| self.release_session(uid));
        }
    }

    pub async fn create_point(
        self: &Arc<Self>,
        addr: Option<SocketAddr>,
    ) -> Result<ProtocolPoint, std::io::Error> {
        let entity = if let Some(entity) = self.protocol_entity_map.get(&addr) {
            entity.clone()
        } else {
            let listener = match addr {
                Some(addr) => Some(protocol_helper::listen(addr).await?),
                None => None,
            };

            let protocol_entity = Arc::new(ProtocolEntity::new(addr, listener));
            protocol_entity.set_backend(self.clone());

            self.protocol_entity_map
                .insert(addr, protocol_entity.clone());

            if protocol_entity.has_listener() {
                let s = self.clone();
                let e = protocol_entity.clone();
                tokio::spawn(async move {
                    s.begin_accept(e).await;
                });
            }
            protocol_entity
        };

        Ok(entity
            .create_point(self.channel_counter.fetch_add(1, atomic::Ordering::SeqCst))
            .unwrap())
    }

    pub fn close_session(&self, entity: &Arc<ProtocolEntity>, addr: SocketAddr) {
        if let Some(session) = entity.sessions.get(&addr) {
            self.release_session(session.get_uid());
        }
    }

    fn release_session(&self, uid: u64) {
        tracing::info!("[SERVICE] released ProtocolSession, uid: {uid}");

        self.protocol_entity_map.iter().for_each(|e| {
            if let Some((_, addr)) = e.value().session_uid_map.remove(&uid) {
                e.value().sessions.remove(&addr);
            }
        });

        self.session_mgr.release(uid);
    }
}
