//! Common error types for all VoIP services

use std::fmt;
use thiserror::Error;

/// Common result type
pub type Result<T> = std::result::Result<T, VoipError>;

/// Main error type for VoIP services
#[derive(Debug, Error)]
pub enum VoipError {
    /// Configuration error
    #[error("Configuration error: {0}")]
    Config(String),

    /// Service discovery error
    #[error("Service discovery error: {0}")]
    Discovery(String),

    /// Authentication error
    #[error("Authentication failed: {0}")]
    Auth(String),

    /// Authorization error
    #[error("Authorization failed: {0}")]
    Unauthorized(String),

    /// Validation error
    #[error("Validation error: {0}")]
    Validation(String),

    /// SIP protocol error
    #[error("SIP error: code={code}, reason={reason}")]
    Sip { code: u16, reason: String },

    /// Media processing error
    #[error("Media error: {0}")]
    Media(String),

    /// Database error
    #[error("Database error: {0}")]
    Database(#[from] sqlx::Error),

    /// Redis error
    #[error("Redis error: {0}")]
    Redis(#[from] redis::RedisError),

    /// gRPC error
    #[error("gRPC error: {0}")]
    Grpc(#[from] tonic::Status),

    /// Message queue error
    #[error("NATS error: {0}")]
    Nats(Box<dyn std::error::Error + Send + Sync>),

    /// HTTP error
    #[error("HTTP error: {0}")]
    Http(String),

    /// I/O error
    #[error("I/O error: {0}")]
    Io(#[from] std::io::Error),

    /// Timeout error
    #[error("Operation timed out: {0}")]
    Timeout(String),

    /// Rate limit exceeded
    #[error("Rate limit exceeded: {0}")]
    RateLimit(String),

    /// Resource not found
    #[error("Resource not found: {0}")]
    NotFound(String),

    /// Resource already exists
    #[error("Resource already exists: {0}")]
    AlreadyExists(String),

    /// Service unavailable
    #[error("Service unavailable: {0}")]
    Unavailable(String),

    /// Internal server error
    #[error("Internal error: {0}")]
    Internal(String),

    /// Other errors
    #[error(transparent)]
    Other(#[from] anyhow::Error),
}

impl VoipError {
    /// Convert to gRPC status
    pub fn to_status(&self) -> tonic::Status {
        match self {
            Self::Config(msg) | Self::Validation(msg) => {
                tonic::Status::invalid_argument(msg)
            }
            Self::Auth(msg) => tonic::Status::unauthenticated(msg),
            Self::Unauthorized(msg) => tonic::Status::permission_denied(msg),
            Self::NotFound(msg) => tonic::Status::not_found(msg),
            Self::AlreadyExists(msg) => tonic::Status::already_exists(msg),
            Self::RateLimit(msg) => tonic::Status::resource_exhausted(msg),
            Self::Timeout(msg) => tonic::Status::deadline_exceeded(msg),
            Self::Unavailable(msg) => tonic::Status::unavailable(msg),
            Self::Grpc(status) => status.clone(),
            _ => tonic::Status::internal(self.to_string()),
        }
    }

    /// Convert to HTTP status code
    pub fn to_http_status(&self) -> u16 {
        match self {
            Self::Config(_) | Self::Validation(_) => 400, // Bad Request
            Self::Auth(_) => 401,                         // Unauthorized
            Self::Unauthorized(_) => 403,                 // Forbidden
            Self::NotFound(_) => 404,                     // Not Found
            Self::AlreadyExists(_) => 409,                // Conflict
            Self::RateLimit(_) => 429,                    // Too Many Requests
            Self::Timeout(_) => 408,                      // Request Timeout
            Self::Unavailable(_) => 503,                  // Service Unavailable
            Self::Sip { code, .. } => *code,
            _ => 500, // Internal Server Error
        }
    }

    /// Get error code for structured logging
    pub fn error_code(&self) -> &'static str {
        match self {
            Self::Config(_) => "CONFIG_ERROR",
            Self::Discovery(_) => "DISCOVERY_ERROR",
            Self::Auth(_) => "AUTH_FAILED",
            Self::Unauthorized(_) => "UNAUTHORIZED",
            Self::Validation(_) => "VALIDATION_ERROR",
            Self::Sip { .. } => "SIP_ERROR",
            Self::Media(_) => "MEDIA_ERROR",
            Self::Database(_) => "DATABASE_ERROR",
            Self::Redis(_) => "REDIS_ERROR",
            Self::Grpc(_) => "GRPC_ERROR",
            Self::Nats(_) => "NATS_ERROR",
            Self::Http(_) => "HTTP_ERROR",
            Self::Io(_) => "IO_ERROR",
            Self::Timeout(_) => "TIMEOUT",
            Self::RateLimit(_) => "RATE_LIMIT_EXCEEDED",
            Self::NotFound(_) => "NOT_FOUND",
            Self::AlreadyExists(_) => "ALREADY_EXISTS",
            Self::Unavailable(_) => "SERVICE_UNAVAILABLE",
            Self::Internal(_) => "INTERNAL_ERROR",
            Self::Other(_) => "UNKNOWN_ERROR",
        }
    }
}

/// Extension trait for error context
pub trait ErrorContext<T> {
    fn context(self, msg: impl fmt::Display) -> Result<T>;
    fn with_context<F>(self, f: F) -> Result<T>
    where
        F: FnOnce() -> String;
}

impl<T, E> ErrorContext<T> for std::result::Result<T, E>
where
    E: Into<anyhow::Error>,
{
    fn context(self, msg: impl fmt::Display) -> Result<T> {
        self.map_err(|e| VoipError::Other(e.into().context(msg.to_string())))
    }

    fn with_context<F>(self, f: F) -> Result<T>
    where
        F: FnOnce() -> String,
    {
        self.map_err(|e| VoipError::Other(e.into().context(f())))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_error_codes() {
        assert_eq!(VoipError::Config("test".into()).error_code(), "CONFIG_ERROR");
        assert_eq!(VoipError::Auth("test".into()).error_code(), "AUTH_FAILED");
        assert_eq!(VoipError::NotFound("test".into()).error_code(), "NOT_FOUND");
    }

    #[test]
    fn test_http_status() {
        assert_eq!(VoipError::Validation("test".into()).to_http_status(), 400);
        assert_eq!(VoipError::Auth("test".into()).to_http_status(), 401);
        assert_eq!(VoipError::NotFound("test".into()).to_http_status(), 404);
        assert_eq!(VoipError::RateLimit("test".into()).to_http_status(), 429);
    }
}