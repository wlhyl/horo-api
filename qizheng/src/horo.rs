use crate::{
    dong_wei::DongWei,
    house::{ASCHouse, House, HouseName},
    lunar_mansions::{calc_distance_star_long, DistanceStarLong, LunarMansionsName},
    DistanceStarConfig, Error, Planet, PlanetConfig, PlanetName,
};
use geo_position::GeoPosition;
use horo_date_time::{horo_date_time, HoroDateTime};

use lunar_calendar::{lunar_calendar, LunarCalendar};
use swe::{
    swe_calc_ut, swe_close, swe_degnorm, swe_houses, swe_set_ephe_path, Body, Flag, HouseSystem,
};

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

/* 紫气计算： 参看https://sites.google.com/site/athomeprojects/
 * Moira 紫炁計算週期是 28 年, 28.0 * 365.2564 = 10227.1792 天, 在1975年3月13日4:00p.m., 紫炁是在 230.5 度。 您可改修週期、開始日期和度數。 按: 零度是在亥宮和戌宮之間; 以逆時針方向前進 。
 * 紫炁, 週期 10227.1792 天, 在格林威治標準時間1975年3月13日4:00p.m., 紫炁是在 230.5 度
 * 岁差：参考https://baike.baidu.com/item/%E5%B2%81%E5%B7%AE%E6%B5%8B%E5%AE%9A%E6%96%B9%E6%B3%95/22644625?fr=aladdin#reference-[1]-23238331-wrap
 * 岁差：一个世纪：5029.0966秒
 */
/*
 * 每日行二分六秒，小餘七二０七七七。以乾隆九年甲子天正冬至，次日子正在七宮十七度五十分十四秒五十三微為元。
 */

//   计算行星
fn calc_planets(
    jd_utc: f64,
    distance_star_long: &[DistanceStarLong],
    planets_config: &[PlanetConfig],
    ephe_path: &str,
) -> Result<Vec<Planet>, Error> {
    swe_set_ephe_path(ephe_path);

    let mut planets = vec![];
    let planet_names = [
        PlanetName::日,
        PlanetName::月,
        PlanetName::水,
        PlanetName::金,
        PlanetName::火,
        PlanetName::木,
        PlanetName::土,
        PlanetName::计,
        PlanetName::孛,
        PlanetName::气,
    ];

    for planet_name in planet_names {
        let body = match planet_name {
            PlanetName::日 => Body::SeSun,
            PlanetName::月 => Body::SeMoon,
            PlanetName::水 => Body::SeMercury,
            PlanetName::金 => Body::SeVenus,
            PlanetName::火 => Body::SeMars,
            PlanetName::木 => Body::SeJupiter,
            PlanetName::土 => Body::SeSaturn,
            PlanetName::孛 => Body::SeMeanApog,
            _ => Body::SeMeanNode,
        };

        let (star_long, speed) = if planet_name == PlanetName::气 {
            // 以下是原kotlin代码，以真步堂数据计算紫气

            // val t_start = HoroDateTime(1975,3,13,16,0,0, 0.0)
            // //            val longStart = 230.5
            //             /*
            //             * 此值按真步堂2019年通胜，2019年2月23日 00:00:00 东八区，紫气在酉宫9度
            //             * 紫气每日速度依《清史稿》2分6.720777秒/每日，不修正岁差
            //              */
            //             // 起算点时刻
            //             val startT = HoroDateTime(2019,2,23,0,0,0, 8.0)
            //             // 起始时刻的黄道经度
            //             val longStart = 39.0
            //             var long = swe_degnorm((t.jdUTC - startT.jdUTC) * (2 / 60.0 + 6.720777/3600) + longStart)
            //             // 修正岁差
            // //            println("year=${t.year} startLong= ${long} 岁差=${(t.jd_utc - t_start.jd_utc) /365.2423  / 100 * (5029.0966 / 3600)}")
            //             // 每100年的回归年天数，以1975年3月13日 16:00:00 UTC为起点的100年的总共天数
            // //            val period = HoroDateTime(2075,3,13,16,0,0, 0.0).jd_utc - HoroDateTime(1975,3,13,16,0,0, 0.0).jd_utc
            // //            long += (t.jd_utc - t_start.jd_utc) / period * (5029.0966 / 3600)
            // //            long = swe_degnorm(long)
            // //            println("year=${t.year} startLong= ${long} 修正后")
            // //            val (starLong,StarName) = determinativeStarDistance(long, stars)
            // //            val t_start = HoroDateTime(1744,12,22,23,0,0, 0.8)
            // //            val longStart = 180.0 + 17.0 + 50 / 60.0 + 14.53/3600
            // //            var long = swe_degnorm((t.jd_utc - t_start.jd_utc) * (2 / 60.0 + 6.720777/3600) + longStart)
            //             // 修正岁差
            // //            println("year=${t.year} startLong= ${long} 岁差=${(t.jd_utc - t_start.jd_utc) /365.2423  / 100 * (5029.0966 / 3600)}")

            // //            long += (t.jd_utc - t_start.jd_utc) /365.2423  / 100 * (5029.0966 / 3600)
            // //            println("year=${t.year} 儒略日差=${t.jd_utc-t_start.jd_utc}")
            // //            long = swe_degnorm(long)
            // //            println("year=${t.year} startLong= ${long} 修正后")
            //             val (starLong,StarName) = determinativeStarDistance(long, stars)
            //             this.long = long
            //             this.speed = 2 / 60.0 + 6.720777/3600
            //             this.xiu = StarName
            //             this.xiuDegree = starLong
            //             this.speedState = listOf("顺", "均")

            // 今仍依moria数据计算紫气，不修正岁差
            let start_date = HoroDateTime::new(1975, 3, 13, 16, 0, 0, 0.0)?;
            let start_long = 230.5;
            let speed = 360.0 / 10227.1792;
            let star_long = swe_degnorm((jd_utc - start_date.jd_utc) * speed + start_long);

            (star_long, speed)
        } else {
            let xx = swe_calc_ut(jd_utc, &body, &[Flag::SeflgSpeed])
                .map_err(|e| Error::Function(format!("计算行星错误:{e}")))?;

            (xx[0], xx[3])
        };
        let (xiu, xiu_degree) = calc_xiu_degree(star_long, distance_star_long)?;

        let default_planet_config = PlanetConfig::default_config(&planet_name);
        let planet_config = planets_config
            .iter()
            .find(|p| p.name == planet_name)
            .unwrap_or(&default_planet_config);
        let p = Planet::new(
            planet_name,
            star_long, // long
            speed,     // speed
            xiu,
            xiu_degree,
            planet_config,
        );
        planets.push(p);
        if planet_name == PlanetName::计 {
            let star_long = swe_degnorm(star_long + 180.0);

            let (xiu, xiu_degree) = calc_xiu_degree(star_long, distance_star_long)?;

            let p = Planet::new(
                PlanetName::罗,
                star_long, // long
                speed,     // speed
                xiu,
                xiu_degree,
                planet_config,
            );
            planets.push(p);
        }
    }

    swe_close();

    Ok(planets)
}

// 计算入宿度
fn calc_xiu_degree(
    star_long: f64,
    distance_star_long: &[DistanceStarLong],
) -> Result<(LunarMansionsName, f64), Error> {
    distance_star_long
        .iter()
        .enumerate()
        .find_map(|(index, distance_star)| {
            let next_distance_star = &distance_star_long[(index + 1) % distance_star_long.len()];

            let distance = swe_degnorm(next_distance_star.long - distance_star.long);
            let planet_distance = swe_degnorm(star_long - distance_star.long);
            if planet_distance < distance {
                Some((distance_star.lunar_mansions, planet_distance))
            } else {
                None
            }
        })
        .ok_or(Error::Function(
            "找不到行星的入宿度，请检查源代码".to_string(),
        ))
}

#[cfg(test)]
mod tests {
    use swe::swe_degnorm;

    use crate::{horo::calc_xiu_degree, DistanceStarLong, LunarMansionsName::*};

    #[test]
    fn test_calc_xiu_degree() {
        let distance_star_long: [DistanceStarLong; 28] = [
            DistanceStarLong {
                lunar_mansions: 角,
                long: swe_degnorm(180.0 + 12.0 + 1.0),
            },
            DistanceStarLong {
                lunar_mansions: 亢,
                long: swe_degnorm(180.0 + 24.0 + 1.0),
            },
            DistanceStarLong {
                lunar_mansions: 氐,
                long: swe_degnorm(180.0 + 36.0 + 1.0),
            },
            DistanceStarLong {
                lunar_mansions: 房,
                long: swe_degnorm(180.0 + 48.0 + 1.0),
            },
            DistanceStarLong {
                lunar_mansions: 心,
                long: swe_degnorm(180.0 + 60.0 + 1.0),
            },
            DistanceStarLong {
                lunar_mansions: 尾,
                long: swe_degnorm(180.0 + 72.0 + 1.0),
            },
            DistanceStarLong {
                lunar_mansions: 箕,
                long: swe_degnorm(180.0 + 84.0 + 1.0),
            },
            //
            DistanceStarLong {
                lunar_mansions: 斗,
                long: swe_degnorm(180.0 + 96.0 + 1.0),
            },
            DistanceStarLong {
                lunar_mansions: 牛,
                long: swe_degnorm(180.0 + 108.0 + 1.0),
            },
            DistanceStarLong {
                lunar_mansions: 女,
                long: swe_degnorm(180.0 + 120.0 + 1.0),
            },
            DistanceStarLong {
                lunar_mansions: 虚,
                long: swe_degnorm(180.0 + 132.0 + 1.0),
            },
            DistanceStarLong {
                lunar_mansions: 危,
                long: swe_degnorm(180.0 + 144.0 + 1.0),
            },
            DistanceStarLong {
                lunar_mansions: 室,
                long: swe_degnorm(180.0 + 156.0 + 1.0),
            },
            DistanceStarLong {
                lunar_mansions: 壁,
                long: swe_degnorm(180.0 + 168.0 + 1.0),
            },
            //
            DistanceStarLong {
                lunar_mansions: 奎,
                long: swe_degnorm(180.0 + 180.0 + 1.0),
            },
            DistanceStarLong {
                lunar_mansions: 娄,
                long: swe_degnorm(180.0 + 192.0 + 1.0),
            },
            DistanceStarLong {
                lunar_mansions: 胃,
                long: swe_degnorm(180.0 + 204.0 + 1.0),
            },
            DistanceStarLong {
                lunar_mansions: 昴,
                long: swe_degnorm(180.0 + 216.0 + 1.0),
            },
            DistanceStarLong {
                lunar_mansions: 毕,
                long: swe_degnorm(180.0 + 228.0 + 1.0),
            },
            DistanceStarLong {
                lunar_mansions: 觜,
                long: swe_degnorm(180.0 + 240.0 + 1.0),
            },
            DistanceStarLong {
                lunar_mansions: 参,
                long: swe_degnorm(180.0 + 252.0 + 1.0),
            },
            //
            DistanceStarLong {
                lunar_mansions: 井,
                long: swe_degnorm(180.0 + 264.0 + 1.0),
            },
            DistanceStarLong {
                lunar_mansions: 鬼,
                long: swe_degnorm(180.0 + 276.0 + 1.0),
            },
            DistanceStarLong {
                lunar_mansions: 柳,
                long: swe_degnorm(180.0 + 288.0 + 1.0),
            },
            DistanceStarLong {
                lunar_mansions: 星,
                long: swe_degnorm(180.0 + 300.0),
            },
            DistanceStarLong {
                lunar_mansions: 张,
                long: swe_degnorm(180.0 + 312.0),
            },
            DistanceStarLong {
                lunar_mansions: 翼,
                long: swe_degnorm(180.0 + 324.0),
            },
            DistanceStarLong {
                lunar_mansions: 轸,
                long: swe_degnorm(180.0 + 336.0),
            },
        ];

        let distance_star_long_sum: f64 = distance_star_long
            .iter()
            .enumerate()
            .map(|(index, star)| {
                let next_star = &distance_star_long[(index + 1) % 28];
                swe_degnorm(next_star.long - star.long)
            })
            .sum();

        assert_eq!(distance_star_long_sum, 360.0);

        // 辰:180, 轸: 24.0
        let xiu_and_degree = calc_xiu_degree(180.0, &distance_star_long);
        assert!(xiu_and_degree.is_ok());
        let (xiu, xiu_degree) = xiu_and_degree.unwrap();
        assert_eq!(xiu, 轸);
        assert_eq!(xiu_degree, 24.0);

        // 卯:210, 亢: 5.0
        let xiu_and_degree = calc_xiu_degree(210.0, &distance_star_long);
        assert!(xiu_and_degree.is_ok());
        let (xiu, xiu_degree) = xiu_and_degree.unwrap();
        assert_eq!(xiu, 亢);
        assert_eq!(xiu_degree, 5.0);

        // 寅:240, 房: 11.0
        let xiu_and_degree = calc_xiu_degree(240.0, &distance_star_long);
        assert!(xiu_and_degree.is_ok());
        let (xiu, xiu_degree) = xiu_and_degree.unwrap();
        assert_eq!(xiu, 房);
        assert_eq!(xiu_degree, 11.0);

        // 丑:270, 箕: 5.0
        let xiu_and_degree = calc_xiu_degree(270.0, &distance_star_long);
        assert!(xiu_and_degree.is_ok());
        let (xiu, xiu_degree) = xiu_and_degree.unwrap();
        assert_eq!(xiu, 箕);
        assert_eq!(xiu_degree, 5.0);

        // 子:300, 牛: 11.0
        let xiu_and_degree = calc_xiu_degree(300.0, &distance_star_long);
        assert!(xiu_and_degree.is_ok());
        let (xiu, xiu_degree) = xiu_and_degree.unwrap();
        assert_eq!(xiu, 牛);
        assert_eq!(xiu_degree, 11.0);

        // 亥:330, 危: 5.0
        let xiu_and_degree = calc_xiu_degree(330.0, &distance_star_long);
        assert!(xiu_and_degree.is_ok());
        let (xiu, xiu_degree) = xiu_and_degree.unwrap();
        assert_eq!(xiu, 危);
        assert_eq!(xiu_degree, 5.0);

        // 戌:0, 壁: 11.0
        let xiu_and_degree = calc_xiu_degree(0.0, &distance_star_long);
        assert!(xiu_and_degree.is_ok());
        let (xiu, xiu_degree) = xiu_and_degree.unwrap();
        assert_eq!(xiu, 壁);
        assert_eq!(xiu_degree, 11.0);

        // 酉:30, 胃: 5.0
        let xiu_and_degree = calc_xiu_degree(30.0, &distance_star_long);
        assert!(xiu_and_degree.is_ok());
        let (xiu, xiu_degree) = xiu_and_degree.unwrap();
        assert_eq!(xiu, 胃);
        assert_eq!(xiu_degree, 5.0);

        // 申:60, 毕: 11.0
        let xiu_and_degree = calc_xiu_degree(60.0, &distance_star_long);
        assert!(xiu_and_degree.is_ok());
        let (xiu, xiu_degree) = xiu_and_degree.unwrap();
        assert_eq!(xiu, 毕);
        assert_eq!(xiu_degree, 11.0);

        // 未:90, 井: 5.0
        let xiu_and_degree = calc_xiu_degree(90.0, &distance_star_long);
        assert!(xiu_and_degree.is_ok());
        let (xiu, xiu_degree) = xiu_and_degree.unwrap();
        assert_eq!(xiu, 井);
        assert_eq!(xiu_degree, 5.0);

        // 午:120, 星: 0.0
        let xiu_and_degree = calc_xiu_degree(120.0, &distance_star_long);
        assert!(xiu_and_degree.is_ok());
        let (xiu, xiu_degree) = xiu_and_degree.unwrap();
        assert_eq!(xiu, 星);
        assert_eq!(xiu_degree, 0.0);

        // 巳:150, 翼: 6.0
        let xiu_and_degree = calc_xiu_degree(150.0, &distance_star_long);
        assert!(xiu_and_degree.is_ok());
        let (xiu, xiu_degree) = xiu_and_degree.unwrap();
        assert_eq!(xiu, 翼);
        assert_eq!(xiu_degree, 6.0);
    }
}
