use actix_web::{web, HttpResponse, Responder};
use log::{error, info};
use sea_orm::DatabaseConnection;
use uuid::Uuid;

use crate::list_detail::{
    dtos::{create_list_detail::CreateListDetailDto, update_list_detail::UpdateListDetailDto},
    services::list_detail_services::ListDetailService,
};

pub async fn get_list_detail(
    db: web::Data<DatabaseConnection>,
    id: web::Path<Uuid>,
) -> impl Responder {
    let list_detail_id = id.into_inner();
    info!("Fetching list detail with id: {}", list_detail_id);
    let db = db.get_ref();
    let list_detail = ListDetailService::get_list_detail(db, list_detail_id).await;

    match list_detail {
        Ok(Some(list_detail)) => {
            info!("Successfully fetched list detail: {:?}", list_detail);
            HttpResponse::Ok().json(list_detail)
        }
        Ok(None) => {
            info!("List detail not found with id: {}", list_detail_id);
            HttpResponse::NotFound().body("List detail not found")
        }
        Err(e) => {
            error!(
                "Failed to fetch list detail with id: {}: {}",
                list_detail_id, e
            );
            HttpResponse::InternalServerError().body("Internal server error")
        }
    }
}

pub async fn get_all_lists_details(db: web::Data<DatabaseConnection>) -> impl Responder {
    info!("Fetching all list detail");
    let db = db.get_ref();
    let list_detail = ListDetailService::get_all_lists_details(db).await;

    match list_detail {
        Ok(list_detail) => {
            info!("Successfully fetched {} lists details", list_detail.len());
            HttpResponse::Ok().json(list_detail)
        }
        Err(e) => {
            error!("Failed to fetch lists details: {}", e);
            HttpResponse::InternalServerError().body("Internal server error")
        }
    }
}

pub async fn create_list_detail(
    db: web::Data<DatabaseConnection>,
    list_detail_dto: web::Json<CreateListDetailDto>,
) -> impl Responder {
    info!("Creating new list detail with data: {:?}", list_detail_dto);
    let db = db.get_ref();

    let result = ListDetailService::create_list_detail(db, list_detail_dto.into_inner()).await;

    match result {
        Ok(list_detail) => {
            info!("Successfully created list detail: {:?}", list_detail);
            HttpResponse::Ok().json(list_detail)
        }
        Err(e) => {
            error!("Failed to create list detail: {}", e);
            HttpResponse::InternalServerError().body("Internal server error")
        }
    }
}

pub async fn update_list_detail(
    db: web::Data<DatabaseConnection>,
    id: web::Path<Uuid>,
    list_detail_dto: web::Json<UpdateListDetailDto>,
) -> impl Responder {
    let list_detail_id = id.into_inner();
    info!(
        "Updating list detail with id: {} and data: {:?}",
        list_detail_id, list_detail_dto
    );
    let db = db.get_ref();
    let result =
        ListDetailService::update_list_detail(db, list_detail_id, list_detail_dto.into_inner())
            .await;

    match result {
        Ok(list_detail) => {
            info!("Successfully updated list detail: {:?}", list_detail);
            HttpResponse::Ok().json(list_detail)
        }
        Err(e) => {
            error!(
                "Failed to update list detail with id: {}: {}",
                list_detail_id, e
            );
            HttpResponse::InternalServerError().body("Internal server error")
        }
    }
}

pub async fn delete_list_detail(
    db: web::Data<DatabaseConnection>,
    id: web::Path<Uuid>,
) -> impl Responder {
    let list_detail_id = id.into_inner();
    info!("Deleting list detail with id: {}", list_detail_id);
    let db = db.get_ref();
    let result = ListDetailService::delete_list_detail(db, list_detail_id).await;

    match result {
        Ok(_) => {
            info!(
                "Successfully deleted list detail with id: {}",
                list_detail_id
            );
            HttpResponse::Ok().body("List detail deleted")
        }
        Err(e) => {
            error!(
                "Failed to delete list detail with id: {}: {}",
                list_detail_id, e
            );
            HttpResponse::InternalServerError().body("Internal server error")
        }
    }
}
