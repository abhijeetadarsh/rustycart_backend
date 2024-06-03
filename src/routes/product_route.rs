use crate::controllers::*;
use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            .route("/", web::get().to(product_controller::hello))
            .route("/hey", web::get().to(product_controller::hey))
            .route("/echo", web::post().to(product_controller::echo)),
    );
}
