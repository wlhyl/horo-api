use crate::{
    dong_wei::DongWei,
    house::{ASCHouse, House, HouseName},
    lunar_mansions::{calc_distance_star_long, calc_xiu_degree, DistanceStarLong},
    planet::calc_planets,
    DistanceStarConfig, Error, Planet, PlanetConfig, PlanetName,
};
use geo_position::GeoPosition;
use horo_date_time::{horo_date_time, HoroDateTime};

use lunar_calendar::{lunar_calendar, LunarCalendar};
use swe::{swe_degnorm, swe_houses, HouseSystem};

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
    /// 洞微大限
    dong_wei: DongWei,
    //    @field:Schema(description = "本命纳间")
    //    val naYin = getNaYinData(nativeLunarCalendar.yearGanZhi)

    //    @field:Schema(description = "本命变曜")
    //    val nativeVirtualStars =  getVirtualStars(nativeTime, geo, ephePath)

    //    @field:Schema(description = "流年变曜")
    //    val processVirtualStars =  getVirtualStars(processTime, geo, ephePath)

    //    @field:Schema(description = "本命神煞")
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

        // 计算洞微
        // 各宫位的洞微所管年数
        let mut ages_of_per_house_for_dong_wei = [
            15.0, 10.0, 11.0, 15.0, 8.0, 7.0, 11.0, 4.5, 4.5, 4.5, 5.0, 5.0,
        ];

        ages_of_per_house_for_dong_wei[0] = (ming_du_long - houses[0].long) / 3.0 + 10.0;

        // 计算给定时间洞微大限所在的黄道经度

        let mut date_of_per_house_for_dong_wei = vec![native_date];
        for age in ages_of_per_house_for_dong_wei {
            // let age = ages_of_per_house_for_dong_wei[index];
            // 年龄数的整数部分
            let age_int_part = age.floor();
            // 非整数年，如:4.5年，(下一年的jd_utc-本年的jd_utc)*年龄的小数部分，得到此限开始的jd_utc

            let start_date_of_current_house = date_of_per_house_for_dong_wei.last().unwrap();
            // 计算一下宫位的起运时间
            // 由于可能出现2月29日，因此使用horo_date_time函数
            let current_start_date = horo_date_time(
                start_date_of_current_house.year + (age_int_part as i32),
                start_date_of_current_house.month,
                start_date_of_current_house.day,
                start_date_of_current_house.hour,
                start_date_of_current_house.minute,
                start_date_of_current_house.second,
                start_date_of_current_house.tz,
                false,
            )?;

            let next_start_date = horo_date_time(
                start_date_of_current_house.year + (age_int_part as i32) + 1,
                start_date_of_current_house.month,
                start_date_of_current_house.day,
                start_date_of_current_house.hour,
                start_date_of_current_house.minute,
                start_date_of_current_house.second,
                start_date_of_current_house.tz,
                false,
            )?;

            let start_jd_utc = current_start_date.jd_utc
                + (age - age_int_part) * (next_start_date.jd_utc - current_start_date.jd_utc);

            let start_date_of_next_house =
                HoroDateTime::from_jd_zone(start_jd_utc, current_start_date.tz)?;

            date_of_per_house_for_dong_wei.push(start_date_of_next_house);
        }

        let dong_long_per_year: Result<Vec<_>, Error> = (native_date.year
            ..date_of_per_house_for_dong_wei.last().unwrap().year)
            .into_iter()
            .map(|year| {
                let date = horo_date_time(
                    year,
                    native_date.month,
                    native_date.day,
                    native_date.hour,
                    native_date.minute,
                    native_date.second,
                    native_date.tz,
                    false,
                )?;

                let long = date_of_per_house_for_dong_wei.iter().enumerate().find_map(
                    |(index, start_date)| {
                        let next_index = (index + 1) % date_of_per_house_for_dong_wei.len();
                        let next_start_date = date_of_per_house_for_dong_wei[next_index];

                        if date.jd_utc < next_start_date.jd_utc {
                            let long = swe_degnorm(
                                // 第一年从第2宫开始
                                houses[1].long
                                    - (index * 30) as f64
                                    - 30.0 * (date.jd_utc - start_date.jd_utc)
                                        / (next_start_date.jd_utc - start_date.jd_utc),
                            );
                            Some(long)
                        } else {
                            None
                        }
                    },
                );

                let long = long.unwrap();

                Ok(long)
            })
            .collect();

        let dong_long_per_year = dong_long_per_year?;

        let process_dong_wei_long =
            date_of_per_house_for_dong_wei
                .iter()
                .enumerate()
                .find_map(|(index, start_date)| {
                    let next_index = (index + 1) % date_of_per_house_for_dong_wei.len();
                    let next_date = date_of_per_house_for_dong_wei[next_index];

                    if process_date.jd_utc < next_date.jd_utc {
                        let long = swe_degnorm(
                            houses[1].long
                                - (index * 30) as f64
                                - 30.0 * (process_date.jd_utc - start_date.jd_utc)
                                    / (next_date.jd_utc - start_date.jd_utc),
                        );
                        Some(long)
                    } else {
                        None
                    }
                });

        let process_dong_wei_long = process_dong_wei_long.unwrap();
        let (process_dong_wei_xiu, process_dong_wei_xiu_degree) =
            calc_xiu_degree(process_dong_wei_long, &distance_star_long)?;

        let dong_wei = DongWei::new(
            dong_long_per_year,
            process_dong_wei_long,
            process_dong_wei_xiu,
            process_dong_wei_xiu_degree,
        );

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
            dong_wei,
        })
    }
}
