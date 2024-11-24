#[path = "../out/action_info.rs"]
pub mod action_pb;
pub use prost::Message;

mod event_config;
pub use event_config::*;
