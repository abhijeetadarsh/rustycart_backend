use actix_web::{App, HttpServer};
use dotenv::dotenv;
use std::env;

mod controllers;
mod database;
mod models;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

    let mongo_client = database::get_mongo_client().await;

    let port = match env::var("PORT") {
        Ok(s) => match s.parse::<u16>() {
            Ok(port_num) => port_num,
            Err(_) => {
                eprintln!("Invalid port number provided. Using default port 8080.");
                8080
            }
        },
        Err(_) => {
            eprintln!("PORT environment variable not set. Using default port 8080.");
            8080
        }
    };

    HttpServer::new(move || {
        App::new()
            .app_data(actix_web::web::Data::new(mongo_client.clone()))
            .configure(routes::init)
    })
    .bind(("127.0.0.1", port))?
    .run()
    .await
}
