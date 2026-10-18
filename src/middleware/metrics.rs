use axum::{
    extract::{Request, State},
    middleware::Next,
    response::Response,
};
use std::time::Instant;
use crate::services::AppState;

pub async fn track_metrics(
    State(state): State<AppState>,
    req: Request,
    next: Next,
) -> Response {
    let start = Instant::now();
    let path = req.uri().path().to_string();
    let method = req.method().to_string();

    let response = next.run(req).await;

    let elapsed = start.elapsed().as_secs_f64() * 1000.0;
    let status = response.status().as_u16();

    // Skip recording the metrics dashboard itself to avoid infinite feedback loop
    if !path.starts_with("/dashboard") && !path.starts_with("/api/metrics") {
        let mut metrics = state.metrics.write().await;
        metrics.record_request(&method, &path, status, elapsed);
    }

    response
}
