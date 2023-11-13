use crate::handlers::{
    horo::__path_horo_native, house::__path_houses, profection::__path_profection,
};
use crate::request::{HoroNativeRenReust, ProfectionRequest};
use horo::{
    Aspect, GeoPosition, HoroDateTime, Horoscope, HouseName, Planet, PlanetName, PlanetSpeedState,
    Profection,
};
use utoipa::OpenApi;

// swagger
#[derive(OpenApi)]
#[openapi(
    paths(
        // 本命星盘
        horo_native,
        houses,
        // 小限
        profection
    ),
    components(schemas(
        HoroNativeRenReust,
        ProfectionRequest,
        Horoscope,
        HouseName,
        Planet,
        PlanetName,
        Aspect,
        HoroDateTime,
        GeoPosition,
        PlanetSpeedState,
        Profection,
    ))
)]
pub struct ApiDoc;
