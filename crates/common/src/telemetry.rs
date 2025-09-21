//! Telemetry implementation with OpenTelemetry

use std::collections::HashMap;
use std::sync::Arc;
use tracing::info;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, EnvFilter, Registry};

use crate::errors::Result;
use crate::types::ServiceConfig;

/// Initialize telemetry for a service
pub fn init_telemetry(service_name: &str, config: &ServiceConfig) -> Result<()> {
    // Temporarily simplified telemetry setup - just basic logging without OpenTelemetry
    // TODO: Re-enable OpenTelemetry when API is updated

    // Set up basic tracing subscriber without OpenTelemetry
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(&config.log_level));

    let fmt_layer = tracing_subscriber::fmt::layer()
        .with_target(false)
        .with_timer(tracing_subscriber::fmt::time::UtcTime::rfc_3339())
        .with_thread_ids(true)
        .json();

    Registry::default()
        .with(env_filter)
        .with(fmt_layer)
        .init();

    info!(
        service = service_name,
        otlp_endpoint = config.otlp_endpoint.as_deref().unwrap_or("none"),
        "Telemetry initialized"
    );

    Ok(())
}


/// Metrics collector
#[derive(Clone)]
pub struct Metrics {
    registry: Arc<prometheus::Registry>,
}

impl Metrics {
    /// Create new metrics collector
    pub fn new() -> Self {
        Self {
            registry: Arc::new(prometheus::Registry::new()),
        }
    }

    /// Register a counter
    pub fn register_counter(&self, name: &str, help: &str) -> prometheus::Counter {
        let counter = prometheus::Counter::new(name, help)
            .expect("Failed to create counter");

        self.registry
            .register(Box::new(counter.clone()))
            .expect("Failed to register counter");

        counter
    }

    /// Register a gauge
    pub fn register_gauge(&self, name: &str, help: &str) -> prometheus::Gauge {
        let gauge = prometheus::Gauge::new(name, help)
            .expect("Failed to create gauge");

        self.registry
            .register(Box::new(gauge.clone()))
            .expect("Failed to register gauge");

        gauge
    }

    /// Register a histogram
    pub fn register_histogram(&self, name: &str, help: &str) -> prometheus::Histogram {
        let histogram = prometheus::Histogram::with_opts(
            prometheus::HistogramOpts::new(name, help),
        )
        .expect("Failed to create histogram");

        self.registry
            .register(Box::new(histogram.clone()))
            .expect("Failed to register histogram");

        histogram
    }

    /// Gather all metrics
    pub fn gather(&self) -> Vec<prometheus::proto::MetricFamily> {
        self.registry.gather()
    }

    /// Export metrics in Prometheus format
    pub fn export(&self) -> String {
        use prometheus::Encoder;
        let encoder = prometheus::TextEncoder::new();
        let metric_families = self.gather();
        let mut buffer = Vec::new();
        encoder
            .encode(&metric_families, &mut buffer)
            .expect("Failed to encode metrics");
        String::from_utf8(buffer).expect("Invalid UTF-8")
    }
}

impl Default for Metrics {
    fn default() -> Self {
        Self::new()
    }
}

/// Trace context for distributed tracing
#[derive(Clone)]
pub struct TraceContext {
    trace_id: String,
    span_id: String,
    trace_flags: u8,
}

impl TraceContext {
    /// Create from current span
    pub fn current() -> Self {
        // Temporarily return a placeholder since OpenTelemetry is disabled
        Self {
            trace_id: "00000000000000000000000000000000".to_string(),
            span_id: "0000000000000000".to_string(),
            trace_flags: 0,
        }
    }

    /// Create from headers
    pub fn from_headers(headers: &HashMap<String, String>) -> Option<Self> {
        let trace_id = headers.get("x-trace-id")?;
        let span_id = headers.get("x-span-id")?;
        let trace_flags = headers
            .get("x-trace-flags")
            .and_then(|s| s.parse().ok())
            .unwrap_or(0);

        Some(Self {
            trace_id: trace_id.clone(),
            span_id: span_id.clone(),
            trace_flags,
        })
    }

    /// Convert to headers
    pub fn to_headers(&self) -> HashMap<String, String> {
        let mut headers = HashMap::new();
        headers.insert("x-trace-id".to_string(), self.trace_id.clone());
        headers.insert("x-span-id".to_string(), self.span_id.clone());
        headers.insert("x-trace-flags".to_string(), self.trace_flags.to_string());
        headers
    }

    /// Get trace ID
    pub fn trace_id(&self) -> &str {
        &self.trace_id
    }

    /// Get span ID
    pub fn span_id(&self) -> &str {
        &self.span_id
    }
}

/// Common metrics for services
pub struct ServiceMetrics {
    pub requests_total: prometheus::Counter,
    pub requests_duration: prometheus::Histogram,
    pub requests_in_flight: prometheus::Gauge,
    pub errors_total: prometheus::Counter,
}

impl ServiceMetrics {
    /// Create standard service metrics
    pub fn new(service_name: &str, metrics: &Metrics) -> Self {
        let prefix = format!("{}_", service_name.replace('-', "_"));

        Self {
            requests_total: metrics.register_counter(
                &format!("{}requests_total", prefix),
                "Total number of requests",
            ),
            requests_duration: metrics.register_histogram(
                &format!("{}request_duration_seconds", prefix),
                "Request duration in seconds",
            ),
            requests_in_flight: metrics.register_gauge(
                &format!("{}requests_in_flight", prefix),
                "Number of requests currently being processed",
            ),
            errors_total: metrics.register_counter(
                &format!("{}errors_total", prefix),
                "Total number of errors",
            ),
        }
    }

    /// Record a request
    pub fn record_request(&self, duration_secs: f64, success: bool) {
        self.requests_total.inc();
        self.requests_duration.observe(duration_secs);

        if !success {
            self.errors_total.inc();
        }
    }

    /// Start tracking a request
    pub fn start_request(&self) {
        self.requests_in_flight.inc();
    }

    /// End tracking a request
    pub fn end_request(&self) {
        self.requests_in_flight.dec();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_trace_context() {
        let mut headers = HashMap::new();
        headers.insert("x-trace-id".to_string(), "12345678901234567890123456789012".to_string());
        headers.insert("x-span-id".to_string(), "1234567890123456".to_string());
        headers.insert("x-trace-flags".to_string(), "1".to_string());

        let context = TraceContext::from_headers(&headers).unwrap();
        assert_eq!(context.trace_id(), "12345678901234567890123456789012");
        assert_eq!(context.span_id(), "1234567890123456");

        let headers2 = context.to_headers();
        assert_eq!(headers2.get("x-trace-id").unwrap(), "12345678901234567890123456789012");
    }

    #[test]
    fn test_metrics() {
        let metrics = Metrics::new();
        let counter = metrics.register_counter("test_counter", "Test counter");
        counter.inc();
        assert_eq!(counter.get(), 1.0);
    }
}