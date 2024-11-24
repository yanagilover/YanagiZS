mod oct_data;
mod property_data;
mod protocol_header;
mod protocol_stream;

pub use oct_data::OctData;
pub use property_data::*;
pub use protocol_header::ProtocolHeader;
pub use protocol_stream::ProtocolStream;
pub use qwer_derive::{OctData, ProtocolID};

pub trait ProtocolID {
    const PROTOCOL_ID: u16;
    const NAME: &str;

    fn get_protocol_id(&self) -> u16 {
        Self::PROTOCOL_ID
    }

    fn get_protocol_name(&self) -> &str {
        Self::NAME
    }
}
