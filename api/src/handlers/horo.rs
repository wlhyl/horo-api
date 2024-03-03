use crate::{error::Error, request::HoroNativeRenReust, responser::Responser, state::AppState};
use actix_web::{post, web};
use horo::{GeoPosition, Horoscope, PlanetConfig};
use horo_date_time::horo_date_time;

/// 本命星盘
#[cfg_attr(feature = "swagger", 
utoipa::path(
    tag="本命星盘",
    context_path="/api/horo",
    request_body=HoroNativeRenReust,
    responses(
        (status = 200, description = "OK", body = Horoscope),
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
) -> Result<Responser<Horoscope>, Error> {
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

    Ok(Responser::Ok(pan))
}
