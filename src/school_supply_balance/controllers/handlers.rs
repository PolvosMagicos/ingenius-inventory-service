use crate::school_supply_balance::{
    dtos::{CreateSchoolSupplyBalanceDto, UpdateSchoolSupplyBalanceDto},
    services::school_supply_balance_services::SchoolSupplyBalanceService,
};
use actix_web::{web, HttpResponse, Responder};
use log::{error, info};
use sea_orm::DatabaseConnection;
use uuid::Uuid;

pub async fn get_school_supply_balance(
    db: web::Data<DatabaseConnection>,
    id: web::Path<Uuid>,
) -> impl Responder {
    let balance_id = id.into_inner();
    info!("Fetching school supply balance with id: {}", balance_id);
    let db = db.get_ref();
    let balance = SchoolSupplyBalanceService::get_school_supply_balance(db, balance_id).await;

    match balance {
        Ok(Some(balance)) => {
            info!("Successfully fetched balance: {:?}", balance);
            HttpResponse::Ok().json(balance)
        }
        Ok(None) => {
            info!("Balance not found with id: {}", balance_id);
            HttpResponse::NotFound().body("Balance not found")
        }
        Err(e) => {
            error!("Failed to fetch balance with id: {}: {}", balance_id, e);
            HttpResponse::InternalServerError().body("Internal server error")
        }
    }
}

pub async fn get_all_school_supply_balances(db: web::Data<DatabaseConnection>) -> impl Responder {
    info!("Fetching all school supply balances");
    let db = db.get_ref();
    let balances = SchoolSupplyBalanceService::get_all_school_supply_balances(db).await;

    match balances {
        Ok(balances) => {
            info!("Successfully fetched {} balances", balances.len());
            HttpResponse::Ok().json(balances)
        }
        Err(e) => {
            error!("Failed to fetch balances: {}", e);
            HttpResponse::InternalServerError().body("Internal server error")
        }
    }
}

pub async fn create_school_supply_balance(
    db: web::Data<DatabaseConnection>,
    balance_dto: web::Json<CreateSchoolSupplyBalanceDto>,
) -> impl Responder {
    info!(
        "Creating new school supply balance with data: {:?}",
        balance_dto
    );
    let db = db.get_ref();
    let result =
        SchoolSupplyBalanceService::create_school_supply_balance(db, balance_dto.into_inner())
            .await;

    match result {
        Ok(balance) => {
            info!("Successfully created balance: {:?}", balance);
            HttpResponse::Ok().json(balance)
        }
        Err(e) => {
            error!("Failed to create balance: {}", e);
            HttpResponse::InternalServerError().body("Internal server error")
        }
    }
}

pub async fn update_school_supply_balance(
    db: web::Data<DatabaseConnection>,
    id: web::Path<Uuid>,
    balance_dto: web::Json<UpdateSchoolSupplyBalanceDto>,
) -> impl Responder {
    let balance_id = id.into_inner();
    info!(
        "Updating school supply balance with id: {} and data: {:?}",
        balance_id, balance_dto
    );
    let db = db.get_ref();
    let result = SchoolSupplyBalanceService::update_school_supply_balance(
        db,
        balance_id,
        balance_dto.into_inner(),
    )
    .await;

    match result {
        Ok(balance) => {
            info!("Successfully updated balance: {:?}", balance);
            HttpResponse::Ok().json(balance)
        }
        Err(e) => {
            error!("Failed to update balance with id: {}: {}", balance_id, e);
            HttpResponse::InternalServerError().body("Internal server error")
        }
    }
}

pub async fn delete_school_supply_balance(
    db: web::Data<DatabaseConnection>,
    id: web::Path<Uuid>,
) -> impl Responder {
    let balance_id = id.into_inner();
    info!("Deleting school supply balance with id: {}", balance_id);
    let db = db.get_ref();
    let result = SchoolSupplyBalanceService::delete_school_supply_balance(db, balance_id).await;

    match result {
        Ok(_) => {
            info!("Successfully deleted balance with id: {}", balance_id);
            HttpResponse::Ok().body("School supply balance deleted")
        }
        Err(e) => {
            error!("Failed to delete balance with id: {}: {}", balance_id, e);
            HttpResponse::InternalServerError().body("Internal server error")
        }
    }
}
