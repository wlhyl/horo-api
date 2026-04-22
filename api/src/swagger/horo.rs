use crate::handlers::{
    compare_horoscop::__path_compare,
    direction::__path_directions,
    firdaria::__path_firdaria,
    horo::__path_horo_native,
    house::__path_houses,
    profection::__path_profection,
    quadrant_process::{__path_quadrant_process_handler, __path_quadrant_process_longitude_handler},
    return_horoscop::{__path_lunar_return_horo, __path_solar_return_horo},
};
use crate::request::{
    DateRequest, FirdariaRequest, GeoRequest, HoroNativeRenReust, HoroscopeComparisonRequst,
    ProfectionRequest, ReturnRequest,
};
use geo_position::GeoPosition;
use horo::{
    Aspect, FirdariaPeriod, FirdariaSubPeriod, Horoscope, HoroscopeComparison, HouseName, Planet,
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
        // 主向推运
        directions,
        // 象限推运
        quadrant_process_handler,
        quadrant_process_longitude_handler,
    ),
    components(schemas(
        DateRequest,
        GeoRequest,
        HoroNativeRenReust,
        ProfectionRequest,
        HoroscopeComparisonRequst,
        FirdariaRequest,
        Horoscope,HoroscopeComparison,
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
