use axum::{extract::State, routing::post, Json, Router};
use serde::{Deserialize, Serialize};

use crate::{util, AppStateRef};

use super::Response;

pub fn routes() -> Router<AppStateRef> {
    Router::new()
        .route("/:product_name/mdk/shield/api/login", post(login))
        .route("/:product_name/mdk/shield/api/verify", post(verify))
}

#[derive(Deserialize)]
struct LoginRequest {
    pub account: String,
    pub password: String,
    pub is_crypto: bool,
}

#[derive(Deserialize)]
struct VerifyRequest {
    pub uid: String,
    pub token: String,
}

#[derive(Serialize, Default)]
struct ResponseData {
    pub account: ResponseAccountData,
    pub device_grant_required: bool,
    pub reactive_required: bool,
    pub realperson_required: bool,
    pub safe_mobile_required: bool,
}

#[derive(Serialize, Default)]
struct ResponseAccountData {
    pub area_code: String,
    pub email: String,
    pub country: String,
    pub is_email_verify: String,
    pub token: String,
    pub uid: String,
}

async fn login(
    state: State<AppStateRef>,
    request: Json<LoginRequest>,
) -> Json<Response<ResponseData>> {
    if !request.is_crypto {
        return Json(Response::error(
            -10,
            "Invalid account format: unencrypted passwords are disabled by SDK security policy",
        ));
    }

    let Ok(password) = util::rsa_decrypt(&request.password) else {
        return Json(Response::error(-10, "Your patch is outdated, get a new one at https://discord.gg/reversedrooms (Password decryption failed)"));
    };

    let account = match state.db.get_account_by_name(request.account.clone()).await {
        Ok(Some(account)) => account,
        Ok(None) => return Json(Response::error(-101, "Account or password error")),
        Err(err) => {
            tracing::error!("database error: {err}");
            return Json(Response::error(-1, "Internal server error"));
        }
    };

    if !account.password.verify(&password) {
        return Json(Response::error(-101, "Account or password error"));
    }

    Json(Response::new(ResponseData {
        account: ResponseAccountData {
            area_code: String::from("**"),
            email: account.username.as_str().to_string(),
            country: String::from("RU"),
            is_email_verify: String::from("1"),
            uid: account.uid(),
            token: account.token,
        },
        ..Default::default()
    }))
}

async fn verify(
    state: State<AppStateRef>,
    request: Json<VerifyRequest>,
) -> Json<Response<ResponseData>> {
    let account = match state.db.get_account_by_uid(&request.uid).await {
        Ok(Some(account)) => account,
        Ok(None) => return Json(Response::error(-101, "Account cache error")),
        Err(err) => {
            tracing::error!("database error: {err}");
            return Json(Response::error(-1, "Internal server error"));
        }
    };

    if account.token == request.token {
        Json(Response::new(ResponseData {
            account: ResponseAccountData {
                area_code: String::from("**"),
                email: account.username.as_str().to_string(),
                country: String::from("RU"),
                is_email_verify: String::from("1"),
                uid: account.uid(),
                token: account.token,
            },
            ..Default::default()
        }))
    } else {
        Json(Response::error(
            -101,
            "For account safety, please log in again",
        ))
    }
}
