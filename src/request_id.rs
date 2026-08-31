//! Request ID middleware for Axum.

use tower_http::request_id::{MakeRequestUuid, PropagateRequestIdLayer, SetRequestIdLayer};

/// Create a request ID layer that generates UUIDs for incoming requests
/// and sets the `X-Request-Id` header on responses.
pub fn request_id_layer() -> (SetRequestIdLayer<MakeRequestUuid>, PropagateRequestIdLayer) {
    (
        SetRequestIdLayer::x_request_id(MakeRequestUuid),
        PropagateRequestIdLayer::x_request_id(),
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn request_id_layer_builds() {
        let (set, propagate) = request_id_layer();
        let _ = set;
        let _ = propagate;
    }
}
