//! Common Axum middleware stack.

use std::time::Duration;
use tower::ServiceBuilder;
use tower::util::BoxCloneSyncServiceLayer;
use tower_http::compression::CompressionLayer;
use tower_http::cors::CorsLayer;
use tower_http::timeout::TimeoutLayer;

use crate::request_id;

/// A cloneable, boxed middleware layer that can be applied with
/// [`axum::Router::layer`].
pub type MiddlewareStack = BoxCloneSyncServiceLayer<
    axum::routing::Route,
    http::Request<axum::body::Body>,
    axum::response::Response,
    std::convert::Infallible,
>;

/// Build a secure-by-default middleware stack with:
/// - Request ID generation
/// - Response compression (gzip)
/// - Request timeout (30 seconds)
///
/// **CORS is not included** — cross-origin access must be opted into
/// explicitly via [`middleware_stack_with_cors`] (or
/// [`crate::cors::cors_permissive`] for development only). There is no
/// allow-all default to forget about.
pub fn default_middleware_stack() -> MiddlewareStack {
    let (set_id, propagate_id) = request_id::request_id_layer();

    BoxCloneSyncServiceLayer::new(
        ServiceBuilder::new()
            .layer(set_id)
            .layer(propagate_id)
            .map_response(
                |res: http::Response<
                    tower_http::compression::CompressionBody<axum::body::Body>,
                >| { res.map(axum::body::Body::new) },
            )
            .layer(CompressionLayer::new())
            .layer(TimeoutLayer::with_status_code(
                http::StatusCode::REQUEST_TIMEOUT,
                Duration::from_secs(30),
            )),
    )
}

/// Like [`default_middleware_stack`], with an explicit CORS layer layered
/// in. Build it with [`crate::cors::cors_restrictive`] /
/// [`crate::cors::cors_api`] (production) or
/// [`crate::cors::cors_permissive`] (development only).
pub fn middleware_stack_with_cors(cors: CorsLayer) -> MiddlewareStack {
    let (set_id, propagate_id) = request_id::request_id_layer();

    BoxCloneSyncServiceLayer::new(
        ServiceBuilder::new()
            .layer(set_id)
            .layer(propagate_id)
            .layer(cors)
            .map_response(
                |res: http::Response<
                    tower_http::compression::CompressionBody<axum::body::Body>,
                >| { res.map(axum::body::Body::new) },
            )
            .layer(CompressionLayer::new())
            .layer(TimeoutLayer::with_status_code(
                http::StatusCode::REQUEST_TIMEOUT,
                Duration::from_secs(30),
            )),
    )
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]
    use super::*;
    use axum::Router;
    use axum::body::Body;
    use axum::routing::get;
    use http::Request;
    use tower::ServiceExt;

    #[tokio::test]
    async fn default_stack_sets_request_id_on_response() {
        let app = Router::new()
            .route("/", get(|| async { "ok" }))
            .layer(default_middleware_stack());

        let res = app
            .oneshot(Request::get("/").body(Body::empty()).unwrap())
            .await
            .unwrap();

        assert_eq!(res.status(), http::StatusCode::OK);
        assert!(res.headers().contains_key("x-request-id"));
    }

    #[tokio::test]
    async fn cors_stack_applies_cors_and_request_id() {
        let app = Router::new()
            .route("/", get(|| async { "ok" }))
            .layer(middleware_stack_with_cors(crate::cors::cors_permissive()));

        let req = Request::get("/")
            .header("origin", "https://example.com")
            .body(Body::empty())
            .unwrap();
        let res = app.oneshot(req).await.unwrap();

        assert_eq!(res.status(), http::StatusCode::OK);
        assert_eq!(
            res.headers().get("access-control-allow-origin").unwrap(),
            "*"
        );
        assert!(res.headers().contains_key("x-request-id"));
    }
}
