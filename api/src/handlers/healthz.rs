use actix_web::{get, HttpResponse, Responder};

#[get("readiness")]
pub async fn readiness_handler() -> impl Responder {
    HttpResponse::Ok().json("ok")
}

#[get("liveness")]
pub async fn liveness_handler() -> impl Responder {
    HttpResponse::Ok().json("ok")
}
