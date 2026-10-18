pub mod audit;
pub mod invoices;
pub mod messages;
pub mod orders;
pub mod products;
pub mod suppliers;
pub mod users;

use sqlx::PgPool;

pub async fn init_database_schema(pool: &PgPool) -> Result<(), sqlx::Error> {
    tracing::info!("Initializing NeonDB database schema (DDL)...");
    users::init_users_table(pool).await?;
    products::init_products_table(pool).await?;
    orders::init_orders_table(pool).await?;
    suppliers::init_suppliers_table(pool).await?;
    invoices::init_invoices_table(pool).await?;
    messages::init_messages_table(pool).await?;
    audit::init_audit_table(pool).await?;
    tracing::info!("✅ NeonDB database schema verified & ready.");
    Ok(())
}
