//! Example demonstrating type system patterns in Samsa
//!
//! This example shows:
//! - NewType pattern for type safety
//! - Parse Don't Validate for input validation
//! - TypeState pattern for consumer lifecycle
//! - Sealed traits for message schemas

use samsa::{
    sealed::{JsonMessage, TextMessage},
    *,
};
use std::sync::Arc;
use std::{
    io::{self, Write},
    result::Result,
};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("=== Samsa Type System Patterns Demo ===\n");
    loop {
        println!("=================================");
        println!("Which demo would you like to run?");
        println!("=================================\n");
        println!("  1. NewType Pattern");
        println!("  2. Parse Don't Validate");
        println!("  3. TypeState Pattern");
        println!("  4. Sealed Traits");
        println!("  5. Quit");
        print!("> ");
        io::stdout().flush()?;

        let mut input = String::new();
        io::stdin().read_line(&mut input)?;
        let choice = input.trim();

        match choice {
            "1" => {
                clear_screen()?;
                demonstrate_newtype_pattern()?;
            }
            "2" => {
                clear_screen()?;
                demonstrate_parse_dont_validate()?;
            }
            "3" => {
                clear_screen()?;
                demonstrate_typestate_pattern()?;
            }
            "4" => {
                clear_screen()?;
                demonstrate_sealed_traits()?;
            }
            "5" => {
                println!("Goodbye!");
                break;
            }
            "" => {}
            _ => println!("Invalid choice, please pick a number 1-5.\n"),
        }
    }

    Ok(())
}

fn clear_screen() -> Result<(), Box<dyn std::error::Error>> {
    print!("\x1b[2J\x1b[H");
    io::stdout().flush()?;
    Ok(())
}

fn demonstrate_newtype_pattern() -> Result<(), Box<dyn std::error::Error>> {
    println!("==================================");
    println!("1. NewType Pattern for Type Safety");
    println!("==================================\n");

    // Valid topic creation
    let topic = TopicId::new("user.events")?;
    println!("✓ Created valid topic: {}", topic);

    let consumer_id = ConsumerId::new("consumer-1")?;
    println!("✓ Created valid consumer ID: {}", consumer_id);

    let message_id = MessageId::new(12345);
    println!("✓ Created message ID: {}", message_id);

    // Invalid inputs would be caught at construction time
    match TopicId::new("") {
        Err(e) => println!("✗ Empty topic rejected: {}", e),
        Ok(_) => println!("✗ Should have failed!"),
    }

    match TopicId::new("user events") {
        // Space not allowed
        Err(e) => println!("✗ Invalid characters rejected: {}", e),
        Ok(_) => println!("✗ Should have failed!"),
    }

    println!();
    Ok(())
}

fn demonstrate_parse_dont_validate() -> Result<(), Box<dyn std::error::Error>> {
    println!("===============================");
    println!("2. Parse Don't Validate Pattern");
    println!("===============================\n");

    // "Parse, don't validate" is a Rust design principle advocating that types should guarantee
    // their own validity by construction, rather than allowing invalid states and checking them
    // separately.

    {
        // The type itself is only constructable from valid data - no Result needed.
        // Parsing IS the construction.
        let topic_id: TopicId = "user.events".parse()?; // if it fails, no TopicId exists
        let consumer_id: ConsumerId = "user-processor".parse()?;

        println!("✓ Topic '{}' is guaranteed valid", topic_id);
        println!("✓ Consumer '{}' is guaranteed valid", consumer_id);
    }

    {
        // Once constructed, we know these are valid
        let topic_id = TopicId::new("orders.created")?;
        let consumer_id = ConsumerId::new("order-processor")?;

        println!("✓ Topic '{}' is guaranteed valid", topic_id);
        println!("✓ Consumer '{}' is guaranteed valid", consumer_id);
    }
    // No need to re-validate when using these values
    // The type system ensures they're correct
    println!("✓ Can safely use values without re-validation");

    // Demonstrate validation at boundaries
    let user_inputs = vec![
        "valid.topic".to_string(),   // valid
        "".to_string(),              // Invalid: empty
        "a".repeat(200),             // Invalid: too long
        "invalid topic".to_string(), // Invalid: space
    ];

    println!("\nValidating user inputs:");
    for input in user_inputs {
        match TopicId::new(&input) {
            Ok(topic) => println!("  '{}' -> ✓ Valid: {}", input, topic),
            Err(e) => println!(
                "  '{}' -> ✗ Invalid: {}",
                if input.len() > 20 {
                    &input[..20]
                } else {
                    &input
                },
                e
            ),
        }
    }

    println!();
    Ok(())
}

fn demonstrate_typestate_pattern() -> Result<(), Box<dyn std::error::Error>> {
    println!("===========================================");
    println!("3. TypeState Pattern for Consumer Lifecycle");
    println!("===========================================\n");

    let broker = Arc::new(Broker::new());
    let consumer_id = ConsumerId::new("lifecycle-demo")?;
    let topic_id = TopicId::new("demo.messages")?;

    // Start with a disconnected consumer
    println!("Creating disconnected consumer...");
    let consumer = DisconnectedConsumer::new(consumer_id, broker);

    // These operations don't compile on a disconnected consumer:
    // consumer.receive(); // Only available for subscribed consumers
    // consumer.pause();   // Only available for subscribed consumers

    // Connect to broker
    println!("Connecting to broker...");
    let connection_info = ConnectionInfo::new("localhost:9092".to_string(), None);
    let consumer = consumer.connect(connection_info)?;

    // Now we can subscribe (but still can't receive messages)
    println!("Subscribing to topic...");
    let consumer = consumer.subscribe(topic_id)?;

    // Now we can receive messages
    println!("Consumer can now receive messages!");
    if let Some(event) = consumer.receive() {
        println!("Received event: {:#?}", event);
    } else {
        println!("No messages available");
    }

    // Pause the consumer
    println!("Pausing consumer...");
    let consumer = consumer.pause();

    // Resume the consumer
    println!("Resuming consumer...");
    let consumer = consumer.resume();

    // Clean shutdown
    println!("Unsubscribing and disconnecting...");
    let consumer = consumer.unsubscribe();
    let _consumer = consumer.disconnect();

    println!("✓ Consumer lifecycle completed successfully");
    println!("✓ Invalid state transitions prevented at compile time");
    println!();

    Ok(())
}

fn demonstrate_sealed_traits() -> Result<(), Box<dyn std::error::Error>> {
    println!("====================================");
    println!("4. Sealed Traits for Message Schemas");
    println!("====================================\n");

    // ---------------- //
    // Raw JSON Content //
    // ---------------- //

    let json_content = serde_json::json!({
        "user_id": 12345,
        "event_type": "login",
        "timestamp": 1640995200,
        "metadata": {
            "ip_address": "192.168.1.100",
            "user_agent": "Mozilla/5.0"
        }
    });

    // -------------------//
    // Typed JSON Message //
    // -------------------//

    println!("---------------------");
    println!("1. Typed JSON Message");
    println!("---------------------\n");

    let message_id = MessageId::new(1001);
    // let json_message  = TypedMessage::<JsonSchema>::new(message_id, json_content)
    let json_message = JsonMessage::new(message_id, json_content)
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

    println!(
        "✓ Created typed JSON message with schema: {}",
        json_message.schema_id()
    );
    println!("  Message ID: {}", json_message.id);
    println!("  Content: {:#?}", json_message.content);

    // ------------------------- //
    // Handle Typed JSON Message //
    // ------------------------- //

    println!("\n----------------------------");
    println!("2. Handle Typed JSON Message");
    println!("----------------------------\n");

    let json_handler = JsonMessageHandler::new();
    json_handler
        .handle(&json_message)
        .map_err(|e| format!("Handler error: {}", e))?;

    // ------------------ //
    // Typed Text Message //
    // ------------------ //

    println!("\n---------------------");
    println!("3. Typed Text Message");
    println!("---------------------\n");

    let text_content = "System notification: High memory usage detected".to_string();
    println!("Original text content: {}", text_content);

    let text_message_id = MessageId::new(2001);
    // let text_message = TypedMessage::<TextSchema>::new(text_message_id, text_content)
    let text_message = TextMessage::new(text_message_id, text_content)
        .map_err(|e| Box::new(e) as Box<dyn std::error::Error>)?;

    println!(
        "✓ Created text message with schema: {}",
        text_message.schema_id()
    );

    // ------------------------- //
    // Handle Typed Text Message //
    // ------------------------- //

    println!("\n----------------------------");
    println!("4. Handle Typed Text Message");
    println!("----------------------------\n");

    // Create a handler for text messages
    let text_handler = TextMessageHandler::new();
    text_handler
        .handle(&text_message)
        .map_err(|e| format!("Handler error: {}", e))?;

    // --------------//
    // Serialization //
    // --------------//

    println!("\n--------------------------------");
    println!("5. Serialization of JSON Message");
    println!("--------------------------------\n");

    let bytes = json_message.to_bytes();
    println!("Serialized to {} bytes.", bytes.len());

    // --------------- //
    // Deserialization //
    // --------------- //

    println!("\n----------------------------------");
    println!("6. Deserialization of JSON Message");
    println!("----------------------------------\n");

    let json_handler = JsonMessageHandler::new();
    match json_handler.handle_bytes(MessageId::new(1002), &bytes) {
        Ok(()) => println!("✓ Deserialization successful"),
        Err(e) => println!("✗ Deserialization failed: {}", e),
    }

    println!("\n✓ Sealed trait pattern ensures type safety");
    println!("✓ Only predefined schemas can be used");
    println!("✓ Compile-time guarantees about message structure");

    println!();
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn newtype_prevents_mixing_types() {
        let topic = TopicId::new("test").unwrap();
        let consumer_id = ConsumerId::new("test").unwrap();

        // Even though both contain the same string,
        // they are different types and cannot be mixed

        // This would not compile:
        // let mixed_up: TopicId = consumer_id;

        assert_eq!(topic.as_str(), "test");
        assert_eq!(consumer_id.as_str(), "test");
        // assert_ne!(
        //     std::mem::discriminant(&topic),
        //     std::mem::discriminant(&consumer_id)
        // );
    }

    #[test]
    fn typestate_prevents_invalid_operations() {
        let broker = Arc::new(Broker::new());
        let consumer_id = ConsumerId::new("test").unwrap();

        let consumer = DisconnectedConsumer::new(consumer_id, broker);

        // These would not compile:
        // consumer.receive(); // Only for subscribed
        // consumer.pause();   // Only for subscribed
        // consumer.resume();  // Only for paused

        // This is the only valid operation for disconnected state:
        let connection_info = ConnectionInfo::new("localhost:9092".to_string(), None);
        let _connected = consumer.connect(connection_info).unwrap();
    }
}
