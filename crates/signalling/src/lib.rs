//! Signalling service entry points and SIP session orchestrator stubs.

use std::{sync::Arc, time::Duration};

use tokio::{sync::broadcast, task::JoinHandle, time};
use tracing::{info, instrument};

use voip_common::Result;

/// Placeholder SIP event enum; will be extended with actual protocol events.
#[derive(Debug, Clone)]
pub enum SipEvent {
    /// A heartbeat used to keep the event loop alive during early development.
    Heartbeat,
}

/// Asynchronous signalling service that listens for SIP events and processes them.
#[derive(Debug)]
pub struct SignallingService {
    shutdown_tx: broadcast::Sender<()>,
}

impl SignallingService {
    /// Create a new signalling service instance.
    pub fn new() -> Self {
        let (shutdown_tx, _rx) = broadcast::channel(1);
        Self { shutdown_tx }
    }

    /// Run the signalling loop until `shutdown` is called.
    #[instrument(name = "signalling.run", skip_all)]
    pub async fn run(&self) -> Result<()> {
        let mut shutdown_rx = self.shutdown_tx.subscribe();
        let mut ticker = time::interval(Duration::from_secs(5));

        loop {
            tokio::select! {
                _ = ticker.tick() => {
                    info!(event = ?SipEvent::Heartbeat, "signalling heartbeat");
                }
                _ = shutdown_rx.recv() => {
                    info!("shutdown signal received");
                    break;
                }
            }
        }

        Ok(())
    }

    /// Trigger a graceful shutdown.
    pub fn shutdown(&self) {
        // Ignore send errors (service already stopped).
        let _ = self.shutdown_tx.send(());
    }

    /// Spawn the service in a background task and return the handle.
    pub fn spawn(self: Arc<Self>) -> JoinHandle<Result<()>> {
        tokio::spawn(async move { self.run().await })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn heartbeat_ticks_and_shutdown() {
        let service = Arc::new(SignallingService::new());
        let handle = service.clone().spawn();

        // Allow the loop to tick at least once.
        time::sleep(Duration::from_millis(100)).await;
        service.shutdown();

        let res = handle.await.expect("join handle");
        assert!(res.is_ok());
    }
}
