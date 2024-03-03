use crate::{error::Error, request::ProfectionRequest};
use actix_web::{post, HttpResponse, Responder};
use horo::Profection;
use horo_date_time::horo_date_time;

/// 小限
#[cfg_attr(feature = "swagger", 
utoipa::path(
    tag="推运",
    context_path="/api/process",
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
        r.native_date.year,
        r.native_date.month,
        r.native_date.day,
        r.native_date.hour,
        r.native_date.minute,
        r.native_date.second,
        r.native_date.tz,
        r.native_date.st,
    )?;

    let process_date = horo_date_time(
        r.process_date.year,
        r.process_date.month,
        r.process_date.day,
        r.process_date.hour,
        r.process_date.minute,
        r.process_date.second,
        r.process_date.tz,
        r.process_date.st,
    )?;
    let pan = Profection::new(native_date, process_date)?;

    Ok(HttpResponse::Created().json(pan))
}
