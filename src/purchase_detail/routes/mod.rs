use crate::purchase_detail::controllers::handlers;
use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("")
            .route(web::post().to(handlers::create_purchase_detail))
            .route(web::get().to(handlers::get_all_purchase_details)),
    )
    .service(
        web::resource("/{id}")
            .route(web::get().to(handlers::get_purchase_detail))
            .route(web::patch().to(handlers::update_purchase_detail))
            .route(web::delete().to(handlers::delete_purchase_detail)),
    );
}
