use axum::{extract::State, response::Json};
use crate::services::{AppState, MetricsSnapshot};

pub async fn get_api_metrics(State(state): State<AppState>) -> Json<MetricsSnapshot> {
    let metrics = state.metrics.read().await;
    Json(metrics.snapshot())
}
