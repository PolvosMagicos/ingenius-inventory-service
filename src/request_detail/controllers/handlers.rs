use crate::request_detail::{
    dtos::{
        create_request_detail::CreateRequestDetailDto,
        update_request_detail::UpdateRequestDetailDto,
    },
    services::request_detail_services::RequestDetailService,
};
use actix_web::{web, HttpResponse, Responder};
use log::{error, info};
use sea_orm::DatabaseConnection;
use uuid::Uuid;

pub async fn get_request_detail(
    db: web::Data<DatabaseConnection>,
    id: web::Path<Uuid>,
) -> impl Responder {
    let request_detail_id = id.into_inner();
    info!("Fetching request detail with id: {}", request_detail_id);
    let db = db.get_ref();
    let request_detail = RequestDetailService::get_request_detail(db, request_detail_id).await;

    match request_detail {
        Ok(Some(request_detail)) => {
            info!("Successfully fetched request detail: {:?}", request_detail);
            HttpResponse::Ok().json(request_detail)
        }
        Ok(None) => {
            info!("Request detail not found with id: {}", request_detail_id);
            HttpResponse::NotFound().body("Request detail not found")
        }
        Err(e) => {
            error!(
                "Failed to fetch request detail with id: {}: {}",
                request_detail_id, e
            );
            HttpResponse::InternalServerError().body("Internal server error")
        }
    }
}

pub async fn get_all_requests_details(db: web::Data<DatabaseConnection>) -> impl Responder {
    info!("Fetching all request details");
    let db = db.get_ref();
    let requests_details = RequestDetailService::get_all_resquests_details(db).await;

    match requests_details {
        Ok(request_details) => {
            info!(
                "Successfully fetched {} requests details",
                request_details.len()
            );
            HttpResponse::Ok().json(request_details)
        }
        Err(e) => {
            error!("Failed to fetch requests details: {}", e);
            HttpResponse::InternalServerError().body("Internal server error")
        }
    }
}

pub async fn create_request_detail(
    db: web::Data<DatabaseConnection>,
    request_detail_dto: web::Json<CreateRequestDetailDto>,
) -> impl Responder {
    info!(
        "Creating new request detail with data: {:?}",
        request_detail_dto
    );
    let db = db.get_ref();
    let result =
        RequestDetailService::create_request_detail(db, request_detail_dto.into_inner()).await;

    match result {
        Ok(request_detail) => {
            info!("Successfully created request detail: {:?}", request_detail);
            HttpResponse::Ok().json(request_detail)
        }
        Err(e) => {
            error!("Failed to create request detail: {}", e);
            HttpResponse::InternalServerError().body("Internal server error")
        }
    }
}

pub async fn update_request_detail(
    db: web::Data<DatabaseConnection>,
    id: web::Path<Uuid>,
    request_detail_dto: web::Json<UpdateRequestDetailDto>,
) -> impl Responder {
    let request_detail_id = id.into_inner();
    info!(
        "Updating request detail with id: {} and data: {:?}",
        request_detail_id, request_detail_dto
    );
    let db = db.get_ref();
    let result = RequestDetailService::update_request_detail(
        db,
        request_detail_id,
        request_detail_dto.into_inner(),
    )
    .await;

    match result {
        Ok(request_detail) => {
            info!("Successfully updated request detail: {:?}", request_detail);
            HttpResponse::Ok().json(request_detail)
        }
        Err(e) => {
            error!(
                "Failed to update request detail with id: {}: {}",
                request_detail_id, e
            );
            HttpResponse::InternalServerError().body("Internal server error")
        }
    }
}

pub async fn delete_request_detail(
    db: web::Data<DatabaseConnection>,
    id: web::Path<Uuid>,
) -> impl Responder {
    let request_detail_id = id.into_inner();
    info!("Deleting request detail with id: {}", request_detail_id);
    let db = db.get_ref();
    let result = RequestDetailService::delete_request_detail(db, request_detail_id).await;

    match result {
        Ok(_) => {
            info!(
                "Successfully deleted request detail with id: {}",
                request_detail_id
            );
            HttpResponse::Ok().body("Request detail deleted")
        }
        Err(e) => {
            error!(
                "Failed to delete request detail with id: {}: {}",
                request_detail_id, e
            );
            HttpResponse::InternalServerError().body("Internal server error")
        }
    }
}
