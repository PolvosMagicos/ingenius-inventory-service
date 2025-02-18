use crate::list_detail::controllers::handlers;
use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/list-detail")
            .route(web::post().to(handlers::create_list_detail))
            .route(web::get().to(handlers::get_all_lists_details)),
    )
    .service(
        web::resource("/list-detail/{id}")
            .route(web::get().to(handlers::get_list_detail))
            .route(web::patch().to(handlers::update_list_detail))
            .route(web::delete().to(handlers::delete_list_detail)),
    );
}
