use actix_web::web;

pub mod product_route;

pub fn init(cfg: &mut web::ServiceConfig) {
    product_route::config(cfg);
}
