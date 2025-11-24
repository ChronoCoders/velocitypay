use actix_web::{web, HttpRequest, HttpResponse, Result};
use rust_decimal::Decimal;
use serde_json::json;
use sqlx::PgPool;
use std::str::FromStr;
use uuid::Uuid;

use crate::middleware::auth::get_user_id;
use crate::models::burn_request::CreateBurnRequest;
use crate::services::BurnService;

/// Create a burn request (requires authentication)
async fn create_burn_request(
    pool: web::Data<PgPool>,
    burn_service: web::Data<std::sync::Arc<BurnService>>,
    req: HttpRequest,
    burn_req: web::Json<CreateBurnRequest>,
) -> Result<HttpResponse> {
    let user_id = get_user_id(&req)?;

    // Get wallet address from user profile (in production, fetch from DB)
    // For now, require it in the request
    let wallet_address = burn_req
        .0
        .wallet_address
        .as_ref()
        .ok_or_else(|| actix_web::error::ErrorBadRequest("wallet_address is required"))?;

    // Parse amount string to Decimal
    let amount = Decimal::from_str(&burn_req.amount)
        .map_err(|_| actix_web::error::ErrorBadRequest("Invalid amount format"))?;

    match burn_service
        .create_burn_request(
            pool.get_ref(),
            user_id,
            wallet_address,
            amount,
            &burn_req.bank_account,
        )
        .await
    {
        Ok(burn_request) => Ok(HttpResponse::Created().json(burn_request)),
        Err(e) => Ok(HttpResponse::BadRequest().json(json!({
            "error": e.to_string()
        }))),
    }
}

/// Get burn request by ID (requires authentication)
async fn get_burn_request(
    pool: web::Data<PgPool>,
    burn_service: web::Data<std::sync::Arc<BurnService>>,
    req: HttpRequest,
    request_id: web::Path<Uuid>,
) -> Result<HttpResponse> {
    let _user_id = get_user_id(&req)?; // Verify authentication

    match burn_service
        .get_burn_request(pool.get_ref(), *request_id)
        .await
    {
        Ok(burn_request) => Ok(HttpResponse::Ok().json(burn_request)),
        Err(e) => Ok(HttpResponse::NotFound().json(json!({
            "error": e.to_string()
        }))),
    }
}

/// Get all burn requests for current user (requires authentication)
async fn get_my_burn_requests(
    pool: web::Data<PgPool>,
    burn_service: web::Data<std::sync::Arc<BurnService>>,
    req: HttpRequest,
) -> Result<HttpResponse> {
    let user_id = get_user_id(&req)?;

    match burn_service
        .get_user_burn_requests(pool.get_ref(), user_id)
        .await
    {
        Ok(burn_requests) => Ok(HttpResponse::Ok().json(burn_requests)),
        Err(e) => Ok(HttpResponse::InternalServerError().json(json!({
            "error": e.to_string()
        }))),
    }
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/burn")
            .route("", web::post().to(create_burn_request))
            .route("/my-requests", web::get().to(get_my_burn_requests))
            .route("/{request_id}", web::get().to(get_burn_request)),
    );
}
