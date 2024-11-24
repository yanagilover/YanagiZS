use std::{
    net::SocketAddr,
    sync::{
        atomic::{AtomicU32, Ordering},
        mpsc, Arc, OnceLock,
    },
};

use qwer_rpc::RpcPtcPoint;
use tokio::sync::Mutex;
use tracing::{debug, instrument};
use yanagi_encryption::xor::MhyXorpad;
use yanagi_proto::{NapMessage, NullMessage, PlayerGetTokenScRsp};

use crate::{
    net::{
        kcp_conn_mgr::KcpEvent,
        packet_handler::{self, PacketHandlingError},
    },
    packet::{read_common_values, DecodeError, NetPacket},
    AppState,
};

pub struct SessionID {
    pub conv: u32,
    pub token: u32,
}

pub struct Session {
    pub conv: u32,
    pub addr: SocketAddr,
    pub rpc_ptc_point: Mutex<Arc<RpcPtcPoint>>,
    player_uid: OnceLock<u32>,
    initial_xorpad: &'static MhyXorpad,
    secret_key: OnceLock<MhyXorpad>,
    kcp_evt_tx: mpsc::Sender<(u32, KcpEvent)>,
    seq_id: AtomicU32,
}

impl Session {
    pub fn new(
        conv: u32,
        addr: SocketAddr,
        initial_xorpad: &'static MhyXorpad,
        kcp_evt_tx: mpsc::Sender<(u32, KcpEvent)>,
        rpc_ptc_point: Mutex<Arc<RpcPtcPoint>>,
    ) -> Self {
        Self {
            conv,
            addr,
            kcp_evt_tx,
            rpc_ptc_point,
            initial_xorpad,
            seq_id: AtomicU32::new(0),
            secret_key: OnceLock::new(),
            player_uid: OnceLock::new(),
        }
    }

    pub async fn process(
        &self,
        buf: &mut [u8],
        state: &'static AppState,
    ) -> Result<(), PacketHandlingError> {
        self.xor_packet_body(buf)?;
        packet_handler::decode_and_handle(self, state, buf).await
    }

    pub fn send_rsp(&self, rpc_id: u32, mut msg: impl NapMessage) {
        use yanagi_proto::CmdID;

        let cmd_id = msg.get_cmd_id();
        debug!("send_rsp: {cmd_id}");

        msg.xor_fields();

        let seq_id = (cmd_id != yanagi_proto::PlayerGetTokenScRsp::CMD_ID)
            .then_some(
                self.seq_id
                    .fetch_add(1, std::sync::atomic::Ordering::SeqCst),
            )
            .unwrap_or(0);

        self.send(NetPacket {
            head: yanagi_proto::PacketHead {
                packet_id: seq_id,
                request_id: rpc_id,
                ..Default::default()
            },
            body: msg,
        });
    }

    #[instrument(skip_all, name = "session", fields(addr = %self.addr))]
    pub fn notify(&self, mut msg: impl NapMessage) {
        debug!("notify: {}", msg.get_cmd_id());

        msg.xor_fields();
        self.send(NetPacket {
            head: yanagi_proto::PacketHead {
                ..Default::default()
            },
            body: msg,
        })
    }

    pub fn send_null_rsp(&self, rpc_id: u32) {
        let seq_id = self.seq_id.fetch_add(1, Ordering::SeqCst);

        self.send(NetPacket {
            head: yanagi_proto::PacketHead {
                packet_id: seq_id,
                request_id: rpc_id,
                ..Default::default()
            },
            body: NullMessage::default(),
        });
    }

    fn send<Proto: NapMessage>(&self, packet: NetPacket<Proto>) {
        let mut buf = packet.encode();
        self.xor_packet_body(&mut buf).unwrap();

        let _ = self.kcp_evt_tx.send((self.conv, KcpEvent::Send(buf)));
    }

    pub fn set_secret_key(&self, seed: u64) {
        let _ = self.secret_key.set(MhyXorpad::new::<byteorder::BE>(seed));
    }

    pub fn is_logged_in(&self) -> bool {
        self.player_uid.get().is_some()
    }

    pub fn get_player_uid(&self) -> u32 {
        self.player_uid.get().copied().unwrap_or(0)
    }

    pub fn set_player_uid(&self, uid: u32) {
        let _ = self.player_uid.set(uid);
    }

    #[inline]
    pub fn xor_packet_body(&self, packet: &mut [u8]) -> Result<(), DecodeError> {
        use yanagi_proto::CmdID;

        let (cmd_id, head_len, body_len) = read_common_values(packet)?;
        let body = &mut packet[12 + head_len..12 + head_len + body_len];

        match self.secret_key.get() {
            _ if cmd_id == PlayerGetTokenScRsp::CMD_ID => self.initial_xorpad.xor(body),
            Some(key) => key.xor(body),
            None => self.initial_xorpad.xor(body),
        }

        Ok(())
    }
}
