use std::net::SocketAddr;
use axum::middleware as axum_mw;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

// High-Performance Mimalloc Global Memory Allocator
#[global_allocator]
static GLOBAL: mimalloc::MiMalloc = mimalloc::MiMalloc;

pub mod config;
pub mod controllers;
pub mod database;
pub mod errors;
pub mod middleware;
pub mod models;
pub mod routes;
pub mod security;
pub mod services;

#[tokio::main]
async fn main() {
    tracing_subscriber::registry()
        .with(
            tracing_subscriber::EnvFilter::try_from_default_env()
                .unwrap_or_else(|_| "backend=info,tower_http=info".into()),
        )
        .with(tracing_subscriber::fmt::layer())
        .init();

    println!("===========================================================");
    println!("PT. MATRISPAN EXIM PHOENIX CORP (PT. MEPC)");
    println!("High-Performance Clean Axum Backend Engine v2.0 (Mimalloc)");
    println!("===========================================================");

    let database_url = std::env::var("DATABASE_URL")
        .unwrap_or_else(|_| config::DEFAULT_DATABASE_URL.to_string());

    println!("Connecting to NeonDB PostgreSQL Cluster...");
    let pool = database::create_neon_pool(Some(&database_url))
        .await
        .expect("Failed to connect to NeonDB PostgreSQL");

    println!("Verifying NeonDB Relational Schema (DDL)...");
    database::init_database_schema(&pool)
        .await
        .expect("Failed to initialize NeonDB database schema");

    println!("Synchronizing authentic corporate data seeds...");
    database::seed_initial_real_data(&pool)
        .await
        .expect("Failed to seed initial data");

    println!("✅ NeonDB Cloud PostgreSQL Cluster Online & Synchronized.");

    let app_state = services::AppState::new(pool);

    let app = routes::create_router(app_state.clone())
        .layer(axum_mw::from_fn_with_state(
            app_state.clone(),
            middleware::track_metrics,
        ))
        .layer(middleware::create_compression_layer())
        .layer(middleware::create_cors_layer());

    let server_config = config::ServerConfig::default();
    let addr = SocketAddr::from(([0, 0, 0, 0], server_config.port));

    println!("Server running on http://127.0.0.1:{}", server_config.port);
    println!("Dashboard & Telemetry:  http://127.0.0.1:{}/dashboard", server_config.port);
    println!("REST API Documentation: http://127.0.0.1:{}/api/metrics", server_config.port);

    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    axum::serve(listener, app).await.unwrap();
}
