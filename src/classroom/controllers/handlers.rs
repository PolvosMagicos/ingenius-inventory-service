use crate::classroom::{
    dtos::{CreateClassroomDto, UpdateClassroomDto},
    services::classroom_services::ClassroomService,
};
use actix_web::{web, HttpResponse, Responder};
use sea_orm::DatabaseConnection;
use uuid::Uuid;

pub async fn get_classroom(
    db: web::Data<DatabaseConnection>,
    id: web::Path<Uuid>,
) -> impl Responder {
    let db = db.get_ref();
    let classroom = ClassroomService::get_classroom(db, id.into_inner()).await;

    match classroom {
        Ok(Some(classroom)) => HttpResponse::Ok().json(classroom),
        Ok(None) => HttpResponse::NotFound().body("Classroom not found"),
        Err(_) => HttpResponse::InternalServerError().body("Internal server error"),
    }
}

pub async fn create_classroom(
    db: web::Data<DatabaseConnection>,
    classroom_dto: web::Json<CreateClassroomDto>,
) -> impl Responder {
    let db = db.get_ref();
    let result = ClassroomService::create_classroom(db, classroom_dto.into_inner()).await;

    match result {
        Ok(classroom) => HttpResponse::Ok().json(classroom),
        Err(_) => HttpResponse::InternalServerError().body("Internal server error"),
    }
}

pub async fn update_classroom(
    db: web::Data<DatabaseConnection>,
    id: web::Path<Uuid>,
    classroom_dto: web::Json<UpdateClassroomDto>,
) -> impl Responder {
    let db = db.get_ref();
    let result =
        ClassroomService::update_classroom(db, id.into_inner(), classroom_dto.into_inner()).await;

    match result {
        Ok(classroom) => HttpResponse::Ok().json(classroom),
        Err(_) => HttpResponse::InternalServerError().body("Internal server error"),
    }
}

pub async fn delete_classroom(
    db: web::Data<DatabaseConnection>,
    id: web::Path<Uuid>,
) -> impl Responder {
    let db = db.get_ref();
    let result = ClassroomService::delete_classroom(db, id.into_inner()).await;

    match result {
        Ok(_) => HttpResponse::Ok().body("Classroom deleted"),
        Err(_) => HttpResponse::InternalServerError().body("Internal server error"),
    }
}
