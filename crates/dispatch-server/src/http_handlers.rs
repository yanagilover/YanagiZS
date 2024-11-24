use axum::extract::{Path, Query, State};
use axum::routing::get;
use axum::{Json, Router};
use serde::{Deserialize, Serialize};

use crate::data::{
    CdnConfExt, CdnDesignData, CdnGameRes, CdnSilenceData, RegionExtension, RegionSwitchFunc,
    ServerDispatchData, ServerGateway,
};
use crate::{data, AppState};
use common::config::{EncryptionConfig, ServerProtocolType};

pub fn routes() -> Router<&'static AppState> {
    Router::new()
        .route("/query_dispatch", get(query_dispatch))
        .route("/query_gateway/:server_name", get(query_gateway))
}

#[derive(Deserialize)]
struct QueryDispatchParam {
    pub version: String,
}

async fn query_dispatch(
    State(state): State<&'static AppState>,
    Query(param): Query<QueryDispatchParam>,
) -> Json<data::QueryDispatchRsp> {
    let Some(server_list_map) = state.server_list.get() else {
        return Json(data::QueryDispatchRsp {
            retcode: 71, // maintenance (stop_server)
            ..Default::default()
        });
    };

    let Some(server_list) = server_list_map.get(&param.version) else {
        return Json(data::QueryDispatchRsp {
            retcode: 70,
            ..Default::default()
        });
    };

    Json(data::QueryDispatchRsp {
        retcode: 0,
        msg: String::new(),
        region_list: server_list
            .iter()
            .map(|info| data::ServerListInfo {
                retcode: 0,
                name: info.notice_region.clone(),
                title: info.server_name.clone(),
                dispatch_url: format!(
                    "{}/query_gateway/{}",
                    &state.dispatch_config.outer_http_url, &info.notice_region
                ),
                biz: String::from("nap_global"),
                env: 2,
                ..Default::default()
            })
            .collect(),
    })
}

#[derive(Serialize)]
#[serde(untagged)]
enum QueryGatewayRsp {
    Plaintext(ServerDispatchData),
    Encrypted { content: String, sign: String },
}

impl QueryGatewayRsp {
    pub fn error(retcode: i32, msg: &str) -> Self {
        Self::Plaintext(ServerDispatchData {
            retcode,
            msg: msg.to_string(),
            ..Default::default()
        })
    }

    pub fn encrypt(self, config: &EncryptionConfig) -> Self {
        match self {
            Self::Encrypted { .. } => self,
            Self::Plaintext(data) => {
                let data = serde_json::to_string(&data).unwrap();
                let content = yanagi_encryption::rsa::encrypt(config, data.as_bytes());
                let sign = yanagi_encryption::rsa::sign(config, data.as_bytes());

                Self::Encrypted {
                    content: rbase64::encode(&content),
                    sign: rbase64::encode(&sign),
                }
            }
        }
    }
}

#[derive(Deserialize)]
struct QueryGatewayParam {
    pub version: String,
    pub rsa_ver: u32,
    pub seed: String,
}

async fn query_gateway(
    State(state): State<&'static AppState>,
    Path(server_name): Path<String>,
    Query(param): Query<QueryGatewayParam>,
) -> Json<QueryGatewayRsp> {
    let (Some(app_config), Some(server_list_map), Some(encryption_conf_map)) = (
        state.app_config.get(),
        state.server_list.get(),
        state.encryption_config.get(),
    ) else {
        tracing::debug!("query_gateway requested, but server is in stop mode");
        return Json(QueryGatewayRsp::error(71, ""));
    };

    let Some(encryption_conf) = encryption_conf_map
        .get(&param.version)
        .map(|k| k.get(&param.rsa_ver))
        .flatten()
    else {
        tracing::debug!("EncryptionConfig for version {} not found", param.version);
        return Json(QueryGatewayRsp::error(74, ""));
    };

    let Some(version_info) = app_config.version_info_groups.get(&param.version) else {
        tracing::debug!("VersionInfoGroup for {} not found", param.version);
        return Json(QueryGatewayRsp::error(70, "").encrypt(encryption_conf));
    };

    if !version_info.seed.is_empty() && version_info.seed != param.seed {
        tracing::debug!(
            "dispatch seed for version {} doesn't match. Config seed: {}, client seed: {}",
            param.version,
            version_info.seed,
            param.seed
        );
        return Json(QueryGatewayRsp::error(75, "").encrypt(encryption_conf));
    }

    let Some(server_list) = server_list_map.get(&param.version) else {
        return Json(QueryGatewayRsp::error(70, "").encrypt(encryption_conf));
    };

    let Some(server_info) = server_list.iter().find(|s| s.notice_region == server_name) else {
        return Json(QueryGatewayRsp::error(70, "").encrypt(encryption_conf));
    };

    Json(QueryGatewayRsp::Plaintext(ServerDispatchData {
        retcode: 0,
        msg: String::new(),
        region_name: server_info.notice_region.clone(),
        title: server_info.server_name.clone(),
        client_secret_key: rbase64::encode(&server_info.client_secret_key),
        cdn_check_url: version_info.cdn_check_url.clone(),
        gateway: Some(ServerGateway {
            ip: server_info.ip.clone(),
            port: server_info.port,
        }),
        oaserver_url: String::new(),
        force_update_url: String::new(),
        stop_jump_url: String::new(),
        cdn_conf_ext: Some(CdnConfExt {
            // TODO: move this stuff to VersionInfo in config.json
            design_data: CdnDesignData {
                base_url: String::from("http://127.0.0.1:10000/design_data/beta_live/output_5016531_79764a0a26/client/"),
                data_revision: String::from("5010994"),
                md5_files: String::from(r#"[{"fileName":"data_version","fileSize":2056,"fileMD5":"847307868890712853"}]"#),
            },
            game_res: CdnGameRes {
                audio_revision: String::from("5010994"),
                base_url: String::from("http://127.0.0.1:10000/game_res/beta_live/output_5016531_79764a0a26/client/"),
                branch: String::from("beta_live"),
                md5_files: String::from(r#"[{"fileName":"res_version","fileSize":1225259,"fileMD5":"13780047615044516895"},{"fileName":"audio_version","fileSize":14386,"fileMD5":"1213735845266261736"},{"fileName":"base_revision","fileSize":4,"fileMD5":"4524394692449115962"}]"#),
                res_revision: String::from("5016531"),
            },
            silence_data: CdnSilenceData {
                base_url: String::from("http://127.0.0.1:10000/design_data/beta_live/output_5016531_79764a0a26/client_silence/"),
                md5_files: String::from(r#"[{"fileName":"silence_version","fileSize":130,"fileMD5":"2077712550601860122"}]"#),
                silence_revision: String::from("5010994"),
            },
            pre_download: None,
        }),
        region_ext: Some(RegionExtension {
            exchange_url: String::new(),
            feedback_url: String::new(),
            func_switch: RegionSwitchFunc {
                disable_frequent_attempts: 1,
                enable_gacha_mobile_console: 1,
                enable_notice_mobile_console: 1,
                enable_performance_log: 1,
                is_kcp: match server_info.protocol {
                    ServerProtocolType::Tcp => 0,
                    ServerProtocolType::Kcp => 1,
                },
                ..Default::default()
            },
            mtr_nap: String::new(),
            mtr_sdk: String::new(),
            pgc_webview_method: 1,
            url_check_nap: String::new(),
            url_check_sdk: String::new(),
        }),
    }).encrypt(encryption_conf))
}
