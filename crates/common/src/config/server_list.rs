use super::from_hex;
use serde::Deserialize;

pub type ServerList = Vec<ServerListInfo>;

#[derive(Deserialize)]
pub enum ServerProtocolType {
    #[serde(rename = "tcp")]
    Tcp,
    #[serde(rename = "kcp")]
    Kcp,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServerListInfo {
    pub sid: u32,
    pub server_name: String,
    pub ip: String,
    pub port: u16,
    pub notice_region: String,
    pub protocol: ServerProtocolType,
    pub rsa_ver: u32,
    #[serde(deserialize_with = "from_hex")]
    pub client_secret_key: Box<[u8]>,
}
