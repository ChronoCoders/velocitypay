use crate::db::transaction_repository::{TransactionRepository, TransactionRecord};
use crate::models::response::TransactionResponse;
use crate::models::transaction::TransactionStatus;
use anyhow::{Result, anyhow};
use chrono::Utc;
use rust_decimal::Decimal;
use sqlx::PgPool;
use std::str::FromStr;
use uuid::Uuid;

pub struct PaymentService;

impl PaymentService {
    pub fn new() -> Self {
        Self
    }

    /// Create a payment transaction
    pub async fn send_payment(
        &self,
        pool: &PgPool,
        from_address: &str,
        to_address: &str,
        amount: Decimal,
    ) -> Result<TransactionResponse> {
        let repo = TransactionRepository::new(pool);

        // Validate amount
        if amount <= Decimal::ZERO {
            return Err(anyhow!("Amount must be greater than zero"));
        }

        // Calculate fee (e.g., 0.1% of amount)
        let fee = amount * Decimal::new(1, 3); // 0.001 = 0.1%

        // Create transaction record
        let transaction = repo.create(from_address, to_address, &amount.to_string(), &fee.to_string()).await?;

        Ok(Self::transaction_to_response(transaction))
    }

    /// Get transaction by ID
    pub async fn get_transaction(
        &self,
        pool: &PgPool,
        transaction_id: Uuid,
    ) -> Result<TransactionResponse> {
        let repo = TransactionRepository::new(pool);

        let transaction = repo
            .find_by_id(transaction_id)
            .await?
            .ok_or_else(|| anyhow!("Transaction not found"))?;

        Ok(Self::transaction_to_response(transaction))
    }

    /// Get transaction history for a wallet
    pub async fn get_transaction_history(
        &self,
        pool: &PgPool,
        wallet_address: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<TransactionResponse>> {
        let repo = TransactionRepository::new(pool);

        let transactions = repo.find_by_wallet(wallet_address, limit, offset).await?;

        Ok(transactions
            .into_iter()
            .map(Self::transaction_to_response)
            .collect())
    }

    /// Update transaction with blockchain details (called after chain confirmation)
    pub async fn update_transaction_status(
        &self,
        pool: &PgPool,
        transaction_id: Uuid,
        tx_hash: &str,
        block_number: Option<i64>,
        status: TransactionStatus,
    ) -> Result<TransactionResponse> {
        let repo = TransactionRepository::new(pool);

        let status_str = match status {
            TransactionStatus::Pending => "pending",
            TransactionStatus::Confirmed => "confirmed",
            TransactionStatus::Failed => "failed",
        };

        let transaction = repo
            .update_with_tx_hash(transaction_id, tx_hash, block_number, status_str)
            .await?;

        Ok(Self::transaction_to_response(transaction))
    }

    /// Convert database record to response model
    fn transaction_to_response(record: TransactionRecord) -> TransactionResponse {
        use crate::db::transaction_repository::TransactionStatus as DbStatus;

        let status = match record.status {
            DbStatus::Confirmed => TransactionStatus::Confirmed,
            DbStatus::Failed => TransactionStatus::Failed,
            DbStatus::Pending => TransactionStatus::Pending,
        };

        // Parse amount and fee from string to Decimal
        let amount = Decimal::from_str(&record.amount).unwrap_or(Decimal::ZERO);
        let fee = Decimal::from_str(&record.fee).unwrap_or(Decimal::ZERO);

        TransactionResponse {
            id: record.id,
            from_address: record.from_address,
            to_address: record.to_address,
            amount,
            fee,
            tx_hash: record.transaction_hash,
            block_number: record.block_number,
            status,
            created_at: record.created_at.unwrap_or_else(|| Utc::now()),
        }
    }
}
