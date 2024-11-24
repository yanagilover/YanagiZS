use common::time_util;
use kcp::{Kcp, KcpResult};
use std::{
    collections::HashMap,
    io::Write,
    net::{SocketAddr, UdpSocket},
    sync::{mpsc, Arc},
    thread,
};
use tracing::error;

use super::packet_processor;

struct UdpOutput {
    peer_addr: SocketAddr,
    socket: Arc<UdpSocket>,
}

pub enum KcpEvent {
    Establish(u32, SocketAddr),
    Recv(Box<[u8]>),
    Send(Box<[u8]>),
    Drop,
}

pub fn start(
    rx: mpsc::Receiver<(u32, KcpEvent)>,
    tx: tokio::sync::mpsc::Sender<packet_processor::Input>,
    udp_socket: Arc<UdpSocket>,
) {
    thread::spawn(move || kcp_loop(rx, tx, udp_socket));
}

fn kcp_loop(
    event_rx: mpsc::Receiver<(u32, KcpEvent)>,
    tx: tokio::sync::mpsc::Sender<packet_processor::Input>,
    udp_socket: Arc<UdpSocket>,
) {
    let mut conv_map = HashMap::new();
    loop {
        match event_rx.recv() {
            Ok((conv, KcpEvent::Establish(token, addr))) => {
                conv_map.insert(
                    conv,
                    Kcp::new(
                        conv,
                        token,
                        time_util::unix_timestamp_ms(),
                        false,
                        UdpOutput {
                            peer_addr: addr,
                            socket: udp_socket.clone(),
                        },
                    ),
                );
            }
            Ok((conv, KcpEvent::Recv(pk))) => {
                if let Some(kcp) = conv_map.get_mut(&conv) {
                    if let Err(err) = perform_recv(kcp, &pk, &tx) {
                        error!("kcp recv fail: {err}");
                    }
                }
            }
            Ok((conv, KcpEvent::Send(pk))) => {
                if let Some(kcp) = conv_map.get_mut(&conv) {
                    if let Err(err) = perform_send(kcp, &pk) {
                        error!("kcp send fail: {err}");
                    }
                }
            }
            Ok((conv, KcpEvent::Drop)) => {
                conv_map.remove(&conv);
            }
            Err(_) => return, // channel closed
        };
    }
}

fn perform_recv(
    kcp: &mut Kcp<UdpOutput>,
    pk: &[u8],
    tx: &tokio::sync::mpsc::Sender<packet_processor::Input>,
) -> KcpResult<()> {
    kcp.input(pk)?;
    kcp.update((time_util::unix_timestamp_ms() - kcp.estab_ts()) as u32)?;

    let mut buf = [0u8; 16384];
    while let Ok(len) = kcp.recv(&mut buf) {
        let _ = tx.blocking_send(packet_processor::Input::Packet(
            kcp.conv(),
            buf[..len].into(),
        ));
    }

    kcp.flush()
}

fn perform_send(kcp: &mut Kcp<UdpOutput>, pk: &[u8]) -> KcpResult<()> {
    kcp.send(pk)?;
    kcp.flush()
}

impl Write for UdpOutput {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.socket.send_to(buf, self.peer_addr)
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}
