# axum-server

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
use axum_server::server;

let app = Router::new()
    .route("/api/hello", get(|| async { "world" }));

server::serve(app, "0.0.0.0:3000").await.unwrap();
```

## License

MIT OR Apache-2.0
