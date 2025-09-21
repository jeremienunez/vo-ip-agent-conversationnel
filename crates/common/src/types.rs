//! Common types shared across services

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;

/// Service configuration
#[derive(Debug, Clone, Deserialize, Serialize)]
pub struct ServiceConfig {
    /// Service name
    pub name: String,

    /// Service version
    pub version: String,

    /// HTTP/gRPC bind address
    pub bind_addr: String,

    /// NATS URL
    pub nats_url: String,

    /// Redis URL
    pub redis_url: Option<String>,

    /// PostgreSQL URL
    pub postgres_url: Option<String>,

    /// Consul address for service discovery
    pub consul_addr: Option<String>,

    /// OpenTelemetry OTLP endpoint
    pub otlp_endpoint: Option<String>,

    /// Log level (trace, debug, info, warn, error)
    pub log_level: String,

    /// Environment (dev, staging, prod)
    pub environment: String,

    /// Additional service-specific config
    #[serde(flatten)]
    pub extra: serde_json::Value,
}

impl Default for ServiceConfig {
    fn default() -> Self {
        Self {
            name: "unnamed-service".to_string(),
            version: "0.1.0".to_string(),
            bind_addr: "0.0.0.0:50051".to_string(),
            nats_url: "nats://localhost:4222".to_string(),
            redis_url: None,
            postgres_url: None,
            consul_addr: None,
            otlp_endpoint: None,
            log_level: "info".to_string(),
            environment: "dev".to_string(),
            extra: serde_json::Value::Null,
        }
    }
}

/// Service information
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServiceInfo {
    pub name: String,
    pub version: String,
    pub instance_id: Uuid,
    pub started_at: DateTime<Utc>,
    pub environment: String,
    pub hostname: String,
}

impl ServiceInfo {
    /// Create new service info
    pub fn new(name: &str, version: &str) -> Self {
        let hostname = hostname::get()
            .map(|h| h.to_string_lossy().to_string())
            .unwrap_or_else(|_| "unknown".to_string());

        Self {
            name: name.to_string(),
            version: version.to_string(),
            instance_id: Uuid::new_v4(),
            started_at: Utc::now(),
            environment: std::env::var("ENVIRONMENT").unwrap_or_else(|_| "dev".to_string()),
            hostname,
        }
    }

    /// Get service uptime
    pub fn uptime(&self) -> chrono::Duration {
        Utc::now() - self.started_at
    }
}

/// Call identifier
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct CallId {
    /// Internal UUID
    pub id: Uuid,

    /// SIP Call-ID header
    pub sip_call_id: String,

    /// Correlation ID for distributed tracing
    pub correlation_id: String,
}

impl CallId {
    /// Create new call ID
    pub fn new() -> Self {
        let id = Uuid::new_v4();
        Self {
            id,
            sip_call_id: format!("{}@voip", id),
            correlation_id: id.to_string(),
        }
    }

    /// Create from SIP Call-ID
    pub fn from_sip(sip_call_id: String) -> Self {
        let id = Uuid::new_v4();
        Self {
            id,
            sip_call_id,
            correlation_id: id.to_string(),
        }
    }
}

impl Default for CallId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for CallId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.id)
    }
}

/// User identifier
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct UserId(pub Uuid);

impl UserId {
    /// Create new user ID
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    /// Parse from string
    pub fn parse(s: &str) -> Result<Self, uuid::Error> {
        Ok(Self(Uuid::parse_str(s)?))
    }
}

impl Default for UserId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for UserId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Device identifier
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct DeviceId(pub Uuid);

impl DeviceId {
    /// Create new device ID
    pub fn new() -> Self {
        Self(Uuid::new_v4())
    }

    /// Parse from string
    pub fn parse(s: &str) -> Result<Self, uuid::Error> {
        Ok(Self(Uuid::parse_str(s)?))
    }
}

impl Default for DeviceId {
    fn default() -> Self {
        Self::new()
    }
}

impl fmt::Display for DeviceId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// Pagination parameters
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageRequest {
    pub page: u32,
    pub page_size: u32,
    pub sort_by: Option<String>,
    pub descending: bool,
}

impl Default for PageRequest {
    fn default() -> Self {
        Self {
            page: 1,
            page_size: 20,
            sort_by: None,
            descending: false,
        }
    }
}

impl PageRequest {
    /// Calculate offset for database queries
    pub fn offset(&self) -> u32 {
        (self.page.saturating_sub(1)) * self.page_size
    }

    /// Calculate limit for database queries
    pub fn limit(&self) -> u32 {
        self.page_size
    }
}

/// Pagination response metadata
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PageInfo {
    pub page: u32,
    pub page_size: u32,
    pub total_pages: u32,
    pub total_items: u64,
    pub has_next: bool,
    pub has_previous: bool,
}

impl PageInfo {
    /// Create page info from request and total count
    pub fn new(request: &PageRequest, total_items: u64) -> Self {
        let total_pages = ((total_items as f64) / (request.page_size as f64)).ceil() as u32;

        Self {
            page: request.page,
            page_size: request.page_size,
            total_pages,
            total_items,
            has_next: request.page < total_pages,
            has_previous: request.page > 1,
        }
    }
}

/// Health status
#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum HealthStatus {
    Healthy,
    Degraded,
    Unhealthy,
    Critical,
}

impl fmt::Display for HealthStatus {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Healthy => write!(f, "healthy"),
            Self::Degraded => write!(f, "degraded"),
            Self::Unhealthy => write!(f, "unhealthy"),
            Self::Critical => write!(f, "critical"),
        }
    }
}

/// Service health check result
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HealthCheck {
    pub status: HealthStatus,
    pub service: ServiceInfo,
    pub checks: Vec<ComponentCheck>,
    pub timestamp: DateTime<Utc>,
}

/// Component health check
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ComponentCheck {
    pub name: String,
    pub status: HealthStatus,
    pub message: Option<String>,
    pub latency_ms: Option<f64>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_call_id() {
        let call_id = CallId::new();
        assert!(!call_id.sip_call_id.is_empty());
        assert!(!call_id.correlation_id.is_empty());
    }

    #[test]
    fn test_page_request() {
        let page = PageRequest {
            page: 3,
            page_size: 20,
            ..Default::default()
        };

        assert_eq!(page.offset(), 40);
        assert_eq!(page.limit(), 20);
    }

    #[test]
    fn test_page_info() {
        let request = PageRequest {
            page: 2,
            page_size: 10,
            ..Default::default()
        };

        let info = PageInfo::new(&request, 25);
        assert_eq!(info.total_pages, 3);
        assert!(info.has_next);
        assert!(info.has_previous);
    }

    #[test]
    fn test_health_status() {
        assert_eq!(HealthStatus::Healthy.to_string(), "healthy");
        assert_eq!(HealthStatus::Critical.to_string(), "critical");
    }
}