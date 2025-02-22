use crate::student::{
    dtos::{CreateStudentDto, UpdateStudentDto},
    services::student_service::StudentService,
};
use actix_web::{web, HttpResponse, Responder};
use log::{error, info};
use sea_orm::DatabaseConnection;
use uuid::Uuid;

pub async fn get_student(db: web::Data<DatabaseConnection>, id: web::Path<Uuid>) -> impl Responder {
    let student_id = id.into_inner();
    info!("Fetching student with id: {}", student_id);
    let db = db.get_ref();
    let student = StudentService::get_student(db, student_id).await;

    match student {
        Ok(Some(student)) => {
            info!("Successfully fetched student: {:?}", student);
            HttpResponse::Ok().json(student)
        }
        Ok(None) => {
            info!("Student not found with id: {}", student_id);
            HttpResponse::NotFound().body("Student not found")
        }
        Err(e) => {
            error!("Failed to fetch student with id: {}: {}", student_id, e);
            HttpResponse::InternalServerError().body("Internal server error")
        }
    }
}

pub async fn get_all_students(db: web::Data<DatabaseConnection>) -> impl Responder {
    info!("Fetching all students");
    let db = db.get_ref();
    let students = StudentService::get_all_students(db).await;

    match students {
        Ok(students) => {
            info!("Successfully fetched {} students", students.len());
            HttpResponse::Ok().json(students)
        }
        Err(e) => {
            error!("Failed to fetch students: {}", e);
            HttpResponse::InternalServerError().body("Internal server error")
        }
    }
}

pub async fn get_students_by_classroom(
    db: web::Data<DatabaseConnection>,
    id: web::Path<Uuid>,
) -> impl Responder {
    let class_id = id.into_inner();
    info!("Fetching students for classroom with id: {}", class_id);
    let db = db.get_ref();
    let students = StudentService::get_students_by_classroom(db, class_id).await;

    match students {
        Ok(students) => {
            info!(
                "Successfully fetched {} students for classroom {}",
                students.len(),
                class_id
            );
            HttpResponse::Ok().json(students)
        }
        Err(e) => {
            error!("Failed to fetch students for classroom {}: {}", class_id, e);
            HttpResponse::InternalServerError().body("Internal server error")
        }
    }
}

pub async fn create_student(
    db: web::Data<DatabaseConnection>,
    student_dto: web::Json<CreateStudentDto>,
) -> impl Responder {
    info!("Creating new student with data: {:?}", student_dto);
    let db = db.get_ref();
    let result = StudentService::create_student(db, student_dto.into_inner()).await;

    match result {
        Ok(student) => {
            info!("Successfully created student: {:?}", student);
            HttpResponse::Ok().json(student)
        }
        Err(e) => {
            error!("Failed to create student: {}", e);
            HttpResponse::InternalServerError().body("Internal server error")
        }
    }
}

pub async fn update_student(
    db: web::Data<DatabaseConnection>,
    id: web::Path<Uuid>,
    student_dto: web::Json<UpdateStudentDto>,
) -> impl Responder {
    let student_id = id.into_inner();
    info!(
        "Updating student with id: {} and data: {:?}",
        student_id, student_dto
    );
    let db = db.get_ref();
    let result = StudentService::update_student(db, student_id, student_dto.into_inner()).await;

    match result {
        Ok(student) => {
            info!("Successfully updated student: {:?}", student);
            HttpResponse::Ok().json(student)
        }
        Err(e) => {
            error!("Failed to update student with id: {}: {}", student_id, e);
            HttpResponse::InternalServerError().body("Internal server error")
        }
    }
}

pub async fn delete_student(
    db: web::Data<DatabaseConnection>,
    id: web::Path<Uuid>,
) -> impl Responder {
    let student_id = id.into_inner();
    info!("Deleting student with id: {}", student_id);
    let db = db.get_ref();
    let result = StudentService::delete_student(db, student_id).await;

    match result {
        Ok(_) => {
            info!("Successfully deleted student with id: {}", student_id);
            HttpResponse::Ok().body("Student deleted")
        }
        Err(e) => {
            error!("Failed to delete student with id: {}: {}", student_id, e);
            HttpResponse::InternalServerError().body("Internal server error")
        }
    }
}
