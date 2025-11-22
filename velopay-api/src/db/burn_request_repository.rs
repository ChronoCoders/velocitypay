use sqlx::PgPool;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use anyhow::Result;

#[derive(Debug, Clone, sqlx::Type)]
#[sqlx(type_name = "burn_request_status", rename_all = "lowercase")]
pub enum BurnRequestStatus {
    Pending,
    Reserved,
    Approved,
    Rejected,
    Completed,
}

#[derive(Debug, Clone)]
pub struct BurnRequestRecord {
    pub id: Uuid,
    pub user_id: Uuid,
    pub wallet_address: String,
    pub amount: String,
    pub bank_account: String,
    pub status: BurnRequestStatus,
    pub chain_request_id: Option<i64>,
    pub approved_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub struct BurnRequestRepository<'a> {
    pool: &'a PgPool,
}

impl<'a> BurnRequestRepository<'a> {
    pub fn new(pool: &'a PgPool) -> Self {
        Self { pool }
    }

    /// Create a new burn request
    pub async fn create(
        &self,
        user_id: Uuid,
        wallet_address: &str,
        amount: &str,
        bank_account: &str,
    ) -> Result<BurnRequestRecord> {
        let request = sqlx::query_as!(
            BurnRequestRecord,
            r#"
            INSERT INTO burn_requests (user_id, wallet_address, amount, bank_account, status)
            VALUES ($1, $2, $3, $4, 'pending')
            RETURNING id, user_id, wallet_address, amount, bank_account,
                      status as "status: BurnRequestStatus", chain_request_id,
                      approved_by, created_at, updated_at
            "#,
            user_id,
            wallet_address,
            amount,
            bank_account
        )
        .fetch_one(self.pool)
        .await?;

        Ok(request)
    }

    /// Update burn request status to reserved
    pub async fn mark_reserved(
        &self,
        id: Uuid,
        chain_request_id: i64,
    ) -> Result<BurnRequestRecord> {
        let request = sqlx::query_as!(
            BurnRequestRecord,
            r#"
            UPDATE burn_requests
            SET status = 'reserved', chain_request_id = $1, updated_at = NOW()
            WHERE id = $2
            RETURNING id, user_id, wallet_address, amount, bank_account,
                      status as "status: BurnRequestStatus", chain_request_id,
                      approved_by, created_at, updated_at
            "#,
            chain_request_id,
            id
        )
        .fetch_one(self.pool)
        .await?;

        Ok(request)
    }

    /// Update burn request status to approved
    pub async fn approve(&self, id: Uuid, approved_by: Uuid) -> Result<BurnRequestRecord> {
        let request = sqlx::query_as!(
            BurnRequestRecord,
            r#"
            UPDATE burn_requests
            SET status = 'approved', approved_by = $1, updated_at = NOW()
            WHERE id = $2
            RETURNING id, user_id, wallet_address, amount, bank_account,
                      status as "status: BurnRequestStatus", chain_request_id,
                      approved_by, created_at, updated_at
            "#,
            approved_by,
            id
        )
        .fetch_one(self.pool)
        .await?;

        Ok(request)
    }

    /// Update burn request status to rejected
    pub async fn reject(&self, id: Uuid, rejected_by: Uuid) -> Result<BurnRequestRecord> {
        let request = sqlx::query_as!(
            BurnRequestRecord,
            r#"
            UPDATE burn_requests
            SET status = 'rejected', approved_by = $1, updated_at = NOW()
            WHERE id = $2
            RETURNING id, user_id, wallet_address, amount, bank_account,
                      status as "status: BurnRequestStatus", chain_request_id,
                      approved_by, created_at, updated_at
            "#,
            rejected_by,
            id
        )
        .fetch_one(self.pool)
        .await?;

        Ok(request)
    }

    /// Mark burn request as completed
    pub async fn complete(&self, id: Uuid) -> Result<BurnRequestRecord> {
        let request = sqlx::query_as!(
            BurnRequestRecord,
            r#"
            UPDATE burn_requests
            SET status = 'completed', updated_at = NOW()
            WHERE id = $1
            RETURNING id, user_id, wallet_address, amount, bank_account,
                      status as "status: BurnRequestStatus", chain_request_id,
                      approved_by, created_at, updated_at
            "#,
            id
        )
        .fetch_one(self.pool)
        .await?;

        Ok(request)
    }

    /// Find burn request by ID
    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<BurnRequestRecord>> {
        let request = sqlx::query_as!(
            BurnRequestRecord,
            r#"
            SELECT id, user_id, wallet_address, amount, bank_account,
                   status as "status: BurnRequestStatus", chain_request_id,
                   approved_by, created_at, updated_at
            FROM burn_requests
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(self.pool)
        .await?;

        Ok(request)
    }

    /// Get all burn requests for a user
    pub async fn find_by_user(&self, user_id: Uuid) -> Result<Vec<BurnRequestRecord>> {
        let requests = sqlx::query_as!(
            BurnRequestRecord,
            r#"
            SELECT id, user_id, wallet_address, amount, bank_account,
                   status as "status: BurnRequestStatus", chain_request_id,
                   approved_by, created_at, updated_at
            FROM burn_requests
            WHERE user_id = $1
            ORDER BY created_at DESC
            "#,
            user_id
        )
        .fetch_all(self.pool)
        .await?;

        Ok(requests)
    }

    /// Get pending burn requests (for admin)
    pub async fn find_pending(&self) -> Result<Vec<BurnRequestRecord>> {
        let requests = sqlx::query_as!(
            BurnRequestRecord,
            r#"
            SELECT id, user_id, wallet_address, amount, bank_account,
                   status as "status: BurnRequestStatus", chain_request_id,
                   approved_by, created_at, updated_at
            FROM burn_requests
            WHERE status = 'pending' OR status = 'reserved'
            ORDER BY created_at ASC
            "#
        )
        .fetch_all(self.pool)
        .await?;

        Ok(requests)
    }
}
