use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize, sqlx::Type)]
#[sqlx(type_name = "transaction_status", rename_all = "lowercase")]
pub enum TransactionStatus {
    Pending,
    Confirmed,
    Failed,
}

#[derive(Debug, Clone, Serialize, Deserialize, FromRow)]
pub struct Transaction {
    pub id: Uuid,
    pub from_address: String,
    pub to_address: String,
    pub amount: String,
    pub fee: String,
    pub transaction_hash: Option<String>,
    pub block_number: Option<i64>,
    pub status: TransactionStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Deserialize)]
pub struct SendPaymentRequest {
    pub from_address: String,
    pub to_address: String,
    pub amount: String,
}

#[derive(Debug, Serialize)]
pub struct TransactionResponse {
    pub id: Uuid,
    pub from_address: String,
    pub to_address: String,
    pub amount: String,
    pub fee: String,
    pub transaction_hash: Option<String>,
    pub block_number: Option<i64>,
    pub status: TransactionStatus,
    pub created_at: DateTime<Utc>,
}

impl From<Transaction> for TransactionResponse {
    fn from(tx: Transaction) -> Self {
        Self {
            id: tx.id,
            from_address: tx.from_address,
            to_address: tx.to_address,
            amount: tx.amount,
            fee: tx.fee,
            transaction_hash: tx.transaction_hash,
            block_number: tx.block_number,
            status: tx.status,
            created_at: tx.created_at,
        }
    }
}
