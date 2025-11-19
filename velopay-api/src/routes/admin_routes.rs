use actix_web::{web, HttpResponse, Result};
use serde::{Deserialize, Serialize};
use serde_json::json;
use sqlx::PgPool;
use uuid::Uuid;

use crate::services::{MintService, BurnService, KYCService};

#[derive(Debug, Deserialize)]
struct PaginationQuery {
    limit: Option<i64>,
    offset: Option<i64>,
}

#[derive(Debug, Deserialize)]
struct ApproveRequest {
    admin_id: Uuid,
    chain_request_id: Option<i64>,
}

#[derive(Debug, Deserialize)]
struct RejectRequest {
    admin_id: Uuid,
}

// ===== MINT ADMIN ENDPOINTS =====

/// Get all pending mint requests (admin only)
async fn get_pending_mint_requests(
    pool: web::Data<PgPool>,
    mint_service: web::Data<MintService>,
) -> Result<HttpResponse> {
    match mint_service.get_pending_mint_requests(pool.get_ref()).await {
        Ok(requests) => Ok(HttpResponse::Ok().json(requests)),
        Err(e) => Ok(HttpResponse::InternalServerError().json(json!({
            "error": e.to_string()
        }))),
    }
}

/// Get all mint requests with pagination (admin only)
async fn get_all_mint_requests(
    pool: web::Data<PgPool>,
    mint_service: web::Data<MintService>,
    query: web::Query<PaginationQuery>,
) -> Result<HttpResponse> {
    let limit = query.limit.unwrap_or(50).min(100);
    let offset = query.offset.unwrap_or(0);

    match mint_service
        .get_all_mint_requests(pool.get_ref(), limit, offset)
        .await
    {
        Ok(requests) => Ok(HttpResponse::Ok().json(requests)),
        Err(e) => Ok(HttpResponse::InternalServerError().json(json!({
            "error": e.to_string()
        }))),
    }
}

/// Approve mint request (admin only)
async fn approve_mint_request(
    pool: web::Data<PgPool>,
    mint_service: web::Data<MintService>,
    request_id: web::Path<Uuid>,
    approve_req: web::Json<ApproveRequest>,
) -> Result<HttpResponse> {
    match mint_service
        .approve_mint_request(
            pool.get_ref(),
            *request_id,
            approve_req.admin_id,
            approve_req.chain_request_id,
        )
        .await
    {
        Ok(mint_request) => Ok(HttpResponse::Ok().json(mint_request)),
        Err(e) => Ok(HttpResponse::BadRequest().json(json!({
            "error": e.to_string()
        }))),
    }
}

/// Reject mint request (admin only)
async fn reject_mint_request(
    pool: web::Data<PgPool>,
    mint_service: web::Data<MintService>,
    request_id: web::Path<Uuid>,
    reject_req: web::Json<RejectRequest>,
) -> Result<HttpResponse> {
    match mint_service
        .reject_mint_request(pool.get_ref(), *request_id, reject_req.admin_id)
        .await
    {
        Ok(mint_request) => Ok(HttpResponse::Ok().json(mint_request)),
        Err(e) => Ok(HttpResponse::BadRequest().json(json!({
            "error": e.to_string()
        }))),
    }
}

// ===== BURN ADMIN ENDPOINTS =====

/// Get all pending burn requests (admin only)
async fn get_pending_burn_requests(
    pool: web::Data<PgPool>,
    burn_service: web::Data<BurnService>,
) -> Result<HttpResponse> {
    match burn_service.get_pending_burn_requests(pool.get_ref()).await {
        Ok(requests) => Ok(HttpResponse::Ok().json(requests)),
        Err(e) => Ok(HttpResponse::InternalServerError().json(json!({
            "error": e.to_string()
        }))),
    }
}

/// Get all burn requests with pagination (admin only)
async fn get_all_burn_requests(
    pool: web::Data<PgPool>,
    burn_service: web::Data<BurnService>,
    query: web::Query<PaginationQuery>,
) -> Result<HttpResponse> {
    let limit = query.limit.unwrap_or(50).min(100);
    let offset = query.offset.unwrap_or(0);

    match burn_service
        .get_all_burn_requests(pool.get_ref(), limit, offset)
        .await
    {
        Ok(requests) => Ok(HttpResponse::Ok().json(requests)),
        Err(e) => Ok(HttpResponse::InternalServerError().json(json!({
            "error": e.to_string()
        }))),
    }
}

/// Approve burn request (admin only)
async fn approve_burn_request(
    pool: web::Data<PgPool>,
    burn_service: web::Data<BurnService>,
    request_id: web::Path<Uuid>,
    approve_req: web::Json<ApproveRequest>,
) -> Result<HttpResponse> {
    match burn_service
        .approve_burn_request(
            pool.get_ref(),
            *request_id,
            approve_req.admin_id,
            approve_req.chain_request_id,
        )
        .await
    {
        Ok(burn_request) => Ok(HttpResponse::Ok().json(burn_request)),
        Err(e) => Ok(HttpResponse::BadRequest().json(json!({
            "error": e.to_string()
        }))),
    }
}

/// Reject burn request (admin only)
async fn reject_burn_request(
    pool: web::Data<PgPool>,
    burn_service: web::Data<BurnService>,
    request_id: web::Path<Uuid>,
    reject_req: web::Json<RejectRequest>,
) -> Result<HttpResponse> {
    match burn_service
        .reject_burn_request(pool.get_ref(), *request_id, reject_req.admin_id)
        .await
    {
        Ok(burn_request) => Ok(HttpResponse::Ok().json(burn_request)),
        Err(e) => Ok(HttpResponse::BadRequest().json(json!({
            "error": e.to_string()
        }))),
    }
}

// ===== KYC ADMIN ENDPOINTS =====

/// Get all pending KYC submissions (admin only)
async fn get_pending_kyc_submissions(
    pool: web::Data<PgPool>,
    kyc_service: web::Data<KYCService>,
) -> Result<HttpResponse> {
    match kyc_service.get_pending_submissions(pool.get_ref()).await {
        Ok(submissions) => Ok(HttpResponse::Ok().json(submissions)),
        Err(e) => Ok(HttpResponse::InternalServerError().json(json!({
            "error": e.to_string()
        }))),
    }
}

/// Get all KYC submissions with pagination (admin only)
async fn get_all_kyc_submissions(
    pool: web::Data<PgPool>,
    kyc_service: web::Data<KYCService>,
    query: web::Query<PaginationQuery>,
) -> Result<HttpResponse> {
    let limit = query.limit.unwrap_or(50).min(100);
    let offset = query.offset.unwrap_or(0);

    match kyc_service
        .get_all_submissions(pool.get_ref(), limit, offset)
        .await
    {
        Ok(submissions) => Ok(HttpResponse::Ok().json(submissions)),
        Err(e) => Ok(HttpResponse::InternalServerError().json(json!({
            "error": e.to_string()
        }))),
    }
}

/// Verify KYC submission (admin only)
async fn verify_kyc_submission(
    pool: web::Data<PgPool>,
    kyc_service: web::Data<KYCService>,
    submission_id: web::Path<Uuid>,
    verify_req: web::Json<serde_json::Value>,
) -> Result<HttpResponse> {
    let admin_id = verify_req
        .get("admin_id")
        .and_then(|v| v.as_str())
        .and_then(|s| Uuid::parse_str(s).ok())
        .ok_or_else(|| actix_web::error::ErrorBadRequest("admin_id is required"))?;

    match kyc_service
        .verify_kyc(pool.get_ref(), *submission_id, admin_id)
        .await
    {
        Ok(kyc_submission) => Ok(HttpResponse::Ok().json(kyc_submission)),
        Err(e) => Ok(HttpResponse::BadRequest().json(json!({
            "error": e.to_string()
        }))),
    }
}

/// Reject KYC submission (admin only)
async fn reject_kyc_submission(
    pool: web::Data<PgPool>,
    kyc_service: web::Data<KYCService>,
    submission_id: web::Path<Uuid>,
    reject_req: web::Json<RejectRequest>,
) -> Result<HttpResponse> {
    match kyc_service
        .reject_kyc(pool.get_ref(), *submission_id, reject_req.admin_id)
        .await
    {
        Ok(kyc_submission) => Ok(HttpResponse::Ok().json(kyc_submission)),
        Err(e) => Ok(HttpResponse::BadRequest().json(json!({
            "error": e.to_string()
        }))),
    }
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/admin")
            // Mint requests
            .route("/mint/pending", web::get().to(get_pending_mint_requests))
            .route("/mint/all", web::get().to(get_all_mint_requests))
            .route("/mint/{request_id}/approve", web::post().to(approve_mint_request))
            .route("/mint/{request_id}/reject", web::post().to(reject_mint_request))
            // Burn requests
            .route("/burn/pending", web::get().to(get_pending_burn_requests))
            .route("/burn/all", web::get().to(get_all_burn_requests))
            .route("/burn/{request_id}/approve", web::post().to(approve_burn_request))
            .route("/burn/{request_id}/reject", web::post().to(reject_burn_request))
            // KYC submissions
            .route("/kyc/pending", web::get().to(get_pending_kyc_submissions))
            .route("/kyc/all", web::get().to(get_all_kyc_submissions))
            .route("/kyc/{submission_id}/verify", web::post().to(verify_kyc_submission))
            .route("/kyc/{submission_id}/reject", web::post().to(reject_kyc_submission)),
    );
}
