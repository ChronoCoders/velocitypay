use crate::db::kyc_repository::{KycRepository, KycSubmissionRecord};
use crate::models::response::KYCSubmissionResponse;
use crate::models::kyc::KYCStatus;
use anyhow::{Result, anyhow};
use chrono::{NaiveDate, Utc};
use sqlx::PgPool;
use uuid::Uuid;

pub struct KYCService;

impl KYCService {
    pub fn new() -> Self {
        Self
    }

    /// Submit KYC information
    pub async fn submit_kyc(
        &self,
        pool: &PgPool,
        user_id: Uuid,
        wallet_address: &str,
        document_hash: &str,
        full_name: &str,
        date_of_birth: NaiveDate,
        country: &str,
    ) -> Result<KYCSubmissionResponse> {
        let repo = KycRepository::new(pool);

        // Check if user already has a KYC submission
        if let Some(existing) = repo.find_by_user_id(user_id).await? {
            use crate::db::kyc_repository::KycStatus as DbStatus;
            if matches!(existing.status, DbStatus::Pending) {
                return Err(anyhow!("You already have a pending KYC submission"));
            }
            if matches!(existing.status, DbStatus::Verified) {
                return Err(anyhow!("Your KYC is already verified"));
            }
        }

        // Create KYC submission
        let kyc_submission = repo
            .submit(
                user_id,
                wallet_address,
                document_hash,
                full_name,
                date_of_birth,
                country,
            )
            .await?;

        Ok(Self::kyc_submission_to_response(kyc_submission))
    }

    /// Get KYC submission by ID
    pub async fn get_kyc_submission(
        &self,
        pool: &PgPool,
        submission_id: Uuid,
    ) -> Result<KYCSubmissionResponse> {
        let repo = KycRepository::new(pool);

        let kyc_submission = repo
            .find_by_id(submission_id)
            .await?
            .ok_or_else(|| anyhow!("KYC submission not found"))?;

        Ok(Self::kyc_submission_to_response(kyc_submission))
    }

    /// Get KYC submission for a user
    pub async fn get_user_kyc_submission(
        &self,
        pool: &PgPool,
        user_id: Uuid,
    ) -> Result<Option<KYCSubmissionResponse>> {
        let repo = KycRepository::new(pool);

        let kyc_submission = repo.find_by_user_id(user_id).await?;

        Ok(kyc_submission.map(Self::kyc_submission_to_response))
    }

    /// Get KYC submission by wallet address
    pub async fn get_kyc_by_wallet(
        &self,
        pool: &PgPool,
        wallet_address: &str,
    ) -> Result<Option<KYCSubmissionResponse>> {
        let repo = KycRepository::new(pool);

        let kyc_submission = repo.find_by_wallet(wallet_address).await?;

        Ok(kyc_submission.map(Self::kyc_submission_to_response))
    }

    /// Get all pending KYC submissions (admin only)
    pub async fn get_pending_submissions(
        &self,
        pool: &PgPool,
    ) -> Result<Vec<KYCSubmissionResponse>> {
        let repo = KycRepository::new(pool);

        let kyc_submissions = repo.find_pending().await?;

        Ok(kyc_submissions
            .into_iter()
            .map(Self::kyc_submission_to_response)
            .collect())
    }

    /// Get all KYC submissions with pagination (admin only)
    pub async fn get_all_submissions(
        &self,
        pool: &PgPool,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<KYCSubmissionResponse>> {
        let repo = KycRepository::new(pool);

        let kyc_submissions = repo.find_all(limit, offset).await?;

        Ok(kyc_submissions
            .into_iter()
            .map(Self::kyc_submission_to_response)
            .collect())
    }

    /// Verify KYC submission (admin only)
    pub async fn verify_kyc(
        &self,
        pool: &PgPool,
        submission_id: Uuid,
        admin_id: Uuid,
    ) -> Result<KYCSubmissionResponse> {
        let repo = KycRepository::new(pool);

        // Check if submission exists and is pending
        let existing = repo
            .find_by_id(submission_id)
            .await?
            .ok_or_else(|| anyhow!("KYC submission not found"))?;

        use crate::db::kyc_repository::KycStatus as DbStatus;
        if !matches!(existing.status, DbStatus::Pending) {
            return Err(anyhow!("KYC submission is not in pending status"));
        }

        // Update status to verified
        let kyc_submission = repo
            .update_status(submission_id, "verified", Some(admin_id))
            .await?;

        Ok(Self::kyc_submission_to_response(kyc_submission))
    }

    /// Reject KYC submission (admin only)
    pub async fn reject_kyc(
        &self,
        pool: &PgPool,
        submission_id: Uuid,
        admin_id: Uuid,
    ) -> Result<KYCSubmissionResponse> {
        let repo = KycRepository::new(pool);

        // Check if submission exists and is pending
        let existing = repo
            .find_by_id(submission_id)
            .await?
            .ok_or_else(|| anyhow!("KYC submission not found"))?;

        use crate::db::kyc_repository::KycStatus as DbStatus;
        if !matches!(existing.status, DbStatus::Pending) {
            return Err(anyhow!("KYC submission is not in pending status"));
        }

        // Update status to rejected
        let kyc_submission = repo
            .update_status(submission_id, "rejected", Some(admin_id))
            .await?;

        Ok(Self::kyc_submission_to_response(kyc_submission))
    }

    /// Convert database record to response model
    fn kyc_submission_to_response(record: KycSubmissionRecord) -> KYCSubmissionResponse {
        use crate::db::kyc_repository::KycStatus as DbStatus;

        let status = match record.status {
            DbStatus::Pending => KYCStatus::Pending,
            DbStatus::Verified => KYCStatus::Verified,
            DbStatus::Rejected => KYCStatus::Rejected,
            DbStatus::NotSubmitted => KYCStatus::NotSubmitted,
        };

        // Convert DateTime<Utc> to NaiveDate for date_of_birth
        let date_of_birth = record.date_of_birth.date_naive();

        KYCSubmissionResponse {
            id: record.id,
            user_id: record.user_id,
            wallet_address: record.wallet_address,
            document_hash: record.document_hash,
            full_name: record.full_name,
            date_of_birth,
            country: record.country,
            status,
            verified_by: record.verified_by,
            created_at: record.created_at.unwrap_or_else(|| Utc::now()),
        }
    }
}
