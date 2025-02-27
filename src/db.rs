use entity::{
    Classroom, Delivery, ListDetail, MoneyDelivery, Purchase, PurchaseDetail, Request,
    SchoolSupplyBalance, Student, User, Util, UtilsDelivery, UtilsDeliveryDetail, UtilsList,
};
use sea_orm::{ConnectionTrait, Database, DatabaseConnection, Schema};
use std::env;

pub async fn establish_connection() -> DatabaseConnection {
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
                let stmt_user = builder.build(&schema.create_table_from_entity(User));
                let stmt_request = builder.build(&schema.create_table_from_entity(Request));
                let stmt_purchase = builder.build(&schema.create_table_from_entity(Purchase));
                let stmt_purchase_detail =
                    builder.build(&schema.create_table_from_entity(PurchaseDetail));
                let stmt_delivery = builder.build(&schema.create_table_from_entity(Delivery));
                let stmt_money_delivery =
                    builder.build(&schema.create_table_from_entity(MoneyDelivery));
                let stmt_utils_delivery =
                    builder.build(&schema.create_table_from_entity(UtilsDelivery));
                let stmt_utils_delivery_detail =
                    builder.build(&schema.create_table_from_entity(UtilsDeliveryDetail));
                let stmt_school_supply_balance =
                    builder.build(&schema.create_table_from_entity(SchoolSupplyBalance));

                // Execute the create table statements in the correct order
                let results = vec![
                    ("UtilsList", db.execute(stmt_utils_list).await),
                    ("Util", db.execute(stmt_util).await),
                    ("ListDetail", db.execute(stmt_list_detail).await),
                    ("Classroom", db.execute(stmt_classroom).await),
                    ("Student", db.execute(stmt_student).await),
                    ("User", db.execute(stmt_user).await),
                    ("Request", db.execute(stmt_request).await),
                    ("RequestDetail", db.execute(stmt_request_detail).await),
                    ("Purchase", db.execute(stmt_purchase).await),
                    ("PurchaseDetail", db.execute(stmt_purchase_detail).await),
                    ("Delivery", db.execute(stmt_delivery).await),
                    ("MoneyDelivery", db.execute(stmt_money_delivery).await),
                    ("UtilsDelivery", db.execute(stmt_utils_delivery).await),
                    (
                        "UtilsDeliveryDetail",
                        db.execute(stmt_utils_delivery_detail).await,
                    ),
                    (
                        "SchoolSupplyBalance",
                        db.execute(stmt_school_supply_balance).await,
                    ),
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
