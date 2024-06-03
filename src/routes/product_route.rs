use crate::controllers::product_controller::*;
use actix_web::web;

pub fn config(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/v1")
            .route("/product", web::get().to(get_products))
            .route("/product", web::post().to(add_product)),
    );
}
