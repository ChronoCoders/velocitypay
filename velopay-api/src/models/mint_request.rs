use serde::{Deserialize, Serialize};
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "mint_request_status", rename_all = "lowercase")]
pub enum MintRequestStatus {
    Pending,
    Approved,
    Rejected,
    Completed,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateMintRequest {
    pub wallet_address: Option<String>,
    #[validate(length(min = 1))]
    pub amount: String,
    #[validate(length(min = 1, max = 256))]
    pub bank_reference: String,
}
