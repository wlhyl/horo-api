use actix_web::web;

use crate::handlers::{healthz::{readiness_handler, liveness_handler}, horo::horo_native, house::houses, profection::profection};


pub fn health_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(readiness_handler).service(liveness_handler);
}

pub fn horo_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(horo_native).service(houses).service(profection);
}