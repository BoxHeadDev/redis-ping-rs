# Redis Ping-Pong in Rust

This repository demonstrates how to build simple client-server applications that communicate using the [Redis protocol](https://redis.io/docs/reference/protocol-spec/), implemented in Rust with a focus on raw I/O and serialization.

It contains two versions of the same system:
1. **Basic I/O Version** – Using `std::io` to directly read/write bytes.
2. **Serde Version** – Using serialization via `serde` to encode/decode messages.

## Exercises Implemented

### 1. `basic-io-client` & `basic-io-server`
Implements a basic client and server that communicate using raw `std::io` streams (TCP). The client sends `PING` messages in Redis protocol format, and the server responds with `PONG`.

- **Client** sends: `*1\r\n$4\r\nPING\r\n`
- **Server** replies: `+PONG\r\n`

This version is a low-level implementation dealing directly with bytes and protocol framing.

### 2. `serde-io-client` & `serde-io-server`
Builds on the basic version by introducing Rust types to model Redis commands and responses. Uses `serde` to serialize and deserialize these messages over TCP.

- Clearer code structure using types like `Command::Ping` and `Response::Pong`
- Easier to extend for more commands or future features

## Getting Started

### Prerequisites

- Rust (latest stable version recommended)

### Running the Basic Version

In two terminals:

```bash
# Terminal 1
cd basic-io-server
cargo run

# Terminal 2
cd basic-io-client
cargo run
```

### Running the Serde Version

In two terminals:

```bash
# Terminal 1
cd serde-io-server
cargo run

# Terminal 2
cd serde-io-client
cargo run
```

## Project Structure

```bash
.
├── basic-io-client/      # Raw IO Redis client
├── basic-io-server/      # Raw IO Redis server
├── serde-io-client/      # Serde-based Redis client
├── serde-io-server/      # Serde-based Redis server
└── README.md             # You're here
```

## Goals

- Understand binary protocols like Redis at the byte level
- Practice TCP networking in Rust
- Learn how to model and serialize network protocols using serde
- Build maintainable code with a clear separation of concerns
