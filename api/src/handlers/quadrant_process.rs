use crate::{error::Error, request::{QuadrantProcessRequest, QuadrantProcessLongitudeRequest}, state::AppState};
use actix_web::{HttpResponse, Responder, post, web};
use geo_position::GeoPosition;
use horo::{PlanetConfig, quadrant_process, quadrant_process_longitude};
use horo_date_time::horo_date_time;

#[cfg(feature = "swagger")]
use horo::QuadrantProcess;

/// 象限推运
#[cfg_attr(feature = "swagger", 
utoipa::path(
    tag="推运",
    context_path="/api/process",
    request_body=QuadrantProcessRequest,
    responses(
        (status = 201, description = "返回象限推运", body = Vec<QuadrantProcess>),
    ),
)
)]
#[post("/quadrant_process")]
pub async fn quadrant_process_handler(
    r: actix_web_validator::Json<QuadrantProcessRequest>,
    app_state: web::Data<AppState>,
) -> Result<impl Responder, Error> {
    let r = r.into_inner();

    let native_date = horo_date_time(
        r.date.year,
        r.date.month,
        r.date.day,
        r.date.hour,
        r.date.minute,
        r.date.second,
        r.date.tz,
        r.date.st,
    )?;

    let geo = GeoPosition::new(r.geo.long, r.geo.lat)?;

    let directions = quadrant_process(
        native_date,
        geo,
        r.house,
        &PlanetConfig::default_all_configs(),
        &app_state.ephe_path,
    )?;

    Ok(HttpResponse::Created().json(directions))
}

/// 象限推运黄经计算
#[cfg_attr(feature = "swagger",
utoipa::path(
    tag="推运",
    context_path="/api/process",
    request_body=QuadrantProcessLongitudeRequest,
    responses(
        (status = 201, description = "返回推运时间对应的黄道经度", body = f64),
    ),
)
)]
#[post("/quadrant_process_longitude")]
pub async fn quadrant_process_longitude_handler(
    r: actix_web_validator::Json<QuadrantProcessLongitudeRequest>,
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

    let longitude = quadrant_process_longitude(
        native_date,
        process_date,
        geo,
        r.house,
        &PlanetConfig::default_all_configs(),
        &app_state.ephe_path,
    )?;

    Ok(HttpResponse::Created().json(longitude))
}
