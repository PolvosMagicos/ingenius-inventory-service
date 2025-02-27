use actix_cors::Cors;
use actix_web::{web, App, HttpServer};
use db::establish_connection;

mod auth;
mod classroom;
mod db;
mod delivery;
mod list_detail;
mod request;
mod request_detail;
mod student;
mod user;
mod util;
mod util_list;
mod utils_delivery;

pub fn configure(cfg: &mut web::ServiceConfig) {
    // Public routes (no authentication required)
    cfg.service(
        web::scope("/auth")
            .route(
                "/register",
                web::post().to(auth::controllers::handlers::register),
            )
            .route("/login", web::post().to(auth::controllers::handlers::login)),
    );

    // Protected routes (require JWT authentication)
    cfg.service(
        web::scope("/api")
            .wrap(auth::middleware::auth_middleware::AuthMiddleware::new(
                std::env::var("JWT_SECRET").unwrap_or_else(|_| "your_secret_key".to_string()),
            ))
            .service(web::scope("/classroom").configure(classroom::routes::config))
            .service(web::scope("/util").configure(util::routes::config))
            .service(web::scope("/util-list").configure(util_list::routes::config))
            .service(web::scope("/list-detail").configure(list_detail::routes::config))
            .service(web::scope("/user").configure(user::routes::config))
            .service(web::scope("/request").configure(request::routes::config))
            .service(web::scope("/request-detail").configure(request_detail::routes::config))
            .service(web::scope("/delivery").configure(delivery::routes::config))
            .service(web::scope("/utils-delivery").configure(utils_delivery::routes::config))
            .service(web::scope("/student").configure(student::routes::config)),
    );
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    let jwt_secret = std::env::var("JWT_SECRET").unwrap_or_else(|_| {
        println!("Warning: JWT_SECRET not set, using default secret");
        "your_secret_key".to_string()
    });

    println!("Starting application...");

    // Establish database connection
    let db = establish_connection().await;
    let db_pool = web::Data::new(db.clone());

    let auth_service = web::Data::new(auth::services::auth_service::AuthService::new(
        db.clone(),
        jwt_secret.clone(),
    ));

    println!("Starting HTTP server...");

    let server = HttpServer::new(move || {
        App::new()
            .app_data(db_pool.clone())
            .app_data(auth_service.clone())
            .wrap(actix_web::middleware::Logger::default())
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allowed_methods(vec!["GET", "POST", "PUT", "PATCH", "DELETE"])
                    .allowed_headers(vec![
                        actix_web::http::header::AUTHORIZATION,
                        actix_web::http::header::ACCEPT,
                        actix_web::http::header::CONTENT_TYPE,
                    ])
                    .max_age(3600),
            )
            .configure(configure)
    })
    .bind(("0.0.0.0", 8080));

    let server = match server {
        Ok(s) => s,
        Err(e) => {
            eprintln!("Failed to bind to address: {}", e);
            return Err(e);
        }
    };

    println!("Server bound to 0.0.0.0:8080");

    match server.run().await {
        Ok(_) => {
            println!("Server shutdown successfully");
            Ok(())
        }
        Err(e) => {
            eprintln!("Server error: {}", e);
            Err(e)
        }
    }
}
