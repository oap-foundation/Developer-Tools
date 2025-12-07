use crate::config::{Config, LatencyMode, SabotageMode};
use clap::ValueEnum;
use tracing::info;

#[derive(ValueEnum, Clone, Debug, PartialEq)]
pub enum ScenarioMode {
    Default,
    Subway,
    MaliciousRelay,
    Ddos,
}

pub fn apply_preset(config: &mut Config, mode: ScenarioMode) {
    match mode {
        ScenarioMode::Default => {
            // Do nothing, use loaded config
        }
        ScenarioMode::Subway => {
            info!("🚇 Applying Preset: SUBWAY (High Latency, Spotty Connection)");
            config.chaos.enabled = true;
            config.chaos.latency_mode = LatencyMode::Jitter;
            config.chaos.latency_min_ms = 500;
            config.chaos.latency_max_ms = 3000;
            // 10% packet loss
            config.sabotage.mode = SabotageMode::PacketLoss;
            config.sabotage.drop_rate = 0.1;
        }
        ScenarioMode::MaliciousRelay => {
            info!("😈 Applying Preset: MALICIOUS RELAY (Corruption, Replay, Downgrade)");
            config.sabotage.mode = SabotageMode::Corrupt;
            config.sabotage.drop_rate = 0.2; // Corrupt 20%
            
            config.security.replay_enabled = true;
            config.security.replay_delay_ms = 2000;
            config.security.mitm_downgrade = true;
        }
        ScenarioMode::Ddos => {
            info!("🔥 Applying Preset: DDOS (Extreme Packet Loss)");
            config.sabotage.mode = SabotageMode::PacketLoss;
            config.sabotage.drop_rate = 0.9;
        }
    }
}
