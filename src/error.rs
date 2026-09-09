//! Unified error handling for Samsa
//!
//! This module consolidates all error types into a single SamsaError enum,
//! providing better ergonomics for library users and demonstrating the
//! thiserror crate for idiomatic Rust error handling.

use thiserror::Error;

/// Convenience type alias for Results using SamsaError
pub type Result<T> = std::result::Result<T, SamsaError>;

/// The main error type for all Samsa operations
///
/// This replaces multiple separate error enums with a single unified type,
/// making it easy for users to handle any Samsa error.
#[derive(Error, Debug, Clone)]
pub enum SamsaError {
    /// Configuration-related errors
    #[error("Configuration error: {0}")]
    Config(String),

    /// Topic validation errors
    #[error("Topic validation failed: {0}")]
    Topic(String),

    /// Consumer-related errors
    #[error("Consumer error: {0}")]
    Consumer(String),

    /// Consumer group errors
    #[error("Consumer group error: {0}")]
    Group(String),

    /// Resource management errors (pools, guards, etc.)
    #[error("Resource error: {0}")]
    Resource(String),

    /// Service lifecycle errors
    #[error("Service error: {0}")]
    Service(String),

    /// Schema and message validation errors
    #[error("Schema validation failed: {0}")]
    Schema(String),

    /// Message routing and delivery errors
    #[error("Message routing failed: {0}")]
    Routing(String),

    /// Generic broker errors
    #[error("Broker error: {0}")]
    Broker(String),

    /// Connection-related errors
    #[error("Connection failed: {0}")]
    Connection(String),

    // ---- Codec errors (from the sealed MessageCodec trait) ----
    /// The message format does not match the codec
    #[error("Invalid message format")]
    InvalidFormat,

    /// The codec version is not known to this codec
    #[error("Unknown codec version")]
    UnknownVersion,

    /// The message data is damaged or truncated
    #[error("Corrupted message data")]
    CorruptedData,

    /// Deserialization failed with the underlying error message
    #[error("Deserialization failed: {0}")]
    DeserializationFailed(String),

    // ---- Validation errors (from the sealed MessageCodec trait) ----
    /// A required field is missing
    #[error("Required field missing: {0}")]
    FieldRequired(String),

    /// A field exceeds the maximum allowed length
    #[error("Field '{0}' too long: {1} characters")]
    FieldTooLong(String, usize),

    /// A field has an invalid value
    #[error("Invalid value for field: {0}")]
    InvalidValue(String),

    /// A field is outside the allowed numeric range
    #[error("Field '{0}' out of range [{1}, {2}]")]
    InvalidRange(String, i64, i64),
}

// Convenience constructors for common error patterns
impl SamsaError {
    pub fn config(msg: impl Into<String>) -> Self {
        SamsaError::Config(msg.into())
    }

    pub fn topic(msg: impl Into<String>) -> Self {
        SamsaError::Topic(msg.into())
    }

    pub fn consumer(msg: impl Into<String>) -> Self {
        SamsaError::Consumer(msg.into())
    }

    pub fn broker(msg: impl Into<String>) -> Self {
        SamsaError::Broker(msg.into())
    }

    pub fn resource(msg: impl Into<String>) -> Self {
        SamsaError::Resource(msg.into())
    }

    pub fn service(msg: impl Into<String>) -> Self {
        SamsaError::Service(msg.into())
    }

    pub fn schema(msg: impl Into<String>) -> Self {
        SamsaError::Schema(msg.into())
    }

    pub fn connection(msg: impl Into<String>) -> Self {
        SamsaError::Connection(msg.into())
    }
}
