//! Sealed trait pattern for message codecs
//!
//! This module demonstrates the sealed trait pattern to create
//! type-safe message handling with controlled extensibility.

use crate::types::MessageId;
use std::{
    error::Error,
    fmt::{Debug, Display},
    marker::PhantomData,
};

// ========================================== //
// 1. Private Module to support sealed traits //
// ========================================== //

/// Private module that contains the sealing trait
///
/// This prevents external crates from implementing MessageCodec
mod private {
    pub trait Sealed {}
}

// =============== //
// 2. Sealed Trait //
// =============== //

// ------------- //
// Message Codec //
// ------------- //

/// A sealed trait for message codecs
///
/// Only types within this crate can implement MessageCodec,
/// ensuring API stability and type safety.
pub trait MessageCodec: private::Sealed {
    /// The Rust type that represents this message
    type Message: Clone + Debug;

    /// Serialize a message to bytes
    fn serialize(message: &Self::Message) -> Vec<u8>;

    /// Deserialize bytes to a message
    fn deserialize(bytes: &[u8]) -> Result<Self::Message, CodecError>;

    /// Get the format identifier
    fn format_id() -> &'static str;

    /// Validate a message according to codec rules
    fn validate(message: &Self::Message) -> Result<(), ValidationError>;
}

// ================ //
// 3. Typed Message //
// ================ //

// A message struct with MessageCodec as a trait bound.

/// A type-safe message container
///
/// Messages are parameterized by their codec, ensuring
/// compile-time guarantees about message structure.
/// see examples
#[derive(Debug, Clone)]
pub struct TypedMessage<S: MessageCodec> {
    pub id: MessageId,
    pub content: S::Message,
    pub codec_type: PhantomData<S>,
}

// calls function in the MessageCodec
impl<S: MessageCodec> TypedMessage<S> {
    /// Create a new typed message
    pub fn new(id: MessageId, content: S::Message) -> Result<Self, ValidationError> {
        // calling function from the trait
        S::validate(&content)?;

        Ok(TypedMessage {
            id,
            content,
            codec_type: PhantomData,
        })
    }

    /// Serialize the message to bytes
    pub fn to_bytes(&self) -> Vec<u8> {
        // calling function from the trait
        S::serialize(&self.content)
    }

    /// Get the codec identifier
    pub fn format_id(&self) -> &'static str {
        // calling function from the trait
        S::format_id()
    }
}

// ====== //
// Errors //
// ====== //

// Maybe errors should be put in file error.rs.

// ------------- //
// A. CodecError //
// ------------- //

/// Errors that can occur during codec operations (deserialization function)
/// Could be called DeserializationError.
#[derive(Debug, Clone)]
pub enum CodecError {
    InvalidFormat,
    UnknownVersion,
    CorruptedData,
    DeserializationFailed(String), // actually the only one really used (in deserialization functions)
}

impl Display for CodecError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            CodecError::InvalidFormat => write!(f, "Invalid message format"),
            CodecError::UnknownVersion => write!(f, "Unknown codec version"),
            CodecError::CorruptedData => write!(f, "Corrupted message data"),
            CodecError::DeserializationFailed(msg) => write!(f, "Deserialization failed: {}", msg),
        }
    }
}

impl Error for CodecError {}

// ------------------ //
// B. ValidationError //
// ------------------ //

/// Errors that can occur during message validation (validation function)
#[derive(Debug, Clone)]
pub enum ValidationError {
    FieldRequired(String),
    FieldTooLong(String, usize),
    InvalidValue(String),
    InvalidRange(String, i64, i64), // only one not really used
}

impl Display for ValidationError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ValidationError::FieldRequired(field) => write!(f, "Required field missing: {}", field),
            ValidationError::FieldTooLong(field, len) => {
                write!(f, "Field '{}' too long: {} characters", field, len)
            }
            ValidationError::InvalidValue(field) => write!(f, "Invalid value for field: {}", field),
            ValidationError::InvalidRange(field, min, max) => {
                write!(f, "Field '{}' out of range [{}, {}]", field, min, max)
            }
        }
    }
}

impl Error for ValidationError {}

// ============== //
// Message Codecs //
// ============== //

// ------------ //
// A. JsonCodec //
// ------------ //

/// JSON Codec using serde_json for flexible message structures
pub struct JsonCodec;

impl private::Sealed for JsonCodec {}

impl MessageCodec for JsonCodec {
    type Message = serde_json::Value;

    fn serialize(message: &Self::Message) -> Vec<u8> {
        serde_json::to_vec(message).unwrap_or_default()
    }

    fn deserialize(bytes: &[u8]) -> Result<Self::Message, CodecError> {
        serde_json::from_slice(bytes).map_err(|e| CodecError::DeserializationFailed(e.to_string()))
    }

    fn format_id() -> &'static str {
        "json_v1"
    }

    fn validate(message: &Self::Message) -> Result<(), ValidationError> {
        // JSON is valid if it can be represented as serde_json::Value
        if message.is_null() {
            return Err(ValidationError::InvalidValue(
                "Message cannot be null".to_string(),
            ));
        }
        Ok(())
    }
}

pub type JsonMessage = TypedMessage<JsonCodec>;

// ------------ //
// B. TextCodec //
// ------------ //

/// Text codec for simple string messages
pub struct TextCodec;

impl private::Sealed for TextCodec {}

impl MessageCodec for TextCodec {
    type Message = String;

    fn serialize(message: &Self::Message) -> Vec<u8> {
        message.as_bytes().to_vec()
    }

    fn deserialize(bytes: &[u8]) -> Result<Self::Message, CodecError> {
        String::from_utf8(bytes.to_vec())
            .map_err(|e| CodecError::DeserializationFailed(e.to_string()))
    }

    fn format_id() -> &'static str {
        "text_v1"
    }

    fn validate(message: &Self::Message) -> Result<(), ValidationError> {
        if message.is_empty() {
            return Err(ValidationError::FieldRequired(
                "message content".to_string(),
            ));
        }

        if message.len() > 10_000 {
            return Err(ValidationError::FieldTooLong(
                "message".to_string(),
                message.len(),
            ));
        }

        Ok(())
    }
}

pub type TextMessage = TypedMessage<TextCodec>;

// =============== //
// Message Handler //
// =============== //

/// Type-safe message handler that works with any valid codec
pub struct MessageHandler<S: MessageCodec> {
    codec_type: PhantomData<S>,
}

impl<S: MessageCodec> Default for MessageHandler<S> {
    fn default() -> Self {
        Self::new()
    }
}

impl<S: MessageCodec> MessageHandler<S> {
    /// Create a new message handler for a specific codec
    pub fn new() -> Self {
        Self {
            codec_type: PhantomData,
        }
    }

    /// Process a typed message
    pub fn handle(&self, message: &TypedMessage<S>) -> Result<(), Box<dyn Error>> {
        println!("Handling message with codec: {}", message.format_id());
        println!("Message content: {:#?}", message.content);
        Ok(())
    }

    /// Deserialize and handle raw bytes
    pub fn handle_bytes(&self, id: MessageId, bytes: &[u8]) -> Result<(), Box<dyn Error>> {
        let content = S::deserialize(bytes)?;
        let message = TypedMessage::new(id, content)?;
        self.handle(&message)
    }
}

pub type JsonMessageHandler = MessageHandler<JsonCodec>;
pub type TextMessageHandler = MessageHandler<TextCodec>;

// ===== //
// Tests //
// ===== //

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;

    #[test]
    fn json_codec_validation() {
        let valid_json = json!({
            "user_id": 123,
            "event_type": "login",
            "timestamp": 1234567890
        });

        assert!(JsonCodec::validate(&valid_json).is_ok());

        let null_json = serde_json::Value::Null;
        assert!(JsonCodec::validate(&null_json).is_err());
    }

    #[test]
    fn json_codec_serialization() {
        let message = json!({
            "action": "publish",
            "topic": "events",
            "data": {"key": "value"}
        });

        let bytes = JsonCodec::serialize(&message);
        assert!(!bytes.is_empty());

        let deserialized = JsonCodec::deserialize(&bytes).unwrap();
        assert_eq!(deserialized["action"], "publish");
        assert_eq!(deserialized["topic"], "events");
    }

    #[test]
    fn text_codec_validation() {
        let valid_text = "Hello, Samsa!".to_string();
        assert!(TextCodec::validate(&valid_text).is_ok());

        let empty_text = String::new();
        assert!(TextCodec::validate(&empty_text).is_err());

        let too_long = "x".repeat(10_001);
        assert!(TextCodec::validate(&too_long).is_err());
    }

    #[test]
    fn text_codec_serialization() {
        let message = "Test message".to_string();

        let bytes = TextCodec::serialize(&message);
        assert!(!bytes.is_empty());

        let deserialized = TextCodec::deserialize(&bytes).unwrap();
        assert_eq!(deserialized, "Test message");
    }

    #[test]
    fn typed_message_json() {
        let content = json!({
            "user_id": 456,
            "event_type": "logout"
        });

        let message_id = MessageId::new(1);
        let typed_message = TypedMessage::<JsonCodec>::new(message_id, content).unwrap();

        assert_eq!(typed_message.format_id(), "json_v1");
    }

    #[test]
    fn typed_message_text() {
        let content = "Hello from Samsa".to_string();

        let message_id = MessageId::new(2);
        let typed_message = TypedMessage::<TextCodec>::new(message_id, content).unwrap();

        assert_eq!(typed_message.format_id(), "text_v1");
    }

    #[test]
    fn message_handler_json() {
        let handler = MessageHandler::<JsonCodec>::new();

        let content = json!({
            "event": "purchase",
            "amount": 99.99
        });

        let message_id = MessageId::new(3);
        let message = TypedMessage::<JsonCodec>::new(message_id, content).unwrap();

        assert!(handler.handle(&message).is_ok());
    }

    #[test]
    fn message_handler_text() {
        let handler = MessageHandler::<TextCodec>::new();

        let content = "System notification".to_string();

        let message_id = MessageId::new(4);
        let message = TypedMessage::<TextCodec>::new(message_id, content).unwrap();

        assert!(handler.handle(&message).is_ok());
    }
}
