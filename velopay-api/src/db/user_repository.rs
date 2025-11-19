use sqlx::PgPool;
use uuid::Uuid;
use anyhow::Result;
use chrono::{DateTime, Utc};

#[derive(Debug, sqlx::FromRow)]
pub struct UserRecord {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
    pub wallet_address: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

pub struct UserRepository<'a> {
    pool: &'a PgPool,
}

impl<'a> UserRepository<'a> {
    pub fn new(pool: &'a PgPool) -> Self {
        Self { pool }
    }

    /// Create a new user
    pub async fn create(
        &self,
        email: &str,
        password_hash: &str,
        wallet_address: Option<&str>,
    ) -> Result<UserRecord> {
        let user = sqlx::query_as::<_, UserRecord>(
            r#"
            INSERT INTO users (email, password_hash, wallet_address)
            VALUES ($1, $2, $3)
            RETURNING id, email, password_hash, wallet_address, created_at, updated_at
            "#,
        )
        .bind(email)
        .bind(password_hash)
        .bind(wallet_address)
        .fetch_one(self.pool)
        .await?;

        Ok(user)
    }

    /// Find user by email
    pub async fn find_by_email(&self, email: &str) -> Result<Option<UserRecord>> {
        let user = sqlx::query_as::<_, UserRecord>(
            r#"
            SELECT id, email, password_hash, wallet_address, created_at, updated_at
            FROM users
            WHERE email = $1
            "#,
        )
        .bind(email)
        .fetch_optional(self.pool)
        .await?;

        Ok(user)
    }

    /// Find user by ID
    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<UserRecord>> {
        let user = sqlx::query_as::<_, UserRecord>(
            r#"
            SELECT id, email, password_hash, wallet_address, created_at, updated_at
            FROM users
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(self.pool)
        .await?;

        Ok(user)
    }

    /// Find user by wallet address
    pub async fn find_by_wallet(&self, wallet_address: &str) -> Result<Option<UserRecord>> {
        let user = sqlx::query_as::<_, UserRecord>(
            r#"
            SELECT id, email, password_hash, wallet_address, created_at, updated_at
            FROM users
            WHERE wallet_address = $1
            "#,
        )
        .bind(wallet_address)
        .fetch_optional(self.pool)
        .await?;

        Ok(user)
    }

    /// Update user's wallet address
    pub async fn update_wallet_address(
        &self,
        user_id: Uuid,
        wallet_address: &str,
    ) -> Result<UserRecord> {
        let user = sqlx::query_as::<_, UserRecord>(
            r#"
            UPDATE users
            SET wallet_address = $1, updated_at = NOW()
            WHERE id = $2
            RETURNING id, email, password_hash, wallet_address, created_at, updated_at
            "#,
        )
        .bind(wallet_address)
        .bind(user_id)
        .fetch_one(self.pool)
        .await?;

        Ok(user)
    }
}
