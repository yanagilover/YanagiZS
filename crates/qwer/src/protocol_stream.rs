use std::io::{Read, Result, Write};

use byteorder::{ReadBytesExt, WriteBytesExt, LE};

pub struct ProtocolStream<Buf> {
    buf: Buf,
}

impl<B> ProtocolStream<B> {
    pub fn new(buf: B) -> Self {
        Self { buf }
    }
}

impl<W: Write> ProtocolStream<W> {
    pub fn push(&mut self, value: &[u8]) -> Result<()> {
        self.buf.write_all(value)
    }

    pub fn push_boolean(&mut self, value: bool) -> Result<()> {
        self.buf.write_u8(if value { 1 } else { 100 })
    }

    pub fn push_u8(&mut self, value: u8) -> Result<()> {
        self.buf.write_u8(value)
    }

    pub fn push_u16(&mut self, value: u16) -> Result<()> {
        self.buf.write_u16::<LE>(value)
    }

    pub fn push_u32(&mut self, value: u32) -> Result<()> {
        self.buf.write_u32::<LE>(value)
    }

    pub fn push_u64(&mut self, value: u64) -> Result<()> {
        self.buf.write_u64::<LE>(value)
    }
}

impl<R: Read> ProtocolStream<R> {
    pub fn pop(&mut self, n: usize) -> Result<Vec<u8>> {
        let mut out = vec![0u8; n];
        self.buf.read_exact(&mut out).map(|_| out)
    }

    pub fn pop_boolean(&mut self) -> Result<bool> {
        Ok(self.buf.read_u8()? != 100)
    }

    pub fn pop_u8(&mut self) -> Result<u8> {
        self.buf.read_u8()
    }

    pub fn pop_u16(&mut self) -> Result<u16> {
        self.buf.read_u16::<LE>()
    }

    pub fn pop_u32(&mut self) -> Result<u32> {
        self.buf.read_u32::<LE>()
    }

    pub fn pop_u64(&mut self) -> Result<u64> {
        self.buf.read_u64::<LE>()
    }
}
