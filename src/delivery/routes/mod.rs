use crate::{delivery::controllers::handlers, utils_delivery};
use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("")
            .route(web::post().to(handlers::create_delivery))
            .route(web::get().to(handlers::get_all_deliveries)),
    )
    .service(
        web::resource("/{id}")
            .route(web::get().to(handlers::get_delivery))
            .route(web::patch().to(handlers::update_delivery))
            .route(web::delete().to(handlers::delete_delivery)),
    )
    .service(web::resource("/{id}/utils-delivery").route(
        web::get().to(utils_delivery::controllers::handlers::get_utils_deliveries_by_delivery),
    ));
}
