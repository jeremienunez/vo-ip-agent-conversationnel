//! Media relay façade managing RTP proxies and QoS telemetry.

use std::time::Duration;

use tokio::{sync::watch, time};
use tracing::{debug, instrument};

use voip_common::Result;

/// Placeholder struct describing a media relay session.
#[derive(Debug, Clone)]
pub struct MediaSession {
    /// Unique identifier for the media flow.
    pub session_id: String,
    /// Preferred codec negotiated for the session.
    pub codec: String,
}

/// Media relay manager in charge of supervising sessions.
#[derive(Debug)]
pub struct MediaRelay {
    stop_tx: watch::Sender<bool>,
}

impl MediaRelay {
    /// Build a new media relay instance.
    pub fn new() -> Self {
        let (stop_tx, _rx) = watch::channel(false);
        Self { stop_tx }
    }

    /// Start a dummy supervision loop.
    #[instrument(name = "media.supervise", skip_all)]
    pub async fn supervise(&self) -> Result<()> {
        let mut stop_rx = self.stop_tx.subscribe();
        let mut ticker = time::interval(Duration::from_secs(10));

        loop {
            tokio::select! {
                _ = ticker.tick() => {
                    debug!("media supervisor heartbeat");
                }
                update = stop_rx.changed() => {
                    if update.is_err() || *stop_rx.borrow() {
                        debug!("media supervisor stop");
                        break;
                    }
                }
            }
        }

        Ok(())
    }

    /// Request the supervision loop to stop.
    pub fn stop(&self) {
        let _ = self.stop_tx.send(true);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::Arc;

    #[tokio::test]
    async fn supervisor_stops_when_requested() {
        let relay = Arc::new(MediaRelay::new());
        let relay_clone = relay.clone();
        let supervisor = tokio::spawn(async move { relay_clone.supervise().await });
        time::sleep(Duration::from_millis(50)).await;
        relay.stop();
        supervisor.await.expect("join").expect("result");
    }
}
