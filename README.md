# Space Traders API Client

A Rust implementation of the Space Traders API client.

## Prerequisites

- Rust (latest stable version)
- Cargo (comes with Rust)

## Setup

1. Clone the repository
2. Copy `.env.example` to `.env` and add your Space Traders API token
3. Build the project:
   ```bash
   cargo build
   ```

## Running

To run the project:

```bash
cargo run
```

For debug logs, set the `RUST_LOG` environment variable:

```bash
RUST_LOG=debug cargo run
```

## Features

- Environment variable configuration
- Async runtime with Tokio
- Error handling with anyhow
- Logging support
- HTTP client with reqwest
- JSON serialization/deserialization with serde

## Development

To run tests:

```bash
cargo test
```
