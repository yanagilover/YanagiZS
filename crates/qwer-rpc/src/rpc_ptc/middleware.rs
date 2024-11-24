use std::io::{Cursor, Read, Write};

use qwer::{OctData, ProtocolStream};
use tracing::debug;

#[derive(Debug, Clone)]
pub enum MiddlewareModel {
    Account(AccountMiddlewareModel),
    Unknown(u16, Box<[u8]>),
}

pub fn unmarshal_middleware_list<R: Read>(
    s: &mut ProtocolStream<R>,
) -> std::io::Result<Vec<MiddlewareModel>> {
    let size = s.pop_u16()?;
    (0..size)
        .map(|_| {
            let middleware_type = s.pop_u16()?;
            let middleware_size = s.pop_u16()?;
            let buf = s.pop(middleware_size as usize)?;

            match middleware_type {
                1 => Ok(MiddlewareModel::Account(
                    AccountMiddlewareModel::unmarshal_from(&mut Cursor::new(&buf), 0)?,
                )),
                _ => {
                    debug!("unknown middleware_type encountered: {middleware_type}");
                    Ok(MiddlewareModel::Unknown(
                        middleware_type,
                        buf.into_boxed_slice(),
                    ))
                }
            }
        })
        .collect()
}

pub fn marshal_middleware_list<W: Write>(
    s: &mut ProtocolStream<W>,
    list: &[MiddlewareModel],
) -> std::io::Result<()> {
    s.push_u16(list.len() as u16)?;
    list.iter()
        .try_for_each(|middleware_model| match middleware_model {
            MiddlewareModel::Account(model) => {
                let mut data = [0u8; 17];
                model.marshal_to(&mut Cursor::new(&mut data[..]), 0)?;

                s.push_u16(1)?;
                s.push_u16(data.len() as u16)?;
                s.push(&data)
            }
            MiddlewareModel::Unknown(ty, data) => {
                s.push_u16(*ty)?;
                s.push_u16(data.len() as u16)?;
                s.push(&data)
            }
        })
}

#[derive(Debug, Clone)]
pub struct AccountMiddlewareModel {
    pub player_uid: u64,
    pub client_protocol_uid: u64,
    pub is_resend: bool,
}

impl OctData for AccountMiddlewareModel {
    fn marshal_to<W: std::io::Write>(&self, w: &mut W, _: u16) -> std::io::Result<()> {
        use byteorder::{WriteBytesExt, LE};
        w.write_u64::<LE>(self.player_uid)?;
        w.write_u64::<LE>(self.client_protocol_uid)?;
        w.write_u8(self.is_resend as u8)
    }

    fn unmarshal_from<R: std::io::Read>(r: &mut R, _: u16) -> std::io::Result<Self>
    where
        Self: Sized,
    {
        use byteorder::{ReadBytesExt, LE};

        Ok(Self {
            player_uid: r.read_u64::<LE>()?,
            client_protocol_uid: r.read_u64::<LE>()?,
            is_resend: r.read_u8()? != 0,
        })
    }
}
