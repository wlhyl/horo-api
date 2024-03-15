use actix_web::{get, HttpResponse, Responder};
use horo::HouseName;

/// 宫位系统
#[cfg_attr(feature = "swagger", 
utoipa::path(
    tag="宫位系统",
    context_path="/api",
    responses(
        (status = 200, description = "返回支持的宫位系统", body = Vec<HouseName>),
    ),
)
)]
#[get("/houses")]
pub async fn houses() -> impl Responder {
    let houses = HouseName::all_house_names();
    HttpResponse::Ok().json(houses)
}
