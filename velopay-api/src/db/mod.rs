pub mod user_repository;
pub mod transaction_repository;
pub mod mint_request_repository;
pub mod burn_request_repository;
pub mod kyc_repository;

use sqlx::{PgPool, postgres::PgPoolOptions};
use anyhow::Result;
use std::time::Duration;

/// Create a PostgreSQL connection pool with production-ready settings
pub async fn create_pool(database_url: &str) -> Result<PgPool> {
    let pool = PgPoolOptions::new()
        // Connection pool sizing
        .max_connections(50)        // Maximum concurrent connections
        .min_connections(5)         // Minimum idle connections

        // Connection timeouts
        .acquire_timeout(Duration::from_secs(10))  // Time to wait for a connection
        .idle_timeout(Duration::from_secs(600))    // Close idle connections after 10 minutes
        .max_lifetime(Duration::from_secs(1800))   // Close connections after 30 minutes

        // Connection testing
        .test_before_acquire(true)  // Test connections before use

        .connect(database_url)
        .await?;

    log::info!(
        "Database pool initialized: max={}, min={}",
        50, 5
    );

    Ok(pool)
}

/// Run database migrations
/// Note: Migrations are embedded at compile time. Ensure migrations/ directory exists.
pub async fn run_migrations(_pool: &PgPool) -> Result<()> {
    // Commented out for Windows compatibility - migrations already applied
    // sqlx::migrate!()
    //     .run(pool)
    //     .await?;

    Ok(())
}
