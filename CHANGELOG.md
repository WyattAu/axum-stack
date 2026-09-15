# Changelog

All notable changes to this project are documented here. Format: [Keep a
Changelog](https://keepachangelog.com/) — versions follow [semver](https://semver.org).

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
