use sqlx::PgPool;
use uuid::Uuid;
use anyhow::Result;
use chrono::{DateTime, Utc};
use rust_decimal::Decimal;

#[derive(Debug, sqlx::FromRow)]
pub struct TransactionRecord {
    pub id: Uuid,
    pub from_address: String,
    pub to_address: String,
    pub amount: Decimal,
    pub fee: Decimal,
    pub tx_hash: Option<String>,
    pub block_number: Option<i64>,
    pub status: String,
    pub created_at: DateTime<Utc>,
}

pub struct TransactionRepository<'a> {
    pool: &'a PgPool,
}

impl<'a> TransactionRepository<'a> {
    pub fn new(pool: &'a PgPool) -> Self {
        Self { pool }
    }

    /// Create a new transaction
    pub async fn create(
        &self,
        from_address: &str,
        to_address: &str,
        amount: Decimal,
        fee: Decimal,
    ) -> Result<TransactionRecord> {
        let transaction = sqlx::query_as::<_, TransactionRecord>(
            r#"
            INSERT INTO transactions (from_address, to_address, amount, fee, status)
            VALUES ($1, $2, $3, $4, 'pending')
            RETURNING id, from_address, to_address, amount, fee, tx_hash, block_number, status, created_at
            "#,
        )
        .bind(from_address)
        .bind(to_address)
        .bind(amount)
        .bind(fee)
        .fetch_one(self.pool)
        .await?;

        Ok(transaction)
    }

    /// Update transaction with blockchain details
    pub async fn update_with_tx_hash(
        &self,
        id: Uuid,
        tx_hash: &str,
        block_number: Option<i64>,
        status: &str,
    ) -> Result<TransactionRecord> {
        let transaction = sqlx::query_as::<_, TransactionRecord>(
            r#"
            UPDATE transactions
            SET tx_hash = $1, block_number = $2, status = $3
            WHERE id = $4
            RETURNING id, from_address, to_address, amount, fee, tx_hash, block_number, status, created_at
            "#,
        )
        .bind(tx_hash)
        .bind(block_number)
        .bind(status)
        .bind(id)
        .fetch_one(self.pool)
        .await?;

        Ok(transaction)
    }

    /// Get transactions by wallet address (sent or received)
    pub async fn find_by_wallet(
        &self,
        wallet_address: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<TransactionRecord>> {
        let transactions = sqlx::query_as::<_, TransactionRecord>(
            r#"
            SELECT id, from_address, to_address, amount, fee, tx_hash, block_number, status, created_at
            FROM transactions
            WHERE from_address = $1 OR to_address = $1
            ORDER BY created_at DESC
            LIMIT $2 OFFSET $3
            "#,
        )
        .bind(wallet_address)
        .bind(limit)
        .bind(offset)
        .fetch_all(self.pool)
        .await?;

        Ok(transactions)
    }

    /// Get transaction by ID
    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<TransactionRecord>> {
        let transaction = sqlx::query_as::<_, TransactionRecord>(
            r#"
            SELECT id, from_address, to_address, amount, fee, tx_hash, block_number, status, created_at
            FROM transactions
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(self.pool)
        .await?;

        Ok(transaction)
    }

    /// Get transaction by tx_hash
    pub async fn find_by_tx_hash(&self, tx_hash: &str) -> Result<Option<TransactionRecord>> {
        let transaction = sqlx::query_as::<_, TransactionRecord>(
            r#"
            SELECT id, from_address, to_address, amount, fee, tx_hash, block_number, status, created_at
            FROM transactions
            WHERE tx_hash = $1
            "#,
        )
        .bind(tx_hash)
        .fetch_optional(self.pool)
        .await?;

        Ok(transaction)
    }
}
