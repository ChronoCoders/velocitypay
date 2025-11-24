use actix_web::{web, HttpRequest, HttpResponse, Result};
use serde_json::json;
use sqlx::PgPool;
use uuid::Uuid;

use crate::middleware::auth::get_user_id;
use crate::models::kyc::SubmitKYCRequest;
use crate::services::KYCService;

/// Submit KYC information (requires authentication)
async fn submit_kyc(
    pool: web::Data<PgPool>,
    kyc_service: web::Data<std::sync::Arc<KYCService>>,
    req: HttpRequest,
    kyc_req: web::Json<SubmitKYCRequest>,
) -> Result<HttpResponse> {
    let user_id = get_user_id(&req)?;

    // Get wallet address from user profile (in production, fetch from DB)
    // For now, require it in the request
    let wallet_address = kyc_req
        .wallet_address
        .as_ref()
        .ok_or_else(|| actix_web::error::ErrorBadRequest("wallet_address is required"))?;

    // Parse date of birth
    let date_of_birth = chrono::NaiveDate::parse_from_str(&kyc_req.date_of_birth, "%Y-%m-%d")
        .map_err(|_| actix_web::error::ErrorBadRequest("Invalid date format. Use YYYY-MM-DD"))?;

    match kyc_service
        .submit_kyc(
            pool.get_ref(),
            user_id,
            wallet_address,
            &kyc_req.document_hash,
            &kyc_req.full_name,
            date_of_birth,
            &kyc_req.country,
        )
        .await
    {
        Ok(kyc_submission) => Ok(HttpResponse::Created().json(kyc_submission)),
        Err(e) => Ok(HttpResponse::BadRequest().json(json!({
            "error": e.to_string()
        }))),
    }
}

/// Get KYC submission for current user (requires authentication)
async fn get_my_kyc(
    pool: web::Data<PgPool>,
    kyc_service: web::Data<std::sync::Arc<KYCService>>,
    req: HttpRequest,
) -> Result<HttpResponse> {
    let user_id = get_user_id(&req)?;

    match kyc_service
        .get_user_kyc_submission(pool.get_ref(), user_id)
        .await
    {
        Ok(Some(kyc_submission)) => Ok(HttpResponse::Ok().json(kyc_submission)),
        Ok(None) => Ok(HttpResponse::NotFound().json(json!({
            "error": "No KYC submission found"
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(json!({
            "error": e.to_string()
        }))),
    }
}

/// Get KYC submission by ID (requires authentication)
async fn get_kyc_by_id(
    pool: web::Data<PgPool>,
    kyc_service: web::Data<std::sync::Arc<KYCService>>,
    req: HttpRequest,
    submission_id: web::Path<Uuid>,
) -> Result<HttpResponse> {
    let _user_id = get_user_id(&req)?; // Verify authentication

    match kyc_service
        .get_kyc_submission(pool.get_ref(), *submission_id)
        .await
    {
        Ok(kyc_submission) => Ok(HttpResponse::Ok().json(kyc_submission)),
        Err(e) => Ok(HttpResponse::NotFound().json(json!({
            "error": e.to_string()
        }))),
    }
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/kyc")
            .route("", web::post().to(submit_kyc))
            .route("/my-submission", web::get().to(get_my_kyc))
            .route("/{submission_id}", web::get().to(get_kyc_by_id)),
    );
}
