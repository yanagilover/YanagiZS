use serde::Serialize;

#[derive(Serialize, Default)]
pub struct ServerDispatchData {
    pub retcode: i32,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub msg: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub title: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub region_name: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub client_secret_key: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub cdn_check_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub gateway: Option<ServerGateway>,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub oaserver_url: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub force_update_url: String,
    #[serde(skip_serializing_if = "String::is_empty")]
    pub stop_jump_url: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub cdn_conf_ext: Option<CdnConfExt>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub region_ext: Option<RegionExtension>,
}

#[derive(Serialize)]
pub struct ServerGateway {
    pub ip: String,
    pub port: u16,
}

#[derive(Serialize)]
pub struct RegionExtension {
    pub func_switch: RegionSwitchFunc,
    pub feedback_url: String,
    pub exchange_url: String,
    pub pgc_webview_method: i32,
    #[serde(rename = "mtrNap")]
    pub mtr_nap: String,
    #[serde(rename = "mtrSdk")]
    pub mtr_sdk: String,
    #[serde(rename = "urlCheckNap")]
    pub url_check_nap: String,
    #[serde(rename = "urlCheckSdk")]
    pub url_check_sdk: String,
}

#[derive(Serialize, Default)]
pub struct RegionSwitchFunc {
    #[serde(rename = "Close_Medium_Package_Download")]
    pub close_medium_package_download: i64,
    #[serde(rename = "Disable_Audio_Download")]
    pub disable_audio_download: i64,
    #[serde(rename = "Hide_Download_complete_resources")]
    pub hide_download_complete_resources: i64,
    #[serde(rename = "Hide_Download_resources_popups")]
    pub hide_download_resources_popups: i64,
    #[serde(rename = "Hide_download_progress")]
    pub hide_download_progress: i64,
    #[serde(rename = "Medium_Package_Play")]
    pub medium_package_play: i64,
    #[serde(rename = "Play_The_Music")]
    pub play_the_music: i64,
    pub disable_anim_allocator_opt: i64,
    #[serde(rename = "disableAsyncSRPSubmit")]
    pub disable_async_srpsubmit: i64,
    pub disable_execute_async: i64,
    #[serde(rename = "disableMetalPSOCreateAsync")]
    pub disable_metal_psocreate_async: i64,
    pub disable_object_instance_cache: i64,
    #[serde(rename = "disableSRPHelper")]
    pub disable_srp_helper: i64,
    #[serde(rename = "disableSRPInstancing")]
    pub disable_srp_instancing: i64,
    pub disable_skin_mesh_strip: i64,
    pub disable_step_preload_monster: i64,
    pub disable_tex_streaming_visbility_opt: i64,
    #[serde(rename = "disableiOSGPUBufferOpt")]
    pub disable_ios_gpubuffer_opt: i64,
    #[serde(rename = "disableiOSShaderHibernation")]
    pub disable_ios_shader_hibernation: i64,
    #[serde(rename = "enableiOSShaderWarmupOnStartup")]
    pub enable_ios_shader_warmup_on_startup: i64,
    #[serde(rename = "isKcp")]
    pub is_kcp: i32,
    #[serde(rename = "mtrConfig")]
    pub mtr_config: Option<String>,
    #[serde(rename = "perfSwitch1")]
    pub perf_switch_1: i32,
    #[serde(rename = "perfSwitch2")]
    pub perf_switch_2: i32,
    #[serde(rename = "enableNoticeMobileConsole")]
    pub enable_notice_mobile_console: i32,
    #[serde(rename = "enableGachaMobileConsole")]
    pub enable_gacha_mobile_console: i32,
    #[serde(rename = "Disable_Popup_Notification")]
    pub disable_popup_notification: i32,
    #[serde(rename = "open_hotfix_popups")]
    pub open_hotfix_popups: i32,
    pub enable_operation_log: i32,
    #[serde(rename = "Turnoff_Push_notifications")]
    pub turnoff_push_notifications: i32,
    #[serde(rename = "Disable_Frequent_attempts")]
    pub disable_frequent_attempts: i32,
    pub enable_performance_log: i32,
    #[serde(rename = "Turnoff_unsafepreload_cloudgame")]
    pub turnoff_unsafepreload_cloudgame: i32,
    #[serde(rename = "Hide_Code_Login")]
    pub hide_code_login: i32,
}

#[derive(Serialize)]
pub struct CdnConfExt {
    pub game_res: CdnGameRes,
    pub design_data: CdnDesignData,
    pub silence_data: CdnSilenceData,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub pre_download: Option<CdnGameRes>,
}

#[derive(Serialize)]
pub struct CdnGameRes {
    pub base_url: String,
    pub res_revision: String,
    pub audio_revision: String,
    pub branch: String,
    pub md5_files: String, // Vec<VersionFileInfo> packed as string
}

#[derive(Serialize)]
pub struct CdnDesignData {
    pub base_url: String,
    pub data_revision: String,
    pub md5_files: String, // Vec<VersionFileInfo> packed as string
}

#[derive(Serialize)]
pub struct CdnSilenceData {
    pub base_url: String,
    pub silence_revision: String,
    pub md5_files: String, // Vec<VersionFileInfo> packed as string
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct VersionFileInfo {
    pub file_name: String,
    pub file_size: i64,
    #[serde(rename = "fileMD5")]
    pub file_md5: String,
}
