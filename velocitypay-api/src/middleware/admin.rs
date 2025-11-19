use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error,
};
use futures::future::LocalBoxFuture;
use std::future::{ready, Ready};

pub struct AdminAuth {
    pub admin_api_key: String,
}

impl AdminAuth {
    pub fn new(admin_api_key: String) -> Self {
        Self { admin_api_key }
    }
}

impl<S, B> Transform<S, ServiceRequest> for AdminAuth
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error>,
    S::Future: 'static,
    B: 'static,
{
    type Response = ServiceResponse<B>;
    type Error = Error;
    type InitError = ();
    type Transform = AdminAuthMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(AdminAuthMiddleware {
            service,
            admin_api_key: self.admin_api_key.clone(),
        }))
    }
}

pub struct AdminAuthMiddleware<S> {
    service: S,
    admin_api_key: String,
}

impl<S, B> Service<ServiceRequest> for AdminAuthMiddleware<S>
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
        let admin_api_key = self.admin_api_key.clone();

        // Extract X-Admin-API-Key header
        let api_key_header = req.headers().get("X-Admin-API-Key");

        if api_key_header.is_none() {
            return Box::pin(async {
                Err(actix_web::error::ErrorUnauthorized("Missing X-Admin-API-Key header"))
            });
        }

        let api_key = match api_key_header.unwrap().to_str() {
            Ok(s) => s,
            Err(_) => {
                return Box::pin(async {
                    Err(actix_web::error::ErrorUnauthorized("Invalid X-Admin-API-Key header"))
                });
            }
        };

        // Validate API key
        if api_key != admin_api_key {
            log::warn!("Invalid admin API key attempt");
            return Box::pin(async {
                Err(actix_web::error::ErrorForbidden("Invalid admin API key"))
            });
        }

        let fut = self.service.call(req);

        Box::pin(async move {
            let res = fut.await?;
            Ok(res)
        })
    }
}
