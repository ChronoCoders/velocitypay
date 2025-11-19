use actix_web::{web, HttpRequest, HttpResponse, Result};
use serde_json::json;
use sqlx::PgPool;

use crate::middleware::auth::{get_user_id, get_claims};
use crate::models::user::{CreateUserRequest, LoginRequest};
use crate::services::AuthService;

/// Register a new user
async fn register(
    pool: web::Data<PgPool>,
    auth_service: web::Data<AuthService>,
    req: web::Json<CreateUserRequest>,
) -> Result<HttpResponse> {
    match auth_service
        .register(
            pool.get_ref(),
            &req.email,
            &req.password,
            req.wallet_address.as_deref(),
        )
        .await
    {
        Ok(auth_response) => Ok(HttpResponse::Created().json(auth_response)),
        Err(e) => Ok(HttpResponse::BadRequest().json(json!({
            "error": e.to_string()
        }))),
    }
}

/// Login user
async fn login(
    pool: web::Data<PgPool>,
    auth_service: web::Data<AuthService>,
    req: web::Json<LoginRequest>,
) -> Result<HttpResponse> {
    match auth_service
        .login(pool.get_ref(), &req.email, &req.password)
        .await
    {
        Ok(auth_response) => Ok(HttpResponse::Ok().json(auth_response)),
        Err(e) => Ok(HttpResponse::Unauthorized().json(json!({
            "error": e.to_string()
        }))),
    }
}

/// Get user profile (requires authentication)
async fn get_profile(
    pool: web::Data<PgPool>,
    auth_service: web::Data<AuthService>,
    req: HttpRequest,
) -> Result<HttpResponse> {
    let user_id = get_user_id(&req)?;

    match auth_service.get_profile(pool.get_ref(), user_id).await {
        Ok(user_response) => Ok(HttpResponse::Ok().json(user_response)),
        Err(e) => Ok(HttpResponse::NotFound().json(json!({
            "error": e.to_string()
        }))),
    }
}

/// Update wallet address (requires authentication)
async fn update_wallet(
    pool: web::Data<PgPool>,
    auth_service: web::Data<AuthService>,
    req: HttpRequest,
    wallet: web::Json<serde_json::Value>,
) -> Result<HttpResponse> {
    let user_id = get_user_id(&req)?;

    let wallet_address = wallet
        .get("wallet_address")
        .and_then(|v| v.as_str())
        .ok_or_else(|| {
            actix_web::error::ErrorBadRequest("wallet_address is required")
        })?;

    match auth_service
        .update_wallet(pool.get_ref(), user_id, wallet_address)
        .await
    {
        Ok(user_response) => Ok(HttpResponse::Ok().json(user_response)),
        Err(e) => Ok(HttpResponse::BadRequest().json(json!({
            "error": e.to_string()
        }))),
    }
}

/// Get JWT token claims info (requires authentication)
async fn get_token_info(req: HttpRequest) -> Result<HttpResponse> {
    let claims = get_claims(&req)?;

    Ok(HttpResponse::Ok().json(json!({
        "user_id": claims.sub,
        "email": claims.email,
        "expires_at": claims.exp,
    })))
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/auth")
            .route("/register", web::post().to(register))
            .route("/login", web::post().to(login))
            .route("/profile", web::get().to(get_profile))
            .route("/wallet", web::put().to(update_wallet))
            .route("/token", web::get().to(get_token_info)),
    );
}
