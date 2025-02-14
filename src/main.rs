use actix_web::{web, App, HttpServer};
use entity::{Classroom, ListDetail, Request, Student, User, Util, UtilsList};
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

                // Create table statements
                let stmt_utils_list = builder.build(&schema.create_table_from_entity(UtilsList));
                let stmt_util = builder.build(&schema.create_table_from_entity(Util));
                let stmt_list_detail = builder.build(&schema.create_table_from_entity(ListDetail));
                let stmt_classroom = builder.build(&schema.create_table_from_entity(Classroom));
                let stmt_student = builder.build(&schema.create_table_from_entity(Student));
                let stmt_request = builder.build(&schema.create_table_from_entity(Request));
                let stmt_user = builder.build(&schema.create_table_from_entity(User));

                // Execute the create table statements in the correct order
                let results = vec![
                    ("UtilsList", db.execute(stmt_utils_list).await),
                    ("Util", db.execute(stmt_util).await),
                    ("ListDetail", db.execute(stmt_list_detail).await),
                    ("Classroom", db.execute(stmt_classroom).await),
                    ("Student", db.execute(stmt_student).await),
                    ("Request", db.execute(stmt_request).await),
                    ("User", db.execute(stmt_user).await),
                ];

                for (table_name, result) in results {
                    match result {
                        Ok(_) => println!("Table '{}' created successfully!", table_name),
                        Err(e) => println!("Error creating table '{}': {}", table_name, e),
                    }
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
