use crate::{classroom::controllers::handlers, student};
use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("")
            .route(web::post().to(handlers::create_classroom))
            .route(web::get().to(handlers::get_all_classrooms)),
    )
    .service(
        web::resource("/{id}")
            .route(web::get().to(handlers::get_classroom))
            .route(web::patch().to(handlers::update_classroom))
            .route(web::delete().to(handlers::delete_classroom)),
    )
    .service(
        web::resource("/{id}/students")
            .route(web::get().to(student::controllers::handlers::get_students_by_classroom)),
    );
}
