use crate::{
    auth::dtos::RegisterUserDto,
    user::{dtos::update_user::UpdateUserDto, services::user_service::UserService},
};
use actix_web::{web, HttpResponse, Responder};
use log::{error, info};
use sea_orm::DatabaseConnection;
use uuid::Uuid;

pub async fn get_user(db: web::Data<DatabaseConnection>, id: web::Path<Uuid>) -> impl Responder {
    let user_id = id.into_inner();
    info!("Fetching user with id: {}", user_id);
    let db = db.get_ref();

    match UserService::get_user(db, user_id).await {
        Ok(user) => {
            info!("Successfully fetched user: {:?}", user);
            HttpResponse::Ok().json(user)
        }
        Err(e) => {
            error!("Failed to fetch user with id: {}: {}", user_id, e);
            HttpResponse::NotFound().body("User not found")
        }
    }
}

pub async fn get_all_users(db: web::Data<DatabaseConnection>) -> impl Responder {
    info!("Fetching all users");
    let db = db.get_ref();

    match UserService::get_all_users(db).await {
        Ok(users) => {
            info!("Successfully fetched {} users", users.len());
            HttpResponse::Ok().json(users)
        }
        Err(e) => {
            error!("Failed to fetch users: {}", e);
            HttpResponse::InternalServerError().body("Internal server error")
        }
    }
}

pub async fn create_user(
    db: web::Data<DatabaseConnection>,
    user_dto: web::Json<RegisterUserDto>,
) -> impl Responder {
    info!("Creating new user with data: {:?}", user_dto);
    let db = db.get_ref();

    match UserService::create_user(db, user_dto.into_inner()).await {
        Ok(user) => {
            info!("Successfully created user: {:?}", user);
            HttpResponse::Created().json(user)
        }
        Err(e) => {
            error!("Failed to create user: {}", e);
            HttpResponse::InternalServerError().body("Internal server error")
        }
    }
}

pub async fn update_user(
    db: web::Data<DatabaseConnection>,
    id: web::Path<Uuid>,
    user_dto: web::Json<UpdateUserDto>,
) -> impl Responder {
    let user_id = id.into_inner();
    info!(
        "Updating user with id: {} and data: {:?}",
        user_id, user_dto
    );
    let db = db.get_ref();

    match UserService::update_user(db, user_id, user_dto.into_inner()).await {
        Ok(user) => {
            info!("Successfully updated user: {:?}", user);
            HttpResponse::Ok().json(user)
        }
        Err(e) => {
            error!("Failed to update user with id: {}: {}", user_id, e);
            HttpResponse::InternalServerError().body("Internal server error")
        }
    }
}

pub async fn delete_user(db: web::Data<DatabaseConnection>, id: web::Path<Uuid>) -> impl Responder {
    let user_id = id.into_inner();
    info!("Deleting user with id: {}", user_id);
    let db = db.get_ref();

    match UserService::delete_user(db, user_id).await {
        Ok(_) => {
            info!("Successfully deleted user with id: {}", user_id);
            HttpResponse::Ok().body("User deleted")
        }
        Err(e) => {
            error!("Failed to delete user with id: {}: {}", user_id, e);
            HttpResponse::InternalServerError().body("Internal server error")
        }
    }
}
