use crate::purchase_detail::dtos::{
    CreatePurchaseDetailDto, UpdatePurchaseDetailDto,
};
use crate::purchase_detail::services::purchase_detail_service::PurchaseDetailService;
use actix_web::{web, HttpResponse, Responder};
use log::{error, info};
use sea_orm::DatabaseConnection;
use uuid::Uuid;

pub async fn get_purchase_detail(
    db: web::Data<DatabaseConnection>,
    id: web::Path<Uuid>,
) -> impl Responder {
    let detail_id = id.into_inner();
    info!("Fetching purchase detail with id: {}", detail_id);
    let db = db.get_ref();
    let result = PurchaseDetailService::get_purchase_detail(db, detail_id).await;
    match result {
        Ok(Some(detail)) => HttpResponse::Ok().json(detail),
        Ok(None) => HttpResponse::NotFound().body("Purchase detail not found"),
        Err(e) => {
            error!("Failed to fetch purchase detail: {}", e);
            HttpResponse::InternalServerError().body("Internal server error")
        }
    }
}

pub async fn get_all_purchase_details(db: web::Data<DatabaseConnection>) -> impl Responder {
    info!("Fetching all purchase details");
    let db = db.get_ref();
    let result = PurchaseDetailService::get_all_purchase_details(db).await;
    match result {
        Ok(details) => HttpResponse::Ok().json(details),
        Err(e) => {
            error!("Failed to fetch purchase details: {}", e);
            HttpResponse::InternalServerError().body("Internal server error")
        }
    }
}

pub async fn create_purchase_detail(
    db: web::Data<DatabaseConnection>,
    detail_dto: web::Json<CreatePurchaseDetailDto>,
) -> impl Responder {
    info!("Creating new purchase detail: {:?}", detail_dto);
    let db = db.get_ref();
    let result =
        PurchaseDetailService::create_purchase_detail(db, detail_dto.into_inner()).await;
    match result {
        Ok(detail) => HttpResponse::Ok().json(detail),
        Err(e) => {
            error!("Failed to create purchase detail: {}", e);
            HttpResponse::InternalServerError().body("Internal server error")
        }
    }
}

pub async fn update_purchase_detail(
    db: web::Data<DatabaseConnection>,
    id: web::Path<Uuid>,
    detail_dto: web::Json<UpdatePurchaseDetailDto>,
) -> impl Responder {
    let detail_id = id.into_inner();
    info!("Updating purchase detail with id: {}", detail_id);
    let db = db.get_ref();
    let result = PurchaseDetailService::update_purchase_detail(
        db,
        detail_id,
        detail_dto.into_inner(),
    )
    .await;
    match result {
        Ok(detail) => HttpResponse::Ok().json(detail),
        Err(e) => {
            error!("Failed to update purchase detail: {}", e);
            HttpResponse::InternalServerError().body("Internal server error")
        }
    }
}

pub async fn delete_purchase_detail(
    db: web::Data<DatabaseConnection>,
    id: web::Path<Uuid>,
) -> impl Responder {
    let detail_id = id.into_inner();
    info!("Deleting purchase detail with id: {}", detail_id);
    let db = db.get_ref();
    let result = PurchaseDetailService::delete_purchase_detail(db, detail_id).await;
    match result {
        Ok(_) => HttpResponse::Ok().body("Purchase detail deleted"),
        Err(e) => {
            error!("Failed to delete purchase detail: {}", e);
            HttpResponse::InternalServerError().body("Internal server error")
        }
    }
}
