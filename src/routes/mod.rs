use axum::{
    routing::{get, patch, post},
    Router,
};
use crate::{
    controllers::{
        create_message, create_order, create_supplier_request, get_api_metrics,
        get_current_user_handler, get_dashboard_stats, get_invoice, get_metrics_dashboard_html,
        get_order, get_product, get_security_audit_logs_handler, list_invoices, list_messages,
        list_orders, list_products, list_supplier_requests, login_handler, mark_message_read,
        register_handler, reply_message, run_security_test_handler, update_order_status,
        update_product, update_supplier_status,
    },
    services::AppState,
};

pub fn create_router(state: AppState) -> Router {
    Router::new()
        // Products Catalog
        .route("/api/products", get(list_products))
        .route("/api/products/:id", get(get_product).patch(update_product))
        
        // Buyer Orders
        .route("/api/orders", get(list_orders).post(create_order))
        .route("/api/orders/:id", get(get_order))
        .route("/api/orders/:id/status", patch(update_order_status))
        
        // Supplier Requests (Farmer Submissions)
        .route("/api/suppliers", get(list_supplier_requests).post(create_supplier_request))
        .route("/api/suppliers/:id/status", patch(update_supplier_status))
        
        // Proforma Export Invoices
        .route("/api/invoices", get(list_invoices))
        .route("/api/invoices/:param", get(get_invoice))
        
        // Inquiries & Messages
        .route("/api/messages", get(list_messages).post(create_message))
        .route("/api/messages/:id/read", patch(mark_message_read))
        .route("/api/messages/:id/reply", post(reply_message))
        
        // Executive Stats & KPIs
        .route("/api/stats", get(get_dashboard_stats))
        
        // Telemetry & Server Metrics
        .route("/api/metrics", get(get_api_metrics))
        .route("/dashboard", get(get_metrics_dashboard_html))
        
        // Enterprise Authentication & Security (RBAC + NIK)
        .route("/api/auth/login", post(login_handler))
        .route("/api/auth/register", post(register_handler))
        .route("/api/auth/me", get(get_current_user_handler))
        .route("/api/auth/audit-logs", get(get_security_audit_logs_handler))
        .route("/api/auth/security-test", get(run_security_test_handler))
        
        // Health Probe
        .route("/api/health", get(|| async { "PT. MEPC High-Performance Axum Backend: OK" }))
        .with_state(state)
}
