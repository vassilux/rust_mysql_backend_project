use actix_web::{web::Data, App, HttpServer};

mod api;
mod db;
mod models;

use crate::db::database::Database;

use api::mysqlapi::{
    add_or_update_unique_identifier, get_locations_by_product_code, get_locations_by_product_name,
    get_unique_identifiers, remove_unique_identifier,
};

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    match Database::init().await {
        Ok(db) => {
            println!("Database initialized successfully");
            let db_data = Data::new(db);

            //
            HttpServer::new(move || {
                App::new()
                    .app_data(db_data.clone())
                    .service(get_unique_identifiers)
                    .service(add_or_update_unique_identifier)
                    .service(remove_unique_identifier)
                    .service(get_locations_by_product_name)
                    .service(get_locations_by_product_code)
            })
            .bind("127.0.0.1:8080")?
            .run()
            .await
        }
        Err(err) => {
            eprintln!("Error connecting to the database: {}", err);
            std::process::exit(1);
        }
    }
}
