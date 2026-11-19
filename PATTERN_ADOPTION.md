# Pattern adoption: axum-stack

Relationship to the WyattAu pattern canon
(`pattern-standards/references/catalog.md`). Validated by
`pattern-standards/scripts/adoption-check.py`.

| Pattern | Status | Evidence |
|---|---|---|
| T33 | refused | R6 (crate ships server plumbing, no state-mutating endpoints of its own; the request-id layer carries no idempotency keys) |
| T45 | refused | R6 (no hypermedia surface: middleware, CORS, request-id, health routes only; no client-facing state machine) |
| P47 | refused | R6 (stacks are caller-constructed at startup via `default_middleware_stack`, src/middleware.rs:30; no config reload surface to pin) |
| P62 | adopted | src/server.rs:28 (`with_graceful_shutdown(shutdown_kit::shutdown_signal())`, SIGINT/SIGTERM drain before exit) |
| P63 | partial | src/request_id.rs:7 (X-Request-Id generated and propagated on every request; no W3C traceparent headers) |
| P72 | partial | src/health.rs:32 (health routes return one JSON status envelope; readiness failures return a bare 503 with no body) |
| P94 | partial | src/health.rs:15 (liveness/readiness probe endpoints for external supervisors; no in-process watchdog) |
| P103 | partial | src/middleware.rs:43 (30s per-request timeout via `TimeoutLayer`; no deadline propagation to downstream calls) |

Reviewed: 2026-11-19
