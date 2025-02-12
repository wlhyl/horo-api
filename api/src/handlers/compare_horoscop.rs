use crate::{error::Error, request::HoroscopeComparisonRequst, state::AppState};
use actix_web::{post, web, HttpResponse, Responder};
use geo_position::GeoPosition;
use horo::{HoroscopeComparison, PlanetConfig};
use horo_date_time::horo_date_time;

/// 比较盘
#[cfg_attr(feature = "swagger", 
utoipa::path(
    tag="推运",
    context_path="/api/process",
    request_body=HoroscopeComparisonRequst,
    responses(
        (status = 201, description = "返回比较盘", body = HoroscopeComparison),
    ),
)
)]
#[post("/compare")]
pub async fn compare(
    r: actix_web_validator::Json<HoroscopeComparisonRequst>,
    app_state: web::Data<AppState>,
) -> Result<impl Responder, Error> {
    let r = r.into_inner();

    let original_date = horo_date_time(
        r.original_date.year,
        r.original_date.month,
        r.original_date.day,
        r.original_date.hour,
        r.original_date.minute,
        r.original_date.second,
        r.original_date.tz,
        r.original_date.st,
    )?;

    let comparison_date = horo_date_time(
        r.comparison_date.year,
        r.comparison_date.month,
        r.comparison_date.day,
        r.comparison_date.hour,
        r.comparison_date.minute,
        r.comparison_date.second,
        r.comparison_date.tz,
        r.comparison_date.st,
    )?;

    let original_geo = GeoPosition::new(r.original_geo.long, r.original_geo.lat)?;
    let comparison_geo = GeoPosition::new(r.comparison_geo.long, r.comparison_geo.lat)?;

    let pan = HoroscopeComparison::new(
        original_date,
        comparison_date,
        original_geo,
        comparison_geo,
        r.house,
        &PlanetConfig::default_all_configs(),
        &app_state.ephe_path,
    )?;

    Ok(HttpResponse::Created().json(pan))
}
