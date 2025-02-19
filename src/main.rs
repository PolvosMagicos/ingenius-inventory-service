use actix_web::{web, App, HttpServer};

mod classroom;
mod db;
mod list_detail;
mod util;
mod util_list;

fn configure(cfg: &mut web::ServiceConfig) {
    classroom::routes::config(cfg);
    util::routes::config(cfg);
    util_list::routes::config(cfg);
    list_detail::routes::config(cfg);
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    println!("Starting application...");

    // Establish database connection
    let db = db::establish_connection().await;
    let db_pool = web::Data::new(db);

    println!("Starting HTTP server...");

    let server = HttpServer::new(move || App::new().app_data(db_pool.clone()).configure(configure))
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
