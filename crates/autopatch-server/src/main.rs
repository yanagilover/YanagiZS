use anyhow::Result;
use axum::{handler::HandlerWithoutStateExt, http::StatusCode, Router};
use common::config::TomlConfig;
use serde::Deserialize;
use tower_http::services::ServeDir;

#[derive(Deserialize)]
struct AutopatchConfig {
    pub http_addr: String,
    pub serve_dir: String,
}

impl TomlConfig for AutopatchConfig {
    const DEFAULT_TOML: &str = include_str!("../autopatch.toml");
}

#[tokio::main]
async fn main() -> Result<()> {
    let config = AutopatchConfig::load_or_create("autopatch.toml");
    common::print_splash();
    common::logging::init(tracing::Level::DEBUG);
    axum_server::bind(config.http_addr.parse()?)
        .serve(
            Router::new()
                .nest_service(
                    "/",
                    ServeDir::new(&config.serve_dir).not_found_service(not_found.into_service()),
                )
                .into_make_service(),
        )
        .await?;

    Ok(())
}

async fn not_found() -> (StatusCode, &'static str) {
    (StatusCode::NOT_FOUND, "404 page not found")
}
