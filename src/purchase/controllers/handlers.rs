use crate::purchase::{
    dtos::{CreatePurchaseDto, UpdatePurchaseDto},
    services::purchase_service::PurchaseService,
};
use actix_web::{web, HttpResponse, Responder};
use log::{error, info};
use sea_orm::DatabaseConnection;
use uuid::Uuid;

pub async fn get_purchase(db: web::Data<DatabaseConnection>, id: web::Path<Uuid>) -> impl Responder {
    let purchase_id = id.into_inner();
    info!("Fetching purchase with id: {}", purchase_id);
    let db = db.get_ref();
    let purchase = PurchaseService::get_purchase(db, purchase_id).await;

    match purchase {
        Ok(Some(purchase)) => {
            info!("Successfully fetched purchase: {:?}", purchase);
            HttpResponse::Ok().json(purchase)
        }
        Ok(None) => {
            info!("Purchase not found with id: {}", purchase_id);
            HttpResponse::NotFound().body("Purchase not found")
        }
        Err(e) => {
            error!("Failed to fetch purchase with id: {}: {}", purchase_id, e);
            HttpResponse::InternalServerError().body("Internal server error")
        }
    }
}

pub async fn get_all_purchases(db: web::Data<DatabaseConnection>) -> impl Responder {
    info!("Fetching all purchases");
    let db = db.get_ref();
    let purchases = PurchaseService::get_all_purchases(db).await;

    match purchases {
        Ok(purchases) => {
            info!("Successfully fetched {} purchases", purchases.len());
            HttpResponse::Ok().json(purchases)
        }
        Err(e) => {
            error!("Failed to fetch purchases: {}", e);
            HttpResponse::InternalServerError().body("Internal server error")
        }
    }
}

pub async fn get_purchases_by_user(
    db: web::Data<DatabaseConnection>,
    id: web::Path<Uuid>,
) -> impl Responder {
    let user_id = id.into_inner();
    info!("Fetching purchases for user with id: {}", user_id);
    let db = db.get_ref();
    let purchases = PurchaseService::get_purchases_by_user(db, user_id).await;

    match purchases {
        Ok(purchases) => {
            info!(
                "Successfully fetched {} purchases for user {}",
                purchases.len(),
                user_id
            );
            HttpResponse::Ok().json(purchases)
        }
        Err(e) => {
            error!("Failed to fetch purchases for user {}: {}", user_id, e);
            HttpResponse::InternalServerError().body("Internal server error")
        }
    }
}

pub async fn create_purchase(
    db: web::Data<DatabaseConnection>,
    purchase_dto: web::Json<CreatePurchaseDto>,
) -> impl Responder {
    info!("Creating new purchase with data: {:?}", purchase_dto);
    let db = db.get_ref();
    let result = PurchaseService::create_purchase(db, purchase_dto.into_inner()).await;

    match result {
        Ok(purchase) => {
            info!("Successfully created purchase: {:?}", purchase);
            HttpResponse::Ok().json(purchase)
        }
        Err(e) => {
            error!("Failed to create purchase: {}", e);
            HttpResponse::InternalServerError().body("Internal server error")
        }
    }
}

pub async fn update_purchase(
    db: web::Data<DatabaseConnection>,
    id: web::Path<Uuid>,
    purchase_dto: web::Json<UpdatePurchaseDto>,
) -> impl Responder {
    let purchase_id = id.into_inner();
    info!(
        "Updating purchase with id: {} and data: {:?}",
        purchase_id, purchase_dto
    );
    let db = db.get_ref();
    let result = PurchaseService::update_purchase(db, purchase_id, purchase_dto.into_inner()).await;

    match result {
        Ok(purchase) => {
            info!("Successfully updated purchase: {:?}", purchase);
            HttpResponse::Ok().json(purchase)
        }
        Err(e) => {
            error!("Failed to update purchase with id: {}: {}", purchase_id, e);
            HttpResponse::InternalServerError().body("Internal server error")
        }
    }
}

pub async fn delete_purchase(
    db: web::Data<DatabaseConnection>,
    id: web::Path<Uuid>,
) -> impl Responder {
    let purchase_id = id.into_inner();
    info!("Deleting purchase with id: {}", purchase_id);
    let db = db.get_ref();
    let result = PurchaseService::delete_purchase(db, purchase_id).await;

    match result {
        Ok(_) => {
            info!("Successfully deleted purchase with id: {}", purchase_id);
            HttpResponse::Ok().body("Purchase deleted")
        }
        Err(e) => {
            error!("Failed to delete purchase with id: {}: {}", purchase_id, e);
            HttpResponse::InternalServerError().body("Internal server error")
        }
    }
}
