use crate::{error::Error, request::HoroNativeRenReust, state::AppState};
use actix_web::{post, web, HttpResponse, Responder};
use geo_position::GeoPosition;
use horo::{Horoscope, PlanetConfig};
use horo_date_time::horo_date_time;

/// 本命星盘
#[cfg_attr(feature = "swagger", 
utoipa::path(
    tag="本命星盘",
    context_path="/api/horo",
    request_body=HoroNativeRenReust,
    responses(
        (status = 201, description = "返回本命星盘", body = Horoscope),
    ),
)
)]
// #[utoipa::path(
//     tag="本命星盘",
//     context_path="/api/horo",
//     request_body=HoroNativeRenReust,
//     responses(
//         (status = 200, description = "OK", body = Horoscope),
//     ),
//     security(
//         ("api_key" = [])
//     ),
// )]
#[post("/native")]
pub async fn horo_native(
    app_state: web::Data<AppState>,
    r: actix_web_validator::Json<HoroNativeRenReust>,
) -> Result<impl Responder, Error> {
    let r = r.into_inner();

    let t = horo_date_time(
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

    let pan = Horoscope::new(
        t,
        geo,
        r.house,
        &PlanetConfig::default_all_configs(),
        &app_state.ephe_path,
    )?;

    Ok(HttpResponse::Created().json(pan))
}
