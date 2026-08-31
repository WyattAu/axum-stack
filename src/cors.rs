//! CORS layer presets for Axum.

use tower_http::cors::{Any, CorsLayer};

/// Create a permissive CORS layer (allows all origins, methods, headers).
///
/// Use only in development. Never in production.
pub fn cors_permissive() -> CorsLayer {
    CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any)
}

/// Create a restrictive CORS layer with explicit origins.
///
/// # Arguments
/// * `origins` — Allowed origins (e.g., `["https://app.example.com"]`)
/// * `credentials` — Whether to allow credentials (cookies, auth headers)
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
        .allow_headers(Any);

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
