use sqlx::PgPool;
use uuid::Uuid;
use anyhow::Result;
use chrono::{DateTime, Utc, NaiveDate};

#[derive(Debug, sqlx::FromRow)]
pub struct KYCSubmissionRecord {
    pub id: Uuid,
    pub user_id: Uuid,
    pub wallet_address: String,
    pub document_hash: String,
    pub full_name: String,
    pub date_of_birth: NaiveDate,
    pub country: String,
    pub status: String,
    pub verified_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}

pub struct KYCRepository<'a> {
    pool: &'a PgPool,
}

impl<'a> KYCRepository<'a> {
    pub fn new(pool: &'a PgPool) -> Self {
        Self { pool }
    }

    /// Create a new KYC submission
    pub async fn create(
        &self,
        user_id: Uuid,
        wallet_address: &str,
        document_hash: &str,
        full_name: &str,
        date_of_birth: NaiveDate,
        country: &str,
    ) -> Result<KYCSubmissionRecord> {
        let kyc_submission = sqlx::query_as::<_, KYCSubmissionRecord>(
            r#"
            INSERT INTO kyc_submissions (user_id, wallet_address, document_hash, full_name, date_of_birth, country, status)
            VALUES ($1, $2, $3, $4, $5, $6, 'pending')
            RETURNING id, user_id, wallet_address, document_hash, full_name, date_of_birth, country, status, verified_by, created_at
            "#,
        )
        .bind(user_id)
        .bind(wallet_address)
        .bind(document_hash)
        .bind(full_name)
        .bind(date_of_birth)
        .bind(country)
        .fetch_one(self.pool)
        .await?;

        Ok(kyc_submission)
    }

    /// Update KYC submission status
    pub async fn update_status(
        &self,
        id: Uuid,
        status: &str,
        verified_by: Option<Uuid>,
    ) -> Result<KYCSubmissionRecord> {
        let kyc_submission = sqlx::query_as::<_, KYCSubmissionRecord>(
            r#"
            UPDATE kyc_submissions
            SET status = $1, verified_by = $2
            WHERE id = $3
            RETURNING id, user_id, wallet_address, document_hash, full_name, date_of_birth, country, status, verified_by, created_at
            "#,
        )
        .bind(status)
        .bind(verified_by)
        .bind(id)
        .fetch_one(self.pool)
        .await?;

        Ok(kyc_submission)
    }

    /// Get KYC submission by ID
    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<KYCSubmissionRecord>> {
        let kyc_submission = sqlx::query_as::<_, KYCSubmissionRecord>(
            r#"
            SELECT id, user_id, wallet_address, document_hash, full_name, date_of_birth, country, status, verified_by, created_at
            FROM kyc_submissions
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(self.pool)
        .await?;

        Ok(kyc_submission)
    }

    /// Get KYC submission by user ID
    pub async fn find_by_user_id(&self, user_id: Uuid) -> Result<Option<KYCSubmissionRecord>> {
        let kyc_submission = sqlx::query_as::<_, KYCSubmissionRecord>(
            r#"
            SELECT id, user_id, wallet_address, document_hash, full_name, date_of_birth, country, status, verified_by, created_at
            FROM kyc_submissions
            WHERE user_id = $1
            ORDER BY created_at DESC
            LIMIT 1
            "#,
        )
        .bind(user_id)
        .fetch_optional(self.pool)
        .await?;

        Ok(kyc_submission)
    }

    /// Get KYC submission by wallet address
    pub async fn find_by_wallet(&self, wallet_address: &str) -> Result<Option<KYCSubmissionRecord>> {
        let kyc_submission = sqlx::query_as::<_, KYCSubmissionRecord>(
            r#"
            SELECT id, user_id, wallet_address, document_hash, full_name, date_of_birth, country, status, verified_by, created_at
            FROM kyc_submissions
            WHERE wallet_address = $1
            ORDER BY created_at DESC
            LIMIT 1
            "#,
        )
        .bind(wallet_address)
        .fetch_optional(self.pool)
        .await?;

        Ok(kyc_submission)
    }

    /// Get all pending KYC submissions (for admin)
    pub async fn find_pending(&self) -> Result<Vec<KYCSubmissionRecord>> {
        let kyc_submissions = sqlx::query_as::<_, KYCSubmissionRecord>(
            r#"
            SELECT id, user_id, wallet_address, document_hash, full_name, date_of_birth, country, status, verified_by, created_at
            FROM kyc_submissions
            WHERE status = 'pending'
            ORDER BY created_at ASC
            "#,
        )
        .fetch_all(self.pool)
        .await?;

        Ok(kyc_submissions)
    }

    /// Get all KYC submissions (for admin)
    pub async fn find_all(&self, limit: i64, offset: i64) -> Result<Vec<KYCSubmissionRecord>> {
        let kyc_submissions = sqlx::query_as::<_, KYCSubmissionRecord>(
            r#"
            SELECT id, user_id, wallet_address, document_hash, full_name, date_of_birth, country, status, verified_by, created_at
            FROM kyc_submissions
            ORDER BY created_at DESC
            LIMIT $1 OFFSET $2
            "#,
        )
        .bind(limit)
        .bind(offset)
        .fetch_all(self.pool)
        .await?;

        Ok(kyc_submissions)
    }
}
