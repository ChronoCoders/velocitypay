use sqlx::PgPool;
use uuid::Uuid;
use chrono::{DateTime, Utc};
use anyhow::Result;

#[derive(Debug, Clone, sqlx::Type)]
#[sqlx(type_name = "transaction_status", rename_all = "lowercase")]
pub enum TransactionStatus {
    Pending,
    Confirmed,
    Failed,
}

#[derive(Debug, Clone)]
pub struct TransactionRecord {
    pub id: Uuid,
    pub from_address: String,
    pub to_address: String,
    pub amount: String,
    pub fee: String,
    pub transaction_hash: Option<String>,
    pub block_number: Option<i64>,
    pub status: TransactionStatus,
    pub created_at: Option<DateTime<Utc>>,
    #[allow(dead_code)]
    pub updated_at: Option<DateTime<Utc>>,
}

pub struct TransactionRepository<'a> {
    pool: &'a PgPool,
}

impl<'a> TransactionRepository<'a> {
    pub fn new(pool: &'a PgPool) -> Self {
        Self { pool }
    }

    /// Create a new transaction record
    pub async fn create(
        &self,
        from_address: &str,
        to_address: &str,
        amount: &str,
        fee: &str,
    ) -> Result<TransactionRecord> {
        let tx = sqlx::query_as!(
            TransactionRecord,
            r#"
            INSERT INTO transactions (from_address, to_address, amount, fee, status)
            VALUES ($1, $2, $3, $4, 'pending')
            RETURNING id, from_address, to_address, amount, fee, transaction_hash,
                      block_number, status as "status: TransactionStatus", created_at, updated_at
            "#,
            from_address,
            to_address,
            amount,
            fee
        )
        .fetch_one(self.pool)
        .await?;

        Ok(tx)
    }

    /// Update transaction with blockchain details
    #[allow(dead_code)]
    pub async fn update_confirmed(
        &self,
        id: Uuid,
        transaction_hash: &str,
        block_number: i64,
    ) -> Result<TransactionRecord> {
        let tx = sqlx::query_as!(
            TransactionRecord,
            r#"
            UPDATE transactions
            SET transaction_hash = $1, block_number = $2, status = 'confirmed', updated_at = NOW()
            WHERE id = $3
            RETURNING id, from_address, to_address, amount, fee, transaction_hash,
                      block_number, status as "status: TransactionStatus", created_at, updated_at
            "#,
            transaction_hash,
            block_number,
            id
        )
        .fetch_one(self.pool)
        .await?;

        Ok(tx)
    }

    /// Mark transaction as failed
    #[allow(dead_code)]
    pub async fn mark_failed(&self, id: Uuid) -> Result<TransactionRecord> {
        let tx = sqlx::query_as!(
            TransactionRecord,
            r#"
            UPDATE transactions
            SET status = 'failed', updated_at = NOW()
            WHERE id = $1
            RETURNING id, from_address, to_address, amount, fee, transaction_hash,
                      block_number, status as "status: TransactionStatus", created_at, updated_at
            "#,
            id
        )
        .fetch_one(self.pool)
        .await?;

        Ok(tx)
    }

    /// Find transaction by ID
    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<TransactionRecord>> {
        let tx = sqlx::query_as!(
            TransactionRecord,
            r#"
            SELECT id, from_address, to_address, amount, fee, transaction_hash,
                   block_number, status as "status: TransactionStatus", created_at, updated_at
            FROM transactions
            WHERE id = $1
            "#,
            id
        )
        .fetch_optional(self.pool)
        .await?;

        Ok(tx)
    }

    /// Find transaction by hash
    pub async fn find_by_hash(&self, hash: &str) -> Result<Option<TransactionRecord>> {
        let tx = sqlx::query_as!(
            TransactionRecord,
            r#"
            SELECT id, from_address, to_address, amount, fee, transaction_hash,
                   block_number, status as "status: TransactionStatus", created_at, updated_at
            FROM transactions
            WHERE transaction_hash = $1
            "#,
            hash
        )
        .fetch_optional(self.pool)
        .await?;

        Ok(tx)
    }

    /// Get transactions for an address
    pub async fn find_by_address(
        &self,
        address: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<TransactionRecord>> {
        let txs = sqlx::query_as!(
            TransactionRecord,
            r#"
            SELECT id, from_address, to_address, amount, fee, transaction_hash,
                   block_number, status as "status: TransactionStatus", created_at, updated_at
            FROM transactions
            WHERE from_address = $1 OR to_address = $1
            ORDER BY created_at DESC
            LIMIT $2 OFFSET $3
            "#,
            address,
            limit,
            offset
        )
        .fetch_all(self.pool)
        .await?;

        Ok(txs)
    }

    /// Get transactions for a wallet (alias for find_by_address)
    pub async fn find_by_wallet(
        &self,
        wallet_address: &str,
        limit: i64,
        offset: i64,
    ) -> Result<Vec<TransactionRecord>> {
        self.find_by_address(wallet_address, limit, offset).await
    }

    /// Find transaction by blockchain hash (alias for find_by_hash)
    pub async fn find_by_tx_hash(&self, hash: &str) -> Result<Option<TransactionRecord>> {
        self.find_by_hash(hash).await
    }

    /// Update transaction with hash, block number, and status
    pub async fn update_with_tx_hash(
        &self,
        id: Uuid,
        transaction_hash: &str,
        block_number: Option<i64>,
        status: &str,
    ) -> Result<TransactionRecord> {
        let status_enum: TransactionStatus = match status {
            "confirmed" => TransactionStatus::Confirmed,
            "failed" => TransactionStatus::Failed,
            _ => TransactionStatus::Pending,
        };

        let tx = sqlx::query_as!(
            TransactionRecord,
            r#"
            UPDATE transactions
            SET transaction_hash = $1, block_number = $2, status = $3, updated_at = NOW()
            WHERE id = $4
            RETURNING id, from_address, to_address, amount, fee, transaction_hash,
                      block_number, status as "status: TransactionStatus", created_at, updated_at
            "#,
            transaction_hash,
            block_number,
            status_enum as TransactionStatus,
            id
        )
        .fetch_one(self.pool)
        .await?;

        Ok(tx)
    }
}
