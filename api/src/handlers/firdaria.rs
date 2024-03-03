use crate::{error::Error, request::FirdariaRequest, state::AppState};
use actix_web::{post, web, HttpResponse, Responder};
use horo::{firdaria_process, GeoPosition, PlanetConfig};
use horo_date_time::horo_date_time;

/// 法达
#[cfg_attr(feature = "swagger", 
utoipa::path(
    tag="推运",
    context_path="/api/process",
    request_body=FirdariaRequest,
    responses(
        (status = 201, description = "返回法达", body = Vec<FirdariaPeriod>),
    ),
)
)]
#[post("/firdaria")]
pub async fn firdaria(
    r: actix_web_validator::Json<FirdariaRequest>,
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

    let geo = GeoPosition::new(r.geo.long, r.geo.lat)?;

    let firdaria_period = firdaria_process(
        native_date,
        geo,
        &PlanetConfig::default_all_configs(),
        &app_state.ephe_path,
    )?;

    Ok(HttpResponse::Created().json(firdaria_period))
}
