mod config;
mod proxy;
mod sabotage;
mod security;
mod metrics;
mod presets;

use axum::{routing::any, Router, Extension};
use std::net::SocketAddr;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use crate::config::Config;
use crate::metrics::Metrics;
use crate::presets::{ScenarioMode, apply_preset};
use clap::Parser;
use std::sync::Arc;
use tokio::signal;
use std::sync::atomic::{AtomicBool, Ordering};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long, value_enum, default_value_t = ScenarioMode::Default)]
    mode: ScenarioMode,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Initialize logging
    tracing_subscriber::registry()
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "oap_chaos_monkey=info,tower_http=info".into()),
        ))
        .with(tracing_subscriber::fmt::layer())
        .init();

    tracing::info!("Starting OAP Chaos Monkey 🐒");

    let args = Args::parse();

    // Load config
    // Load config
    let mut config = Config::load("chaos.toml").unwrap_or_else(|e| {
        tracing::warn!("Could not load chaos.toml: {}. Using default configuration.", e);
        Config::default()
    });

    // Apply Preset
    apply_preset(&mut config, args.mode);
    
    let port = config.server.port;
    let client = reqwest::Client::new();
    let metrics = Arc::new(Metrics::new());

    // Build application with a single catch-all route
    let app = Router::new()
        .route("/*path", any(proxy::handler))
        .layer(Extension(config.clone()))
        .layer(Extension(client))
        .layer(Extension(metrics.clone()));

    let flood_control = Arc::new(AtomicBool::new(true));

    if config.security.exhaustion_flood {
        use crate::security;
        security::start_flood(config.server.target_url.clone(), flood_control.clone());
    }

    let addr = SocketAddr::from(([0, 0, 0, 0], port));
    tracing::info!("Listening on {}", addr);
    tracing::info!("Forwarding to: {}", config.server.target_url);

    let listener = tokio::net::TcpListener::bind(addr).await?;
    
    // Graceful shutdown
    axum::serve(listener, app)
        .with_graceful_shutdown(shutdown_signal(metrics.clone(), flood_control))
        .await?;

    Ok(())
}

async fn shutdown_signal(metrics: Arc<Metrics>, flood_control: Arc<AtomicBool>) {
    let ctrl_c = async {
        signal::ctrl_c()
            .await
            .expect("failed to install Ctrl+C handler");
    };

    #[cfg(unix)]
    let terminate = async {
        signal::unix::signal(signal::unix::SignalKind::terminate())
            .expect("failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let terminate = std::future::pending::<()>();

    tokio::select! {
        _ = ctrl_c => {},
        _ = terminate => {},
    }

    tracing::info!("Signal received, shutting down...");
    flood_control.store(false, Ordering::Relaxed);
    metrics.print_report();
}
