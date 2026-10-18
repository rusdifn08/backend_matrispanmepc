use std::sync::Arc;
use tokio::sync::RwLock;
use sqlx::PgPool;
use super::auth_service::BruteForceProtector;
use super::metrics::ApiMetrics;

#[derive(Clone)]
pub struct AppState {
    pub pool: PgPool,
    pub protector: BruteForceProtector,
    pub metrics: Arc<RwLock<ApiMetrics>>,
}

impl AppState {
    pub fn new(pool: PgPool) -> Self {
        Self {
            pool,
            protector: BruteForceProtector::new(),
            metrics: Arc::new(RwLock::new(ApiMetrics::new())),
        }
    }
}
