use bytes::Bytes;
use crate::config::SecurityConfig;
use reqwest::{Client, Method, Url};
use serde_json::Value;
use std::time::Duration;
use tracing::{info, warn};
use tokio::task;

// --- REPLAY ATTACK ---
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

// --- REPLAY ATTACK ---
pub fn schedule_replay(
    client: Client,
    target_uri: String,
    method: Method,
    headers: reqwest::header::HeaderMap,
    body: Bytes,
    delay_ms: u64,
) {
    task::spawn(async move {
        // Wait configured delay
        tokio::time::sleep(Duration::from_millis(delay_ms)).await;
        
        warn!("🕵️ SCHEDULED REPLAY: Resending request to {}", target_uri);
        
        // Resend request
        let _ = client.request(method, &target_uri)
            .headers(headers)
            .body(body)
            .send()
            .await
            .map_err(|e| warn!("Replay failed: {}", e));
    });
}

// --- MITM DOWNGRADE ---
pub fn downgrade_handshake(body: Bytes) -> (Bytes, bool) {
    if let Ok(mut json) = serde_json::from_slice::<Value>(&body) {
        if let Some(cipher_suite) = json.get_mut("cipher_suite") {
            warn!("🕵️ MitM: Downgrading cipher_suite from {} to 'WEAK_RC4'", cipher_suite);
            *cipher_suite = Value::String("WEAK_RC4".to_string());
            
            if let Ok(new_body) = serde_json::to_vec(&json) {
                return (Bytes::from(new_body), true);
            }
        }
    }
    // Return original if parsing failed or no target field
    (body, false)
}

// --- STORAGE EXHAUSTION FLOOD ---
pub fn start_flood(target_url: String, keep_running: Arc<AtomicBool>) {
    task::spawn(async move {
        let client = Client::new();
        let target = format!("{}/inbox", target_url.trim_end_matches('/')); // Assumptions
        
        warn!("🌊 FLOOD: Starting Storage Exhaustion Attack on {}", target);
        
        while keep_running.load(Ordering::Relaxed) {
            // Generate orphan shard (random garbage)
            let garbage = serde_json::json!({
                "type": "Shard",
                "shard_index": rand::random::<u8>(), // Random index to confuse
                "data": "GarbageDataToFillDiskSpace..."
            });
            
            let _ = client.post(&target)
                .json(&garbage)
                .send()
                .await;
                
            // Rate limit slightly to avoid self-DoS, but fast enough to annoy
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
        warn!("🌊 FLOOD: Stopped.");
    });
}
