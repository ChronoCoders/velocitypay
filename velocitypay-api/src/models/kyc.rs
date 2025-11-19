use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "kyc_status", rename_all = "lowercase")]
pub enum KYCStatus {
    NotSubmitted,
    Pending,
    Verified,
    Rejected,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct KYCSubmission {
    pub id: Uuid,
    pub user_id: Uuid,
    pub wallet_address: String,
    pub document_hash: String,
    pub full_name: String,
    pub date_of_birth: DateTime<Utc>,
    pub country: String,
    pub status: KYCStatus,
    pub verified_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct SubmitKYCRequest {
    pub wallet_address: Option<String>,
    pub document_hash: String,
    #[validate(length(min = 1))]
    pub full_name: String,
    pub date_of_birth: String, // Format: YYYY-MM-DD
    #[validate(length(min = 2, max = 2))]
    pub country: String,
}

#[derive(Debug, Serialize)]
pub struct KYCResponse {
    pub id: Uuid,
    pub wallet_address: String,
    pub status: KYCStatus,
    pub created_at: DateTime<Utc>,
}

impl From<KYCSubmission> for KYCResponse {
    fn from(kyc: KYCSubmission) -> Self {
        Self {
            id: kyc.id,
            wallet_address: kyc.wallet_address,
            status: kyc.status,
            created_at: kyc.created_at,
        }
    }
}
