use crate::handlers::{
    horo::__path_horo_native, house::__path_houses, profection::__path_profection,
    transit::__path_transit,
};
use crate::request::{HoroNativeRenReust, ProfectionRequest, TransitRenReust};
use horo::{
    Aspect, GeoPosition, HoroDateTime, Horoscope, HoroscopeCompare, HouseName, Planet, PlanetName,
    PlanetSpeedState, Profection,
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
        profection,
        // 行运
        transit,
    ),
    components(schemas(
        HoroNativeRenReust,
        ProfectionRequest,
        TransitRenReust,
        Horoscope,HoroscopeCompare,
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
