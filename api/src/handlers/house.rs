use actix_web::get;
use horo::HouseName;

use crate::responser::Responser;

/// 宫位系统
#[cfg_attr(feature = "swagger", 
utoipa::path(
    tag="宫位系统",
    context_path="/api",
    responses(
        (status = 200, description = "OK", body = Vec<HouseName>),
    ),
)
)]
#[get("/houses")]
pub async fn houses() -> Responser<Vec<HouseName>> {
    let houses = HouseName::all_house_names();

    Responser::Ok(houses)
}
