use crate::handlers::{
    compare_horoscop::__path_compare,
    firdaria::__path_firdaria,
    horo::__path_horo_native,
    house::__path_houses,
    profection::__path_profection,
    return_horoscop::{__path_lunar_return_horo, __path_solar_return_horo},
};
use crate::request::{
    CompareRequst, DateRequest, FirdariaRequest, GeoRequest, HoroNativeRenReust, ProfectionRequest,
    ReturnRequest,
};
use geo_position::GeoPosition;
use horo::{
    Aspect, FirdariaPeriod, FirdariaSubPeriod, Horoscope, HoroscopeCompare, HouseName, Planet,
    PlanetName, PlanetSpeedState, Profection, ReturnHoroscop,
};
use horo_date_time::HoroDateTime;
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
        lunar_return_horo,
        // 法达
        firdaria,
    ),
    components(schemas(
        DateRequest,
        GeoRequest,
        HoroNativeRenReust,
        ProfectionRequest,
        CompareRequst,
        FirdariaRequest,
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
        // 法达
        FirdariaPeriod,
        FirdariaSubPeriod
    ))
)]
pub struct HoroApiDoc;
