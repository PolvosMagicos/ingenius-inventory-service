use crate::util_list::controllers::handlers;
use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/util_list")
            .route(web::post().to(handlers::create_util_list))
            .route(web::get().to(handlers::get_all_util_lists)),
    )
    .service(
        web::resource("/util_list/{id}")
            .route(web::get().to(handlers::get_util_list))
            .route(web::patch().to(handlers::update_util_list))
            .route(web::delete().to(handlers::delete_util_list)),
    );
}
