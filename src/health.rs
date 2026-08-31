//! Health check routes for Axum, extending `healthkit`.

use axum::routing::get;
use axum::Router;
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
    let (status, _results) = registry.check_readiness().await.unwrap_or_else(|_| {
        (
            healthkit::HealthStatus::Unhealthy,
            Vec::new(),
        )
    });

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
