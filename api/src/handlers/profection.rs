use actix_web::{post, HttpResponse, Responder};
use horo::{horo_date_time, Profection};

use crate::{error::Error, request::ProfectionRequest};

/// 小限
#[cfg_attr(feature = "swagger", 
utoipa::path(
    tag="推运",
    context_path="/api/horo",
    request_body=ProfectionRequest,
    responses(
        (status = 201, description = "返回小限", body = Profection),
    ),
)
)]
#[post("/profection")]
pub async fn profection(
    r: actix_web_validator::Json<ProfectionRequest>,
) -> Result<impl Responder, Error> {
    let r = r.into_inner();

    let native_date = horo_date_time(
        r.year, r.month, r.day, r.hour, r.minute, r.second, r.tz, r.st,
    )?;

    let process_date = horo_date_time(
        r.process_year,
        r.process_month,
        r.process_day,
        r.process_hour,
        r.process_minute,
        r.process_second,
        r.tz,
        false,
    )?;
    let profection = Profection::new(native_date, process_date)?;

    Ok(HttpResponse::Created().json(profection))
}
