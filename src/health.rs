//! Health check routes for Axum, extending `healthkit`.

use axum::Router;
use axum::routing::get;
use healthkit::HealthRegistry;

/// Create health check routes.
///
/// Returns:
/// - `GET /health/live` — liveness probe (always 200)
/// - `GET /health/ready` — readiness probe (200 if healthy, 503 if not)
/// - `GET /health` — detailed health check
pub fn health_routes(registry: HealthRegistry) -> Router {
    Router::new()
        .route("/health/live", get(liveness))
        .route(
            "/health/ready",
            get({
                let registry = registry.clone();
                move || readiness(registry)
            }),
        )
        .route(
            "/health",
            get({
                let registry = registry.clone();
                move || detailed(registry)
            }),
        )
}

async fn liveness() -> axum::Json<serde_json::Value> {
    axum::Json(serde_json::json!({ "status": "alive" }))
}

async fn readiness(
    registry: HealthRegistry,
) -> Result<axum::Json<serde_json::Value>, axum::http::StatusCode> {
    let (status, _results) = registry
        .check_readiness()
        .await
        .unwrap_or_else(|_| (healthkit::HealthStatus::Unhealthy, Vec::new()));

    if status.is_healthy() || status.is_ready() {
        Ok(axum::Json(serde_json::json!({ "status": "ready" })))
    } else {
        Err(axum::http::StatusCode::SERVICE_UNAVAILABLE)
    }
}

async fn detailed(registry: HealthRegistry) -> axum::Json<serde_json::Value> {
    let results = registry.check_all().await;
    let checks: Vec<serde_json::Value> = results
        .iter()
        .map(|r| {
            serde_json::json!({
                "name": r.name,
                "healthy": r.status.is_healthy(),
                "message": r.message,
            })
        })
        .collect();

    let all_healthy = results.iter().all(|r| r.status.is_healthy());
    axum::Json(serde_json::json!({
        "status": if all_healthy { "healthy" } else { "degraded" },
        "checks": checks,
    }))
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used)]
    use super::*;
    use axum::body::Body;
    use healthkit::{HealthCheckError, HealthStatus};
    use http::Request;
    use tower::ServiceExt;

    async fn get(router: Router, uri: &str) -> axum::response::Response {
        router
            .oneshot(Request::get(uri).body(Body::empty()).unwrap())
            .await
            .unwrap()
    }

    async fn body_json(res: axum::response::Response) -> serde_json::Value {
        let bytes = axum::body::to_bytes(res.into_body(), usize::MAX)
            .await
            .unwrap();
        serde_json::from_slice(&bytes).unwrap()
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn liveness_is_always_ok() {
        let res = get(health_routes(HealthRegistry::new()), "/health/live").await;
        assert_eq!(res.status(), http::StatusCode::OK);
        assert_eq!(body_json(res).await["status"], "alive");
    }

    /// Register checks in sync context (`add_check` uses a blocking write)
    /// and drive the HTTP calls with a dedicated runtime.
    fn test(
        registry: HealthRegistry,
        uri: &'static str,
        assert: impl FnOnce(&[u8], http::StatusCode),
    ) {
        let app = health_routes(registry);
        tokio::runtime::Builder::new_multi_thread()
            .enable_all()
            .build()
            .unwrap()
            .block_on(async move {
                let res = get(app, uri).await;
                let status = res.status();
                let bytes = axum::body::to_bytes(res.into_body(), usize::MAX)
                    .await
                    .unwrap();
                assert(&bytes, status);
            });
    }

    fn json(bytes: &[u8]) -> serde_json::Value {
        serde_json::from_slice(bytes).unwrap()
    }

    #[test]
    fn readiness_is_ok_when_checks_pass() {
        let registry = HealthRegistry::new();
        registry.add_check("db", || async { Ok(HealthStatus::Healthy) });

        test(registry, "/health/ready", |bytes, status| {
            assert_eq!(status, http::StatusCode::OK);
            assert_eq!(json(bytes)["status"], "ready");
        });
    }

    #[test]
    fn readiness_is_503_when_a_check_fails() {
        let registry = HealthRegistry::new();
        registry.add_check("db", || async {
            Err::<HealthStatus, _>(HealthCheckError::CheckFailed("connection refused".into()))
        });

        test(registry, "/health/ready", |_, status| {
            // axum renders `Err(StatusCode)` as a bare 503 with no body.
            assert_eq!(status, http::StatusCode::SERVICE_UNAVAILABLE);
        });
    }

    #[test]
    fn detailed_reports_each_check() {
        let registry = HealthRegistry::new();
        registry.add_check("ok-check", || async { Ok(HealthStatus::Healthy) });
        registry.add_check("bad-check", || async {
            Err::<HealthStatus, _>(HealthCheckError::CheckFailed("boom".into()))
        });

        test(registry, "/health", |bytes, status| {
            assert_eq!(status, http::StatusCode::OK);
            let body = json(bytes);
            assert_eq!(body["status"], "degraded");
            let checks = body["checks"].as_array().unwrap();
            assert_eq!(checks.len(), 2);
            for check in checks {
                match check["name"].as_str().unwrap() {
                    "ok-check" => assert_eq!(check["healthy"], true),
                    "bad-check" => {
                        assert_eq!(check["healthy"], false);
                        // healthkit leaves `message` unset for failed checks.
                        assert!(check["message"].is_null());
                    }
                    other => panic!("unexpected check: {other}"),
                }
            }
        });
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn detailed_is_healthy_with_no_checks() {
        let res = get(health_routes(HealthRegistry::new()), "/health").await;
        assert_eq!(res.status(), http::StatusCode::OK);
        assert_eq!(body_json(res).await["status"], "healthy");
    }
}
