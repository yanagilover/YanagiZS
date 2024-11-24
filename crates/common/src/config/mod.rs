mod local_toml;
pub use local_toml::{DatabaseSettings, TomlConfig};

mod app;
mod encryption;
mod server_list;

pub use app::*;
pub use encryption::*;
use serde::{Deserialize, Deserializer};
pub use server_list::*;

pub fn from_hex<'de, D>(deserializer: D) -> Result<Box<[u8]>, D::Error>
where
    D: Deserializer<'de>,
{
    use serde::de::Error;
    String::deserialize(deserializer).and_then(|string| {
        hex::decode(&string)
            .map(|vec| vec.into_boxed_slice())
            .map_err(|err| Error::custom(err.to_string()))
    })
}
