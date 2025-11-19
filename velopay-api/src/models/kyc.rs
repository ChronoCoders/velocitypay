use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "kyc_status", rename_all = "lowercase")]
pub enum KYCStatus {
    NotSubmitted,
    Pending,
    Verified,
    Rejected,
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
