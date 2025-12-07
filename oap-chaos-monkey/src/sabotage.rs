use bytes::{Bytes, BytesMut};
use crate::config::{SabotageConfig, SabotageMode};
use rand::Rng;
use serde_json::Value;
use tracing::{info, warn};

pub fn apply_sabotage(body: Bytes, config: &SabotageConfig) -> (Option<Bytes>, bool) {
    match config.mode {
        SabotageMode::None => (Some(body), false),
        SabotageMode::PacketLoss => {
            if should_drop(&body, config) {
                warn!("🦖 SABOTAGE: Shard Eaten (Dropping request)");
                (None, true)
            } else {
                (Some(body), false)
            }
        }
        SabotageMode::Corrupt => {
            (Some(corrupt_body(body)), true)
        }
        SabotageMode::Truncate => {
            (Some(truncate_body(body)), true)
        }
    }
}

fn should_drop(body: &Bytes, config: &SabotageConfig) -> bool {
    let mut rng = rand::thread_rng();

    // 1. Check random drop rate
    if config.drop_rate > 0.0 && rng.gen_bool(config.drop_rate) {
        // If specific indices are targeted, verify if this body matches
        if !config.target_shard_indices.is_empty() {
             return is_target_shard(body, &config.target_shard_indices);
        }
        return true;
    }
    
    // 2. Deterministic drop for targeted indices if rate is 0? 
    // Plan said: "Drop Rate" AND "Smart Drop". 
    // If indices are set, we ONLY drop those specific indices? Or we drop randomly among them?
    // Let's assume: If indices are set, we check if it is one of them. If it IS, then we check rng against drop_rate.
    // Wait, simpler: If indices are set, we treat non-matching indices as "safe". Matching indices are subject to drop_rate.
    // If drop_rate is 1.0, we always drop matching indices.
    
    false
}

fn is_target_shard(body: &Bytes, targets: &[usize]) -> bool {
    // Try to parse as JSON to find shard_index
    if let Ok(v) = serde_json::from_slice::<Value>(body) {
        // OAP fields variance: 'shard_index', 'index', 'shardId', etc.
        // Let's look for common ones.
        let index = v.get("shard_index")
            .or_else(|| v.get("index"))
            .and_then(|i| i.as_u64());

        if let Some(idx) = index {
            return targets.contains(&(idx as usize));
        }
    }
    false
}

fn corrupt_body(body: Bytes) -> Bytes {
    let mut mut_body = BytesMut::from(body.as_ref());
    if !mut_body.is_empty() {
        let mut rng = rand::thread_rng();
        let idx = rng.gen_range(0..mut_body.len());
        let original = mut_body[idx];
        // Flip a bit
        mut_body[idx] = original ^ 0xFF; 
        warn!("🦖 SABOTAGE: Bit Flipped at index {} ({} -> {})", idx, original, mut_body[idx]);
    }
    mut_body.into()
}

fn truncate_body(body: Bytes) -> Bytes {
    let len = body.len() / 2;
    warn!("🦖 SABOTAGE: Truncated body from {} to {} bytes", body.len(), len);
    body.slice(0..len)
}
