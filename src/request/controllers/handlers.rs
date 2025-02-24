use crate::request::{
    dtos::{create_request::CreateRequestDto, update_request::UpdateRequestDto},
    services::request_service::RequestService,
};
use actix_web::{web, HttpResponse, Responder};
use log::{error, info};
use sea_orm::DatabaseConnection;
use uuid::Uuid;

pub async fn get_request(db: web::Data<DatabaseConnection>, id: web::Path<Uuid>) -> impl Responder {
    let request_id = id.into_inner();
    info!("Fetching request with id: {}", request_id);
    let db = db.get_ref();
    let request = RequestService::get_request(db, request_id).await;

    match request {
        Ok(Some(request)) => {
            info!("Successfully fetched request: {:?}", request);
            HttpResponse::Ok().json(request)
        }
        Ok(None) => {
            info!("Request not found with id: {}", request_id);
            HttpResponse::NotFound().body("Request not found")
        }
        Err(e) => {
            error!("Failed to fetch request with id: {}: {}", request_id, e);
            HttpResponse::InternalServerError().body("Internal server error")
        }
    }
}

pub async fn get_all_requests(db: web::Data<DatabaseConnection>) -> impl Responder {
    info!("Fetching all requests");
    let db = db.get_ref();
    let requests = RequestService::get_all_requests(db).await;

    match requests {
        Ok(requests) => {
            info!("Successfully fetched {} requests", requests.len());
            HttpResponse::Ok().json(requests)
        }
        Err(e) => {
            error!("Failed to fetch requests: {}", e);
            HttpResponse::InternalServerError().body("Internal server error")
        }
    }
}

pub async fn create_request(
    db: web::Data<DatabaseConnection>,
    request_dto: web::Json<CreateRequestDto>,
) -> impl Responder {
    info!("Creating new request with data: {:?}", request_dto);
    let db = db.get_ref();
    let result = RequestService::create_request(db, request_dto.into_inner()).await;

    match result {
        Ok(request) => {
            info!("Successfully created request: {:?}", request);
            HttpResponse::Ok().json(request)
        }
        Err(e) => {
            error!("Failed to create request: {}", e);
            HttpResponse::InternalServerError().body("Internal server error")
        }
    }
}

pub async fn update_request(
    db: web::Data<DatabaseConnection>,
    id: web::Path<Uuid>,
    request_dto: web::Json<UpdateRequestDto>,
) -> impl Responder {
    let request_id = id.into_inner();
    info!(
        "Updating request with id: {} and data: {:?}",
        request_id, request_dto
    );
    let db = db.get_ref();
    let result = RequestService::update_request(db, request_id, request_dto.into_inner()).await;

    match result {
        Ok(request) => {
            info!("Successfully updated request: {:?}", request);
            HttpResponse::Ok().json(request)
        }
        Err(e) => {
            error!("Failed to update request with id: {}: {}", request_id, e);
            HttpResponse::InternalServerError().body("Internal server error")
        }
    }
}

pub async fn delete_request(
    db: web::Data<DatabaseConnection>,
    id: web::Path<Uuid>,
) -> impl Responder {
    let request_id = id.into_inner();
    info!("Deleting request with id: {}", request_id);
    let db = db.get_ref();
    let result = RequestService::delete_request(db, request_id).await;

    match result {
        Ok(_) => {
            info!("Successfully deleted request with id: {}", request_id);
            HttpResponse::Ok().body("Request deleted")
        }
        Err(e) => {
            error!("Failed to delete request with id: {}: {}", request_id, e);
            HttpResponse::InternalServerError().body("Internal server error")
        }
    }
}
