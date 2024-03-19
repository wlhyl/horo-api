use crate::{
    config::PlanetConfig,
    lunar_mansions::{calc_xiu_degree, LunarMansionsName},
    DistanceStarLong, Error,
};
use horo_date_time::HoroDateTime;
use swe::{swe_calc_ut, swe_close, swe_degnorm, swe_set_ephe_path, Body, Flag};
use PlanetName::*;
use PlanetSpeedState::*;

#[cfg(feature = "serde")]
use serde::Serialize;

#[cfg(feature = "swagger")]
use utoipa::ToSchema;

#[derive(PartialEq, Eq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
#[cfg_attr(feature = "swagger", derive(ToSchema))]
pub enum PlanetSpeedState {
    疾,
    均,
    迟,
    // 留,伏,逆，此三者由前端计算
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[cfg_attr(feature = "swagger", derive(ToSchema))]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum PlanetName {
    日,
    月,
    水,
    金,
    火,
    木,
    土,
    计, // 北交
    罗, // 南交
    孛,
    气,
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[cfg_attr(feature = "swagger", derive(ToSchema))]
pub struct Planet {
    pub name: PlanetName,
    /// 行星的黄经
    pub long: f64,
    /// 行星在黄道上每日的移动速度
    speed: f64,

    /// 行星在黄道上的宿
    xiu: LunarMansionsName,

    /// 行星在黄道上的入宿度
    xiu_degree: f64,
    /// 行星速度状态：快、平均、慢
    speed_state: PlanetSpeedState,
    /// 停滞，行星移动速度小于1度，是停滞，只有，水、金、火、木、土，有停滞
    is_stationary: bool,
}

impl Planet {
    pub fn new(
        name: PlanetName,
        long: f64,
        speed: f64,
        xiu: LunarMansionsName,
        xiu_degree: f64,
        is_stationary: bool,
        config: &PlanetConfig,
    ) -> Self {
        let speed_state = if config.min < config.max {
            if speed.abs() > config.max {
                疾
            } else if speed.abs() < config.min {
                迟
            } else {
                均
            }
        } else {
            均
        };

        let is_stationary = if [水, 金, 火, 木, 土].contains(&name) {
            is_stationary
        } else {
            false
        };

        Self {
            name,
            long,
            speed,
            speed_state,
            xiu,
            xiu_degree,
            is_stationary,
        }
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
pub(crate) fn calc_planets(
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

        // 计算停滞，以当前时间，前两日，后两日的速度改变作为本日是停滞
        let is_stationary = if [水, 金, 火, 木, 土].contains(&planet_name) {
            let xx = swe_calc_ut(jd_utc - 2.0, &body, &[Flag::SeflgSpeed])
                .map_err(|e| Error::Function(format!("计算行星错误:{e}")))?;
            let speed0 = xx[3];
            let xx = swe_calc_ut(jd_utc + 2.0, &body, &[Flag::SeflgSpeed])
                .map_err(|e| Error::Function(format!("计算行星错误:{e}")))?;
            let speed1 = xx[3];

            if speed0 * speed1 > 0.0 {
                false
            } else {
                true
            }
        } else {
            false
        };
        let p = Planet::new(
            planet_name,
            star_long, // long
            speed,     // speed
            xiu,
            xiu_degree,
            is_stationary,
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
                false,
                planet_config,
            );
            planets.push(p);
        }
    }

    swe_close();

    Ok(planets)
}

#[cfg(test)]
mod tests {
    use crate::config::PlanetConfig;
    use crate::LunarMansionsName::角;

    use super::Planet;
    use super::PlanetName::*;
    use super::PlanetSpeedState::*;

    #[test]
    fn test_new() {
        let p = Planet::new(
            日,
            1.0,
            1.0,
            角,
            1.0,
            false,
            &PlanetConfig::new(日, 1.0, 2.0),
        );

        assert_eq!(日, p.name, "name");
        assert_eq!(1.0, p.long, "黄经");
        assert_eq!(1.0, p.speed, "speed");
        assert_eq!(角, p.xiu, "二十八宿");
        assert_eq!(1.0, p.xiu_degree, "speed");
        assert!(!p.is_stationary);
    }

    // 快
    #[test]
    fn test_faster() {
        // 逆行
        let p0 = Planet::new(
            日,
            1.0,
            -3.1,
            角,
            1.0,
            false,
            &PlanetConfig::new(日, 1.0, 2.0),
        );
        assert_eq!(疾, p0.speed_state, "逆行，快");

        // 顺行
        let p1 = Planet::new(
            日,
            1.0,
            3.1,
            角,
            1.0,
            false,
            &PlanetConfig::new(日, 1.0, 2.0),
        );
        assert_eq!(疾, p1.speed_state, "顺行，快")
    }

    // 慢
    #[test]
    fn test_slower() {
        // 逆行
        let p0 = Planet::new(
            日,
            1.0,
            -0.1,
            角,
            1.0,
            false,
            &PlanetConfig::new(日, 1.0, 2.0),
        );
        assert_eq!(迟, p0.speed_state, "逆行，慢");

        // 顺行
        let p1 = Planet::new(
            日,
            1.0,
            0.1,
            角,
            1.0,
            false,
            &PlanetConfig::new(日, 1.0, 2.0),
        );
        assert_eq!(迟, p1.speed_state, "顺行，慢");
    }

    // 平均
    #[test]
    fn test_average() {
        let p0 = Planet::new(
            日,
            1.0,
            -0.1,
            角,
            1.0,
            false,
            &PlanetConfig::new(日, 2.0, 2.0),
        );
        assert_eq!(均, p0.speed_state, "逆行，均");

        let p1 = Planet::new(
            日,
            1.0,
            0.1,
            角,
            1.0,
            false,
            &PlanetConfig::new(日, 2.0, 2.0),
        );
        assert_eq!(均, p1.speed_state, "顺行，均");
    }

    // 停滞
    #[test]
    fn test_stationary() {
        // 日、月、计、罗、孛、气,无停滞
        for planet_name in [日, 月, 计, 罗, 孛, 气] {
            // 顺行，停滞
            let p = Planet::new(
                planet_name,
                1.0,
                0.1,
                角,
                1.0,
                true,
                &PlanetConfig::new(日, 2.0, 2.0),
            );
            assert!(!p.is_stationary);

            // 逆行，停滞
            let p = Planet::new(
                planet_name,
                1.0,
                -0.1,
                角,
                1.0,
                true,
                &PlanetConfig::new(日, 2.0, 2.0),
            );
            assert!(!p.is_stationary);

            // 顺行，非停滞
            let p = Planet::new(
                planet_name,
                1.0,
                2.1,
                角,
                1.0,
                false,
                &PlanetConfig::new(日, 2.0, 2.0),
            );
            assert!(!p.is_stationary);

            // 逆行，非停滞
            let p = Planet::new(
                planet_name,
                1.0,
                -2.1,
                角,
                1.0,
                false,
                &PlanetConfig::new(日, 2.0, 2.0),
            );
            assert!(!p.is_stationary);
        }

        // 水、金、火、木、土,有停滞
        for planet_name in [水, 金, 火, 木, 土] {
            // 顺行，停滞
            let p = Planet::new(
                planet_name,
                1.0,
                0.1,
                角,
                1.0,
                true,
                &PlanetConfig::new(日, 2.0, 2.0),
            );
            assert!(p.is_stationary);

            // 逆行，停滞
            let p = Planet::new(
                planet_name,
                1.0,
                -0.1,
                角,
                1.0,
                true,
                &PlanetConfig::new(日, 2.0, 2.0),
            );
            assert!(p.is_stationary);

            // 顺行，非停滞
            let p = Planet::new(
                planet_name,
                1.0,
                2.1,
                角,
                1.0,
                false,
                &PlanetConfig::new(日, 2.0, 2.0),
            );
            assert!(!p.is_stationary);

            // 逆行，非停滞
            let p = Planet::new(
                planet_name,
                1.0,
                -2.1,
                角,
                1.0,
                false,
                &PlanetConfig::new(日, 2.0, 2.0),
            );
            assert!(!p.is_stationary);
        }
    }
}
