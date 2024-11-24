use axum::{extract::State, routing::post, Json, Router};
use serde::{Deserialize, Serialize};

use crate::AppStateRef;

use super::Response;

#[derive(Deserialize)]
struct RequestData {
    pub uid: String,
    pub token: String,
}

#[derive(Deserialize)]
struct GranterTokenRequest {
    pub data: String,
}

pub fn routes() -> Router<AppStateRef> {
    Router::new().route(
        "/:product_name/combo/granter/login/v2/login",
        post(login_v2),
    )
}

#[derive(Serialize)]
struct ResponseData {
    pub account_type: u32,
    pub combo_id: String,
    pub combo_token: String,
    pub data: &'static str,
    pub heartbeat: bool,
    pub open_id: String,
}

async fn login_v2(
    state: State<AppStateRef>,
    request: Json<GranterTokenRequest>,
) -> Json<Response<ResponseData>> {
    let Ok(data) = serde_json::from_str::<RequestData>(&request.data) else {
        return Json(Response::error(-101, "Account token error"));
    };

    match state.db.get_account_by_uid(&data.uid).await {
        Ok(Some(account)) if account.token == data.token => {
            success_rsp(data.uid.clone(), account.token)
        }
        _ => Json(Response::error(-101, "Account token error")),
    }
}

fn success_rsp(uid: String, token: String) -> Json<Response<ResponseData>> {
    Json(Response::new(ResponseData {
        account_type: 1,
        combo_id: uid.clone(),
        combo_token: token,
        data: r#"{"guest":false}"#,
        heartbeat: false,
        open_id: uid,
    }))
}
