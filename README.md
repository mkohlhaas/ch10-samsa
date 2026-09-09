# Samsa

## 1. Sealing

The `Sealed` marker trait lives in a private module. Because the module is
private, external crates cannot name it, so they can never implement
`MessageCodec`.

```text
                +---------------------------------------------+
                |               private module                |
                |                                             |
                |     +-------------------------------+       |
                |     |   pub trait Sealed {}         |       |
                |     |                               |       |
                |     |   implementable only inside   |       |
                |     |   this crate                  |       |
                |     +-------------------------------+       |
                +---------------------------------------------+
                                      |
                                      |  required (sealed) supertrait
                                      V
```

## 2. Sealed Trait: MessageCodec

`MessageCodec` is public, but its supertrait is the sealed marker, so only the
two codecs shown below can implement it. The trait members cover serialization,
deserialization, format identification, and validation.

```text
            +------------------------------------------------------+
            |                 pub trait MessageCodec               |
            |                 : private::Sealed                    |
            |                                                      |
            |   fn serialize(message)                              |
            |   fn deserialize(bytes)                              |
            |   fn format_id()                                     |
            |   fn validate(message)                               |
            +------------------------------------------------------+
                     ^                                     ^
                     |  implementable only                 |  implementable only
                     |  inside this crate                  |  inside this crate
                     |                                     |
     +------------------------------+      +------------------------------+
     |                              |      |                              |
     |          JsonCodec           |      |          TextCodec           |
     |                              |      |                              |
     +------------------------------+      +------------------------------+
```

## 3. Typed Message

`TypedMessage<S>` wraps the actual payload and delegates every operation to the
schema `S`. The `PhantomData<S>` field carries the schema only in the type
system; it costs nothing at runtime.

```text
              +--------------------------------------------------+
              |                                                  |
              |          TypedMessage<S: MessageCodec>           |
              |                                                  |
              |        calls function in the MessageCodec        |
              +--------------------------------------------------+
```

## 4. Type Aliases

```text
   JsonMessage = TypedMessage<JsonCodec>
   TextMessage = TypedMessage<TextCodec>
```

## 5. Message Handler

`MessageHandler<S>` is schema-generic. It processes typed messages and can
also take raw bytes, which it routes through deserialization, validation, and
construction before handling.

```text
              +--------------------------------------------------+
              |                                                  |
              |          MessageHandler<S: MessageCodec>         |
              |                                                  |
              |             processes typed messages             |
              +--------------------------------------------------+
```

## 6. Type Aliases

```text
   JsonMessageHandler = MessageHandler<JsonCodec>
   TextMessageHandler = MessageHandler<TextCodec>
```
