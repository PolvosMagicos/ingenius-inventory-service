use crate::request::controllers::handlers;
use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("")
            .route(web::post().to(handlers::create_request))
            .route(web::get().to(handlers::get_all_requests)),
    )
    .service(
        web::resource("/{id}")
            .route(web::get().to(handlers::get_request))
            .route(web::patch().to(handlers::update_request))
            .route(web::delete().to(handlers::delete_request)),
    );
}
