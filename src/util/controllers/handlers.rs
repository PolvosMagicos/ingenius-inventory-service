use crate::util::{
    dtos::{CreateUtilDto, UpdateUtilDto},
    services::util_services::UtilService,
};
use actix_web::{web, HttpResponse, Responder};
use log::{error, info};
use sea_orm::DatabaseConnection;
use uuid::Uuid;

pub async fn get_util(db: web::Data<DatabaseConnection>, id: web::Path<Uuid>) -> impl Responder {
    let util_id = id.into_inner();
    info!("Fetching util with id: {}", util_id);
    let db = db.get_ref();
    let util = UtilService::get_util(db, util_id).await;

    match util {
        Ok(Some(util)) => {
            info!("Successfully fetched util: {:?}", util);
            HttpResponse::Ok().json(util)
        }
        Ok(None) => {
            info!("Util not found with id: {}", util_id);
            HttpResponse::NotFound().body("Util not found")
        }
        Err(e) => {
            error!("Failed to fetch util with id: {}: {}", util_id, e);
            HttpResponse::InternalServerError().body("Internal server error")
        }
    }
}

pub async fn get_all_utils(db: web::Data<DatabaseConnection>) -> impl Responder {
    info!("Fetching all utils");
    let db = db.get_ref();
    let utils = UtilService::get_all_utils(db).await;

    match utils {
        Ok(utils) => {
            info!("Successfully fetched {} utils", utils.len());
            HttpResponse::Ok().json(utils)
        }
        Err(e) => {
            error!("Failed to fetch utils: {}", e);
            HttpResponse::InternalServerError().body("Internal server error")
        }
    }
}

pub async fn create_util(
    db: web::Data<DatabaseConnection>,
    util_dto: web::Json<CreateUtilDto>,
) -> impl Responder {
    info!("Creating new util with data: {:?}", util_dto);
    let db = db.get_ref();

    let result = UtilService::create_util(db, util_dto.into_inner()).await;

    match result {
        Ok(util) => {
            info!("Successfully created util: {:?}", util);
            HttpResponse::Ok().json(util)
        }
        Err(e) => {
            error!("Failed to create util: {}", e);
            HttpResponse::InternalServerError().body("Internal server error")
        }
    }
}

pub async fn update_util(
    db: web::Data<DatabaseConnection>,
    id: web::Path<Uuid>,
    util_dto: web::Json<UpdateUtilDto>,
) -> impl Responder {
    let util_id = id.into_inner();
    info!(
        "Updating util with id: {} and data: {:?}",
        util_id, util_dto
    );
    let db = db.get_ref();
    let result = UtilService::update_util(db, util_id, util_dto.into_inner()).await;

    match result {
        Ok(util) => {
            info!("Successfully updated util: {:?}", util);
            HttpResponse::Ok().json(util)
        }
        Err(e) => {
            error!("Failed to update util with id: {}: {}", util_id, e);
            HttpResponse::InternalServerError().body("Internal server error")
        }
    }
}

pub async fn delete_util(db: web::Data<DatabaseConnection>, id: web::Path<Uuid>) -> impl Responder {
    let util_id = id.into_inner();
    info!("Deleting util with id: {}", util_id);
    let db = db.get_ref();
    let result = UtilService::delete_util(db, util_id).await;

    match result {
        Ok(_) => {
            info!("Successfully deleted util with id: {}", util_id);
            HttpResponse::Ok().body("Util deleted")
        }
        Err(e) => {
            error!("Failed to delete util with id: {}: {}", util_id, e);
            HttpResponse::InternalServerError().body("Internal server error")
        }
    }
}
