use sqlx::PgPool;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use anyhow::Result;

#[derive(Debug, Clone, sqlx::Type)]
#[sqlx(type_name = "mint_request_status", rename_all = "lowercase")]
pub enum MintRequestStatus {
    Pending,
    Approved,
    Rejected,
    Completed,
}

#[derive(Debug, Clone)]
pub struct MintRequestRecord {
    pub id: Uuid,
    pub user_id: Uuid,
    pub wallet_address: String,
    pub amount: String,
    pub bank_reference: String,
    pub status: MintRequestStatus,
    pub chain_request_id: Option<i64>,
    pub approved_by: Option<Uuid>,
    pub created_at: Option<DateTime<Utc>>,
    pub updated_at: Option<DateTime<Utc>>,
}

pub struct MintRequestRepository<'a> {
    pool: &'a PgPool,
}

impl<'a> MintRequestRepository<'a> {
    pub fn new(pool: &'a PgPool) -> Self {
        Self { pool }
    }

    /// Create a new mint request
    pub async fn create(
        &self,
        user_id: Uuid,
        wallet_address: &str,
        amount: &str,
        bank_reference: &str,
    ) -> Result<MintRequestRecord> {
        let request = sqlx::query_as!(
            MintRequestRecord,
            r#"
            INSERT INTO mint_requests (user_id, wallet_address, amount, bank_reference, status)
            VALUES ($1, $2, $3, $4, 'pending')
            RETURNING id, user_id, wallet_address, amount, bank_reference,
                      status as "status: MintRequestStatus", chain_request_id,
                      approved_by, created_at, updated_at
            "#,
            user_id,
            wallet_address,
            amount,
            bank_reference
        )
        .fetch_one(self.pool)
        .await?;

        Ok(request)
    }

    /// Update mint request status to approved
    pub async fn approve(
        &self,
        id: Uuid,
        approved_by: Uuid,
        chain_request_id: Option<i64>,
    ) -> Result<MintRequestRecord> {
        let request = sqlx::query_as!(
            MintRequestRecord,
            r#"
            UPDATE mint_requests
            SET status = 'approved', approved_by = $1, chain_request_id = $2, updated_at = NOW()
            WHERE id = $3
            RETURNING id, user_id, wallet_address, amount, bank_reference,
                      status as "status: MintRequestStatus", chain_request_id,
                      approved_by, created_at, updated_at
            "#,
            approved_by,
            chain_request_id,
            id
        )
        .fetch_one(self.pool)
        .await?;

        Ok(request)
    }

    /// Update mint request status to rejected
    pub async fn reject(&self, id: Uuid, rejected_by: Uuid) -> Result<MintRequestRecord> {
        let request = sqlx::query_as!(
            MintRequestRecord,
            r#"
            UPDATE mint_requests
            SET status = 'rejected', approved_by = $1, updated_at = NOW()
            WHERE id = $2
            RETURNING id, user_id, wallet_address, amount, bank_reference,
                      status as "status: MintRequestStatus", chain_request_id,
                      approved_by, created_at, updated_at
            "#,
            rejected_by,
            id
        )
        .fetch_one(self.pool)
        .await?;

        Ok(request)
    }

    /// Mark mint request as completed
    pub async fn complete(&self, id: Uuid) -> Result<MintRequestRecord> {
        let request = sqlx::query_as!(
            MintRequestRecord,
            r#"
            UPDATE mint_requests
            SET status = 'completed', updated_at = NOW()
            WHERE id = $1
            RETURNING id, user_id, wallet_address, amount, bank_reference,
                      status as "status: MintRequestStatus", chain_request_id,
                      approved_by, created_at, updated_at
            "#,
            id
        )
        .fetch_one(self.pool)
        .await?;

        Ok(request)
    }

    /// Find mint request by ID
    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<MintRequestRecord>> {
        let request = sqlx::query_as!(
            MintRequestRecord,
            r#"
            SELECT id, user_id, wallet_address, amount, bank_reference,
                   status as "status: MintRequestStatus", chain_request_id,
                   approved_by, created_at, updated_at
            FROM mint_requests
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(self.pool)
        .await?;

        Ok(request)
    }

    /// Get all mint requests for a user
    pub async fn find_by_user(&self, user_id: Uuid) -> Result<Vec<MintRequestRecord>> {
        let requests = sqlx::query_as!(
            MintRequestRecord,
            r#"
            SELECT id, user_id, wallet_address, amount, bank_reference,
                   status as "status: MintRequestStatus", chain_request_id,
                   approved_by, created_at, updated_at
            FROM mint_requests
            WHERE user_id = $1
            ORDER BY created_at DESC
            "#,
            user_id
        )
        .fetch_all(self.pool)
        .await?;

        Ok(requests)
    }

    /// Get all mint requests for a user (alias for find_by_user)
    pub async fn find_by_user_id(&self, user_id: Uuid) -> Result<Vec<MintRequestRecord>> {
        self.find_by_user(user_id).await
    }

    /// Get pending mint requests (for admin)
    pub async fn find_pending(&self) -> Result<Vec<MintRequestRecord>> {
        let requests = sqlx::query_as!(
            MintRequestRecord,
            r#"
            SELECT id, user_id, wallet_address, amount, bank_reference,
                   status as "status: MintRequestStatus", chain_request_id,
                   approved_by, created_at, updated_at
            FROM mint_requests
            WHERE status = 'pending'
            ORDER BY created_at ASC
            "#
        )
        .fetch_all(self.pool)
        .await?;

        Ok(requests)
    }

    /// Get all mint requests with pagination (for admin)
    pub async fn find_all(&self, limit: i64, offset: i64) -> Result<Vec<MintRequestRecord>> {
        let requests = sqlx::query_as!(
            MintRequestRecord,
            r#"
            SELECT id, user_id, wallet_address, amount, bank_reference,
                   status as "status: MintRequestStatus", chain_request_id,
                   approved_by, created_at, updated_at
            FROM mint_requests
            ORDER BY created_at DESC
            LIMIT $1 OFFSET $2
            "#,
            limit,
            offset
        )
        .fetch_all(self.pool)
        .await?;

        Ok(requests)
    }

    /// Update mint request status
    pub async fn update_status(
        &self,
        id: Uuid,
        status: &str,
        chain_request_id: Option<i64>,
        approved_by: Option<Uuid>,
    ) -> Result<MintRequestRecord> {
        let status_enum: MintRequestStatus = match status {
            "approved" => MintRequestStatus::Approved,
            "rejected" => MintRequestStatus::Rejected,
            "completed" => MintRequestStatus::Completed,
            _ => MintRequestStatus::Pending,
        };

        let request = sqlx::query_as!(
            MintRequestRecord,
            r#"
            UPDATE mint_requests
            SET status = $1, chain_request_id = $2, approved_by = $3, updated_at = NOW()
            WHERE id = $4
            RETURNING id, user_id, wallet_address, amount, bank_reference,
                      status as "status: MintRequestStatus", chain_request_id,
                      approved_by, created_at, updated_at
            "#,
            status_enum as MintRequestStatus,
            chain_request_id,
            approved_by,
            id
        )
        .fetch_one(self.pool)
        .await?;

        Ok(request)
    }
}
