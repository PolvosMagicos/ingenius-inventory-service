use crate::student::controllers::handlers;
use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("")
            .route(web::post().to(handlers::create_student))
            .route(web::get().to(handlers::get_all_students)),
    )
    .service(
        web::resource("/{id}")
            .route(web::get().to(handlers::get_student))
            .route(web::patch().to(handlers::update_student))
            .route(web::delete().to(handlers::delete_student)),
    );
}
