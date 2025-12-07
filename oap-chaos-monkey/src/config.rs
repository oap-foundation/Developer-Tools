use serde::Deserialize;
use std::fs;
use anyhow::Result;

#[derive(Clone, Debug, Deserialize)]
pub struct Config {
    pub server: ServerConfig,
    pub chaos: ChaosConfig,
    #[serde(default)]
    pub sabotage: SabotageConfig,
    #[serde(default)]
    pub security: SecurityConfig,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ServerConfig {
    pub port: u16,
    pub target_url: String,
}

#[derive(Clone, Debug, Deserialize)]
pub struct ChaosConfig {
    pub enabled: bool,
    pub latency_mode: LatencyMode,
    pub latency_fixed_ms: u64,
    pub latency_min_ms: u64,
    pub latency_max_ms: u64,
    pub failure_rate: f64,
    pub failure_codes: Vec<u16>,
}

#[derive(Clone, Debug, Deserialize, Default)]
pub struct SabotageConfig {
    pub mode: SabotageMode,
    pub drop_rate: f64,
    pub target_shard_indices: Vec<usize>,
}

#[derive(Clone, Debug, Deserialize, PartialEq, Default)]
pub enum SabotageMode {
    #[default]
    None,
    PacketLoss,
    Corrupt,
    Truncate,
}

#[derive(Clone, Debug, Deserialize, Default)]
pub struct SecurityConfig {
    pub replay_enabled: bool,
    pub replay_delay_ms: u64,
    pub mitm_downgrade: bool,
    pub exhaustion_flood: bool,
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub enum LatencyMode {
    None,
    Fixed,
    Jitter,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            server: ServerConfig {
                port: 8080,
                target_url: "https://httpbin.org".to_string(),
            },
            chaos: ChaosConfig {
                enabled: false,
                latency_mode: LatencyMode::None,
                latency_fixed_ms: 0,
                latency_min_ms: 0,
                latency_max_ms: 0,
                failure_rate: 0.0,
                failure_codes: vec![],
            },
            sabotage: SabotageConfig::default(),
            security: SecurityConfig::default(),
        }
    }
}

impl Config {
    pub fn load(path: &str) -> Result<Self> {
        let content = fs::read_to_string(path)?;
        let config: Config = toml::from_str(&content)?;
        Ok(config)
    }
}
