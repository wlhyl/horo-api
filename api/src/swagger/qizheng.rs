use crate::handlers::qizheng::__path_qizheng_horo;
use crate::request::{DateRequest, GeoRequest, QiZhengRequst};

use geo_position::GeoPosition;
use horo_date_time::HoroDateTime;
use qizheng::{Horoscope, ASCHouse, LunarMansionsName, Planet, PlanetName, DistanceStarLong, House,HouseName,PlanetSpeedState, DongWei};
use lunar_calendar::{LunarCalendar,SolarTerm};
use ganzhiwuxing::GanZhi;

use utoipa::OpenApi;

// swagger
#[derive(OpenApi)]
#[openapi(
    paths(
        // 七政
        qizheng_horo
    ),
    components(schemas(
        GanZhi,
        // 农历
        LunarCalendar,SolarTerm,
        
        DateRequest, GeoRequest,
        // 七政 request
        QiZhengRequst,

        HoroDateTime, GeoPosition,

        Planet, PlanetName,PlanetSpeedState,
        ASCHouse,House,HouseName,
        LunarMansionsName,DistanceStarLong,
        // 星盘
        Horoscope,
        // 洞微
        DongWei,
        
    ))
)]
pub struct QiZhengApiDoc;

 



