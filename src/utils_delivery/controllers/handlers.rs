use crate::utils_delivery::{
    dtos::{CreateUtilsDeliveryDto, UpdateUtilsDeliveryDto},
    services::utils_delivery_service::UtilsDeliveryService,
};
use actix_web::{web, HttpResponse, Responder};
use log::{error, info};
use sea_orm::DatabaseConnection;
use uuid::Uuid;

pub async fn get_utils_delivery(
    db: web::Data<DatabaseConnection>,
    id: web::Path<Uuid>,
) -> impl Responder {
    let utils_delivery_id = id.into_inner();
    info!("Fetching utils delivery with id: {}", utils_delivery_id);
    let db = db.get_ref();
    let utils_delivery = UtilsDeliveryService::get_utils_delivery(db, utils_delivery_id).await;
    match utils_delivery {
        Ok(Some(utils_delivery)) => {
            info!("Successfully fetched utils delivery: {:?}", utils_delivery);
            HttpResponse::Ok().json(utils_delivery)
        }
        Ok(None) => {
            info!("Utils delivery not found with id: {}", utils_delivery_id);
            HttpResponse::NotFound().body("Utils delivery not found")
        }
        Err(e) => {
            error!(
                "Failed to fetch utils delivery with id: {}: {}",
                utils_delivery_id, e
            );
            HttpResponse::InternalServerError().body("Internal server error")
        }
    }
}

pub async fn get_all_utils_deliveries(db: web::Data<DatabaseConnection>) -> impl Responder {
    info!("Fetching all utils deliveries");
    let db = db.get_ref();
    let utils_deliveries = UtilsDeliveryService::get_all_utils_deliveries(db).await;
    match utils_deliveries {
        Ok(utils_deliveries) => {
            info!(
                "Successfully fetched {} utils deliveries",
                utils_deliveries.len()
            );
            HttpResponse::Ok().json(utils_deliveries)
        }
        Err(e) => {
            error!("Failed to fetch utils deliveries: {}", e);
            HttpResponse::InternalServerError().body("Internal server error")
        }
    }
}

pub async fn get_utils_deliveries_by_delivery(
    db: web::Data<DatabaseConnection>,
    delivery_id: web::Path<Uuid>,
) -> impl Responder {
    let delivery_id = delivery_id.into_inner();
    info!("Fetching utils deliveries for delivery id: {}", delivery_id);
    let db = db.get_ref();
    let result = UtilsDeliveryService::get_utils_deliveries_by_delivery(db, delivery_id).await;
    match result {
        Ok(utils_deliveries) => {
            info!(
                "Successfully fetched {} utils deliveries for delivery id: {}",
                utils_deliveries.len(),
                delivery_id
            );
            HttpResponse::Ok().json(utils_deliveries)
        }
        Err(e) => {
            error!(
                "Failed to fetch utils deliveries for delivery id {}: {}",
                delivery_id, e
            );
            HttpResponse::InternalServerError().body("Internal server error")
        }
    }
}

pub async fn create_utils_delivery(
    db: web::Data<DatabaseConnection>,
    utils_delivery_dto: web::Json<CreateUtilsDeliveryDto>,
) -> impl Responder {
    info!(
        "Creating new utils delivery with data: {:?}",
        utils_delivery_dto
    );
    let db = db.get_ref();
    let result =
        UtilsDeliveryService::create_utils_delivery(db, utils_delivery_dto.into_inner()).await;
    match result {
        Ok(utils_delivery) => {
            info!("Successfully created utils delivery: {:?}", utils_delivery);
            HttpResponse::Ok().json(utils_delivery)
        }
        Err(e) => {
            error!("Failed to create utils delivery: {}", e);
            HttpResponse::InternalServerError().body("Internal server error")
        }
    }
}

pub async fn update_utils_delivery(
    db: web::Data<DatabaseConnection>,
    id: web::Path<Uuid>,
    utils_delivery_dto: web::Json<UpdateUtilsDeliveryDto>,
) -> impl Responder {
    let utils_delivery_id = id.into_inner();
    info!(
        "Updating utils delivery with id: {} and data: {:?}",
        utils_delivery_id, utils_delivery_dto
    );
    let db = db.get_ref();
    let result = UtilsDeliveryService::update_utils_delivery(
        db,
        utils_delivery_id,
        utils_delivery_dto.into_inner(),
    )
    .await;
    match result {
        Ok(utils_delivery) => {
            info!("Successfully updated utils delivery: {:?}", utils_delivery);
            HttpResponse::Ok().json(utils_delivery)
        }
        Err(e) => {
            error!(
                "Failed to update utils delivery with id: {}: {}",
                utils_delivery_id, e
            );
            HttpResponse::InternalServerError().body("Internal server error")
        }
    }
}

pub async fn delete_utils_delivery(
    db: web::Data<DatabaseConnection>,
    id: web::Path<Uuid>,
) -> impl Responder {
    let utils_delivery_id = id.into_inner();
    info!("Deleting utils delivery with id: {}", utils_delivery_id);
    let db = db.get_ref();
    let result = UtilsDeliveryService::delete_utils_delivery(db, utils_delivery_id).await;
    match result {
        Ok(_) => {
            info!(
                "Successfully deleted utils delivery with id: {}",
                utils_delivery_id
            );
            HttpResponse::Ok().body("Utils delivery deleted")
        }
        Err(e) => {
            error!(
                "Failed to delete utils delivery with id: {}: {}",
                utils_delivery_id, e
            );
            HttpResponse::InternalServerError().body("Internal server error")
        }
    }
}
