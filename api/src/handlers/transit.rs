use actix_web::{post, web, HttpResponse, Responder};
use horo::{horo_date_time, GeoPosition, HoroscopeCompare, PlanetConfig};

use crate::{error::Error, request::TransitRenReust, state::AppState};

/// 行运
#[cfg_attr(feature = "swagger", 
utoipa::path(
    tag="推运",
    context_path="/api/horo",
    request_body=TransitRenReust,
    responses(
        (status = 201, description = "返回行运", body = HoroscopeCompare),
    ),
)
)]
#[post("/transit")]
pub async fn transit(
    r: actix_web_validator::Json<TransitRenReust>,
    app_state: web::Data<AppState>,
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

    let geo = GeoPosition::new(r.geo_long, r.geo_lat)?;

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
