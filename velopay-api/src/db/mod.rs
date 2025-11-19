use sqlx::postgres::PgPoolOptions;
use sqlx::PgPool;
use anyhow::Result;

pub mod user_repository;
pub mod transaction_repository;
pub mod mint_repository;
pub mod burn_repository;
pub mod kyc_repository;

/// Initialize database connection pool
pub async fn create_pool(database_url: &str) -> Result<PgPool> {
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(database_url)
        .await?;

    log::info!("Database connection pool created successfully");
    Ok(pool)
}

/// Run database migrations
pub async fn run_migrations(pool: &PgPool) -> Result<()> {
    sqlx::migrate!("./migrations")
        .run(pool)
        .await?;

    log::info!("Database migrations completed successfully");
    Ok(())
}
