use sqlx::PgPool;
use uuid::Uuid;
use chrono::{DateTime, NaiveDate, Utc};
use anyhow::Result;

#[derive(Debug, Clone, sqlx::Type)]
#[sqlx(type_name = "kyc_status", rename_all = "lowercase")]
pub enum KycStatus {
    NotSubmitted,
    Pending,
    Verified,
    Rejected,
}

#[derive(Debug, Clone)]
pub struct KycSubmissionRecord {
    pub id: Uuid,
    pub user_id: Uuid,
    pub wallet_address: String,
    pub document_hash: String,
    pub full_name: String,
    pub date_of_birth: DateTime<Utc>,
    pub country: String,
    pub status: KycStatus,
    pub verified_by: Option<Uuid>,
    pub created_at: Option<DateTime<Utc>>,
}

pub struct KycRepository<'a> {
    pool: &'a PgPool,
}

impl<'a> KycRepository<'a> {
    pub fn new(pool: &'a PgPool) -> Self {
        Self { pool }
    }

    /// Submit KYC information
    pub async fn submit(
        &self,
        user_id: Uuid,
        wallet_address: &str,
        document_hash: &str,
        full_name: &str,
        date_of_birth: NaiveDate,
        country: &str,
    ) -> Result<KycSubmissionRecord> {
        // Convert NaiveDate to DateTime<Utc> for database storage
        let dob_datetime = date_of_birth.and_hms_opt(0, 0, 0)
            .unwrap()
            .and_local_timezone(Utc)
            .single()
            .unwrap();

        let kyc = sqlx::query_as!(
            KycSubmissionRecord,
            r#"
            INSERT INTO kyc_submissions (
                user_id, wallet_address, document_hash, full_name, date_of_birth, country, status
            )
            VALUES ($1, $2, $3, $4, $5, $6, 'pending')
            RETURNING id, user_id, wallet_address, document_hash, full_name, date_of_birth, country,
                      status as "status: KycStatus", verified_by, created_at
            "#,
            user_id,
            wallet_address,
            document_hash,
            full_name,
            dob_datetime,
            country
        )
        .fetch_one(self.pool)
        .await?;

        Ok(kyc)
    }

    /// Find KYC submission by ID
    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<KycSubmissionRecord>> {
        let kyc = sqlx::query_as!(
            KycSubmissionRecord,
            r#"
            SELECT id, user_id, wallet_address, document_hash, full_name, date_of_birth, country,
                   status as "status: KycStatus", verified_by, created_at
            FROM kyc_submissions
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(self.pool)
        .await?;

        Ok(kyc)
    }

    /// Find KYC submission by user ID
    pub async fn find_by_user(&self, user_id: Uuid) -> Result<Option<KycSubmissionRecord>> {
        let kyc = sqlx::query_as!(
            KycSubmissionRecord,
            r#"
            SELECT id, user_id, wallet_address, document_hash, full_name, date_of_birth, country,
                   status as "status: KycStatus", verified_by, created_at
            FROM kyc_submissions
            WHERE user_id = $1
            ORDER BY created_at DESC
            LIMIT 1
            "#,
            user_id
        )
        .fetch_optional(self.pool)
        .await?;

        Ok(kyc)
    }

    /// Find KYC submission by user ID (alias for find_by_user)
    pub async fn find_by_user_id(&self, user_id: Uuid) -> Result<Option<KycSubmissionRecord>> {
        self.find_by_user(user_id).await
    }

    /// Find KYC submission by wallet address
    pub async fn find_by_wallet(&self, wallet_address: &str) -> Result<Option<KycSubmissionRecord>> {
        let kyc = sqlx::query_as!(
            KycSubmissionRecord,
            r#"
            SELECT id, user_id, wallet_address, document_hash, full_name, date_of_birth, country,
                   status as "status: KycStatus", verified_by, created_at
            FROM kyc_submissions
            WHERE wallet_address = $1
            ORDER BY created_at DESC
            LIMIT 1
            "#,
            wallet_address
        )
        .fetch_optional(self.pool)
        .await?;

        Ok(kyc)
    }

    /// Get pending KYC submissions (for admin)
    pub async fn find_pending(&self) -> Result<Vec<KycSubmissionRecord>> {
        let kycs = sqlx::query_as!(
            KycSubmissionRecord,
            r#"
            SELECT id, user_id, wallet_address, document_hash, full_name, date_of_birth, country,
                   status as "status: KycStatus", verified_by, created_at
            FROM kyc_submissions
            WHERE status = 'pending'
            ORDER BY created_at ASC
            "#
        )
        .fetch_all(self.pool)
        .await?;

        Ok(kycs)
    }

    /// Get all KYC submissions with pagination (for admin)
    pub async fn find_all(&self, limit: i64, offset: i64) -> Result<Vec<KycSubmissionRecord>> {
        let kycs = sqlx::query_as!(
            KycSubmissionRecord,
            r#"
            SELECT id, user_id, wallet_address, document_hash, full_name, date_of_birth, country,
                   status as "status: KycStatus", verified_by, created_at
            FROM kyc_submissions
            ORDER BY created_at DESC
            LIMIT $1 OFFSET $2
            "#,
            limit,
            offset
        )
        .fetch_all(self.pool)
        .await?;

        Ok(kycs)
    }

    /// Update KYC submission status
    pub async fn update_status(
        &self,
        id: Uuid,
        status: &str,
        verified_by: Option<Uuid>,
    ) -> Result<KycSubmissionRecord> {
        let status_enum: KycStatus = match status {
            "pending" => KycStatus::Pending,
            "verified" => KycStatus::Verified,
            "rejected" => KycStatus::Rejected,
            _ => KycStatus::NotSubmitted,
        };

        let kyc = sqlx::query_as!(
            KycSubmissionRecord,
            r#"
            UPDATE kyc_submissions
            SET status = $1, verified_by = $2, updated_at = NOW()
            WHERE id = $3
            RETURNING id, user_id, wallet_address, document_hash, full_name, date_of_birth, country,
                      status as "status: KycStatus", verified_by, created_at
            "#,
            status_enum as KycStatus,
            verified_by,
            id
        )
        .fetch_one(self.pool)
        .await?;

        Ok(kyc)
    }
}
