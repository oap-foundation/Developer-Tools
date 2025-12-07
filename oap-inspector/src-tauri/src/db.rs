use sqlx::{sqlite::SqlitePoolOptions, Pool, Sqlite, FromRow, SqlitePool, Row};
use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize, Clone, FromRow)]
pub struct TrafficLog {
    pub id: i64,
    pub timestamp: String,
    pub method: String,
    pub url: String,
    pub status: u16,
    pub request_headers: String,
    pub request_body: Option<String>,
    pub response_headers: String,
    pub response_body: Option<String>,
    pub error: Option<String>,
    pub decrypted_request_body: Option<String>,
    pub decrypted_response_body: Option<String>,
    pub is_replay: bool,
}

pub struct DbState {
    pub pool: Pool<Sqlite>,
}

pub async fn init_db(database_url: &str) -> Result<Pool<Sqlite>, sqlx::Error> {
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect(database_url).await?;

    sqlx::query("DROP TABLE IF EXISTS traffic_logs").execute(&pool).await?;

    sqlx::query(
        "CREATE TABLE IF NOT EXISTS traffic_logs (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            timestamp TEXT NOT NULL,
            method TEXT NOT NULL,
            url TEXT NOT NULL,
            status INTEGER NOT NULL,
            request_headers TEXT NOT NULL,
            request_body TEXT,
            response_headers TEXT NOT NULL,
            response_body TEXT,
            error TEXT,
            decrypted_request_body TEXT,
            decrypted_response_body TEXT,
            is_replay BOOLEAN DEFAULT 0
        )",
    )
    .execute(&pool)
    .await?;
    Ok(pool)
}

pub async fn log_traffic(
    pool: &SqlitePool,
    entry: &TrafficLog,
) -> Result<i64, sqlx::Error> {
    let row = sqlx::query(
        "INSERT INTO traffic_logs (timestamp, method, url, status, request_headers, request_body, response_headers, response_body, error, decrypted_request_body, decrypted_response_body, is_replay)
        VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)
        RETURNING id"
    )
    .bind(&entry.timestamp)
    .bind(&entry.method)
    .bind(&entry.url)
    .bind(entry.status)
    .bind(&entry.request_headers)
    .bind(&entry.request_body)
    .bind(&entry.response_headers)
    .bind(&entry.response_body)
    .bind(&entry.error)
    .bind(&entry.decrypted_request_body)
    .bind(&entry.decrypted_response_body)
    .bind(entry.is_replay)
    .fetch_one(pool)
    .await?;

    let id: i64 = row.get("id");
    Ok(id)
}

pub async fn get_logs(pool: &Pool<Sqlite>, limit: i64) -> Result<Vec<TrafficLog>, sqlx::Error> {
    sqlx::query_as::<_, TrafficLog>(
        "SELECT id, timestamp, method, url, status, request_headers, request_body, response_headers, response_body, error, decrypted_request_body, decrypted_response_body, is_replay
        FROM traffic_logs
        ORDER BY id DESC
        LIMIT ?"
    )
    .bind(limit)
    .fetch_all(pool)
    .await
}

pub async fn get_log_by_id(pool: &Pool<Sqlite>, id: i64) -> Result<TrafficLog, sqlx::Error> {
    sqlx::query_as::<_, TrafficLog>(
        "SELECT id, timestamp, method, url, status, request_headers, request_body, response_headers, response_body, error, decrypted_request_body, decrypted_response_body, is_replay
        FROM traffic_logs
        WHERE id = ?"
    )
    .bind(id)
    .fetch_one(pool)
    .await
}

#[tauri::command]
pub async fn export_logs(state: tauri::State<'_, DbState>, path: String) -> Result<String, String> {
    let logs = get_logs(&state.pool, 1000).await.map_err(|e| e.to_string())?;
    let json = serde_json::to_string_pretty(&logs).map_err(|e| e.to_string())?;
    std::fs::write(&path, json).map_err(|e| e.to_string())?;
    Ok(format!("Exported {} logs to {}", logs.len(), path))
}

#[tauri::command]
pub async fn import_logs(state: tauri::State<'_, DbState>, path: String) -> Result<String, String> {
    let content = std::fs::read_to_string(&path).map_err(|e| e.to_string())?;
    let logs: Vec<TrafficLog> = serde_json::from_str(&content).map_err(|e| e.to_string())?;
    
    let mut count = 0;
    for log in logs {
        // Reset ID to allow autoincrement
        // log.id = 0; // Handled by struct if we ignore it, but `log_traffic` binds fields explicitely
        
        // Ensure is_replay is set if missing (handled by default serde?)
        // log_traffic inserts explicitly.
        
        let _ = log_traffic(&state.pool, &log).await;
        count += 1;
    }
    
    Ok(format!("Imported {} logs from {}", count, path))
}
