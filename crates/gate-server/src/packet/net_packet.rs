use byteorder::{ByteOrder, BE};
use yanagi_proto::{PacketHead, Protobuf, ProtobufDecodeError};

pub struct NetPacket<Proto> {
    pub head: PacketHead,
    pub body: Proto,
}

#[derive(thiserror::Error, Debug)]
pub enum DecodeError {
    #[error("head magic mismatch")]
    HeadMagicMismatch,
    #[error("tail magic mismatch")]
    TailMagicMismatch,
    #[error("input buffer is less than overhead, len: {0}, overhead: {1}")]
    InputLessThanOverhead(usize, usize),
    #[error("out of bounds ({0}/{1})")]
    OutOfBounds(usize, usize),
    #[error("failed to decode PacketHead: {0}")]
    HeadDecode(ProtobufDecodeError),
    #[error("failed to decode body: {0}")]
    BodyDecode(ProtobufDecodeError),
}

const OVERHEAD: usize = 16;
const HEAD_MAGIC: [u8; 4] = 0x01234567_u32.to_be_bytes();
const TAIL_MAGIC: [u8; 4] = 0x89ABCDEF_u32.to_be_bytes();

impl<Proto> NetPacket<Proto>
where
    Proto: yanagi_proto::NapMessage,
{
    pub fn decode(buf: &[u8]) -> Result<Self, DecodeError> {
        let (_, head_len, body_len) = read_common_values(buf)?;

        if &buf[(12 + head_len + body_len)..(16 + head_len + body_len)] != TAIL_MAGIC {
            return Err(DecodeError::TailMagicMismatch);
        }

        let head = PacketHead::decode(&buf[12..12 + head_len]).map_err(DecodeError::HeadDecode)?;
        let mut body = Proto::decode(&buf[12 + head_len..12 + head_len + body_len])
            .map_err(DecodeError::BodyDecode)?;

        body.xor_fields();

        Ok(NetPacket { head, body })
    }

    pub fn encode(&self) -> Box<[u8]> {
        let head_len = self.head.encoded_len();
        let body_len = self.body.encoded_len();
        let encoded_len = OVERHEAD + head_len + body_len;

        let mut buf = vec![0u8; encoded_len];
        (&mut buf[0..4]).copy_from_slice(&HEAD_MAGIC);
        BE::write_u16(&mut buf[4..6], self.body.get_cmd_id());
        BE::write_u16(&mut buf[6..8], head_len as u16);
        BE::write_u32(&mut buf[8..12], body_len as u32);

        self.head
            .encode(&mut buf[12..12 + head_len].as_mut())
            .unwrap();

        self.body
            .encode(&mut buf[12 + head_len..12 + head_len + body_len].as_mut())
            .unwrap();

        (&mut buf[12 + head_len + body_len..16 + head_len + body_len]).copy_from_slice(&TAIL_MAGIC);
        buf.into_boxed_slice()
    }
}

pub fn read_common_values(buf: &[u8]) -> Result<(u16, usize, usize), DecodeError> {
    if buf.len() < OVERHEAD {
        return Err(DecodeError::InputLessThanOverhead(buf.len(), OVERHEAD));
    }

    if &buf[0..4] != HEAD_MAGIC {
        return Err(DecodeError::HeadMagicMismatch);
    }

    let cmd_id = BE::read_u16(&buf[4..6]);
    let head_len = BE::read_u16(&buf[6..8]) as usize;
    let body_len = BE::read_u32(&buf[8..12]) as usize;

    let required_len = 4 + head_len + body_len;
    if required_len > buf.len() {
        Err(DecodeError::OutOfBounds(required_len, buf.len()))
    } else {
        Ok((cmd_id, head_len, body_len))
    }
}
