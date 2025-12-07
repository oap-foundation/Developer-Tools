use tauri::State;
use crate::db::{self, DbState, TrafficLog};
use crate::xray::XRayState;
use serde_json::Value;
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};

#[tauri::command]
pub async fn replay_request(
    state_db: State<'_, DbState>,
    state_xray: State<'_, XRayState>,
    id: i64,
    new_decrypted_body: String
) -> Result<String, String> {
    println!("Replaying request {}", id);

    // 1. Get original log
    let log = db::get_log_by_id(&state_db.pool, id).await.map_err(|e| e.to_string())?;

    // 2. Extract threadId from original body to find correct keys
    // The key store is indexed by Connection ID / Request ID.
    // In OAP, the initial handshake ID often becomes the threadId.
    let original_body = log.decrypted_request_body.ok_or("No decrypted body available for this request")?;
    let json: Value = serde_json::from_str(&original_body).map_err(|_| "Original body is not valid JSON")?;
    
    let thread_id = json.get("threadId").and_then(|v| v.as_str())
         .or_else(|| json.get("id").and_then(|v| v.as_str())) // Fallback for Handshake Request itself
         .ok_or("Could not find threadId or id in original message")?;

    println!("Attempting re-encryption for thread: {}", thread_id);

    // 3. Encrypt new body
    let encrypted_body = state_xray.encrypt_packet(thread_id, &new_decrypted_body)
        .ok_or("Failed to encrypt packet. Session keys not found or encryption failed. ensure you injected the keys used for this session.")?;

    // 4. Prepare Request
    let client = reqwest::Client::new();
    let mut req_builder = client.request(
        reqwest::Method::from_bytes(log.method.as_bytes()).unwrap_or(reqwest::Method::POST),
        &log.url
    );

    // Copy headers (excluding transfer-encoding/content-length/host)
    // Simplified: Just Content-Type
    req_builder = req_builder.header("Content-Type", "application/json"); 

    // 5. Send
    let res = req_builder.body(encrypted_body.clone())
        .send()
        .await
        .map_err(|e| format!("Request failed: {}", e))?;

    let status = res.status().as_u16();
    let res_headers = format!("{:?}", res.headers());
    let res_body_bytes = res.bytes().await.map_err(|e| e.to_string())?;
    let res_body_str = String::from_utf8(res_body_bytes.to_vec()).unwrap_or_else(|_| BASE64.encode(&res_body_bytes));

    // 6. Log the Replay
    let replay_log = TrafficLog {
        id: 0, // Ignored on insert
        timestamp: chrono::Utc::now().to_rfc3339(),
        method: log.method,
        url: log.url,
        status,
        request_headers: "Replayed".to_string(), // Simplified
        request_body: Some(encrypted_body), // The actual encrypted payload sent
        response_headers: res_headers,
        response_body: Some(res_body_str.clone()),
        error: None,
        decrypted_request_body: Some(new_decrypted_body), // We know what we sent
        decrypted_response_body: state_xray.process_packet(&res_body_str, &[]), // Try to decrypt response (keys already loaded)
        is_replay: true,
    };
    
    let _ = db::log_traffic(&state_db.pool, &replay_log).await;
    
    Ok(format!("Replay successful. Status: {}", status))
}
