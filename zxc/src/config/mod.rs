pub mod error;
pub mod global;
pub mod local;
mod misc;
mod parsed_config;
mod windows;

pub use global::GlobalConfig;
pub use global::addons::Addon;
pub use local::CliArgs;
pub use parsed_config::Config;
pub use windows::*;

const CONFIG_FILE_NAME: &str = "config.toml";
