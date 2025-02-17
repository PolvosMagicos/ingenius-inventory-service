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
    let db = db.get_ref();
    let util = UtilListService::get_util_list(db, id.into_inner()).await;

    match util {
        Ok(Some(util)) => HttpResponse::Ok().json(util),
        Ok(None) => HttpResponse::NotFound().body("Util not found"),
        Err(_) => HttpResponse::InternalServerError().body("Internal server error"),
    }
}

pub async fn get_all_util_lists(db: web::Data<DatabaseConnection>) -> impl Responder {
    let db = db.get_ref();
    let utils = UtilListService::get_all_util_lists(db).await;

    match utils {
        Ok(utils) => HttpResponse::Ok().json(utils),
        Err(_) => HttpResponse::InternalServerError().body("Internal server error"),
    }
}

pub async fn create_util_list(
    db: web::Data<DatabaseConnection>,
    util_list_dto: web::Json<CreateUtilListDto>,
) -> impl Responder {
    let db = db.get_ref();

    let result = UtilListService::create_util_list(db, util_list_dto.into_inner()).await;

    match result {
        Ok(util) => {
            info!("Util created successfully: {:?}", util);
            HttpResponse::Ok().json(util)
        }
        Err(e) => {
            error!("Error creating util: {}", e);
            HttpResponse::InternalServerError().body("Internal server error")
        }
    }
}

pub async fn update_util_list(
    db: web::Data<DatabaseConnection>,
    id: web::Path<Uuid>,
    util_dto: web::Json<UpdateUtilListDto>,
) -> impl Responder {
    let db = db.get_ref();
    let result =
        UtilListService::update_util_list(db, id.into_inner(), util_dto.into_inner()).await;

    match result {
        Ok(util) => HttpResponse::Ok().json(util),
        Err(_) => HttpResponse::InternalServerError().body("Internal server error"),
    }
}

pub async fn delete_util_list(
    db: web::Data<DatabaseConnection>,
    id: web::Path<Uuid>,
) -> impl Responder {
    let db = db.get_ref();
    let result = UtilListService::delete_util_list(db, id.into_inner()).await;

    match result {
        Ok(_) => HttpResponse::Ok().body("Util deleted"),
        Err(_) => HttpResponse::InternalServerError().body("Internal server error"),
    }
}
