use crate::util_list::{
    dtos::{create_util_list::CreateUtilListDto, update_util_list::UpdateUtilListDto},
    services::util_list_services::UtilListService,
};
use actix_web::{web, HttpResponse, Responder};
use log::{error, info};
use sea_orm::DatabaseConnection;
use uuid::Uuid;

pub async fn get_util_list(
    db: web::Data<DatabaseConnection>,
    id: web::Path<Uuid>,
) -> impl Responder {
    let util_list_id = id.into_inner();
    info!("Fetching util list with id: {}", util_list_id);
    let db = db.get_ref();
    let util_list = UtilListService::get_util_list(db, util_list_id).await;

    match util_list {
        Ok(Some(util_list)) => {
            info!("Successfully fetched util list: {:?}", util_list);
            HttpResponse::Ok().json(util_list)
        }
        Ok(None) => {
            info!("Util list not found with id: {}", util_list_id);
            HttpResponse::NotFound().body("Util not found")
        }
        Err(e) => {
            error!("Failed to fetch util list with id: {}: {}", util_list_id, e);
            HttpResponse::InternalServerError().body("Internal server error")
        }
    }
}

pub async fn get_all_util_lists(db: web::Data<DatabaseConnection>) -> impl Responder {
    info!("Fetching all utils lists");
    let db = db.get_ref();
    let utils_lists = UtilListService::get_all_util_lists(db).await;

    match utils_lists {
        Ok(utils) => {
            info!("Successfully fetched {} utils lists", utils.len());
            HttpResponse::Ok().json(utils)
        }
        Err(e) => {
            error!("Failed to fetch utils lists: {}", e);
            HttpResponse::InternalServerError().body("Internal server error")
        }
    }
}

pub async fn create_util_list(
    db: web::Data<DatabaseConnection>,
    util_list_dto: web::Json<CreateUtilListDto>,
) -> impl Responder {
    info!("Creating new util list with data: {:?}", util_list_dto);
    let db = db.get_ref();

    let result = UtilListService::create_util_list(db, util_list_dto.into_inner()).await;

    match result {
        Ok(util_list) => {
            info!("Successfully created util list: {:?}", util_list);
            HttpResponse::Ok().json(util_list)
        }
        Err(e) => {
            error!("Failed to create util list: {}", e);
            HttpResponse::InternalServerError().body("Internal server error")
        }
    }
}

pub async fn update_util_list(
    db: web::Data<DatabaseConnection>,
    id: web::Path<Uuid>,
    util_dto: web::Json<UpdateUtilListDto>,
) -> impl Responder {
    let util_list_id = id.into_inner();
    info!(
        "Updating utils list with id: {} and data: {:?}",
        util_list_id, util_dto
    );
    let db = db.get_ref();
    let result = UtilListService::update_util_list(db, util_list_id, util_dto.into_inner()).await;

    match result {
        Ok(util_list_id) => {
            info!("Successfully updated utils list: {:?}", util_list_id);
            HttpResponse::Ok().json(util_list_id)
        }
        Err(e) => {
            error!(
                "Failed to update utils list with id: {}: {}",
                util_list_id, e
            );
            HttpResponse::InternalServerError().body("Internal server error")
        }
    }
}

pub async fn delete_util_list(
    db: web::Data<DatabaseConnection>,
    id: web::Path<Uuid>,
) -> impl Responder {
    let util_list_id = id.into_inner();
    info!("Deleting util list with id: {}", util_list_id);
    let db = db.get_ref();
    let result = UtilListService::delete_util_list(db, util_list_id).await;

    match result {
        Ok(_) => {
            info!("Successfully deleted utils list with id: {}", util_list_id);
            HttpResponse::Ok().body("Util deleted")
        }
        Err(e) => {
            error!(
                "Failed to delete utils list with id: {}: {}",
                util_list_id, e
            );
            HttpResponse::InternalServerError().body("Internal server error")
        }
    }
}
