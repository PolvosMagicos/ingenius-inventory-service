use crate::school_supply_balance::controllers::handlers;
use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/school_supply_balance")
            .route(web::post().to(handlers::create_school_supply_balance))
            .route(web::get().to(handlers::get_all_school_supply_balances)),
    )
    .service(
        web::resource("/school_supply_balance/{id}")
            .route(web::get().to(handlers::get_school_supply_balance))
            .route(web::patch().to(handlers::update_school_supply_balance))
            .route(web::delete().to(handlers::delete_school_supply_balance)),
    );
}
