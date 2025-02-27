use crate::money_delivery::controllers::handlers;
use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("")
            .route(web::post().to(handlers::create_money_delivery))
            .route(web::get().to(handlers::get_all_money_deliveries)),
    )
    .service(
        web::resource("/{id}")
            .route(web::get().to(handlers::get_money_delivery))
            .route(web::patch().to(handlers::update_money_delivery))
            .route(web::delete().to(handlers::delete_money_delivery)),
    );
}
