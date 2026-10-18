pub mod invoices;
pub mod messages;
pub mod orders;
pub mod products;
pub mod suppliers;
pub mod users;

use sqlx::PgPool;

pub async fn seed_initial_real_data(pool: &PgPool) -> Result<(), sqlx::Error> {
    users::seed_users(pool).await?;
    products::seed_products(pool).await?;
    orders::seed_orders(pool).await?;
    suppliers::seed_suppliers(pool).await?;
    invoices::seed_invoices(pool).await?;
    messages::seed_messages(pool).await?;
    tracing::info!("✅ All real database entities successfully synchronized to NeonDB PostgreSQL!");
    Ok(())
}
