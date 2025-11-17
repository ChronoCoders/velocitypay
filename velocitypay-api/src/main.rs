mod config;
mod models;
mod chain;

use actix_web::{web, App, HttpResponse, HttpServer};
use actix_cors::Cors;
use config::Config;
use std::sync::Arc;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init();

    let config = Config::from_env().expect("Failed to load configuration");
    let server_address = config.server_address();

    log::info!("Starting VelocityPay API Gateway");
    log::info!("Connecting to chain at: {}", config.chain_rpc_url);
    log::info!("Database: {}", config.database_url);

    let chain_client = chain::client::connect_to_chain(&config.chain_rpc_url)
        .await
        .expect("Failed to connect to blockchain");

    let chain_client = Arc::new(chain_client);
    let config = Arc::new(config);

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
            ])
            .max_age(3600);

        App::new()
            .wrap(cors)
            .wrap(actix_web::middleware::Logger::default())
            .app_data(web::Data::new(chain_client.clone()))
            .app_data(web::Data::new(config.clone()))
            .route("/health", web::get().to(health_check))
            .service(
                web::scope("/api/v1")
                    .route("/status", web::get().to(api_status))
            )
    })
    .bind(&server_address)?
    .run()
    .await
}

async fn health_check() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "status": "healthy",
        "service": "velocitypay-api"
    }))
}

async fn api_status() -> HttpResponse {
    HttpResponse::Ok().json(serde_json::json!({
        "version": "1.0.0",
        "status": "operational"
    }))
}
