use crate::db::burn_repository::{BurnRepository, BurnRequestRecord};
use crate::models::response::BurnRequestResponse;
use crate::models::burn_request::BurnRequestStatus;
use anyhow::{Result, anyhow};
use rust_decimal::Decimal;
use sqlx::PgPool;
use uuid::Uuid;

pub struct BurnService;

impl BurnService {
    pub fn new() -> Self {
        Self
    }

    /// Create a new burn request
    pub async fn create_burn_request(
        &self,
        pool: &PgPool,
        user_id: Uuid,
        wallet_address: &str,
        amount: Decimal,
        bank_account: &str,
    ) -> Result<BurnRequestResponse> {
        let repo = BurnRepository::new(pool);

        // Validate amount
        if amount <= Decimal::ZERO {
            return Err(anyhow!("Amount must be greater than zero"));
        }

        // Create burn request
        let burn_request = repo
            .create(user_id, wallet_address, amount, bank_account)
            .await?;

        Ok(Self::burn_request_to_response(burn_request))
    }

    /// Get burn request by ID
    pub async fn get_burn_request(
        &self,
        pool: &PgPool,
        request_id: Uuid,
    ) -> Result<BurnRequestResponse> {
        let repo = BurnRepository::new(pool);

        let burn_request = repo
            .find_by_id(request_id)
            .await?
            .ok_or_else(|| anyhow!("Burn request not found"))?;

        Ok(Self::burn_request_to_response(burn_request))
    }

    /// Get all burn requests for a user
    pub async fn get_user_burn_requests(
        &self,
        pool: &PgPool,
        user_id: Uuid,
    ) -> Result<Vec<BurnRequestResponse>> {
        let repo = BurnRepository::new(pool);

        let burn_requests = repo.find_by_user_id(user_id).await?;

        Ok(burn_requests
            .into_iter()
            .map(Self::burn_request_to_response)
            .collect())
    }

    /// Get all pending burn requests (admin only)
    pub async fn get_pending_burn_requests(
        &self,
        pool: &PgPool,
    ) -> Result<Vec<BurnRequestResponse>> {
        let repo = BurnRepository::new(pool);

        let burn_requests = repo.find_pending().await?;

        Ok(burn_requests
            .into_iter()
            .map(Self::burn_request_to_response)
            .collect())
    }

    /// Get all burn requests with pagination (admin only)
    pub async fn get_all_burn_requests(
        &self,
        pool: &PgPool,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<BurnRequestResponse>> {
        let repo = BurnRepository::new(pool);

        let burn_requests = repo.find_all(limit, offset).await?;

        Ok(burn_requests
            .into_iter()
            .map(Self::burn_request_to_response)
            .collect())
    }

    /// Approve burn request (admin only)
    pub async fn approve_burn_request(
        &self,
        pool: &PgPool,
        request_id: Uuid,
        admin_id: Uuid,
        chain_request_id: Option<i64>,
    ) -> Result<BurnRequestResponse> {
        let repo = BurnRepository::new(pool);

        // Check if request exists and is pending
        let existing = repo
            .find_by_id(request_id)
            .await?
            .ok_or_else(|| anyhow!("Burn request not found"))?;

        if existing.status != "pending" && existing.status != "reserved" {
            return Err(anyhow!("Burn request is not in pending or reserved status"));
        }

        // Update status to approved
        let burn_request = repo
            .update_status(request_id, "approved", chain_request_id, Some(admin_id))
            .await?;

        Ok(Self::burn_request_to_response(burn_request))
    }

    /// Reject burn request (admin only)
    pub async fn reject_burn_request(
        &self,
        pool: &PgPool,
        request_id: Uuid,
        admin_id: Uuid,
    ) -> Result<BurnRequestResponse> {
        let repo = BurnRepository::new(pool);

        // Check if request exists and is pending
        let existing = repo
            .find_by_id(request_id)
            .await?
            .ok_or_else(|| anyhow!("Burn request not found"))?;

        if existing.status != "pending" && existing.status != "reserved" {
            return Err(anyhow!("Burn request is not in pending or reserved status"));
        }

        // Update status to rejected
        let burn_request = repo
            .update_status(request_id, "rejected", None, Some(admin_id))
            .await?;

        Ok(Self::burn_request_to_response(burn_request))
    }

    /// Mark burn request as completed (called after blockchain confirmation)
    pub async fn complete_burn_request(
        &self,
        pool: &PgPool,
        request_id: Uuid,
        chain_request_id: i64,
    ) -> Result<BurnRequestResponse> {
        let repo = BurnRepository::new(pool);

        let burn_request = repo
            .update_status(request_id, "completed", Some(chain_request_id), None)
            .await?;

        Ok(Self::burn_request_to_response(burn_request))
    }

    /// Mark burn request as reserved (after blockchain reservation)
    pub async fn reserve_burn_request(
        &self,
        pool: &PgPool,
        request_id: Uuid,
        chain_request_id: i64,
    ) -> Result<BurnRequestResponse> {
        let repo = BurnRepository::new(pool);

        let burn_request = repo
            .update_status(request_id, "reserved", Some(chain_request_id), None)
            .await?;

        Ok(Self::burn_request_to_response(burn_request))
    }

    /// Convert database record to response model
    fn burn_request_to_response(record: BurnRequestRecord) -> BurnRequestResponse {
        let status = match record.status.as_str() {
            "reserved" => BurnRequestStatus::Reserved,
            "approved" => BurnRequestStatus::Approved,
            "rejected" => BurnRequestStatus::Rejected,
            "completed" => BurnRequestStatus::Completed,
            _ => BurnRequestStatus::Pending,
        };

        BurnRequestResponse {
            id: record.id,
            user_id: record.user_id,
            wallet_address: record.wallet_address,
            amount: record.amount,
            bank_account: record.bank_account,
            status,
            chain_request_id: record.chain_request_id,
            approved_by: record.approved_by,
            created_at: record.created_at,
        }
    }
}
