use crate::delivery::{
    dtos::{CreateDeliveryDto, UpdateDeliveryDto},
    services::delivery_services::DeliveryService,
};
use actix_web::{web, HttpResponse, Responder};
use log::{error, info};
use sea_orm::DatabaseConnection;
use uuid::Uuid;

pub async fn get_delivery(
    db: web::Data<DatabaseConnection>,
    id: web::Path<Uuid>,
) -> impl Responder {
    let delivery_id = id.into_inner();
    info!("Fetching delivery with id: {}", delivery_id);
    let db = db.get_ref();
    let delivery = DeliveryService::get_delivery(db, delivery_id).await;

    match delivery {
        Ok(Some(delivery)) => {
            info!("Successfully fetched delivery: {:?}", delivery);
            HttpResponse::Ok().json(delivery)
        }
        Ok(None) => {
            info!("Delivery not found with id: {}", delivery_id);
            HttpResponse::NotFound().body("Delivery not found")
        }
        Err(e) => {
            error!("Failed to fetch delivery with id: {}: {}", delivery_id, e);
            HttpResponse::InternalServerError().body("Internal server error")
        }
    }
}

pub async fn get_all_deliveries(db: web::Data<DatabaseConnection>) -> impl Responder {
    info!("Fetching all deliveries");
    let db = db.get_ref();
    let deliveries = DeliveryService::get_all_deliveries(db).await;

    match deliveries {
        Ok(deliveries) => {
            info!("Successfully fetched {} deliveries", deliveries.len());
            HttpResponse::Ok().json(deliveries)
        }
        Err(e) => {
            error!("Failed to fetch deliveries: {}", e);
            HttpResponse::InternalServerError().body("Internal server error")
        }
    }
}

pub async fn create_delivery(
    db: web::Data<DatabaseConnection>,
    delivery_dto: web::Json<CreateDeliveryDto>,
) -> impl Responder {
    info!("Creating new delivery with data: {:?}", delivery_dto);
    let db = db.get_ref();
    let result = DeliveryService::create_delivery(db, delivery_dto.into_inner()).await;

    match result {
        Ok(delivery) => {
            info!("Successfully created delivery: {:?}", delivery);
            HttpResponse::Ok().json(delivery)
        }
        Err(e) => {
            error!("Failed to create delivery: {}", e);
            HttpResponse::InternalServerError().body("Internal server error")
        }
    }
}

pub async fn update_delivery(
    db: web::Data<DatabaseConnection>,
    id: web::Path<Uuid>,
    delivery_dto: web::Json<UpdateDeliveryDto>,
) -> impl Responder {
    let delivery_id = id.into_inner();
    info!(
        "Updating delivery with id: {} and data: {:?}",
        delivery_id, delivery_dto
    );
    let db = db.get_ref();
    let result =
        DeliveryService::update_delivery(db, delivery_id, delivery_dto.into_inner()).await;

    match result {
        Ok(delivery) => {
            info!("Successfully updated delivery: {:?}", delivery);
            HttpResponse::Ok().json(delivery)
        }
        Err(e) => {
            error!(
                "Failed to update delivery with id: {}: {}",
                delivery_id, e
            );
            HttpResponse::InternalServerError().body("Internal server error")
        }
    }
}

pub async fn delete_delivery(
    db: web::Data<DatabaseConnection>,
    id: web::Path<Uuid>,
) -> impl Responder {
    let delivery_id = id.into_inner();
    info!("Deleting delivery with id: {}", delivery_id);
    let db = db.get_ref();
    let result = DeliveryService::delete_delivery(db, delivery_id).await;

    match result {
        Ok(_) => {
            info!("Successfully deleted delivery with id: {}", delivery_id);
            HttpResponse::Ok().body("Delivery deleted")
        }
        Err(e) => {
            error!(
                "Failed to delete delivery with id: {}: {}",
                delivery_id, e
            );
            HttpResponse::InternalServerError().body("Internal server error")
        }
    }
}
