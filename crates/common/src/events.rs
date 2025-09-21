//! Event bus implementation using NATS

use async_nats::{Client, Message, Subscriber};
use async_trait::async_trait;
use bytes::Bytes;
use futures_util::StreamExt;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tracing::{debug, error, info};

use crate::errors::{Result, VoipError};
use crate::proto;

/// Event bus for inter-service communication
#[derive(Clone)]
pub struct EventBus {
    client: Client,
    service_name: String,
}

impl EventBus {
    /// Connect to NATS server
    pub async fn connect(url: &str) -> Result<Self> {
        let client = async_nats::connect(url)
            .await
            .map_err(|e| VoipError::Nats(Box::new(e)))?;

        info!("Connected to NATS at {}", url);

        Ok(Self {
            client,
            service_name: String::new(),
        })
    }

    /// Set service name for event attribution
    pub fn with_service_name(mut self, name: impl Into<String>) -> Self {
        self.service_name = name.into();
        self
    }

    /// Publish an event
    pub async fn publish<T>(&self, subject: &str, event: &T) -> Result<()>
    where
        T: Serialize,
    {
        let payload = bincode::serialize(event)
            .map_err(|e| VoipError::Internal(format!("Failed to serialize event: {}", e)))?;

        self.client
            .publish(subject.to_string(), Bytes::from(payload))
            .await
            .map_err(|e| VoipError::Nats(Box::new(e)))?;

        debug!(
            service = %self.service_name,
            subject = %subject,
            "Event published"
        );

        Ok(())
    }

    /// Publish a proto event
    pub async fn publish_proto(&self, subject: &str, event: proto::common::Event) -> Result<()> {
        use prost::Message;

        let payload = event.encode_to_vec();

        self.client
            .publish(subject.to_string(), Bytes::from(payload))
            .await
            .map_err(|e| VoipError::Nats(Box::new(e)))?;

        debug!(
            service = %self.service_name,
            subject = %subject,
            event_type = ?event.r#type,
            correlation_id = %event.correlation_id,
            "Proto event published"
        );

        Ok(())
    }

    /// Subscribe to events
    pub async fn subscribe(&self, subject: &str) -> Result<Subscriber> {
        let subscriber = self.client
            .subscribe(subject.to_string())
            .await
            .map_err(|e| VoipError::Nats(Box::new(e)))?;

        info!(
            service = %self.service_name,
            subject = %subject,
            "Subscribed to events"
        );

        Ok(subscriber)
    }

    /// Subscribe to a queue group (load balancing)
    pub async fn queue_subscribe(&self, subject: &str, queue: &str) -> Result<Subscriber> {
        let subscriber = self.client
            .queue_subscribe(subject.to_string(), queue.to_string())
            .await
            .map_err(|e| VoipError::Nats(Box::new(e)))?;

        info!(
            service = %self.service_name,
            subject = %subject,
            queue = %queue,
            "Subscribed to queue group"
        );

        Ok(subscriber)
    }

    /// Request-reply pattern
    pub async fn request<T, R>(&self, subject: &str, request: &T) -> Result<R>
    where
        T: Serialize,
        R: for<'de> Deserialize<'de>,
    {
        let payload = bincode::serialize(request)
            .map_err(|e| VoipError::Internal(format!("Failed to serialize request: {}", e)))?;

        let response = self.client
            .request(subject.to_string(), Bytes::from(payload))
            .await
            .map_err(|e| VoipError::Nats(Box::new(e)))?;

        bincode::deserialize(&response.payload)
            .map_err(|e| VoipError::Internal(format!("Failed to deserialize response: {}", e)))
    }

    /// Create a responder for request-reply
    pub async fn responder(&self, subject: &str) -> Result<Subscriber> {
        self.subscribe(subject).await
    }

    /// Health check
    pub async fn ping(&self) -> Result<()> {
        self.client
            .flush()
            .await
            .map_err(|e| VoipError::Nats(Box::new(e)))?;
        Ok(())
    }
}

/// Event handler trait
#[async_trait]
pub trait EventHandler: Send + Sync {
    /// Handle an event
    async fn handle(&self, event: Message) -> Result<()>;
}

/// Event processor that runs handlers
pub struct EventProcessor {
    handlers: Vec<Arc<dyn EventHandler>>,
}

impl EventProcessor {
    /// Create new processor
    pub fn new() -> Self {
        Self {
            handlers: Vec::new(),
        }
    }

    /// Add an event handler
    pub fn add_handler<H>(&mut self, handler: H) -> &mut Self
    where
        H: EventHandler + 'static,
    {
        self.handlers.push(Arc::new(handler));
        self
    }

    /// Process events from a subscriber
    pub async fn process(&self, mut subscriber: Subscriber) -> Result<()> {
        while let Some(message) = subscriber.next().await {
            let handlers = self.handlers.clone();

            // Process each message concurrently
            tokio::spawn(async move {
                for handler in handlers {
                    if let Err(e) = handler.handle(message.clone()).await {
                        error!(
                            error = %e,
                            subject = %message.subject,
                            "Failed to handle event"
                        );
                    }
                }
            });
        }

        Ok(())
    }
}

/// Common event types
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallStartedEvent {
    pub call_id: String,
    pub from: String,
    pub to: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CallEndedEvent {
    pub call_id: String,
    pub duration: chrono::Duration,
    pub reason: String,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceHealthEvent {
    pub service: String,
    pub instance_id: String,
    pub status: String,
    pub metrics: ServiceMetrics,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceMetrics {
    pub cpu_usage: f64,
    pub memory_usage: f64,
    pub active_connections: u64,
    pub requests_per_second: f64,
    pub error_rate: f64,
}

/// Event subject patterns
pub mod subjects {
    /// Call events
    pub const CALL_STARTED: &str = "voip.call.started";
    pub const CALL_ENDED: &str = "voip.call.ended";
    pub const CALL_FAILED: &str = "voip.call.failed";

    /// Registration events
    pub const REGISTRATION_SUCCESS: &str = "voip.registration.success";
    pub const REGISTRATION_FAILED: &str = "voip.registration.failed";

    /// Service events
    pub const SERVICE_HEALTH: &str = "voip.service.health";
    pub const SERVICE_STARTED: &str = "voip.service.started";
    pub const SERVICE_STOPPED: &str = "voip.service.stopped";

    /// Media events
    pub const MEDIA_STARTED: &str = "voip.media.started";
    pub const MEDIA_STOPPED: &str = "voip.media.stopped";
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_event_subjects() {
        assert_eq!(subjects::CALL_STARTED, "voip.call.started");
        assert_eq!(subjects::SERVICE_HEALTH, "voip.service.health");
    }

    #[test]
    fn test_service_metrics() {
        let metrics = ServiceMetrics {
            cpu_usage: 45.5,
            memory_usage: 60.2,
            active_connections: 150,
            requests_per_second: 1000.0,
            error_rate: 0.01,
        };

        assert_eq!(metrics.cpu_usage, 45.5);
        assert_eq!(metrics.active_connections, 150);
    }
}