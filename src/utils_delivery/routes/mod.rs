use crate::utils_delivery::controllers::handlers;
use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("")
            .route(web::post().to(handlers::create_utils_delivery))
            .route(web::get().to(handlers::get_all_utils_deliveries)),
    )
    .service(
        web::resource("/{id}")
            .route(web::get().to(handlers::get_utils_delivery))
            .route(web::patch().to(handlers::update_utils_delivery))
            .route(web::delete().to(handlers::delete_utils_delivery)),
    );
}
