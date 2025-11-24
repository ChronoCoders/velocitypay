mod config;
mod models;
mod chain;
mod db;
mod middleware;
mod services;
mod routes;
mod utils;

use actix_web::{web, App, HttpResponse, HttpServer};
use actix_cors::Cors;
use actix_governor::{Governor, GovernorConfigBuilder};
use config::Config;
use std::sync::Arc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let config = Config::from_env().expect("Failed to load configuration");
    let server_address = config.server_address();

    log::info!("Starting VeloPay API Gateway");
    log::info!("Connecting to chain at: {}", config.chain_rpc_url);
    log::info!("Database: {}", config.database_url);

    // Initialize database connection pool
    let db_pool = db::create_pool(&config.database_url)
        .await
        .expect("Failed to create database pool");

    // Run database migrations
    db::run_migrations(&db_pool)
        .await
        .expect("Failed to run database migrations");

    // Connect to blockchain
    let chain_client = chain::client::connect_to_chain(&config.chain_rpc_url)
        .await
        .expect("Failed to connect to blockchain");

    // Initialize blockchain operations signer
    let chain_ops = chain::operations::ChainOperations::new(&config.admin_seed)
        .expect("Failed to create blockchain operations signer");

    log::info!("Blockchain operations initialized with admin account");

    // Initialize services
    let auth_service = Arc::new(services::auth_service::AuthService::new(
        config.jwt_secret.clone(),
        config.jwt_expiration,
    ));
    let payment_service = Arc::new(services::PaymentService::new());
    let mint_service = Arc::new(services::MintService::new());
    let burn_service = Arc::new(services::BurnService::new());
    let kyc_service = Arc::new(services::KYCService::new());

    // ChainOperations must be wrapped in Arc as it contains non-cloneable cryptographic keys
    let chain_ops = Arc::new(chain_ops);

    log::info!("Server starting on http://{}", server_address);

    HttpServer::new(move || {
        // Configure CORS with whitelisted origins only
        let mut cors = Cors::default();
        for origin in &config.cors_allowed_origins {
            cors = cors.allowed_origin(origin);
        }
        cors = cors
            .allowed_methods(vec!["GET", "POST", "PUT", "DELETE", "OPTIONS"])
            .allowed_headers(vec![
                actix_web::http::header::AUTHORIZATION,
                actix_web::http::header::CONTENT_TYPE,
                actix_web::http::header::ACCEPT,
                actix_web::http::header::HeaderName::from_static("x-admin-api-key"),
            ])
            .max_age(3600);

        // Configure rate limiting - general endpoints
        let governor_conf = GovernorConfigBuilder::default()
            .per_second(config.rate_limit_window_seconds)
            .burst_size(config.rate_limit_requests)
            .finish()
            .expect("Failed to create rate limiter configuration");

        // Configure stricter rate limiting for auth endpoints (5 requests per minute)
        let _auth_governor_conf = GovernorConfigBuilder::default()
            .per_second(60) // 1 minute window
            .burst_size(5)  // 5 requests per minute
            .finish()
            .expect("Failed to create auth rate limiter configuration");

        // Create middleware instances
        let auth_middleware = middleware::Auth::new(config.jwt_secret.clone());
        let admin_middleware = middleware::AdminAuth::new(config.admin_api_key.clone());

        App::new()
            .wrap(cors)
            .wrap(actix_web::middleware::Logger::default())
            // Inject shared data
            .app_data(web::Data::new(db_pool.clone()))
            .app_data(web::Data::new(chain_client.clone()))
            .app_data(web::Data::new(chain_ops.clone()))
            .app_data(web::Data::new(config.clone()))
            .app_data(web::Data::new(auth_service.clone()))
            .app_data(web::Data::new(payment_service.clone()))
            .app_data(web::Data::new(mint_service.clone()))
            .app_data(web::Data::new(burn_service.clone()))
            .app_data(web::Data::new(kyc_service.clone()))
            // Health and status endpoints (no auth required)
            .route("/health", web::get().to(health_check))
            .route("/api/v1/status", web::get().to(api_status))
            // Public auth routes with strict rate limiting
            .configure(routes::auth_routes::configure)
            // Protected routes (require authentication) with general rate limiting
            .service(
                web::scope("/api/v1")
                    .wrap(Governor::new(&governor_conf))
                    .wrap(auth_middleware)
                    .configure(routes::payment_routes::configure)
                    .configure(routes::mint_routes::configure)
                    .configure(routes::burn_routes::configure)
                    .configure(routes::kyc_routes::configure)
            )
            // Admin routes (require admin API key) with general rate limiting
            .service(
                web::scope("/admin/v1")
                    .wrap(Governor::new(&governor_conf))
                    .wrap(admin_middleware)
                    .configure(routes::admin_routes::configure)
            )
    })
    .bind(&server_address)?
    .run()
    .await
}

async fn health_check(
    db_pool: web::Data<sqlx::PgPool>,
    chain_client: web::Data<chain::client::VelocityClient>,
    config: web::Data<Config>,
) -> HttpResponse {
    let mut checks = serde_json::Map::new();
    let mut all_healthy = true;

    // Check database connection
    match sqlx::query("SELECT 1").fetch_one(db_pool.get_ref()).await {
        Ok(_) => {
            checks.insert("database".to_string(), serde_json::json!({
                "status": "healthy",
                "message": "Database connection successful"
            }));
        }
        Err(e) => {
            all_healthy = false;
            checks.insert("database".to_string(), serde_json::json!({
                "status": "unhealthy",
                "message": format!("Database connection failed: {}", e)
            }));
        }
    }

    // Check blockchain connection by getting genesis hash
    let genesis_hash = chain_client.get_ref().genesis_hash();
    checks.insert("blockchain".to_string(), serde_json::json!({
        "status": "healthy",
        "genesis_hash": format!("{:?}", genesis_hash)
    }));

    // Check critical configuration
    let config_healthy = config.jwt_secret.len() >= 32 && config.admin_api_key.len() >= 32;
    checks.insert("configuration".to_string(), serde_json::json!({
        "status": if config_healthy { "healthy" } else { "unhealthy" },
        "message": if config_healthy { "Configuration valid" } else { "Invalid configuration" }
    }));

    if !config_healthy {
        all_healthy = false;
    }

    let status_code = if all_healthy {
        actix_web::http::StatusCode::OK
    } else {
        actix_web::http::StatusCode::SERVICE_UNAVAILABLE
    };

    HttpResponse::build(status_code).json(serde_json::json!({
        "status": if all_healthy { "healthy" } else { "unhealthy" },
        "service": "velopay-api",
        "checks": checks
    }))
}

async fn api_status() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "version": "1.0.0",
        "status": "operational"
    }))
}