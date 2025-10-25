use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "mint_request_status", rename_all = "lowercase")]
pub enum MintRequestStatus {
    Pending,
    Approved,
    Rejected,
    Completed,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct MintRequest {
    pub id: Uuid,
    pub user_id: Uuid,
    pub wallet_address: String,
    pub amount: String,
    pub bank_reference: String,
    pub status: MintRequestStatus,
    pub chain_request_id: Option<i64>,
    pub approved_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize, Validate)]
pub struct CreateMintRequest {
    #[validate(length(min = 1))]
    pub amount: String,
    #[validate(length(min = 1, max = 256))]
    pub bank_reference: String,
}

#[derive(Debug, Serialize)]
pub struct MintRequestResponse {
    pub id: Uuid,
    pub wallet_address: String,
    pub amount: String,
    pub bank_reference: String,
    pub status: MintRequestStatus,
    pub chain_request_id: Option<i64>,
    pub created_at: DateTime<Utc>,
}

impl From<MintRequest> for MintRequestResponse {
    fn from(req: MintRequest) -> Self {
        Self {
            id: req.id,
            wallet_address: req.wallet_address,
            amount: req.amount,
            bank_reference: req.bank_reference,
            status: req.status,
            chain_request_id: req.chain_request_id,
            created_at: req.created_at,
        }
    }
}
