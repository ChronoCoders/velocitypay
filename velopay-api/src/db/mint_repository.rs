use sqlx::PgPool;
use uuid::Uuid;
use anyhow::Result;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

#[derive(Debug, sqlx::FromRow)]
pub struct MintRequestRecord {
    pub id: Uuid,
    pub user_id: Uuid,
    pub wallet_address: String,
    pub amount: Decimal,
    pub bank_reference: String,
    pub status: String,
    pub chain_request_id: Option<i64>,
    pub approved_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}

pub struct MintRepository<'a> {
    pool: &'a PgPool,
}

impl<'a> MintRepository<'a> {
    pub fn new(pool: &'a PgPool) -> Self {
        Self { pool }
    }

    /// Create a new mint request
    pub async fn create(
        &self,
        user_id: Uuid,
        wallet_address: &str,
        amount: Decimal,
        bank_reference: &str,
    ) -> Result<MintRequestRecord> {
        let mint_request = sqlx::query_as::<_, MintRequestRecord>(
            r#"
            INSERT INTO mint_requests (user_id, wallet_address, amount, bank_reference, status)
            VALUES ($1, $2, $3, $4, 'pending')
            RETURNING id, user_id, wallet_address, amount, bank_reference, status, chain_request_id, approved_by, created_at
            "#,
        )
        .bind(user_id)
        .bind(wallet_address)
        .bind(amount)
        .bind(bank_reference)
        .fetch_one(self.pool)
        .await?;

        Ok(mint_request)
    }

    /// Update mint request status and chain details
    pub async fn update_status(
        &self,
        id: Uuid,
        status: &str,
        chain_request_id: Option<i64>,
        approved_by: Option<Uuid>,
    ) -> Result<MintRequestRecord> {
        let mint_request = sqlx::query_as::<_, MintRequestRecord>(
            r#"
            UPDATE mint_requests
            SET status = $1, chain_request_id = $2, approved_by = $3
            WHERE id = $4
            RETURNING id, user_id, wallet_address, amount, bank_reference, status, chain_request_id, approved_by, created_at
            "#,
        )
        .bind(status)
        .bind(chain_request_id)
        .bind(approved_by)
        .bind(id)
        .fetch_one(self.pool)
        .await?;

        Ok(mint_request)
    }

    /// Get mint request by ID
    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<MintRequestRecord>> {
        let mint_request = sqlx::query_as::<_, MintRequestRecord>(
            r#"
            SELECT id, user_id, wallet_address, amount, bank_reference, status, chain_request_id, approved_by, created_at
            FROM mint_requests
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(self.pool)
        .await?;

        Ok(mint_request)
    }

    /// Get all mint requests for a user
    pub async fn find_by_user_id(&self, user_id: Uuid) -> Result<Vec<MintRequestRecord>> {
        let mint_requests = sqlx::query_as::<_, MintRequestRecord>(
            r#"
            SELECT id, user_id, wallet_address, amount, bank_reference, status, chain_request_id, approved_by, created_at
            FROM mint_requests
            WHERE user_id = $1
            ORDER BY created_at DESC
            "#,
        )
        .bind(user_id)
        .fetch_all(self.pool)
        .await?;

        Ok(mint_requests)
    }

    /// Get all pending mint requests (for admin)
    pub async fn find_pending(&self) -> Result<Vec<MintRequestRecord>> {
        let mint_requests = sqlx::query_as::<_, MintRequestRecord>(
            r#"
            SELECT id, user_id, wallet_address, amount, bank_reference, status, chain_request_id, approved_by, created_at
            FROM mint_requests
            WHERE status = 'pending'
            ORDER BY created_at ASC
            "#,
        )
        .fetch_all(self.pool)
        .await?;

        Ok(mint_requests)
    }

    /// Get all mint requests (for admin)
    pub async fn find_all(&self, limit: i64, offset: i64) -> Result<Vec<MintRequestRecord>> {
        let mint_requests = sqlx::query_as::<_, MintRequestRecord>(
            r#"
            SELECT id, user_id, wallet_address, amount, bank_reference, status, chain_request_id, approved_by, created_at
            FROM mint_requests
            ORDER BY created_at DESC
            LIMIT $1 OFFSET $2
            "#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(self.pool)
        .await?;

        Ok(mint_requests)
    }
}
