use axum::{
    extract::State,
    response::Html,
    routing::{get, post},
    Form, Router,
};
use serde::Deserialize;

use crate::{
    database::schema::{Password, PasswordError, Username},
    AppStateRef,
};

const REGISTER_PAGE: Html<&str> = Html(include_str!("../../html/register.html"));

pub fn routes() -> Router<AppStateRef> {
    Router::new()
        .route("/account/register", get(|| async { REGISTER_PAGE }))
        .route("/account/register", post(process_register_request))
}

#[derive(Deserialize)]
struct RegisterForm {
    pub username: String,
    pub password: String,
    pub password_v2: String,
}

async fn process_register_request(
    State(state): State<AppStateRef>,
    Form(form): Form<RegisterForm>,
) -> Html<String> {
    let Some(username) = Username::parse(form.username) else {
        return html_result(ResultStatus::Failure, "Invalid username format; should consists of characters [A-Za-z0-9_] and be at least 6 characters long.");
    };

    let password = match Password::new(form.password, form.password_v2) {
        Ok(password) => password,
        Err(PasswordError::PairMismatch) => {
            return html_result(ResultStatus::Failure, "Passwords pair doesn't match")
        }
        Err(PasswordError::LengthMismatch) => {
            return html_result(
                ResultStatus::Failure,
                "Password should contain at least 8 and not more than 30 characters",
            )
        }
        Err(PasswordError::HashFailed(err)) => {
            tracing::error!("failed to hash password, err: {err}");
            return html_result(ResultStatus::Failure, "Internal server error");
        }
    };

    match state.db.create_account(username, password).await {
        Ok(Some(_)) => html_result(
            ResultStatus::Success,
            "Account successfully registered, now you can use in-game login",
        ),
        Ok(None) => html_result(
            ResultStatus::Failure,
            "Account with specified username already exists",
        ),
        Err(err) => {
            tracing::error!("database operation error: {err}");
            html_result(ResultStatus::Failure, "Internal server error")
        }
    }
}

enum ResultStatus {
    Failure,
    Success,
}

impl ResultStatus {
    pub fn as_str(&self) -> &str {
        match self {
            Self::Failure => "error",
            Self::Success => "success",
        }
    }
}

fn html_result(result: ResultStatus, message: &str) -> Html<String> {
    static RESULT_HTML: &str = include_str!("../../html/result.html");

    Html(
        RESULT_HTML
            .replace("%RESULT%", result.as_str())
            .replace("%MESSAGE%", message),
    )
}
