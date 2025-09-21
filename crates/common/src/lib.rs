//! Common types, utilities, and proto definitions for VoIP microservices

pub mod errors;
pub mod events;
pub mod telemetry;
pub mod types;

// Re-export generated proto code
pub mod proto {
    pub mod common {
        tonic::include_proto!("voip.common");
    }

    pub mod sip {
        tonic::include_proto!("voip.sip");
    }

    pub mod media {
        tonic::include_proto!("voip.media");
    }

    pub mod routing {
        tonic::include_proto!("voip.routing");
    }

    pub mod provisioning {
        tonic::include_proto!("voip.provisioning");
    }

    pub mod auth {
        tonic::include_proto!("voip.auth");
    }

    pub mod monitoring {
        tonic::include_proto!("voip.monitoring");
    }
}

// Re-export commonly used types
pub use errors::{VoipError, Result};
pub use events::{EventBus, EventHandler};
pub use telemetry::{init_telemetry, Metrics, TraceContext};
pub use types::{CallId, ServiceConfig, ServiceInfo};

// Version information
pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const SERVICE_PREFIX: &str = "voip";

/// Initialize a service with common setup
pub async fn init_service(
    name: &str,
    config: ServiceConfig,
) -> Result<(ServiceInfo, EventBus)> {
    // Initialize telemetry
    init_telemetry(name, &config)?;

    // Create service info
    let service_info = ServiceInfo::new(name, VERSION);

    // Connect to event bus
    let event_bus = EventBus::connect(&config.nats_url).await?;

    // Register with service discovery
    if let Some(consul_addr) = &config.consul_addr {
        register_service(&service_info, consul_addr).await?;
    }

    tracing::info!(
        service = name,
        version = VERSION,
        instance_id = %service_info.instance_id,
        "Service initialized"
    );

    Ok((service_info, event_bus))
}

async fn register_service(_info: &ServiceInfo, consul_addr: &str) -> Result<()> {
    // TODO: Implement Consul registration
    tracing::debug!("Registering service with Consul at {}", consul_addr);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert!(!VERSION.is_empty());
        assert_eq!(SERVICE_PREFIX, "voip");
    }
}