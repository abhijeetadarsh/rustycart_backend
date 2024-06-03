use actix_web::{App, HttpServer};
use dotenv::dotenv;
use std::env;

mod controllers;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();

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

    HttpServer::new(|| App::new().configure(routes::product_route::config))
        .bind(("127.0.0.1", port))?
        .run()
        .await
}
