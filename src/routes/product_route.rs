use crate::controllers::product_controller::*;
use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            .route("/products", web::get().to(get_products))
            .route("/products", web::post().to(add_product)),
    );
}
