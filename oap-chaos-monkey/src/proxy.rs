use crate::metrics::Metrics;
use std::sync::Arc;
use axum::{
    body::Body,
    http::{Request, StatusCode},
    response::{IntoResponse, Response},
    Extension,
};
use bytes::Bytes;
use rand::Rng;
use reqwest::Client;
use std::time::Duration;
use tracing::{info, warn};
use crate::config::{Config, LatencyMode};

pub async fn handler(
    Extension(config): Extension<Config>,
    Extension(client): Extension<Client>,
    Extension(metrics): Extension<Arc<Metrics>>,
    mut req: Request<Body>,
) -> Response {
    metrics.inc_total();
    
    let path = req.uri().path().to_string();
    let query = req.uri().query().unwrap_or("").to_string();
    
    info!("Incoming request: {} {}", req.method(), path);

    // 1. Latency Injection
    inject_latency(&config).await;

    // 2. Error Injection
    if let Some(error_response) = inject_error(&config, &metrics) {
        return error_response;
    }

    // 3. Prepare Target URI
    let target_base = config.server.target_url.trim_end_matches('/');
    let target_uri = if query.is_empty() {
        format!("{}{}", target_base, path)
    } else {
        format!("{}{}?{}", target_base, path, query)
    };
    
    info!("Forwarding to: {}", target_uri);

    // 4. Transform Request
    // Remove host header to let reqwest set it correctly for the target
    req.headers_mut().remove(axum::http::header::HOST);
    let method = req.method().clone();
    let headers = req.headers().clone();
    
    // 5. Body Processing (Sabotage/Security)
    let body_bytes_result = axum::body::to_bytes(req.into_body(), 10 * 1024 * 1024).await; // 10MB limit

    match body_bytes_result {
        Ok(bytes) => {
            handle_body_forwarding(bytes, &config, &metrics, &client, method, headers, target_uri).await
        }
        Err(e) => {
            warn!("Failed to read request body: {}", e);
            (StatusCode::BAD_REQUEST, format!("Failed to read body: {}", e)).into_response()
        }
    }
}

async fn inject_latency(config: &Config) {
    if config.chaos.enabled {
        let mut delay = Duration::from_millis(0);
        match config.chaos.latency_mode {
            LatencyMode::Fixed => {
                delay = Duration::from_millis(config.chaos.latency_fixed_ms);
            }
            LatencyMode::Jitter => {
                let mut rng = rand::thread_rng();
                let ms = rng.gen_range(config.chaos.latency_min_ms..=config.chaos.latency_max_ms);
                delay = Duration::from_millis(ms);
            }
            LatencyMode::None => {}
        }

        if delay.as_millis() > 0 {
            info!("🐒 Injecting latency: {}ms", delay.as_millis());
            tokio::time::sleep(delay).await;
        }
    }
}

fn inject_error(config: &Config, metrics: &Arc<Metrics>) -> Option<Response> {
    if config.chaos.enabled && !config.chaos.failure_codes.is_empty() {
        let mut rng = rand::thread_rng();
        if rng.gen_bool(config.chaos.failure_rate) {
            let code_idx = rng.gen_range(0..config.chaos.failure_codes.len());
            let code = config.chaos.failure_codes[code_idx];
            
            warn!("🐒 Injecting ERROR: {}", code);
            metrics.inc_dropped(); 
            
            let status = StatusCode::from_u16(code).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
            return Some((status, format!("OAP Chaos Monkey Simulated Error: {}", status)).into_response());
        }
    }
    None
}

async fn handle_body_forwarding(
    body: Bytes, 
    config: &Config, 
    metrics: &Arc<Metrics>,
    client: &Client,
    method: reqwest::Method,
    headers: reqwest::header::HeaderMap,
    target_uri: String
) -> Response {
    use crate::sabotage;
    
    // Sabotage
    // Sabotage
    let (mut processed_bytes, was_modified) = sabotage::apply_sabotage(body, &config.sabotage);
    
    // Metrics for sabotage
    if was_modified {
         metrics.inc_corrupted();
    }

    // MitM
    if config.security.mitm_downgrade {
        if let Some(b) = processed_bytes {
            use crate::security;
            let (new_body, downgraded) = security::downgrade_handshake(b);
            processed_bytes = Some(new_body);
            if downgraded {
                metrics.inc_corrupted();
            }
        }
    }

    match processed_bytes {
        Some(final_bytes) => {
            // Replay
            if config.security.replay_enabled {
                use crate::security;
                metrics.inc_replayed();
                security::schedule_replay(
                    client.clone(),
                    target_uri.clone(),
                    method.clone(),
                    headers.clone(),
                    final_bytes.clone(),
                    config.security.replay_delay_ms
                );
            }

            // Forward
            match client.request(method, &target_uri)
                .headers(headers)
                .body(final_bytes)
                .send()
                .await 
            {
                Ok(resp) => {
                    metrics.inc_success();
                    let status = resp.status();
                    let headers = resp.headers().clone();
                    let body = resp.bytes().await.unwrap_or_default();
                    let mut response = Body::from(body).into_response();
                    *response.status_mut() = status;
                    for (k, v) in headers.iter() {
                        if let Ok(val) = v.to_str() { 
                           if let Ok(hdr_name) = axum::http::header::HeaderName::from_bytes(k.as_str().as_bytes()) {
                                response.headers_mut().insert(hdr_name, axum::http::HeaderValue::from_str(val).unwrap_or(v.clone()));
                           } 
                        } 
                    }
                    response
                }
                Err(e) => {
                    warn!("Upstream error: {}", e);
                    (StatusCode::BAD_GATEWAY, format!("Upstream error: {}", e)).into_response()
                }
            }
        }
        None => {
            metrics.inc_dropped();
            warn!("🦖 Dropped request to {} (simulated packet loss)", target_uri);
            (StatusCode::OK, "Simulated Packet Loss (Accepted)").into_response()
        }
    }
}
