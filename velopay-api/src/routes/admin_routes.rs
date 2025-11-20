use actix_web::{web, HttpResponse, Result};
use serde::Deserialize;
use serde_json::json;
use sqlx::PgPool;
use uuid::Uuid;

use crate::chain::client::VelocityClient;
use crate::chain::operations::ChainOperations;
use crate::models::transaction::TransactionStatus;
use crate::services::{MintService, BurnService, KYCService, PaymentService};

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

#[derive(Debug, Deserialize)]
struct CompleteRequest {
    chain_request_id: i64,
}

#[derive(Debug, Deserialize)]
struct UpdateTransactionRequest {
    tx_hash: String,
    block_number: Option<i64>,
    status: String,
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
    chain_client: web::Data<VelocityClient>,
    chain_ops: web::Data<ChainOperations>,
    mint_service: web::Data<MintService>,
    request_id: web::Path<Uuid>,
    approve_req: web::Json<ApproveRequest>,
) -> Result<HttpResponse> {
    // Get the request ID to approve on blockchain (0 for now, will be parsed from events later)
    let blockchain_request_id = approve_req.chain_request_id.unwrap_or(0) as u64;

    // Submit approve transaction to blockchain
    match chain_ops.approve_mint(chain_client.get_ref(), blockchain_request_id).await {
        Ok(tx_hash) => {
            log::info!("Mint approval submitted to blockchain - request_id: {}, tx_hash: {}", blockchain_request_id, tx_hash);

            // Update database with approval
            match mint_service
                .approve_mint_request(
                    pool.get_ref(),
                    *request_id,
                    approve_req.admin_id,
                    Some(blockchain_request_id as i64),
                )
                .await
            {
                Ok(mint_request) => Ok(HttpResponse::Ok().json(json!({
                    "mint_request": mint_request,
                    "blockchain_tx_hash": tx_hash,
                }))),
                Err(e) => Ok(HttpResponse::BadRequest().json(json!({
                    "error": format!("Database update failed: {}", e),
                    "blockchain_tx_hash": tx_hash,
                }))),
            }
        },
        Err(e) => Ok(HttpResponse::InternalServerError().json(json!({
            "error": format!("Blockchain transaction failed: {}", e)
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

/// Complete mint request after blockchain confirmation (admin only)
async fn complete_mint_request(
    pool: web::Data<PgPool>,
    mint_service: web::Data<MintService>,
    request_id: web::Path<Uuid>,
    complete_req: web::Json<CompleteRequest>,
) -> Result<HttpResponse> {
    match mint_service
        .complete_mint_request(pool.get_ref(), *request_id, complete_req.chain_request_id)
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
    chain_client: web::Data<VelocityClient>,
    chain_ops: web::Data<ChainOperations>,
    burn_service: web::Data<BurnService>,
    request_id: web::Path<Uuid>,
    approve_req: web::Json<ApproveRequest>,
) -> Result<HttpResponse> {
    // Get the request ID to approve on blockchain
    let blockchain_request_id = approve_req.chain_request_id.unwrap_or(0) as u64;

    // Submit approve transaction to blockchain
    match chain_ops.approve_burn(chain_client.get_ref(), blockchain_request_id).await {
        Ok(tx_hash) => {
            log::info!("Burn approval submitted to blockchain - request_id: {}, tx_hash: {}", blockchain_request_id, tx_hash);

            // Update database with approval
            match burn_service
                .approve_burn_request(
                    pool.get_ref(),
                    *request_id,
                    approve_req.admin_id,
                    Some(blockchain_request_id as i64),
                )
                .await
            {
                Ok(burn_request) => Ok(HttpResponse::Ok().json(json!({
                    "burn_request": burn_request,
                    "blockchain_tx_hash": tx_hash,
                }))),
                Err(e) => Ok(HttpResponse::BadRequest().json(json!({
                    "error": format!("Database update failed: {}", e),
                    "blockchain_tx_hash": tx_hash,
                }))),
            }
        },
        Err(e) => Ok(HttpResponse::InternalServerError().json(json!({
            "error": format!("Blockchain transaction failed: {}", e)
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

/// Complete burn request after blockchain confirmation (admin only)
async fn complete_burn_request(
    pool: web::Data<PgPool>,
    burn_service: web::Data<BurnService>,
    request_id: web::Path<Uuid>,
    complete_req: web::Json<CompleteRequest>,
) -> Result<HttpResponse> {
    match burn_service
        .complete_burn_request(pool.get_ref(), *request_id, complete_req.chain_request_id)
        .await
    {
        Ok(burn_request) => Ok(HttpResponse::Ok().json(burn_request)),
        Err(e) => Ok(HttpResponse::BadRequest().json(json!({
            "error": e.to_string()
        }))),
    }
}

/// Reserve burn request after blockchain reservation (admin only)
async fn reserve_burn_request(
    pool: web::Data<PgPool>,
    burn_service: web::Data<BurnService>,
    request_id: web::Path<Uuid>,
    reserve_req: web::Json<CompleteRequest>,
) -> Result<HttpResponse> {
    match burn_service
        .reserve_burn_request(pool.get_ref(), *request_id, reserve_req.chain_request_id)
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

/// Get KYC submission by wallet address (admin only)
async fn get_kyc_by_wallet(
    pool: web::Data<PgPool>,
    kyc_service: web::Data<KYCService>,
    wallet_address: web::Path<String>,
) -> Result<HttpResponse> {
    match kyc_service
        .get_kyc_by_wallet(pool.get_ref(), &wallet_address)
        .await
    {
        Ok(Some(kyc_submission)) => Ok(HttpResponse::Ok().json(kyc_submission)),
        Ok(None) => Ok(HttpResponse::NotFound().json(json!({
            "error": "KYC submission not found for this wallet"
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(json!({
            "error": e.to_string()
        }))),
    }
}

// ===== TRANSACTION ADMIN ENDPOINTS =====

/// Update transaction status after blockchain confirmation (admin only)
async fn update_transaction(
    pool: web::Data<PgPool>,
    payment_service: web::Data<PaymentService>,
    transaction_id: web::Path<Uuid>,
    update_req: web::Json<UpdateTransactionRequest>,
) -> Result<HttpResponse> {
    let status = match update_req.status.as_str() {
        "pending" => TransactionStatus::Pending,
        "confirmed" => TransactionStatus::Confirmed,
        "failed" => TransactionStatus::Failed,
        _ => return Ok(HttpResponse::BadRequest().json(json!({
            "error": "Invalid status. Must be: pending, confirmed, or failed"
        }))),
    };

    match payment_service
        .update_transaction_status(
            pool.get_ref(),
            *transaction_id,
            &update_req.tx_hash,
            update_req.block_number,
            status,
        )
        .await
    {
        Ok(transaction) => Ok(HttpResponse::Ok().json(transaction)),
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
            .route("/mint/{request_id}/complete", web::post().to(complete_mint_request))
            // Burn requests
            .route("/burn/pending", web::get().to(get_pending_burn_requests))
            .route("/burn/all", web::get().to(get_all_burn_requests))
            .route("/burn/{request_id}/approve", web::post().to(approve_burn_request))
            .route("/burn/{request_id}/reject", web::post().to(reject_burn_request))
            .route("/burn/{request_id}/complete", web::post().to(complete_burn_request))
            .route("/burn/{request_id}/reserve", web::post().to(reserve_burn_request))
            // KYC submissions
            .route("/kyc/pending", web::get().to(get_pending_kyc_submissions))
            .route("/kyc/all", web::get().to(get_all_kyc_submissions))
            .route("/kyc/{submission_id}/verify", web::post().to(verify_kyc_submission))
            .route("/kyc/{submission_id}/reject", web::post().to(reject_kyc_submission))
            .route("/kyc/wallet/{wallet_address}", web::get().to(get_kyc_by_wallet))
            // Transactions
            .route("/transactions/{transaction_id}/update", web::post().to(update_transaction)),
    );
}
