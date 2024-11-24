use tracing::level_filters::LevelFilter;
use tracing_subscriber::filter::EnvFilter;

pub fn init(level: tracing::Level) {
    tracing_subscriber::fmt()
        .with_max_level(level)
        .with_env_filter(
            EnvFilter::builder()
                .with_default_directive(LevelFilter::DEBUG.into())
                .from_env()
                .unwrap()
                .add_directive("ureq=error".parse().unwrap()),
        )
        .without_time()
        .with_target(false)
        .init();
}
