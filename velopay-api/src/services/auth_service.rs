use crate::db::user_repository::{UserRepository, UserRecord};
use crate::middleware::auth::Claims;
use crate::models::user::{UserResponse, AuthResponse};
use anyhow::{Result, anyhow};
use bcrypt::{hash, verify, DEFAULT_COST};
use jsonwebtoken::{encode, EncodingKey, Header};
use sqlx::PgPool;
use uuid::Uuid;
use chrono::Utc;

pub struct AuthService {
    jwt_secret: String,
    jwt_expiration: i64,
}

impl AuthService {
    pub fn new(jwt_secret: String, jwt_expiration: i64) -> Self {
        Self {
            jwt_secret,
            jwt_expiration,
        }
    }

    /// Register a new user
    pub async fn register(
        &self,
        pool: &PgPool,
        email: &str,
        password: &str,
        wallet_address: Option<&str>,
    ) -> Result<AuthResponse> {
        let repo = UserRepository::new(pool);

        // Check if user already exists
        if repo.find_by_email(email).await?.is_some() {
            return Err(anyhow!("User with this email already exists"));
        }

        // Check if wallet address is already taken
        if let Some(wallet) = wallet_address {
            if repo.find_by_wallet(wallet).await?.is_some() {
                return Err(anyhow!("Wallet address already registered"));
            }
        }

        // Hash password
        let password_hash = hash(password, DEFAULT_COST)?;

        // Create user
        let user = repo.create(email, &password_hash, wallet_address).await?;

        // Generate JWT token
        let token = self.generate_token(&user)?;

        Ok(AuthResponse {
            user: UserResponse {
                id: user.id,
                email: user.email,
                wallet_address: user.wallet_address,
                created_at: user.created_at,
                updated_at: user.updated_at,
            },
            token,
        })
    }

    /// Login user
    pub async fn login(
        &self,
        pool: &PgPool,
        email: &str,
        password: &str,
    ) -> Result<AuthResponse> {
        let repo = UserRepository::new(pool);

        // Find user by email
        let user = repo
            .find_by_email(email)
            .await?
            .ok_or_else(|| anyhow!("Invalid email or password"))?;

        // Verify password
        if !verify(password, &user.password_hash)? {
            return Err(anyhow!("Invalid email or password"));
        }

        // Generate JWT token
        let token = self.generate_token(&user)?;

        Ok(AuthResponse {
            user: UserResponse {
                id: user.id,
                email: user.email.clone(),
                wallet_address: user.wallet_address.clone(),
                created_at: user.created_at,
                updated_at: user.updated_at,
            },
            token,
        })
    }

    /// Get user profile
    pub async fn get_profile(&self, pool: &PgPool, user_id: Uuid) -> Result<UserResponse> {
        let repo = UserRepository::new(pool);

        let user = repo
            .find_by_id(user_id)
            .await?
            .ok_or_else(|| anyhow!("User not found"))?;

        Ok(UserResponse {
            id: user.id,
            email: user.email,
            wallet_address: user.wallet_address,
            created_at: user.created_at,
            updated_at: user.updated_at,
        })
    }

    /// Update user's wallet address
    pub async fn update_wallet(
        &self,
        pool: &PgPool,
        user_id: Uuid,
        wallet_address: &str,
    ) -> Result<UserResponse> {
        let repo = UserRepository::new(pool);

        // Check if wallet address is already taken by another user
        if let Some(existing_user) = repo.find_by_wallet(wallet_address).await? {
            if existing_user.id != user_id {
                return Err(anyhow!("Wallet address already registered to another user"));
            }
        }

        let user = repo.update_wallet_address(user_id, wallet_address).await?;

        Ok(UserResponse {
            id: user.id,
            email: user.email,
            wallet_address: user.wallet_address,
            created_at: user.created_at,
            updated_at: user.updated_at,
        })
    }

    /// Generate JWT token for user
    fn generate_token(&self, user: &UserRecord) -> Result<String> {
        let expiration = Utc::now()
            .checked_add_signed(chrono::Duration::seconds(self.jwt_expiration))
            .ok_or_else(|| anyhow!("Failed to calculate token expiration"))?
            .timestamp() as usize;

        let claims = Claims {
            sub: user.id.to_string(),
            email: user.email.clone(),
            exp: expiration,
        };

        let token = encode(
            &Header::default(),
            &claims,
            &EncodingKey::from_secret(self.jwt_secret.as_bytes()),
        )?;

        Ok(token)
    }
}
