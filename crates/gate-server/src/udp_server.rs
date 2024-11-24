use std::{
    collections::HashMap,
    net::{SocketAddr, UdpSocket},
    sync::{mpsc, Arc},
};

use rand::RngCore;

use crate::{
    net::{
        kcp_conn_mgr::{self, KcpEvent},
        packet_processor,
    },
    packet::{self, ControlPacket, ControlPacketType},
    session::SessionID,
    AppState,
};

pub struct UdpServer {
    socket: Arc<UdpSocket>,
    sessions: HashMap<u32, SessionID>,
    kcp_conn_mgr_tx: mpsc::Sender<(u32, KcpEvent)>,
    packet_processor_tx: tokio::sync::mpsc::Sender<packet_processor::Input>,
}

impl UdpServer {
    pub fn new(udp_addr: &str, state: &'static AppState) -> std::io::Result<Self> {
        let socket = Arc::new(UdpSocket::bind(udp_addr)?);
        let (kcp_tx, kcp_rx) = mpsc::channel();

        let proc_tx = packet_processor::start(kcp_tx.clone(), state);
        kcp_conn_mgr::start(kcp_rx, proc_tx.clone(), socket.clone());

        Ok(Self {
            kcp_conn_mgr_tx: kcp_tx,
            packet_processor_tx: proc_tx,
            socket,
            sessions: HashMap::new(),
        })
    }

    pub fn serve(mut self) {
        tracing::info!("UDP server is up at {}", self.socket.local_addr().unwrap());

        let mut session_counter = 0;
        let mut buf = [0u8; 1400];
        loop {
            let Ok((len, addr)) = self
                .socket
                .recv_from(&mut buf)
                .inspect_err(|err| tracing::debug!("recv_from failed: {err}"))
            else {
                continue;
            };

            match len {
                packet::CONTROL_PACKET_SIZE => self.handle_control_packet(
                    buf[..packet::CONTROL_PACKET_SIZE].try_into().unwrap(),
                    addr,
                    &mut session_counter,
                ),
                kcp::KCP_OVERHEAD.. => {
                    let buf = &buf[..len];
                    if let Some(id) = self.sessions.get(&kcp::get_conv(buf)) {
                        let token = kcp::get_token(buf);
                        if token != id.token {
                            tracing::debug!(
                                "session token mismatch! Expected: {}, received: {}, conv: {} client_addr: {}",
                                id.token,
                                token,
                                id.conv,
                                addr,
                            );
                        }

                        self.kcp_conn_mgr_tx
                            .send((id.conv, KcpEvent::Recv(buf.into())))
                            .unwrap();
                    }
                }
                _ => (),
            }
        }
    }

    fn handle_control_packet(
        &mut self,
        buf: [u8; packet::CONTROL_PACKET_SIZE],
        addr: SocketAddr,
        s_counter: &mut u32,
    ) {
        let Ok(packet) = ControlPacket::try_from(buf).inspect_err(|err| {
            tracing::debug!("ControlPacket::try_from failed: {err}, client_addr: {addr}")
        }) else {
            return;
        };

        match packet.get_type() {
            ControlPacketType::Connect => {
                tracing::info!("new connection from {addr}, data: {}", packet.get_data());

                *s_counter += 1;
                self.on_connect(*s_counter, rand::thread_rng().next_u32(), addr);
            }
            ControlPacketType::Disconnect => {
                if let Some(id) = self.sessions.get(&packet.get_conv()) {
                    if id.token != packet.get_token() {
                        tracing::debug!(
                                "disconnect: session token mismatch! Expected: {}, received: {}, conv: {} client_addr: {}",
                                id.token,
                                packet.get_token(),
                                id.conv,
                                addr,
                            );
                        return;
                    }

                    self.on_disconnect(packet, addr);
                }
            }
            unsupported => tracing::debug!("received {unsupported:?} from client_addr: {addr}"),
        }
    }

    fn on_connect(&mut self, conv: u32, token: u32, addr: SocketAddr) {
        self.kcp_conn_mgr_tx
            .send((conv, KcpEvent::Establish(token, addr)))
            .unwrap();
        self.packet_processor_tx
            .blocking_send(packet_processor::Input::CreateSession(conv, addr))
            .unwrap();

        self.sessions.insert(conv, SessionID { conv, token });

        self.send_control_packet(
            ControlPacket::build(ControlPacketType::Establish, conv, token, 0),
            addr,
        );
    }

    fn on_disconnect(&mut self, pk: ControlPacket, addr: SocketAddr) {
        let conv = pk.get_conv();
        self.send_control_packet(pk, addr);

        tracing::info!("client from {addr} disconnected, (conv: {conv})");

        self.sessions.remove(&conv);
        self.kcp_conn_mgr_tx.send((conv, KcpEvent::Drop)).unwrap();
        self.packet_processor_tx
            .blocking_send(packet_processor::Input::RemoveSession(conv))
            .unwrap();
    }

    fn send_control_packet(&self, packet: ControlPacket, addr: SocketAddr) {
        let socket = self.socket.clone();
        let _ = socket.send_to(packet.as_slice(), addr);
    }
}
