# Requirements — axum-stack

Numbered, testable requirements. Every requirement maps to at least one named
test or doc-comment contract; security-relevant items cite THREAT-MODEL.md rows.

Scope: Opinionated Axum stack — CORS, request IDs, health endpoints, server bootstrap in one layer

## Functional

| ID | Requirement | Priority |
|----|-------------|----------|
| REQ-XS-001 | `cors` layers emit exactly the configured origins; empty config means permissive-by-default documented behavior | MUST |
| REQ-XS-002 | Every response carries a unique request id (generated or propagated) via the request-id layer | MUST |
| REQ-XS-003 | Health endpoints (`/health` liveness, readiness) respond 200 with JSON status without touching app state | MUST |

## Security

| ID | Requirement | Priority |
|----|-------------|----------|
| REQ-XS-100 | CORS origins are parsed strictly; invalid origin strings abort at construction, never silently ignored | MUST |
| REQ-XS-101 | Request ids are bounded-length and sanitized (header injection impossible via typed header types) | MUST |

## Observability & API hygiene

| ID | Requirement | Priority |
|----|-------------|----------|
| REQ-XS-900 | All fallible public APIs return typed errors; production `unwrap`/`expect` is denied or explicitly justified with an invariant comment | MUST |
| REQ-XS-901 | Public items carry doc comments with runnable examples where practical | SHOULD |

Reviewed: 2026-09-11
