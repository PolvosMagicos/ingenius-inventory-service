use crate::classroom::controllers::handlers;
use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/classroom").route(web::post().to(handlers::create_classroom)))
        .service(
            web::resource("/classroom/{id}")
                .route(web::get().to(handlers::get_classroom))
                .route(web::patch().to(handlers::update_classroom))
                .route(web::delete().to(handlers::delete_classroom)),
        );
}
