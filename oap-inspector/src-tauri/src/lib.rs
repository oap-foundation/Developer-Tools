mod db;
mod proxy;
mod keystore;
mod xray;
mod replay;

use db::{TrafficLog, DbState};
use keystore::KeyStore;
use xray::XRayState;
use tauri::{State, Manager};
use sqlx::{Pool, Sqlite};
use std::sync::Arc;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
async fn greet(name: String) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn get_traffic_logs(state: State<'_, DbState>) -> Result<Vec<TrafficLog>, String> {
    db::get_logs(&state.pool, 100).await.map_err(|e| e.to_string())
}

#[tauri::command]
async fn add_ephemeral_secret(state: State<'_, KeyStore>, key: String) -> Result<(), String> {
    // Basic validation or Key ID extraction could happen here.
    // For now, we use the key itself as ID or generate one?
    // KeyStore::add_key(id, key). 
    // Let's assume the user just pastes the key. We'll use a hash or just the key string as ID if lazy, 
    // or let the KeyStore handle it. 
    // My keystore impl takes (key_id, private_key).
    // I'll calculate a pseudo-ID or just use "key-N".
    // Better: Helper in keystore to add without ID?
    // Let's just use the key content as ID for deduplication.
    state.add_key(key.clone(), key);
    Ok(())
}

#[tauri::command]
async fn remove_ephemeral_secret(state: State<'_, KeyStore>, key: String) -> Result<(), String> {
    state.remove_key(&key);
    Ok(())
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            tauri::async_runtime::block_on(async {
                // Initialize DB
                let db_url = "sqlite:traffic.db";
                if !std::path::Path::new("traffic.db").exists() {
                    std::fs::File::create("traffic.db").expect("Failed to create db file");
                }
                let pool = db::init_db(db_url).await.expect("Failed to init DB");
                
                // Initialize State
                let keystore = KeyStore::new();
                let xray_state = Arc::new(XRayState::new());

                // Start Proxy
                let proxy_pool = pool.clone();
                let proxy_keystore = keystore.clone();
                let proxy_xray = xray_state.clone();
                
                tauri::async_runtime::spawn(async move {
                    let target = "https://httpbin.org".to_string(); 
                    proxy::start_proxy(proxy_pool, target, proxy_keystore, proxy_xray).await;
                });

                app.manage(DbState { pool });
                app.manage(keystore);
                app.manage(xray_state);
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            greet, 
            get_traffic_logs,
            add_ephemeral_secret,
            remove_ephemeral_secret,
            replay::replay_request,
            db::export_logs,
            db::import_logs
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
