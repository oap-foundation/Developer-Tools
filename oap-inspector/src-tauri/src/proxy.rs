use axum::{
    body::{Body, Bytes},
    extract::{State, Request},
    http::{StatusCode, HeaderMap, Method, Uri},
    response::{Response, IntoResponse},
    routing::any,
    Router,
};
use std::sync::Arc;
use tokio::net::TcpListener;
use reqwest::Client;
use std::time::Instant;
use crate::db::{log_traffic, DbState, TrafficLog};
use crate::keystore::KeyStore;
use crate::xray::XRayState;
use sqlx::{Pool, Sqlite};
use chrono::Utc;
use base64::{Engine as _, engine::general_purpose::STANDARD as BASE64};

#[derive(Clone)]
pub struct ProxyState {
    pub db_pool: Pool<Sqlite>,
    pub client: Client,
    pub target_url: String, // e.g. "http://localhost:8080"
    pub keystore: KeyStore,
    pub xray: Arc<XRayState>,
}

pub async fn start_proxy(pool: Pool<Sqlite>, target_url: String, keystore: KeyStore, xray: Arc<XRayState>) {
    let state = ProxyState {
        db_pool: pool,
        client: Client::new(),
        target_url,
        keystore,
        xray,
    };

    let app = Router::new()
        .route("/*path", any(handle_request)) // Capture all paths
        .route("/", any(handle_request)) // Capture root
        .with_state(state);

    println!("Starting Proxy on 0.0.0.0:9000");
    let listener = TcpListener::bind("0.0.0.0:9000").await.unwrap();
    axum::serve(listener, app).await.unwrap();
}

async fn handle_request(
    State(state): State<ProxyState>,
    method: Method,
    uri: Uri,
    headers: HeaderMap,
    body: Bytes,
) -> Response {
    let _start_time = Instant::now();

    // Construct target URL
    let path = uri.path_and_query().map(|p| p.as_str()).unwrap_or("/");
    let target = format!("{}{}", state.target_url, path);
    
    // Parse headers (simplify for logging)
    let req_headers_str = format!("{:?}", headers);
    
    // Process Request Body
    let req_body_vec = body.to_vec();
    let req_body_str = match String::from_utf8(req_body_vec.clone()) {
        Ok(s) => Some(s),
        Err(_) => Some(BASE64.encode(&req_body_vec)),
    };
    
    // X-Ray Decryption Attempt (Request)
    let secrets = state.keystore.get_all_secrets();
    let decrypted_req = if let Some(ref s) = req_body_str {
        state.xray.process_packet(s, &secrets)
    } else {
        None
    };

    // Forward Request
    let response_result = state.client
        .request(method.clone(), &target)
        .headers(headers.clone())
        .body(body.clone())
        .send()
        .await;

    // let duration_ms = start_time.elapsed().as_millis() as i64; // DB doesn't store duration yet? 
    // Wait, my db.rs removed duration_ms in favor of error/decrypted columns?
    // Let's check struct. Yes, I removed duration_ms to match MS 2. schema needs. 
    // I can add it back if needed but for now ignore.

    match response_result {
        Ok(res) => {
            let status = res.status();
            let res_headers = res.headers().clone();
            let res_headers_str = format!("{:?}", res_headers);
            
            // Read response body
            let res_bytes = match res.bytes().await {
                Ok(b) => b,
                Err(e) => return (StatusCode::BAD_GATEWAY, format!("Error reading response body: {}", e)).into_response(),
            };
            
            let res_body_vec = res_bytes.to_vec();
            let res_body_str = match String::from_utf8(res_body_vec.clone()) {
                Ok(s) => Some(s),
                Err(_) => Some(BASE64.encode(&res_body_vec)),
            };

            // X-Ray Decryption Attempt (Response)
            let decrypted_res = if let Some(ref s) = res_body_str {
                state.xray.process_packet(s, &secrets)
            } else {
                None
            };

            // Log to DB
            let log_entry = TrafficLog {
                id: 0, // Ignored on insert
                timestamp: Utc::now().to_rfc3339(),
                method: method.to_string(),
                url: target.clone(),
                status: status.as_u16(),
                request_headers: req_headers_str,
                request_body: req_body_str,
                response_headers: res_headers_str,
                response_body: res_body_str,
                error: None,
                decrypted_request_body: decrypted_req,
                decrypted_response_body: decrypted_res,
                is_replay: false,
            };
            
            let _ = log_traffic(&state.db_pool, &log_entry).await;

            // Construct Response
            let mut builder = Response::builder().status(status);
            for (key, value) in res_headers {
                if let Some(key) = key {
                     builder = builder.header(key, value);
                }
            }
            
            builder.body(Body::from(res_bytes)).unwrap_or_else(|_| StatusCode::INTERNAL_SERVER_ERROR.into_response())
        }
        Err(e) => {
             // Log error
             let log_entry = TrafficLog {
                id: 0,
                timestamp: Utc::now().to_rfc3339(),
                method: method.to_string(),
                url: target.clone(),
                status: 502, // Bad Gateway equivalent
                request_headers: req_headers_str,
                request_body: req_body_str, // We still log request for failed calls
                response_headers: "".to_string(),
                response_body: None,
                error: Some(e.to_string()),
                decrypted_request_body: decrypted_req,
                decrypted_response_body: None,
                is_replay: false,
             };
             let _ = log_traffic(&state.db_pool, &log_entry).await;

            (StatusCode::BAD_GATEWAY, format!("Proxy Error: {}", e)).into_response()
        }
    }
}
