use actix_web::{web, HttpRequest, HttpResponse, Result};
use rust_decimal::Decimal;
use serde_json::json;
use sqlx::PgPool;
use std::str::FromStr;
use uuid::Uuid;

use crate::middleware::auth::get_user_id;
use crate::models::mint_request::CreateMintRequest;
use crate::services::MintService;

/// Create a mint request (requires authentication)
async fn create_mint_request(
    pool: web::Data<PgPool>,
    mint_service: web::Data<std::sync::Arc<MintService>>,
    req: HttpRequest,
    mint_req: web::Json<CreateMintRequest>,
) -> Result<HttpResponse> {
    let user_id = get_user_id(&req)?;

    // Get wallet address from user profile (in production, fetch from DB)
    // For now, require it in the request
    let wallet_address = mint_req
        .0
        .wallet_address
        .as_ref()
        .ok_or_else(|| actix_web::error::ErrorBadRequest("wallet_address is required"))?;

    // Parse amount string to Decimal
    let amount = Decimal::from_str(&mint_req.amount)
        .map_err(|_| actix_web::error::ErrorBadRequest("Invalid amount format"))?;

    match mint_service
        .create_mint_request(
            pool.get_ref(),
            user_id,
            wallet_address,
            amount,
            &mint_req.bank_reference,
        )
        .await
    {
        Ok(mint_request) => Ok(HttpResponse::Created().json(mint_request)),
        Err(e) => Ok(HttpResponse::BadRequest().json(json!({
            "error": e.to_string()
        }))),
    }
}

/// Get mint request by ID (requires authentication)
async fn get_mint_request(
    pool: web::Data<PgPool>,
    mint_service: web::Data<std::sync::Arc<MintService>>,
    req: HttpRequest,
    request_id: web::Path<Uuid>,
) -> Result<HttpResponse> {
    let _user_id = get_user_id(&req)?; // Verify authentication

    match mint_service
        .get_mint_request(pool.get_ref(), *request_id)
        .await
    {
        Ok(mint_request) => Ok(HttpResponse::Ok().json(mint_request)),
        Err(e) => Ok(HttpResponse::NotFound().json(json!({
            "error": e.to_string()
        }))),
    }
}

/// Get all mint requests for current user (requires authentication)
async fn get_my_mint_requests(
    pool: web::Data<PgPool>,
    mint_service: web::Data<std::sync::Arc<MintService>>,
    req: HttpRequest,
) -> Result<HttpResponse> {
    let user_id = get_user_id(&req)?;

    match mint_service
        .get_user_mint_requests(pool.get_ref(), user_id)
        .await
    {
        Ok(mint_requests) => Ok(HttpResponse::Ok().json(mint_requests)),
        Err(e) => Ok(HttpResponse::InternalServerError().json(json!({
            "error": e.to_string()
        }))),
    }
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/mint")
            .route("", web::post().to(create_mint_request))
            .route("/my-requests", web::get().to(get_my_mint_requests))
            .route("/{request_id}", web::get().to(get_mint_request)),
    );
}
