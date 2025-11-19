mod config;
mod models;
mod chain;
mod db;
mod middleware;
mod services;
mod routes;

use actix_web::{web, App, HttpResponse, HttpServer};
use actix_cors::Cors;
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

    // Initialize services
    let auth_service = Arc::new(services::AuthService::new(
        config.jwt_secret.clone(),
        config.jwt_expiration,
    ));
    let payment_service = Arc::new(services::PaymentService::new());
    let mint_service = Arc::new(services::MintService::new());
    let burn_service = Arc::new(services::BurnService::new());
    let kyc_service = Arc::new(services::KYCService::new());

    let chain_client = Arc::new(chain_client);
    let config = Arc::new(config);
    let db_pool = Arc::new(db_pool);

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

        // Create middleware instances
        let auth_middleware = middleware::Auth::new(config.jwt_secret.clone());
        let admin_middleware = middleware::AdminAuth::new(config.admin_api_key.clone());

        App::new()
            .wrap(cors)
            .wrap(actix_web::middleware::Logger::default())
            // Inject shared data
            .app_data(web::Data::new(db_pool.clone()))
            .app_data(web::Data::new(chain_client.clone()))
            .app_data(web::Data::new(config.clone()))
            .app_data(web::Data::new(auth_service.clone()))
            .app_data(web::Data::new(payment_service.clone()))
            .app_data(web::Data::new(mint_service.clone()))
            .app_data(web::Data::new(burn_service.clone()))
            .app_data(web::Data::new(kyc_service.clone()))
            // Health and status endpoints (no auth required)
            .route("/health", web::get().to(health_check))
            .route("/api/v1/status", web::get().to(api_status))
            // Public auth routes (no auth middleware)
            .service(
                web::scope("/api/v1")
                    .configure(routes::auth_routes::configure)
            )
            // Protected routes (require authentication)
            .service(
                web::scope("/api/v1")
                    .wrap(auth_middleware)
                    .configure(routes::payment_routes::configure)
                    .configure(routes::mint_routes::configure)
                    .configure(routes::burn_routes::configure)
                    .configure(routes::kyc_routes::configure)
            )
            // Admin routes (require admin API key)
            .service(
                web::scope("/api/v1")
                    .wrap(admin_middleware)
                    .configure(routes::admin_routes::configure)
            )
    })
    .bind(&server_address)?
    .run()
    .await
}

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "service": "velopay-api"
    }))
}

async fn api_status() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "version": "1.0.0",
        "status": "operational"
    }))
}
