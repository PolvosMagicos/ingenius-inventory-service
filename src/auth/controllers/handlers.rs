use crate::auth::dtos::{LoginUserDto, RegisterUserDto};
use crate::auth::services::auth_service::AuthService;
use actix_web::{web, HttpResponse, Responder};

pub async fn register(
    service: web::Data<AuthService>,
    dto: web::Json<RegisterUserDto>,
) -> impl Responder {
    match service.register(dto.into_inner()).await {
        Ok(response) => HttpResponse::Ok().json(response),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

pub async fn login(
    service: web::Data<AuthService>,
    dto: web::Json<LoginUserDto>,
) -> impl Responder {
    match service.login(dto.into_inner()).await {
        Ok(Some(response)) => HttpResponse::Ok().json(response),
        Ok(None) => HttpResponse::Unauthorized().finish(),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}
