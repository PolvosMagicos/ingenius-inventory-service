use crate::util::controllers::handlers;
use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/util")
            .route(web::post().to(handlers::create_util))
            .route(web::get().to(handlers::get_all_utils)),
    )
    .service(
        web::resource("/util/{id}")
            .route(web::get().to(handlers::get_util))
            .route(web::patch().to(handlers::update_util))
            .route(web::delete().to(handlers::delete_util)),
    );
}
