use actix_web::{post, web};
use horo::{horo_date_time, GeoPosition, Horoscope, PlanetConfig};

use crate::{error::Error, request::HoroNativeRenReust, responser::Responser, state::AppState};

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
        r.year, r.month, r.day, r.hour, r.minute, r.second, r.tz, r.st,
    )?;
    let geo = GeoPosition::new(r.geo_long, r.geo_lat)?;

    let pan = Horoscope::new(
        t,
        geo,
        r.house,
        &PlanetConfig::default_all_configs(),
        &app_state.ephe_path,
    )?;

    Ok(Responser::Ok(pan))
}
