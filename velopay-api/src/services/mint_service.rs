use crate::db::mint_request_repository::{MintRequestRepository, MintRequestRecord};
use crate::models::response::MintRequestResponse;
use crate::models::mint_request::MintRequestStatus;
use anyhow::{Result, anyhow};
use chrono::Utc;
use rust_decimal::Decimal;
use sqlx::PgPool;
use std::str::FromStr;
use uuid::Uuid;

pub struct MintService;

impl MintService {
    pub fn new() -> Self {
        Self
    }

    /// Create a new mint request
    pub async fn create_mint_request(
        &self,
        pool: &PgPool,
        user_id: Uuid,
        wallet_address: &str,
        amount: Decimal,
        bank_reference: &str,
    ) -> Result<MintRequestResponse> {
        let repo = MintRequestRepository::new(pool);

        // Validate amount
        if amount <= Decimal::ZERO {
            return Err(anyhow!("Amount must be greater than zero"));
        }

        // Create mint request
        let mint_request = repo
            .create(user_id, wallet_address, &amount.to_string(), bank_reference)
            .await?;

        Ok(Self::mint_request_to_response(mint_request))
    }

    /// Get mint request by ID
    pub async fn get_mint_request(
        &self,
        pool: &PgPool,
        request_id: Uuid,
    ) -> Result<MintRequestResponse> {
        let repo = MintRequestRepository::new(pool);

        let mint_request = repo
            .find_by_id(request_id)
            .await?
            .ok_or_else(|| anyhow!("Mint request not found"))?;

        Ok(Self::mint_request_to_response(mint_request))
    }

    /// Get all mint requests for a user
    pub async fn get_user_mint_requests(
        &self,
        pool: &PgPool,
        user_id: Uuid,
    ) -> Result<Vec<MintRequestResponse>> {
        let repo = MintRequestRepository::new(pool);

        let mint_requests = repo.find_by_user_id(user_id).await?;

        Ok(mint_requests
            .into_iter()
            .map(Self::mint_request_to_response)
            .collect())
    }

    /// Get all pending mint requests (admin only)
    pub async fn get_pending_mint_requests(
        &self,
        pool: &PgPool,
    ) -> Result<Vec<MintRequestResponse>> {
        let repo = MintRequestRepository::new(pool);

        let mint_requests = repo.find_pending().await?;

        Ok(mint_requests
            .into_iter()
            .map(Self::mint_request_to_response)
            .collect())
    }

    /// Get all mint requests with pagination (admin only)
    pub async fn get_all_mint_requests(
        &self,
        pool: &PgPool,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<MintRequestResponse>> {
        let repo = MintRequestRepository::new(pool);

        let mint_requests = repo.find_all(limit, offset).await?;

        Ok(mint_requests
            .into_iter()
            .map(Self::mint_request_to_response)
            .collect())
    }

    /// Approve mint request (admin only)
    pub async fn approve_mint_request(
        &self,
        pool: &PgPool,
        request_id: Uuid,
        admin_id: Uuid,
        chain_request_id: Option<i64>,
    ) -> Result<MintRequestResponse> {
        let repo = MintRequestRepository::new(pool);

        // Check if request exists and is pending
        let existing = repo
            .find_by_id(request_id)
            .await?
            .ok_or_else(|| anyhow!("Mint request not found"))?;

        use crate::db::mint_request_repository::MintRequestStatus as DbStatus;
        if !matches!(existing.status, DbStatus::Pending) {
            return Err(anyhow!("Mint request is not in pending status"));
        }

        // Update status to approved
        let mint_request = repo
            .update_status(request_id, "approved", chain_request_id, Some(admin_id))
            .await?;

        Ok(Self::mint_request_to_response(mint_request))
    }

    /// Reject mint request (admin only)
    pub async fn reject_mint_request(
        &self,
        pool: &PgPool,
        request_id: Uuid,
        admin_id: Uuid,
    ) -> Result<MintRequestResponse> {
        let repo = MintRequestRepository::new(pool);

        // Check if request exists and is pending
        let existing = repo
            .find_by_id(request_id)
            .await?
            .ok_or_else(|| anyhow!("Mint request not found"))?;

        use crate::db::mint_request_repository::MintRequestStatus as DbStatus;
        if !matches!(existing.status, DbStatus::Pending) {
            return Err(anyhow!("Mint request is not in pending status"));
        }

        // Update status to rejected
        let mint_request = repo
            .update_status(request_id, "rejected", None, Some(admin_id))
            .await?;

        Ok(Self::mint_request_to_response(mint_request))
    }

    /// Mark mint request as completed (called after blockchain confirmation)
    pub async fn complete_mint_request(
        &self,
        pool: &PgPool,
        request_id: Uuid,
        chain_request_id: i64,
    ) -> Result<MintRequestResponse> {
        let repo = MintRequestRepository::new(pool);

        let mint_request = repo
            .update_status(request_id, "completed", Some(chain_request_id), None)
            .await?;

        Ok(Self::mint_request_to_response(mint_request))
    }

    /// Convert database record to response model
    fn mint_request_to_response(record: MintRequestRecord) -> MintRequestResponse {
        use crate::db::mint_request_repository::MintRequestStatus as DbStatus;

        let status = match record.status {
            DbStatus::Approved => MintRequestStatus::Approved,
            DbStatus::Rejected => MintRequestStatus::Rejected,
            DbStatus::Completed => MintRequestStatus::Completed,
            DbStatus::Pending => MintRequestStatus::Pending,
        };

        // Parse amount from string to Decimal
        let amount = Decimal::from_str(&record.amount).unwrap_or(Decimal::ZERO);

        MintRequestResponse {
            id: record.id,
            user_id: record.user_id,
            wallet_address: record.wallet_address,
            amount,
            bank_reference: record.bank_reference,
            status,
            chain_request_id: record.chain_request_id,
            approved_by: record.approved_by,
            created_at: record.created_at.unwrap_or_else(|| Utc::now()),
        }
    }
}
