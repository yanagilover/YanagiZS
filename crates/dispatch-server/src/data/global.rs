use serde::Serialize;

#[derive(Serialize, Default)]
pub struct ServerListInfo {
    pub area: u8,
    pub biz: String,
    pub dispatch_url: String,
    pub env: u8,
    pub is_recommend: bool,
    pub name: String,
    pub ping_url: String,
    pub retcode: i32,
    pub title: String,
}

#[derive(Serialize, Default)]
pub struct QueryDispatchRsp {
    #[serde(skip_serializing_if = "Vec::is_empty")]
    pub region_list: Vec<ServerListInfo>,
    pub retcode: i32,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub msg: String,
}
