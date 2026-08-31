//! Common Axum middleware stack.

use std::time::Duration;
use tower::ServiceBuilder;
use tower_http::compression::CompressionLayer;
use tower_http::timeout::TimeoutLayer;

use crate::cors;
use crate::request_id;

/// Build a default middleware stack with:
/// - Request ID generation
/// - CORS (permissive, override with your own origins)
/// - Response compression (gzip)
/// - Request timeout (30 seconds)
pub fn default_middleware_stack() -> impl tower::Layer<axum::routing::Route> {
    let (set_id, propagate_id) = request_id::request_id_layer();

    ServiceBuilder::new()
        .layer(propagate_id)
        .layer(set_id)
        .layer(cors::cors_permissive())
        .layer(CompressionLayer::new())
        .layer(TimeoutLayer::with_status_code(
            http::StatusCode::REQUEST_TIMEOUT,
            Duration::from_secs(30),
        ))
}
