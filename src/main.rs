use actix_web::{web, App, HttpServer};
use entity::Student;
use sea_orm::{ConnectionTrait, Database, DatabaseConnection, Schema};
use std::env;

async fn establish_connection() -> DatabaseConnection {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Wait for database to be ready
    let mut attempts = 0;
    let max_attempts = 5;

    println!("Trying to connect to db");
    while attempts < max_attempts {
        match Database::connect(&database_url).await {
            Ok(db) => {
                println!("Database connected successfully!");

                // Create the schema builder
                let builder = db.get_database_backend();
                let schema = Schema::new(builder);

                // Create table statement
                let stmt = builder.build(&schema.create_table_from_entity(Student));

                // Execute the create table statement
                match db.execute(stmt).await {
                    Ok(_) => println!("Table created successfully!"),
                    Err(e) => println!("Error creating table: {}", e),
                }

                return db;
            }
            Err(e) => {
                println!(
                    "Failed to connect to database: {}. Attempt {} of {}",
                    e,
                    attempts + 1,
                    max_attempts
                );
                tokio::time::sleep(tokio::time::Duration::from_secs(5)).await;
                attempts += 1;
            }
        }
    }

    panic!(
        "Failed to connect to database after {} attempts",
        max_attempts
    );
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    env_logger::init();

    println!("Starting application...");

    // Establish database connection
    let db = establish_connection().await;
    let db_pool = web::Data::new(db);

    println!("Starting HTTP server...");

    let server =
        HttpServer::new(move || App::new().app_data(db_pool.clone())).bind(("0.0.0.0", 8080));

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
