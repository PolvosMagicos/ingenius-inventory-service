use crate::money_delivery::{
    dtos::{
        create_money_delivery::CreateMoneyDeliveryDto,
        update_money_delivery::UpdateMoneyDeliveryDto,
    },
    services::money_delivery_services::MoneyDeliveryService,
};
use actix_web::{web, HttpResponse, Responder};
use log::{error, info};
use sea_orm::DatabaseConnection;
use uuid::Uuid;

pub async fn get_money_delivery(
    db: web::Data<DatabaseConnection>,
    id: web::Path<Uuid>,
) -> impl Responder {
    let money_delivery_id = id.into_inner();
    info!("Fetching money delivery with id: {}", money_delivery_id);
    let db = db.get_ref();
    let money_delivery = MoneyDeliveryService::get_money_delivery(db, money_delivery_id).await;

    match money_delivery {
        Ok(Some(money_delivery)) => {
            info!("Successfully fetched money delivery: {:?}", money_delivery);
            HttpResponse::Ok().json(money_delivery)
        }
        Ok(None) => {
            info!("Money delivery not found with id: {}", money_delivery_id);
            HttpResponse::NotFound().body("Money delivery not found")
        }
        Err(e) => {
            error!(
                "Failed to fetch money delivery with id: {}: {}",
                money_delivery_id, e
            );
            HttpResponse::InternalServerError().body("Internal server error")
        }
    }
}

pub async fn get_all_money_deliveries(db: web::Data<DatabaseConnection>) -> impl Responder {
    info!("Fetching all money deliveries");
    let db = db.get_ref();
    let money_deliveries = MoneyDeliveryService::get_all_money_deliveries(db).await;

    match money_deliveries {
        Ok(money_deliveries) => {
            info!(
                "Successfully fetched {} money deliveries",
                money_deliveries.len()
            );
            HttpResponse::Ok().json(money_deliveries)
        }
        Err(e) => {
            error!("Failed to fetch money deliveries: {}", e);
            HttpResponse::InternalServerError().body("Internal server error")
        }
    }
}

pub async fn create_money_delivery(
    db: web::Data<DatabaseConnection>,
    money_delivery_dto: web::Json<CreateMoneyDeliveryDto>,
) -> impl Responder {
    info!("Creating new money delivery with data: {:?}", money_delivery_dto);
    let db = db.get_ref();
    let result =
        MoneyDeliveryService::create_money_delivery(db, money_delivery_dto.into_inner()).await;

    match result {
        Ok(money_delivery) => {
            info!("Successfully created money delivery: {:?}", money_delivery);
            HttpResponse::Ok().json(money_delivery)
        }
        Err(e) => {
            error!("Failed to create money delivery: {}", e);
            HttpResponse::InternalServerError().body("Internal server error")
        }
    }
}

pub async fn update_money_delivery(
    db: web::Data<DatabaseConnection>,
    id: web::Path<Uuid>,
    money_delivery_dto: web::Json<UpdateMoneyDeliveryDto>,
) -> impl Responder {
    let money_delivery_id = id.into_inner();
    info!(
        "Updating money delivery with id: {} and data: {:?}",
        money_delivery_id, money_delivery_dto
    );
    let db = db.get_ref();
    let result = MoneyDeliveryService::update_money_delivery(
        db,
        money_delivery_id,
        money_delivery_dto.into_inner(),
    )
    .await;

    match result {
        Ok(money_delivery) => {
            info!("Successfully updated money delivery: {:?}", money_delivery);
            HttpResponse::Ok().json(money_delivery)
        }
        Err(e) => {
            error!(
                "Failed to update money delivery with id: {}: {}",
                money_delivery_id, e
            );
            HttpResponse::InternalServerError().body("Internal server error")
        }
    }
}

pub async fn delete_money_delivery(
    db: web::Data<DatabaseConnection>,
    id: web::Path<Uuid>,
) -> impl Responder {
    let money_delivery_id = id.into_inner();
    info!("Deleting money delivery with id: {}", money_delivery_id);
    let db = db.get_ref();
    let result = MoneyDeliveryService::delete_money_delivery(db, money_delivery_id).await;

    match result {
        Ok(_) => {
            info!(
                "Successfully deleted money delivery with id: {}",
                money_delivery_id
            );
            HttpResponse::Ok().body("Money delivery deleted")
        }
        Err(e) => {
            error!(
                "Failed to delete money delivery with id: {}: {}",
                money_delivery_id, e
            );
            HttpResponse::InternalServerError().body("Internal server error")
        }
    }
}
