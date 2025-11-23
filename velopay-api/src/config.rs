use serde::Deserialize;
use std::env;

// Environment variable name constants
static JWT_SECRET: &str = "JWT_SECRET";
static ADMIN_API_KEY: &str = "ADMIN_API_KEY";
static ADMIN_SEED: &str = "ADMIN_SEED";
static CORS_ALLOWED_ORIGINS: &str = "CORS_ALLOWED_ORIGINS";
static DATABASE_URL: &str = "DATABASE_URL";
static CHAIN_RPC_URL: &str = "CHAIN_RPC_URL";
static SERVER_HOST: &str = "SERVER_HOST";
static SERVER_PORT: &str = "SERVER_PORT";
static JWT_EXPIRATION: &str = "JWT_EXPIRATION";
static RATE_LIMIT_REQUESTS: &str = "RATE_LIMIT_REQUESTS";
static RATE_LIMIT_WINDOW_SECONDS: &str = "RATE_LIMIT_WINDOW_SECONDS";

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub database_url: String,
    pub chain_rpc_url: String,
    pub server_host: String,
    pub server_port: u16,
    pub jwt_secret: String,
    pub jwt_expiration: i64,
    pub admin_api_key: String,
    pub admin_seed: String,
    pub rate_limit_requests: u32,
    pub rate_limit_window_seconds: u64,
    pub cors_allowed_origins: Vec<String>,
}

impl Config {
    pub fn from_env() -> Result<Self, anyhow::Error> {
        dotenv::dotenv().ok();

        // Load JWT secret - MUST be set in production
        let jwt_secret = env::var(JWT_SECRET)
            .map_err(|_| anyhow::anyhow!("{} must be set in environment variables", JWT_SECRET))?;

        // Validate JWT secret strength
        if jwt_secret.len() < 32 {
            return Err(anyhow::anyhow!("{} must be at least 32 characters long", JWT_SECRET));
        }

        // Load admin API key - MUST be set in production
        let admin_api_key = env::var(ADMIN_API_KEY)
            .map_err(|_| anyhow::anyhow!("{} must be set in environment variables", ADMIN_API_KEY))?;

        if admin_api_key.len() < 32 {
            return Err(anyhow::anyhow!("{} must be at least 32 characters long", ADMIN_API_KEY));
        }

        // Load admin seed phrase for blockchain operations - MUST be set in production
        let admin_seed = env::var(ADMIN_SEED)
            .map_err(|_| anyhow::anyhow!("{} must be set in environment variables (e.g., //Alice for dev)", ADMIN_SEED))?;

        // Parse and validate CORS allowed origins
        let cors_allowed_origins_str = env::var(CORS_ALLOWED_ORIGINS)
            .unwrap_or_else(|_| "http://localhost:3000,http://localhost:5173".to_string());

        let cors_allowed_origins: Vec<String> = cors_allowed_origins_str
            .split(',')
            .map(|s| s.trim().to_string())
            .filter(|s| !s.is_empty())
            .collect();

        // Validate CORS origins
        for origin in &cors_allowed_origins {
            // Reject wildcard in production
            if origin == "*" {
                log::warn!("CORS wildcard (*) detected. This is insecure for production!");
            }

            // Validate URL format
            if !origin.starts_with("http://") && !origin.starts_with("https://") {
                return Err(anyhow::anyhow!(
                    "Invalid CORS origin '{}': must start with http:// or https://",
                    origin
                ));
            }
        }

        if cors_allowed_origins.is_empty() {
            return Err(anyhow::anyhow!("At least one CORS origin must be configured"));
        }

        Ok(Config {
            database_url: env::var(DATABASE_URL)
                .unwrap_or_else(|_| "postgresql://localhost/velopay".to_string()),
            chain_rpc_url: env::var(CHAIN_RPC_URL)
                .unwrap_or_else(|_| "ws://127.0.0.1:9944".to_string()),
            server_host: env::var(SERVER_HOST).unwrap_or_else(|_| "127.0.0.1".to_string()),
            server_port: env::var(SERVER_PORT)
                .unwrap_or_else(|_| "8080".to_string())
                .parse()?,
            jwt_secret,
            jwt_expiration: env::var(JWT_EXPIRATION)
                .unwrap_or_else(|_| "86400".to_string())
                .parse()?,
            admin_api_key,
            admin_seed,
            rate_limit_requests: env::var(RATE_LIMIT_REQUESTS)
                .unwrap_or_else(|_| "100".to_string())
                .parse()?,
            rate_limit_window_seconds: env::var(RATE_LIMIT_WINDOW_SECONDS)
                .unwrap_or_else(|_| "60".to_string())
                .parse()?,
            cors_allowed_origins,
        })
    }

    pub fn server_address(&self) -> String {
        format!("{}:{}", self.server_host, self.server_port)
    }
}
