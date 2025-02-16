use crate::controllers::classroom;
use actix_web::web;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/classrooms/{id}")
            .route(web::get().to(classroom::get_classroom))
            .route(web::post().to(classroom::create_classroom))
            .route(web::patch().to(classroom::update_classroom))
            .route(web::delete().to(classroom::delete_classroom)),
    );
}
