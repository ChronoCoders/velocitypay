use chrono::{DateTime, Utc, NaiveDate};
use rust_decimal::Decimal;
use serde::Serialize;
use uuid::Uuid;

use super::transaction::TransactionStatus;
use super::mint_request::MintRequestStatus;
use super::burn_request::BurnRequestStatus;
use super::kyc::KYCStatus;

// Transaction Response
#[derive(Debug, Serialize)]
pub struct TransactionResponse {
    pub id: Uuid,
    pub from_address: String,
    pub to_address: String,
    pub amount: Decimal,
    pub fee: Decimal,
    pub tx_hash: Option<String>,
    pub block_number: Option<i64>,
    pub status: TransactionStatus,
    pub created_at: DateTime<Utc>,
}

// Mint Request Response
#[derive(Debug, Serialize)]
pub struct MintRequestResponse {
    pub id: Uuid,
    pub user_id: Uuid,
    pub wallet_address: String,
    pub amount: Decimal,
    pub bank_reference: String,
    pub status: MintRequestStatus,
    pub chain_request_id: Option<i64>,
    pub approved_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}

// Burn Request Response
#[derive(Debug, Serialize)]
pub struct BurnRequestResponse {
    pub id: Uuid,
    pub user_id: Uuid,
    pub wallet_address: String,
    pub amount: Decimal,
    pub bank_account: String,
    pub status: BurnRequestStatus,
    pub chain_request_id: Option<i64>,
    pub approved_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}

// KYC Submission Response
#[derive(Debug, Serialize)]
pub struct KYCSubmissionResponse {
    pub id: Uuid,
    pub user_id: Uuid,
    pub wallet_address: String,
    pub document_hash: String,
    pub full_name: String,
    pub date_of_birth: NaiveDate,
    pub country: String,
    pub status: KYCStatus,
    pub verified_by: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}
