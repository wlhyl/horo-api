use actix_web::web;

use crate::handlers::{
    healthz::{liveness_handler, readiness_handler},
    horo::horo_native,
    house::houses,
    profection::profection,
    return_horoscop::{lunar_return_horo, solar_return_horo},
    transit::transit,
};

pub fn health_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(readiness_handler).service(liveness_handler);
}

pub fn horo_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(web::scope("/horo").service(horo_native))
        .service(houses)
        .service(
            web::scope("/process")
                .service(profection)
                .service(transit)
                .service(solar_return_horo)
                .service(lunar_return_horo),
        );
}
