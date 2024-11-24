use qwer::ProtocolID;
use qwer_rpc::{ProtocolServiceFrontend, RpcPtcContext, RpcPtcServiceFrontend};
use std::{
    collections::HashMap,
    net::SocketAddr,
    sync::{Arc, OnceLock},
};
use tokio::sync::{mpsc, Mutex};
use tracing::{info_span, Instrument};
use yanagi_proto::{forward_as_notify, register_ptc_handlers};

use crate::{session::Session, AppState};

use super::kcp_conn_mgr::KcpEvent;

pub enum Input {
    CreateSession(u32, SocketAddr),
    RemoveSession(u32),
    Packet(u32, Box<[u8]>),
    Notify(u32, RpcPtcContext),
}

static TX: OnceLock<mpsc::Sender<Input>> = OnceLock::new();

pub fn start(
    kcp_evt_tx: std::sync::mpsc::Sender<(u32, KcpEvent)>,
    state: &'static AppState,
) -> mpsc::Sender<Input> {
    let (tx, rx) = mpsc::channel(32);

    tokio::spawn(async move {
        processing_loop(rx, kcp_evt_tx, state).await;
    });

    TX.get_or_init(|| tx.clone());
    tx
}

async fn processing_loop(
    mut rx: mpsc::Receiver<Input>,
    tx: std::sync::mpsc::Sender<(u32, KcpEvent)>,
    state: &'static AppState,
) {
    let rpc_ptc_service = RpcPtcServiceFrontend::new(ProtocolServiceFrontend::new());
    let mut session_map: HashMap<u32, Arc<Session>> = HashMap::new();

    loop {
        match rx.recv().await {
            Some(Input::CreateSession(conv, addr)) => {
                let rpc_ptc_point = rpc_ptc_service.create_point(None).await.unwrap();

                let conv_id = conv;
                register_ptc_handlers!(rpc_ptc_point, conv_id, TX);

                session_map.insert(
                    conv,
                    Arc::new(Session::new(
                        conv,
                        addr,
                        &state.remote_config.xorpad,
                        tx.clone(),
                        Mutex::new(rpc_ptc_point),
                    )),
                );
            }
            Some(Input::RemoveSession(conv)) => {
                session_map.remove(&conv);
            }
            Some(Input::Packet(conv, mut pk)) => {
                if let Some(session) = session_map.get_mut(&conv).cloned() {
                    let addr = session.addr;
                    tokio::spawn(
                        async move {
                            let _ = session
                                .process(pk.as_mut(), state)
                                .await
                                .inspect_err(|err| {
                                    tracing::warn!("Session::process failed: {err}")
                                });
                        }
                        .instrument(info_span!("session", addr = %addr)),
                    );
                }
            }
            Some(Input::Notify(conv, ctx)) => {
                if let Some(session) = session_map.get(&conv) {
                    forward_as_notify!(session, ctx);
                }
            }
            None => break,
        }
    }
}
