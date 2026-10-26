# Changelog

All notable changes are documented here. Format: [Keep a
Changelog](https://keepachangelog.com/): versions follow [semver](https://semver.org).

## [1.0.0] - 2026-10-26

### Declared stable

First 1.0. `axum-stack` wires production axum services; this release commits the shutdown, CORS, request-ID, and health-route surfaces as API-stable.

### Guarantees from here on

- Breaking changes to public items require a minor bump plus a `#[deprecated]`
  window of at least one minor release.
- MSRV bumps are minor releases, not patch releases.
- `#![deny(missing_docs)]` and `#![forbid(unsafe_code)]` are enforced.

### Explicitly not guaranteed

- New middleware may arrive in minors behind features; existing middleware behaviour will not change.

## [0.2.0] - 2026-09-15

### Changed (breaking: 0.x minor bump)

- **`cors_restrictive` / `cors_api` are fallible** (`Result<CorsLayer,
  CorsError>`): malformed origins return `CorsError::InvalidOrigin`
  instead of panicking on caller input.
- **An empty origin list is now `CorsError::NoOrigins`**: the old
  silently-allow-all behavior on empty slices was a secure-by-default
  violation in a function named "restrictive".
- **`default_middleware_stack` no longer includes a permissive CORS
  layer.** Cross-origin access is opt-in via
  `middleware_stack_with_cors(layer)`; there is no allow-all default to
  forget about.

### Added

- `CorsError` typed error (`InvalidOrigin`, `NoOrigins`).
- `middleware_stack_with_cors(CorsLayer)` for explicit CORS layering.

# Changelog

All notable changes to this project are documented here. Format: [Keep a
Changelog](https://keepachangelog.com/): versions follow [semver](https://semver.org).

## [Unreleased]

## [0.1.2] - 2026-09-15

### Added

- `cors_restrictive` now exposes `x-request-id` to browser JavaScript via
  the CORS `expose-headers` list, so front ends can correlate requests
  with server logs.

### Fixed

- Dropped unused dependencies (`serde`, `tower-layer`, `uuid`) from the
  dependency graph.
- Pinned `healthkit` / `shutdown-kit` dependencies to registry versions
  so the crate publishes cleanly from crates.io metadata.
- Lint hygiene: justified `cors` expects, allowed `unwrap` in unit tests.

### Docs

- Added `THREAT-MODEL.md` (STRIDE analysis) and repository metadata
  normalization.

## [0.1.1] - 2026-09-11

### Fixed

- 22-gate quality audit pass: documentation completeness
  (README badges, REQUIREMENTS/THREAT-MODEL coverage) and
  feature-gated test hygiene.

## [0.1.0] - 2026-09-05

### Added
- Initial public release.
