use crate::classroom::{
    dtos::{CreateClassroomDto, UpdateClassroomDto},
    services::classroom_services::ClassroomService,
};
use actix_web::{web, HttpResponse, Responder};
use log::{error, info};
use sea_orm::DatabaseConnection;
use uuid::Uuid;

pub async fn get_classroom(
    db: web::Data<DatabaseConnection>,
    id: web::Path<Uuid>,
) -> impl Responder {
    let classroom_id = id.into_inner();
    info!("Fetching classroom with id: {}", classroom_id);
    let db = db.get_ref();
    let classroom = ClassroomService::get_classroom(db, classroom_id).await;

    match classroom {
        Ok(Some(classroom)) => {
            info!("Successfully fetched classroom: {:?}", classroom);
            HttpResponse::Ok().json(classroom)
        }
        Ok(None) => {
            info!("Classroom not found with id: {}", classroom_id);
            HttpResponse::NotFound().body("Classroom not found")
        }
        Err(e) => {
            error!("Failed to fetch classroom with id: {}: {}", classroom_id, e);
            HttpResponse::InternalServerError().body("Internal server error")
        }
    }
}

pub async fn get_all_classrooms(db: web::Data<DatabaseConnection>) -> impl Responder {
    info!("Fetching all classrooms");
    let db = db.get_ref();
    let classrooms = ClassroomService::get_all_classrooms(db).await;

    match classrooms {
        Ok(classrooms) => {
            info!("Successfully fetched {} classrooms", classrooms.len());
            HttpResponse::Ok().json(classrooms)
        }
        Err(e) => {
            error!("Failed to fetch classrooms: {}", e);
            HttpResponse::InternalServerError().body("Internal server error")
        }
    }
}

pub async fn create_classroom(
    db: web::Data<DatabaseConnection>,
    classroom_dto: web::Json<CreateClassroomDto>,
) -> impl Responder {
    info!("Creating new classroom with data: {:?}", classroom_dto);
    let db = db.get_ref();
    let result = ClassroomService::create_classroom(db, classroom_dto.into_inner()).await;

    match result {
        Ok(classroom) => {
            info!("Successfully created classroom: {:?}", classroom);
            HttpResponse::Ok().json(classroom)
        }
        Err(e) => {
            error!("Failed to create classroom: {}", e);
            HttpResponse::InternalServerError().body("Internal server error")
        }
    }
}

pub async fn update_classroom(
    db: web::Data<DatabaseConnection>,
    id: web::Path<Uuid>,
    classroom_dto: web::Json<UpdateClassroomDto>,
) -> impl Responder {
    let classroom_id = id.into_inner();
    info!(
        "Updating classroom with id: {} and data: {:?}",
        classroom_id, classroom_dto
    );
    let db = db.get_ref();
    let result =
        ClassroomService::update_classroom(db, classroom_id, classroom_dto.into_inner()).await;

    match result {
        Ok(classroom) => {
            info!("Successfully updated classroom: {:?}", classroom);
            HttpResponse::Ok().json(classroom)
        }
        Err(e) => {
            error!(
                "Failed to update classroom with id: {}: {}",
                classroom_id, e
            );
            HttpResponse::InternalServerError().body("Internal server error")
        }
    }
}

pub async fn delete_classroom(
    db: web::Data<DatabaseConnection>,
    id: web::Path<Uuid>,
) -> impl Responder {
    let classroom_id = id.into_inner();
    info!("Deleting classroom with id: {}", classroom_id);
    let db = db.get_ref();
    let result = ClassroomService::delete_classroom(db, classroom_id).await;

    match result {
        Ok(_) => {
            info!("Successfully deleted classroom with id: {}", classroom_id);
            HttpResponse::Ok().body("Classroom deleted")
        }
        Err(e) => {
            error!(
                "Failed to delete classroom with id: {}: {}",
                classroom_id, e
            );
            HttpResponse::InternalServerError().body("Internal server error")
        }
    }
}
