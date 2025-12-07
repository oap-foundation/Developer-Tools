use anyhow::Result;
use std::path::PathBuf;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Config {
    pub default_relay: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            default_relay: "http://localhost:3000".to_string(),
        }
    }
}

pub async fn load(_path: Option<PathBuf>) -> Result<Config> {
    // TODO: Load from file
    Ok(Config::default())
}
