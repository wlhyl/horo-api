use crate::{
    DistanceStarConfig, Error, Planet, PlanetConfig, PlanetName,
    dong_wei::{DongWei, calc_dong_wei},
    house::{ASCHouse, House, HouseName},
    lunar_mansions::{DistanceStarLong, calc_distance_star_long, calc_xiu_degree},
    planet::calc_planets,
    transformed_stars::{StarTransformedStar, transformed_stars},
};
use ganzhiwuxing::GanZhi;
use geo_position::GeoPosition;
use horo_date_time::HoroDateTime;

use lunar_calendar::{LunarCalendar, lunar_calendar};
use swe::{HouseSystem, swe_degnorm, swe_houses};

#[cfg(feature = "serde")]
use serde::Serialize;

#[cfg(feature = "swagger")]
use utoipa::ToSchema;

#[cfg_attr(feature = "serde", derive(Serialize))]
#[cfg_attr(feature = "swagger", derive(ToSchema))]
pub struct Horoscope {
    /// 出生时间
    native_date: HoroDateTime,
    /// 推运时间
    process_date: HoroDateTime,
    /// 出生地大地经纬度
    geo: GeoPosition,
    // 十一颗行星
    /// 本命行星
    native_planets: Vec<Planet>,
    /// 流年行星
    process_planets: Vec<Planet>,
    /// 距星的黄道经度和名称
    distance_star_long: Vec<DistanceStarLong>,
    /// 命宫
    asc_house: ASCHouse,
    /// 宫位
    houses: Vec<House>,

    ///  出生时刻的农历
    native_lunar_calendar: LunarCalendar,
    /// 推运时刻的农历
    process_lunar_calendar: LunarCalendar,
    /// 本命八字
    bazi: Vec<GanZhi>,
    /// 洞微大限
    dong_wei: DongWei,
    //    @field:Schema(description = "本命纳间")
    //    val naYin = getNaYinData(nativeLunarCalendar.yearGanZhi)
    /// 本命变曜
    native_transformed_stars: Vec<StarTransformedStar>,
    /// 流年变曜
    process_transformed_stars: Vec<StarTransformedStar>, //    @field:Schema(description = "本命神煞")
                                                         //    val nativeShenShas = getShenShas(nativeTime, geo, ephePath)

                                                         //    @field:Schema(description = "流年变曜")
                                                         //    val processShenShas = getShenShas(processTime, geo, ephePath)
}

impl Horoscope {
    pub fn new(
        native_date: HoroDateTime,
        process_date: HoroDateTime,
        geo: GeoPosition,
        planets_config: &[PlanetConfig],
        distance_star_config: &[DistanceStarConfig],
        ephe_path: &str,
    ) -> Result<Self, Error> {
        if process_date.jd_utc < native_date.jd_utc {
            return Err(Error::InvalidProcessDateTime(
                "推运时间必需大于等于出生时间".to_string(),
            ));
        }

        let distance_star_long =
            calc_distance_star_long(native_date.jd_utc, distance_star_config, ephe_path)?;
        // 计算行星
        let native_planets = calc_planets(
            native_date.jd_utc,
            &distance_star_long,
            &planets_config,
            ephe_path,
        )?;

        let process_planets = calc_planets(
            process_date.jd_utc,
            &distance_star_long,
            &planets_config,
            ephe_path,
        )?;

        // 计算命宫
        let (_, ascmc) =
            if let Ok(v) = swe_houses(native_date.jd_ut1, geo.lat, geo.long, &HouseSystem::B) {
                v
            } else {
                return Err(Error::Function("swe_houses()调用失败".to_owned()));
            };

        let asc_long = ascmc[0];

        // 命宫的黄道经经度
        let asc_house_long = (asc_long / 30.0).floor() * 30.0;

        // 算命度
        let sun_long = native_planets
            .iter()
            .find_map(|p| {
                if p.name == PlanetName::日 {
                    Some(p.long)
                } else {
                    None
                }
            })
            .ok_or(Error::Function(
                "在已经计算完成 的本命行星中找不到太阳的黄道经度，请检查源代码".to_string(),
            ))?;

        //计算命度的黄道经度
        let ming_du_long = sun_long - (sun_long / 30.0).floor() * 30.0 + asc_house_long;

        let (ming_du_xiu, ming_du_xiu_degree) = calc_xiu_degree(ming_du_long, &distance_star_long)?;

        let asc_house = ASCHouse::new(asc_long, ming_du_xiu, ming_du_xiu_degree);

        // 计算宫位

        let houses = [
            HouseName::命,
            HouseName::财,
            HouseName::兄,
            HouseName::田,
            HouseName::子,
            HouseName::奴,
            HouseName::妻,
            HouseName::疾,
            HouseName::迁,
            HouseName::官,
            HouseName::福,
            HouseName::相,
        ]
        .into_iter()
        .enumerate()
        .map(|(index, house_name)| {
            let long = swe_degnorm(asc_house_long + 30.0 * index as f64);
            let (xiu, xiu_degree) = calc_xiu_degree(long, &distance_star_long)?;
            let house = House::new(house_name, long, xiu, xiu_degree);
            Ok(house)
        })
        .collect::<Result<Vec<_>, Error>>()?;

        // 计算农历
        // 假定出生的时区是东八区
        let native_lunar_calendar = lunar_calendar(
            native_date.year,
            native_date.month,
            native_date.day,
            native_date.hour,
            native_date.minute,
            native_date.second,
            ephe_path,
        )
        .map_err(|error| Error::Function(format!("计算出生时间农历错误：{error}")))?;

        let process_lunar_calendar = lunar_calendar(
            process_date.year,
            process_date.month,
            process_date.day,
            process_date.hour,
            process_date.minute,
            process_date.second,
            ephe_path,
        )
        .map_err(|error| Error::Function(format!("计算推运时间农历错误：{error}")))?;

        // 计算八字
        // 计算时差
        // 15度=1小时, 15度=1/24天, 1度=1/(24*15)天
        let delta_days = (geo.long - 120.0) / (24.0 * 15.0);
        // 本地时间
        let local_time = native_date
            .plus_days(delta_days)
            .map_err(|error| Error::Function(format!("计算真太阳时错误：{error}")))?;
        // 当地太阳时的农历
        let local_lunar_calendar = lunar_calendar(
            local_time.year,
            local_time.month,
            local_time.day,
            local_time.hour,
            local_time.minute,
            local_time.second,
            ephe_path,
        )
        .map_err(|error| Error::Function(format!("计算真太阳时农历错误：{error}")))?;
        let bazi = vec![
            native_lunar_calendar.lunar_year_gan_zhi,
            native_lunar_calendar.lunar_month_gan_zhi,
            local_lunar_calendar.lunar_day_gan_zhi,
            local_lunar_calendar.time_gan_zhi,
        ];

        // 计算洞微
        let dong_wei = calc_dong_wei(
            ming_du_long,
            houses[0].long,
            &native_date,
            &process_date,
            &distance_star_long,
        )?;

        let native_transformed_stars = transformed_stars(&native_lunar_calendar);
        let process_transformed_stars = transformed_stars(&process_lunar_calendar);

        Ok(Self {
            native_date,
            process_date,
            geo,
            native_planets,
            process_planets,
            distance_star_long,
            asc_house,
            houses,
            native_lunar_calendar,
            process_lunar_calendar,
            bazi,
            dong_wei,
            native_transformed_stars,
            process_transformed_stars,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use geo_position::GeoPosition;
    use horo_date_time::horo_date_time;
    use std::env;

    #[test]
    fn test_horoscope() {
        dotenvy::dotenv().ok();
        let ephe_path = env::var("EPHE_PATH").unwrap();
        let native_date = horo_date_time(1983, 10, 27, 18, 30, 0, 8.0, false).unwrap();
        let process_date = horo_date_time(2023, 10, 27, 18, 30, 0, 8.0, false).unwrap();
        let geo = GeoPosition::new(116.383333, 39.9).unwrap();
        let planets_config = PlanetConfig::default_all_configs();
        let distance_star_config = DistanceStarConfig::default_all_configs();

        let horoscope = Horoscope::new(
            native_date,
            process_date,
            geo,
            &planets_config,
            &distance_star_config,
            &ephe_path,
        )
        .unwrap();

        insta::assert_yaml_snapshot!(horoscope);
    }
}
