use sqlx::PgPool;
use uuid::Uuid;
use anyhow::Result;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

#[derive(Debug, sqlx::FromRow)]
pub struct BurnRequestRecord {
    pub id: Uuid,
    pub user_id: Uuid,
    pub wallet_address: String,
    pub amount: Decimal,
    pub bank_account: String,
    pub status: String,
    pub chain_request_id: Option<i64>,
    pub approved_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}

pub struct BurnRepository<'a> {
    pool: &'a PgPool,
}

impl<'a> BurnRepository<'a> {
    pub fn new(pool: &'a PgPool) -> Self {
        Self { pool }
    }

    /// Create a new burn request
    pub async fn create(
        &self,
        user_id: Uuid,
        wallet_address: &str,
        amount: Decimal,
        bank_account: &str,
    ) -> Result<BurnRequestRecord> {
        let burn_request = sqlx::query_as::<_, BurnRequestRecord>(
            r#"
            INSERT INTO burn_requests (user_id, wallet_address, amount, bank_account, status)
            VALUES ($1, $2, $3, $4, 'pending')
            RETURNING id, user_id, wallet_address, amount, bank_account, status, chain_request_id, approved_by, created_at
            "#,
        )
        .bind(user_id)
        .bind(wallet_address)
        .bind(amount)
        .bind(bank_account)
        .fetch_one(self.pool)
        .await?;

        Ok(burn_request)
    }

    /// Update burn request status and chain details
    pub async fn update_status(
        &self,
        id: Uuid,
        status: &str,
        chain_request_id: Option<i64>,
        approved_by: Option<Uuid>,
    ) -> Result<BurnRequestRecord> {
        let burn_request = sqlx::query_as::<_, BurnRequestRecord>(
            r#"
            UPDATE burn_requests
            SET status = $1, chain_request_id = $2, approved_by = $3
            WHERE id = $4
            RETURNING id, user_id, wallet_address, amount, bank_account, status, chain_request_id, approved_by, created_at
            "#,
        )
        .bind(status)
        .bind(chain_request_id)
        .bind(approved_by)
        .bind(id)
        .fetch_one(self.pool)
        .await?;

        Ok(burn_request)
    }

    /// Get burn request by ID
    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<BurnRequestRecord>> {
        let burn_request = sqlx::query_as::<_, BurnRequestRecord>(
            r#"
            SELECT id, user_id, wallet_address, amount, bank_account, status, chain_request_id, approved_by, created_at
            FROM burn_requests
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(self.pool)
        .await?;

        Ok(burn_request)
    }

    /// Get all burn requests for a user
    pub async fn find_by_user_id(&self, user_id: Uuid) -> Result<Vec<BurnRequestRecord>> {
        let burn_requests = sqlx::query_as::<_, BurnRequestRecord>(
            r#"
            SELECT id, user_id, wallet_address, amount, bank_account, status, chain_request_id, approved_by, created_at
            FROM burn_requests
            WHERE user_id = $1
            ORDER BY created_at DESC
            "#,
        )
        .bind(user_id)
        .fetch_all(self.pool)
        .await?;

        Ok(burn_requests)
    }

    /// Get all pending burn requests (for admin)
    pub async fn find_pending(&self) -> Result<Vec<BurnRequestRecord>> {
        let burn_requests = sqlx::query_as::<_, BurnRequestRecord>(
            r#"
            SELECT id, user_id, wallet_address, amount, bank_account, status, chain_request_id, approved_by, created_at
            FROM burn_requests
            WHERE status = 'pending'
            ORDER BY created_at ASC
            "#,
        )
        .fetch_all(self.pool)
        .await?;

        Ok(burn_requests)
    }

    /// Get all burn requests (for admin)
    pub async fn find_all(&self, limit: i64, offset: i64) -> Result<Vec<BurnRequestRecord>> {
        let burn_requests = sqlx::query_as::<_, BurnRequestRecord>(
            r#"
            SELECT id, user_id, wallet_address, amount, bank_account, status, chain_request_id, approved_by, created_at
            FROM burn_requests
            ORDER BY created_at DESC
            LIMIT $1 OFFSET $2
            "#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(self.pool)
        .await?;

        Ok(burn_requests)
    }
}
