use actix_web::{web, HttpRequest, HttpResponse, Result};
use rust_decimal::Decimal;
use serde::Deserialize;
use serde_json::json;
use sqlx::PgPool;
use std::str::FromStr;
use uuid::Uuid;

use crate::middleware::auth::get_user_id;
use crate::models::transaction::SendPaymentRequest;
use crate::services::PaymentService;

#[derive(Debug, Deserialize)]
struct HistoryQuery {
    limit: Option<i64>,
    offset: Option<i64>,
}

/// Send payment (requires authentication)
async fn send_payment(
    pool: web::Data<PgPool>,
    payment_service: web::Data<PaymentService>,
    req: HttpRequest,
    payment: web::Json<SendPaymentRequest>,
) -> Result<HttpResponse> {
    let _user_id = get_user_id(&req)?; // Verify authentication

    // Parse amount string to Decimal
    let amount = Decimal::from_str(&payment.amount)
        .map_err(|_| actix_web::error::ErrorBadRequest("Invalid amount format"))?;

    match payment_service
        .send_payment(
            pool.get_ref(),
            &payment.from_address,
            &payment.to_address,
            amount,
        )
        .await
    {
        Ok(transaction) => Ok(HttpResponse::Created().json(transaction)),
        Err(e) => Ok(HttpResponse::BadRequest().json(json!({
            "error": e.to_string()
        }))),
    }
}

/// Get transaction by ID (requires authentication)
async fn get_transaction(
    pool: web::Data<PgPool>,
    payment_service: web::Data<PaymentService>,
    req: HttpRequest,
    transaction_id: web::Path<Uuid>,
) -> Result<HttpResponse> {
    let _user_id = get_user_id(&req)?; // Verify authentication

    match payment_service
        .get_transaction(pool.get_ref(), *transaction_id)
        .await
    {
        Ok(transaction) => Ok(HttpResponse::Ok().json(transaction)),
        Err(e) => Ok(HttpResponse::NotFound().json(json!({
            "error": e.to_string()
        }))),
    }
}

/// Get transaction history for wallet (requires authentication)
async fn get_transaction_history(
    pool: web::Data<PgPool>,
    payment_service: web::Data<PaymentService>,
    req: HttpRequest,
    wallet_address: web::Path<String>,
    query: web::Query<HistoryQuery>,
) -> Result<HttpResponse> {
    let _user_id = get_user_id(&req)?; // Verify authentication

    let limit = query.limit.unwrap_or(50).min(100);
    let offset = query.offset.unwrap_or(0);

    match payment_service
        .get_transaction_history(pool.get_ref(), &wallet_address, limit, offset)
        .await
    {
        Ok(transactions) => Ok(HttpResponse::Ok().json(transactions)),
        Err(e) => Ok(HttpResponse::InternalServerError().json(json!({
            "error": e.to_string()
        }))),
    }
}

/// Get transaction by blockchain hash (requires authentication)
async fn get_transaction_by_hash(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    tx_hash: web::Path<String>,
) -> Result<HttpResponse> {
    let _user_id = get_user_id(&req)?; // Verify authentication

    let repo = crate::db::transaction_repository::TransactionRepository::new(pool.get_ref());

    match repo.find_by_tx_hash(&tx_hash).await {
        Ok(Some(transaction)) => {
            let response = crate::models::response::TransactionResponse {
                id: transaction.id,
                from_address: transaction.from_address,
                to_address: transaction.to_address,
                amount: transaction.amount,
                fee: transaction.fee,
                tx_hash: transaction.tx_hash,
                block_number: transaction.block_number,
                status: match transaction.status.as_str() {
                    "confirmed" => crate::models::transaction::TransactionStatus::Confirmed,
                    "failed" => crate::models::transaction::TransactionStatus::Failed,
                    _ => crate::models::transaction::TransactionStatus::Pending,
                },
                created_at: transaction.created_at,
            };
            Ok(HttpResponse::Ok().json(response))
        }
        Ok(None) => Ok(HttpResponse::NotFound().json(json!({
            "error": "Transaction not found"
        }))),
        Err(e) => Ok(HttpResponse::InternalServerError().json(json!({
            "error": e.to_string()
        }))),
    }
}

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/payments")
            .route("", web::post().to(send_payment))
            .route("/{transaction_id}", web::get().to(get_transaction))
            .route("/hash/{tx_hash}", web::get().to(get_transaction_by_hash))
            .route(
                "/history/{wallet_address}",
                web::get().to(get_transaction_history),
            ),
    );
}
