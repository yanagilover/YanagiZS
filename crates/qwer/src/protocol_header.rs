use std::io::{Read, Result, Write};

use crate::protocol_stream::ProtocolStream;

#[derive(Debug, Default)]
pub struct ProtocolHeader {
    pub to_channel: u16,
    pub from_channel: u16,
    pub is_rpc_ret: bool,
    pub rpc_arg_uid: u64,
}

impl ProtocolHeader {
    pub const SIZE: usize = 13;

    pub fn marshal_to<W: Write>(&self, ps: &mut ProtocolStream<W>) -> Result<()> {
        ps.push_u16(self.to_channel)?;
        ps.push_u16(self.from_channel)?;
        ps.push_boolean(self.is_rpc_ret)?;
        ps.push_u64(self.rpc_arg_uid)
    }

    pub fn unmarshal_from<R: Read>(ps: &mut ProtocolStream<R>) -> Result<Self> {
        Ok(Self {
            to_channel: ps.pop_u16()?,
            from_channel: ps.pop_u16()?,
            is_rpc_ret: ps.pop_boolean()?,
            rpc_arg_uid: ps.pop_u64()?,
        })
    }
}
