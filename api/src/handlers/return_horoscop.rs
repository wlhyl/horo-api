use crate::{error::Error, request::ReturnRequest, state::AppState};
use actix_web::{post, web, HttpResponse, Responder};
use geo_position::GeoPosition;
use horo::{lunar_return, solar_return, PlanetConfig};
use horo_date_time::horo_date_time;

#[cfg(feature = "swagger")]
use horo::ReturnHoroscop;

/// 太阳返照
#[cfg_attr(feature = "swagger", 
utoipa::path(
    tag="推运",
    context_path="/api/process",
    request_body=ReturnRequest,
    responses(
        (status = 201, description = "返回太阳返照盘", body = ReturnHoroscop),
        (status = 400, description = "返回太阳返照盘400错误", body = String),
    ),
)
)]
#[post("/return/solar")]
pub async fn solar_return_horo(
    app_state: web::Data<AppState>,
    r: actix_web_validator::Json<ReturnRequest>,
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

    let pan = solar_return(
        native_date,
        process_date,
        geo,
        r.house,
        &PlanetConfig::default_all_configs(),
        &app_state.ephe_path,
    )?;

    Ok(HttpResponse::Created().json(pan))
}

/// 月亮返照
#[cfg_attr(feature = "swagger", 
utoipa::path(
    tag="推运",
    context_path="/api/process",
    request_body=ReturnRequest,
    responses(
        (status = 201, description = "返回月亮返照盘", body = ReturnHoroscop),
        (status = 400, description = "返回月亮返照盘400错误", body = String),
    ),
)
)]
#[post("/return/lunar")]
pub async fn lunar_return_horo(
    app_state: web::Data<AppState>,
    r: actix_web_validator::Json<ReturnRequest>,
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

    let pan = lunar_return(
        native_date,
        process_date,
        geo,
        r.house,
        &PlanetConfig::default_all_configs(),
        &app_state.ephe_path,
    )?;

    Ok(HttpResponse::Created().json(pan))
}
