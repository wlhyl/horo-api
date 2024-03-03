use crate::{error::Error, request::CompareRequst, state::AppState};
use actix_web::{post, web, HttpResponse, Responder};
use geo_position::GeoPosition;
use horo::{HoroscopeCompare, PlanetConfig};
use horo_date_time::horo_date_time;

/// 比较盘
#[cfg_attr(feature = "swagger", 
utoipa::path(
    tag="推运",
    context_path="/api/process",
    request_body=CompareRequst,
    responses(
        (status = 201, description = "返回比较盘", body = HoroscopeCompare),
    ),
)
)]
#[post("/compare")]
pub async fn compare(
    r: actix_web_validator::Json<CompareRequst>,
    app_state: web::Data<AppState>,
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

    let geo = GeoPosition::new(r.geo.long, r.geo.lat)?;

    let pan = HoroscopeCompare::new(
        native_date,
        process_date,
        geo,
        r.house,
        &PlanetConfig::default_all_configs(),
        &app_state.ephe_path,
    )?;

    Ok(HttpResponse::Created().json(pan))
}
