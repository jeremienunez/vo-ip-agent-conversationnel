//! Command-line entrypoint for the signalling service.

use std::sync::Arc;

use tokio::signal;
use tracing::info;

use voip_common::{init_telemetry, Result, VoipError};
use voip_signalling::SignallingService;

#[tokio::main]
async fn main() -> Result<()> {
    let config = voip_common::types::ServiceConfig::default();
    init_telemetry("signalling-service", &config)
        .map_err(|e| VoipError::Internal(format!("init telemetry failed: {}", e)))?;
    info!("starting signalling service");

    let service = Arc::new(SignallingService::new());
    let handle = service.clone().spawn();

    signal::ctrl_c().await
        .map_err(|e| VoipError::Internal(format!("waiting for ctrl+c failed: {}", e)))?;
    info!("ctrl+c received");
    service.shutdown();

    handle.await
        .map_err(|e| VoipError::Internal(format!("joining signalling task failed: {}", e)))??;
    info!("signalling service stopped");
    Ok(())
}
