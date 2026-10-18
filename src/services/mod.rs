pub mod auth_service;
pub mod db;
pub mod invoice_service;
pub mod metrics;
pub mod whatsapp_service;

pub use auth_service::*;
pub use db::AppState;
pub use invoice_service::InvoiceService;
pub use metrics::{ApiMetrics, EndpointStat, MetricsSnapshot};
pub use whatsapp_service::{WhatsAppService, MEPC_ADMIN_WA};

// Re-export database utilities for convenience
pub use crate::database::{create_neon_pool, init_database_schema, seed_initial_real_data};
pub use crate::config::DEFAULT_DATABASE_URL;
