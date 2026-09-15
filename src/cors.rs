//! CORS layer presets for Axum.

use tower_http::cors::{AllowHeaders, Any, CorsLayer};

/// Errors from CORS layer construction.
///
/// Construction is fallible because origins come from configuration; a
/// malformed origin must surface as a typed startup error, not a panic.
#[derive(Debug, thiserror::Error)]
pub enum CorsError {
    /// An origin string is not a valid `scheme://host[:port]` value.
    #[error("invalid CORS origin: {origin:?}")]
    InvalidOrigin {
        /// The rejected origin string.
        origin: String,
    },
    /// `cors_restrictive` was called with no origins. Silently allowing
    /// every origin from a "restrictive" builder is a footgun; callers
    /// must opt into `cors_permissive` explicitly.
    #[error("no origins given; use cors_permissive() explicitly for allow-all")]
    NoOrigins,
}

/// Headers allowed on credentialed requests. The CORS spec (and tower-http's
/// validator) forbids `Access-Control-Allow-Headers: *` combined with
/// `Access-Control-Allow-Credentials: true`, so credentialed layers must
/// enumerate headers explicitly.
fn credentialed_headers() -> AllowHeaders {
    [
        http::header::AUTHORIZATION,
        http::header::CONTENT_TYPE,
        http::header::ACCEPT,
        http::header::ORIGIN,
        http::header::HeaderName::from_static("x-requested-with"),
        http::header::HeaderName::from_static("x-csrf-token"),
    ]
    .into()
}

/// Create a permissive CORS layer (allows all origins, methods, headers).
///
/// Use only in development. Never in production. The default middleware
/// stack does **not** include this layer; enabling CORS is always an
/// explicit choice.
pub fn cors_permissive() -> CorsLayer {
    CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any)
        .max_age(std::time::Duration::from_secs(3600))
}

/// Response headers exposed to browser JS on cross-origin requests.
fn exposed_headers() -> [http::HeaderName; 1] {
    [http::HeaderName::from_static("x-request-id")]
}

/// Validate one origin string and convert it to a header value.
///
/// Accepts `scheme://host[:port]` (e.g. `https://app.example.com`).
fn parse_origin(origin: &str) -> Result<http::HeaderValue, CorsError> {
    let looks_like_origin = {
        let mut parts = origin.split("://");
        let scheme = parts.next().unwrap_or_default();
        let authority = parts.next().unwrap_or_default();
        let no_more = parts.next().is_none();
        let scheme_ok = (scheme == "http" || scheme == "https") && !scheme.is_empty();
        let authority_ok = !authority.is_empty()
            && !authority.contains('/')
            && !authority.contains('\\')
            && !authority.contains(' ');
        scheme_ok && authority_ok && no_more
    };
    if !looks_like_origin {
        return Err(CorsError::InvalidOrigin {
            origin: origin.to_string(),
        });
    }
    origin.parse().map_err(|_| CorsError::InvalidOrigin {
        origin: origin.to_string(),
    })
}

/// Create a restrictive CORS layer with explicit origins.
///
/// # Arguments
/// * `origins` — Allowed origins (e.g. `["https://app.example.com"]`).
///   Must be non-empty; an empty slice is
///   [`CorsError::NoOrigins`] rather than a silent allow-all.
/// * `credentials` — Whether to allow credentials (cookies, auth headers).
///
/// # Errors
///
/// Returns [`CorsError::InvalidOrigin`] for any origin that is not a
/// well-formed `scheme://host[:port]` string, and [`CorsError::NoOrigins`]
/// for an empty origin list.
pub fn cors_restrictive(origins: &[&str], credentials: bool) -> Result<CorsLayer, CorsError> {
    if origins.is_empty() {
        return Err(CorsError::NoOrigins);
    }

    let mut layer = CorsLayer::new();

    if origins.len() == 1 {
        let origin = parse_origin(origins[0])?;
        layer = layer.allow_origin(origin);
    } else {
        let mut parsed: Vec<http::HeaderValue> = Vec::with_capacity(origins.len());
        for o in origins {
            parsed.push(parse_origin(o)?);
        }
        layer = layer.allow_origin(parsed);
    }

    layer = layer
        .allow_methods([
            http::Method::GET,
            http::Method::POST,
            http::Method::PUT,
            http::Method::DELETE,
            http::Method::PATCH,
            http::Method::OPTIONS,
        ])
        .allow_headers(if credentials {
            // `*` + credentials is an illegal CORS combination — enumerate.
            credentialed_headers()
        } else {
            Any.into()
        })
        .max_age(std::time::Duration::from_secs(3600));

    if credentials {
        layer = layer.allow_credentials(true);
    }

    Ok(layer.expose_headers(exposed_headers()))
}

/// Create a CORS layer for API-only access (no credentials, specific origins).
///
/// # Errors
///
/// See [`cors_restrictive`].
pub fn cors_api(origins: &[&str]) -> Result<CorsLayer, CorsError> {
    cors_restrictive(origins, false)
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]
    use super::*;

    #[test]
    fn permissive_allows_any() {
        let layer = cors_permissive();
        let _ = layer;
    }

    #[test]
    fn restrictive_single_origin() {
        let _ = cors_restrictive(&["https://app.example.com"], true).expect("valid origin");
    }

    #[test]
    fn restrictive_multiple_origins() {
        let _ = cors_restrictive(&["https://a.example.com", "https://b.example.com"], false)
            .expect("valid origins");
    }

    #[test]
    fn empty_origins_are_rejected_not_allow_all() {
        let err = cors_restrictive(&[], true).expect_err("empty slice");
        assert!(matches!(err, CorsError::NoOrigins));
    }

    #[test]
    fn invalid_origins_are_rejected() {
        for bad in [
            "app.example.com",   // no scheme
            "https://",          // no host
            "ftp://host",        // wrong scheme
            "https://host/path", // path is not part of an origin
            "",                  // empty
        ] {
            let err = cors_restrictive(&[bad], false).expect_err(bad);
            assert!(matches!(err, CorsError::InvalidOrigin { .. }), "{bad}");
        }
    }

    #[test]
    fn api_no_credentials() {
        let _ = cors_api(&["https://api.example.com"]).expect("valid origin");
    }

    #[test]
    fn origin_with_port_is_accepted() {
        let _ = cors_restrictive(&["http://localhost:3000"], false).expect("localhost with port");
    }
}
