use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpMessage, HttpResponse,
};
use futures::future::LocalBoxFuture;
use jsonwebtoken::{decode, DecodingKey, Validation, Algorithm};
use serde::{Deserialize, Serialize};
use std::future::{ready, Ready};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,  // User ID
    pub email: String,
    pub exp: usize,   // Expiration time
}

pub struct Auth {
    pub jwt_secret: String,
}

impl Auth {
    pub fn new(jwt_secret: String) -> Self {
        Self { jwt_secret }
    }
}

impl<S, B> Transform<S, ServiceRequest> for Auth
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AuthMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AuthMiddleware {
            service,
            jwt_secret: self.jwt_secret.clone(),
        }))
    }
}

pub struct AuthMiddleware<S> {
    service: S,
    jwt_secret: String,
}

impl<S, B> Service<ServiceRequest> for AuthMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let jwt_secret = self.jwt_secret.clone();

        // Extract Authorization header
        let auth_header = req.headers().get("Authorization");

        if auth_header.is_none() {
            return Box::pin(async {
                Err(actix_web::error::ErrorUnauthorized(
                    "Missing Authorization header",
                ))
            });
        }

        let auth_str = match auth_header.unwrap().to_str() {
            Ok(s) => s,
            Err(_) => {
                return Box::pin(async {
                    Err(actix_web::error::ErrorUnauthorized("Invalid Authorization header"))
                });
            }
        };

        // Extract Bearer token
        if !auth_str.starts_with("Bearer ") {
            return Box::pin(async {
                Err(actix_web::error::ErrorUnauthorized(
                    "Authorization header must start with 'Bearer '",
                ))
            });
        }

        let token = auth_str.trim_start_matches("Bearer ");

        // Validate JWT
        let validation = Validation::new(Algorithm::HS256);
        let token_data = match decode::<Claims>(
            token,
            &DecodingKey::from_secret(jwt_secret.as_bytes()),
            &validation,
        ) {
            Ok(data) => data,
            Err(e) => {
                log::warn!("JWT validation failed: {}", e);
                return Box::pin(async {
                    Err(actix_web::error::ErrorUnauthorized("Invalid or expired token"))
                });
            }
        };

        // Parse user_id from claims
        let user_id = match Uuid::parse_str(&token_data.claims.sub) {
            Ok(id) => id,
            Err(_) => {
                return Box::pin(async {
                    Err(actix_web::error::ErrorUnauthorized("Invalid user ID in token"))
                });
            }
        };

        // Store user_id in request extensions for handlers to use
        req.extensions_mut().insert(user_id);
        req.extensions_mut().insert(token_data.claims);

        let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await?;
            Ok(res)
        })
    }
}

// Helper to extract user_id from request extensions
pub fn get_user_id(req: &actix_web::HttpRequest) -> Result<Uuid, actix_web::Error> {
    req.extensions()
        .get::<Uuid>()
        .copied()
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("User ID not found in request"))
}

// Helper to extract claims from request extensions
pub fn get_claims(req: &actix_web::HttpRequest) -> Result<Claims, actix_web::Error> {
    req.extensions()
        .get::<Claims>()
        .cloned()
        .ok_or_else(|| actix_web::error::ErrorUnauthorized("Claims not found in request"))
}
