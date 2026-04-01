use crate::{error::Error, request::DirectionRequest, state::AppState};
use actix_web::{HttpResponse, Responder, post, web};
use geo_position::GeoPosition;
use horo::{PlanetConfig, direction_process};
use horo_date_time::horo_date_time;

#[cfg(feature = "swagger")]
use horo::Direction;

/// 主向推运
#[cfg_attr(feature = "swagger", 
utoipa::path(
    tag="推运",
    context_path="/api/process",
    request_body=DirectionRequest,
    responses(
        (status = 201, description = "返回主向推运", body = Vec<Direction>),
    ),
)
)]
#[post("/directions")]
pub async fn directions(
    r: actix_web_validator::Json<DirectionRequest>,
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

    let directions = direction_process(
        native_date,
        geo,
        &PlanetConfig::default_all_configs(),
        &app_state.ephe_path,
    )?;

    Ok(HttpResponse::Created().json(directions))
}
