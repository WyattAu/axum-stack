# Threat Model — axum-stack

Reference: STRIDE. Scope: the crate's public API surface (`cors::*`,
`request_id_layer`, `health_routes`, `default_middleware_stack`, `serve`)
as used by a downstream Axum service. Trust boundaries: (1) HTTP requests
arriving at the mounted routes ( CORS `Origin` headers, health probes),
(2) configuration strings entering the CORS builders, (3) the dependency
tree (tower-http, axum, shutdown-kit).

## Assets

| ID | Asset | Example |
|----|-------|---------|
| A1 | Browser-enforced origin policy of the consuming service | A permissive preset leaks credentialed responses cross-origin |
| A2 | Correlation of requests to audit trails | Untraceable requests after middleware drops IDs |
| A3 | Confidentiality of dependency health detail | `/health` internals exposed to anonymous scanners |

## STRIDE Analysis

| # | Threat | Category | Surface | Mitigation | Verifying test |
|---|--------|----------|---------|------------|----------------|
| T1 | Cross-origin abuse via the permissive preset | Elevation | `cors_permissive()` | Documented as deliberately permissive (all origins/methods/headers, no credentials); restrictive presets exist: `cors_api` (single origin, specific methods), `cors_restrictive` (allow-list) | `permissive_allows_any`, `restrictive_single_origin`, `api_no_credentials`, `cors_api` |
| T2 | `Allow-Origin: *` combined with credentials | Elevation | `cors_*` builders | The credentialed variant (`cors_*_with_credentials` path) pins the echo to explicit origins rather than `Any`; tower-http refuses `Any` + `allow_credentials(true)` combos | `credentialed_headers`, `api_no_credentials` |
| T3 | Client-forged request IDs poisoning audit trails | Spoofing | `request_id_layer` | `SetRequestIdLayer<MakeRequestUuid>` **overwrites** any incoming `x-request-id` with a server-generated UUID v4; the propagate layer then copies the server-set value to the response | `request_id_layer_builds`, `request_id_layer_builds_multiple_times`, `request_id_layer_builds_with_any_params` |
| T4 | Health-endpoint information disclosure | Info disclosure | `health_routes`, `/health` detailed | Liveness (`/live`) and readiness (`/ready`) return static status only; the **detailed** route returns per-dependency health JSON and is mounted at the caller's discretion — **no auth is applied by this crate**. Documented residual risk: mount `/health` behind authN/authZ | `health_routes`, `detailed`, `liveness`, `readiness` |
| T5 | Shutdown hangs (graceful drain bypassed) | DoS | `shutdown_signal` (shutdown-kit) | Delegated to `shutdown_kit::shutdown_signal`; this crate only re-exports it | Covered in shutdown-kit's own suite; not re-tested here — documented |
| T6 | Malformed CORS config strings panic at startup | DoS | `cors_api`/`cors_restrictive` inputs | Builders parse origins at startup; invalid input fails fast at server construction (documented programming-error contract), never per-request | `restrictive_single_origin` (valid config path); invalid-input behavior follows `HeaderValue::from_*` fallibility |

## Repudiation

Partially supported: `request_id_layer` gives every request a
server-controlled ID propagated to responses, enabling downstream log
correlation (anti-repudiation for the *service*). The crate itself logs
nothing; durable audit trails are the caller's responsibility.

## Out of Scope

- Authentication, authorization, rate limiting, and body-size limits:
  `default_middleware_stack` composes request-ID + CORS + health only;
  security middleware must be added by the service.
- TLS termination and HSTS: transport is assumed terminated upstream.
- `shutdown-kit` internals (separate crate; re-export only).

## Residual Risks

- **R1 (Medium, accepted):** `cors_permissive()` is one typo away from
  production. It exists for local/dev convenience; the mitigate-by-default
  path is `cors_api`/`cors_restrictive`. Reviewed as a footgun, kept for
  ergonomics.
- **R2 (Medium, accepted):** The detailed health route exposes dependency
  status (names + healthy flags) unauthenticated (T4). Exposing internals
  to an unauthenticated scanner aids recon. Callers must gate it.
- **R3 (Low, accepted):** Request IDs are UUID v4 — globally unguessable but
  not cryptographic attestations; anything downstream that *trusts* an ID as
  proof of origin is misusing it.
- **R4 (Low, accepted):** No in-repo `cargo audit` gate for tower-http/axum
  advisories; relies on org-level Dependabot.
