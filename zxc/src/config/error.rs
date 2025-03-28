use std::env::VarError;

use thiserror::Error;

#[derive(Debug, Error)]
pub enum ConfigError {
    // ----- Local Config -----
    #[error("Unable to read config| {0}")]
    FileRead(#[from] std::io::Error),
    #[error("Unable to Parse config| {0}")]
    ParseConfig(#[from] toml::de::Error),

    // ----- Global Config -----
    #[error("Home env | {0}")]
    Env(#[from] VarError),
}
