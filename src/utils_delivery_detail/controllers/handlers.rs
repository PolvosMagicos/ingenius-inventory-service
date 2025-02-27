use crate::utils_delivery_detail::dtos::{
    CreateUtilsDeliveryDetailDto, UpdateUtilsDeliveryDetailDto,
};
use crate::utils_delivery_detail::services::utils_delivery_detail_service::UtilsDeliveryDetailService;
use actix_web::{web, HttpResponse, Responder};
use log::{error, info};
use sea_orm::DatabaseConnection;
use uuid::Uuid;

pub async fn get_utils_delivery_detail(
    db: web::Data<DatabaseConnection>,
    id: web::Path<Uuid>,
) -> impl Responder {
    let detail_id = id.into_inner();
    info!("Fetching utils delivery detail with id: {}", detail_id);
    let db = db.get_ref();
    let result = UtilsDeliveryDetailService::get_utils_delivery_detail(db, detail_id).await;
    match result {
        Ok(Some(detail)) => HttpResponse::Ok().json(detail),
        Ok(None) => HttpResponse::NotFound().body("Utils delivery detail not found"),
        Err(e) => {
            error!("Failed to fetch utils delivery detail: {}", e);
            HttpResponse::InternalServerError().body("Internal server error")
        }
    }
}

pub async fn get_all_utils_delivery_details(db: web::Data<DatabaseConnection>) -> impl Responder {
    info!("Fetching all utils delivery details");
    let db = db.get_ref();
    let result = UtilsDeliveryDetailService::get_all_utils_delivery_details(db).await;
    match result {
        Ok(details) => HttpResponse::Ok().json(details),
        Err(e) => {
            error!("Failed to fetch utils delivery details: {}", e);
            HttpResponse::InternalServerError().body("Internal server error")
        }
    }
}

pub async fn create_utils_delivery_detail(
    db: web::Data<DatabaseConnection>,
    detail_dto: web::Json<CreateUtilsDeliveryDetailDto>,
) -> impl Responder {
    info!("Creating new utils delivery detail: {:?}", detail_dto);
    let db = db.get_ref();
    let result =
        UtilsDeliveryDetailService::create_utils_delivery_detail(db, detail_dto.into_inner()).await;
    match result {
        Ok(detail) => HttpResponse::Ok().json(detail),
        Err(e) => {
            error!("Failed to create utils delivery detail: {}", e);
            HttpResponse::InternalServerError().body("Internal server error")
        }
    }
}

pub async fn update_utils_delivery_detail(
    db: web::Data<DatabaseConnection>,
    id: web::Path<Uuid>,
    detail_dto: web::Json<UpdateUtilsDeliveryDetailDto>,
) -> impl Responder {
    let detail_id = id.into_inner();
    info!("Updating utils delivery detail with id: {}", detail_id);
    let db = db.get_ref();
    let result = UtilsDeliveryDetailService::update_utils_delivery_detail(
        db,
        detail_id,
        detail_dto.into_inner(),
    )
    .await;
    match result {
        Ok(detail) => HttpResponse::Ok().json(detail),
        Err(e) => {
            error!("Failed to update utils delivery detail: {}", e);
            HttpResponse::InternalServerError().body("Internal server error")
        }
    }
}

pub async fn delete_utils_delivery_detail(
    db: web::Data<DatabaseConnection>,
    id: web::Path<Uuid>,
) -> impl Responder {
    let detail_id = id.into_inner();
    info!("Deleting utils delivery detail with id: {}", detail_id);
    let db = db.get_ref();
    let result = UtilsDeliveryDetailService::delete_utils_delivery_detail(db, detail_id).await;
    match result {
        Ok(_) => HttpResponse::Ok().body("Utils delivery detail deleted"),
        Err(e) => {
            error!("Failed to delete utils delivery detail: {}", e);
            HttpResponse::InternalServerError().body("Internal server error")
        }
    }
}
