//! Samsa - A simple publish/subscribe microservice (Chapter 10)
//!
//! This crate builds on Chapter 9 by adding type system patterns:
//! - NewType pattern for type-safe domain concepts
//! - Parse Don't Validate for input validation
//! - TypeState pattern for lifecycle management
//! - Sealed traits for controlled extensibility

// Core error handling - unified across all modules
pub use broker::Broker;
pub use consumer::Consumer;
pub use error::{Result, SamsaError};
pub use message::{Event, Message};
pub use producer::Producer;
// Chapter 10 additions - Type System Patterns
pub use sealed::{JsonSchema, MessageHandler, MessageSchema, TextSchema, TypedMessage};
pub use types::{ConsumerId, MessageId, TopicId};
pub use typestate_consumer::{
    ConnectedConsumer, ConnectionInfo, Consumer as TypedConsumer, DisconnectedConsumer,
    PausedConsumer, SubscribedConsumer,
};

mod broker;
mod consumer;
mod error;
mod message;
mod producer;
mod storage;

// New modules for Chapter 10
pub mod sealed;
pub mod types;
pub mod typestate_consumer;
