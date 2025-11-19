pub mod auth_routes;
pub mod payment_routes;
pub mod mint_routes;
pub mod burn_routes;
pub mod kyc_routes;
pub mod admin_routes;

use actix_web::web;

/// Configure all API routes
pub fn configure_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            .configure(auth_routes::configure)
            .configure(payment_routes::configure)
            .configure(mint_routes::configure)
            .configure(burn_routes::configure)
            .configure(kyc_routes::configure)
            .configure(admin_routes::configure),
    );
}
