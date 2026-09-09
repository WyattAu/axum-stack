//! CORS layer presets for Axum.

use tower_http::cors::{AllowHeaders, Any, CorsLayer};

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
/// Use only in development. Never in production.
pub fn cors_permissive() -> CorsLayer {
    CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any)
        .max_age(std::time::Duration::from_secs(3600))
}

/// Create a restrictive CORS layer with explicit origins.
///
/// # Arguments
/// * `origins` — Allowed origins (e.g., `["https://app.example.com"]`)
/// * `credentials` — Whether to allow credentials (cookies, auth headers)
// Justified: invalid origin strings are a caller programming error; this
// builder is infallible by contract, so panicking with a clear message is
// preferable to silently dropping origins.
#[allow(clippy::expect_used)]
pub fn cors_restrictive(origins: &[&str], credentials: bool) -> CorsLayer {
    let mut layer = CorsLayer::new();

    if origins.is_empty() {
        layer = layer.allow_origin(Any);
    } else if origins.len() == 1 {
        let origin: http::HeaderValue = origins[0].parse().expect("valid origin");
        layer = layer.allow_origin(origin);
    } else {
        let origins: Vec<http::HeaderValue> = origins
            .iter()
            .map(|o| o.parse().expect("valid origin"))
            .collect();
        layer = layer.allow_origin(origins);
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

    layer
}

/// Create a CORS layer for API-only access (no credentials, specific origins).
pub fn cors_api(origins: &[&str]) -> CorsLayer {
    cors_restrictive(origins, false)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn permissive_allows_any() {
        let layer = cors_permissive();
        let _ = layer;
    }

    #[test]
    fn restrictive_single_origin() {
        let layer = cors_restrictive(&["https://app.example.com"], true);
        let _ = layer;
    }

    #[test]
    fn api_no_credentials() {
        let layer = cors_api(&["https://app.example.com"]);
        let _ = layer;
    }
}
