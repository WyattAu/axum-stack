# axum-stack

[![docs.rs](https://docs.rs/axum-stack/badge.svg)](https://docs.rs/axum-stack)
[![crates.io](https://img.shields.io/crates/v/axum-stack.svg)](https://crates.io/crates/axum-stack)
[![License](https://img.shields.io/badge/license-MIT%2FApache--2.0-blue.svg)](LICENSE)

Axum server utilities — shutdown, CORS, request ID, health routes, middleware stack.

## Features

- **Shutdown** — Graceful shutdown on SIGINT/SIGTERM via `graceful-shutdown`
- **CORS** — Permissive, restrictive, and API-only presets
- **Request ID** — UUID-based `X-Request-Id` generation and propagation
- **Health** — Liveness, readiness, and detailed health routes via `healthkit`
- **Middleware** — Pre-built stack (CORS, compression, timeout, request ID)

## Usage

```rust
use axum::Router;
use axum_stack::server;

let app = Router::new()
    .route("/api/hello", get(|| async { "world" }));

server::serve(app, "0.0.0.0:3000").await.unwrap();
```

## License

MIT OR Apache-2.0
