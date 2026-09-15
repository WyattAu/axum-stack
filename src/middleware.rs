//! Common Axum middleware stack.

use std::time::Duration;
use tower::ServiceBuilder;
use tower_http::compression::CompressionLayer;
use tower_http::cors::CorsLayer;
use tower_http::timeout::TimeoutLayer;

use crate::request_id;

/// Build a secure-by-default middleware stack with:
/// - Request ID generation
/// - Response compression (gzip)
/// - Request timeout (30 seconds)
///
/// **CORS is not included** — cross-origin access must be opted into
/// explicitly via [`middleware_stack_with_cors`] (or
/// [`crate::cors::cors_permissive`] for development only). There is no
/// allow-all default to forget about.
pub fn default_middleware_stack() -> impl tower::Layer<axum::routing::Route> {
    let (set_id, propagate_id) = request_id::request_id_layer();

    ServiceBuilder::new()
        .layer(propagate_id)
        .layer(set_id)
        .layer(CompressionLayer::new())
        .layer(TimeoutLayer::with_status_code(
            http::StatusCode::REQUEST_TIMEOUT,
            Duration::from_secs(30),
        ))
}

/// Like [`default_middleware_stack`], with an explicit CORS layer layered
/// in. Build it with [`crate::cors::cors_restrictive`] /
/// [`crate::cors::cors_api`] (production) or
/// [`crate::cors::cors_permissive`] (development only).
pub fn middleware_stack_with_cors(cors: CorsLayer) -> impl tower::Layer<axum::routing::Route> {
    let (set_id, propagate_id) = request_id::request_id_layer();

    ServiceBuilder::new()
        .layer(propagate_id)
        .layer(set_id)
        .layer(cors)
        .layer(CompressionLayer::new())
        .layer(TimeoutLayer::with_status_code(
            http::StatusCode::REQUEST_TIMEOUT,
            Duration::from_secs(30),
        ))
}
