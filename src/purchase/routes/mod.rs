use crate::purchase::controllers::handlers;
use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("")
            .route(web::post().to(handlers::create_purchase))
            .route(web::get().to(handlers::get_all_purchases)),
    )
    .service(
        web::resource("/{id}")
            .route(web::get().to(handlers::get_purchase))
            .route(web::patch().to(handlers::update_purchase))
            .route(web::delete().to(handlers::delete_purchase)),
    );
}
