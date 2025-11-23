pub mod user_repository;
pub mod transaction_repository;
pub mod mint_request_repository;
pub mod burn_request_repository;
pub mod kyc_repository;

use sqlx::{PgPool, postgres::PgPoolOptions};
use anyhow::Result;

/// Create a PostgreSQL connection pool
pub async fn create_pool(database_url: &str) -> Result<PgPool> {
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(database_url)
        .await?;

    Ok(pool)
}

/// Run database migrations
pub async fn run_migrations(pool: &PgPool) -> Result<()> {
    sqlx::migrate!("migrations")
        .run(pool)
        .await?;

    Ok(())
}
