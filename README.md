# Samsa

```shell
cargo run --example type-system-patterns
```

## 1. Parse, don't validate

"Parse, don't validate" means to enforce validity at construction time. Thereby
we ensure that these types can only ever hold valid values. No need that
functions receiving values of these types have to revalidate again, just use
the values!

This moves all validation to the boundary of our system and eliminates
defensive programming in business logic.

Instead of checking invariants at every use site, parse once at the boundary
into a type where the invariant holds by construction—then the rest of the code
takes that type and needs no checks.

It doesn't mean necessarily to use parsing to apply this principle. In Rust you
can use the builder pattern which is a very good fit.

## 2. Sealing

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

### A. Sealed Trait: MessageCodec

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

### B. Typed Message

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

### C. Type Aliases

```text
   JsonMessage = TypedMessage<JsonCodec>
   TextMessage = TypedMessage<TextCodec>
```

### D. Message Handler

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

### E. Type Aliases

```text
   JsonMessageHandler = MessageHandler<JsonCodec>
   TextMessageHandler = MessageHandler<TextCodec>
```
