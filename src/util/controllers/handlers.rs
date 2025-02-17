use crate::util::{
    dtos::{CreateUtilDto, UpdateUtilDto},
    services::util_services::UtilService,
};
use actix_web::{web, HttpResponse, Responder};
use log::{error, info};
use sea_orm::DatabaseConnection;
use uuid::Uuid;

pub async fn get_util(db: web::Data<DatabaseConnection>, id: web::Path<Uuid>) -> impl Responder {
    let db = db.get_ref();
    let util = UtilService::get_util(db, id.into_inner()).await;

    match util {
        Ok(Some(util)) => HttpResponse::Ok().json(util),
        Ok(None) => HttpResponse::NotFound().body("Util not found"),
        Err(_) => HttpResponse::InternalServerError().body("Internal server error"),
    }
}

pub async fn get_all_utils(db: web::Data<DatabaseConnection>) -> impl Responder {
    let db = db.get_ref();
    let utils = UtilService::get_all_utils(db).await;

    match utils {
        Ok(utils) => HttpResponse::Ok().json(utils),
        Err(_) => HttpResponse::InternalServerError().body("Internal server error"),
    }
}

pub async fn create_util(
    db: web::Data<DatabaseConnection>,
    util_dto: web::Json<CreateUtilDto>,
) -> impl Responder {
    let db = db.get_ref();
    info!("Creating new util: {:?}", util_dto);

    let result = UtilService::create_util(db, util_dto.into_inner()).await;

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

pub async fn update_util(
    db: web::Data<DatabaseConnection>,
    id: web::Path<Uuid>,
    util_dto: web::Json<UpdateUtilDto>,
) -> impl Responder {
    let db = db.get_ref();
    let result = UtilService::update_util(db, id.into_inner(), util_dto.into_inner()).await;

    match result {
        Ok(util) => HttpResponse::Ok().json(util),
        Err(_) => HttpResponse::InternalServerError().body("Internal server error"),
    }
}

pub async fn delete_util(db: web::Data<DatabaseConnection>, id: web::Path<Uuid>) -> impl Responder {
    let db = db.get_ref();
    let result = UtilService::delete_util(db, id.into_inner()).await;

    match result {
        Ok(_) => HttpResponse::Ok().body("Util deleted"),
        Err(_) => HttpResponse::InternalServerError().body("Internal server error"),
    }
}
