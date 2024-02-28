use crate::handlers::{
    horo::__path_horo_native,
    house::__path_houses,
    profection::__path_profection,
    return_horoscop::{__path_lunar_return_horo, __path_solar_return_horo},
    compare_horoscop::__path_compare,
};
use crate::request::{
    DateRequest, GeoRequest, HoroNativeRenReust, ProfectionRequest, ReturnRequest, CompareRequst,
};
use horo::{
    Aspect, GeoPosition, HoroDateTime, Horoscope, HoroscopeCompare, HouseName, Planet, PlanetName,
    PlanetSpeedState, Profection, ReturnHoroscop,
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
        // 比较盘
        compare,
        // 日返
        solar_return_horo,
        // 月返
        lunar_return_horo
    ),
    components(schemas(
        DateRequest,
        GeoRequest,
        HoroNativeRenReust,
        ProfectionRequest,
        CompareRequst,
        Horoscope,HoroscopeCompare,
        HouseName,
        Planet,
        PlanetName,
        Aspect,
        HoroDateTime,
        GeoPosition,
        PlanetSpeedState,
        Profection,
        ReturnRequest,
        ReturnHoroscop,
    ))
)]
pub struct ApiDoc;
