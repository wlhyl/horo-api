use crate::{
    utils::{calc_eps, mod180, newton_iteration},
    Aspect, Error, GeoPosition, HoroDateTime, HouseName, Planet, PlanetConfig, PlanetName,
};
use swe::{
    swe_calc_ut, swe_close, swe_cotrans, swe_degnorm, swe_houses, swe_set_ephe_path, Body, Flag,
    HouseSystem,
};

#[cfg(feature = "serde")]
use serde::Serialize;

#[cfg(feature = "swagger")]
use utoipa::ToSchema;

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
#[cfg_attr(feature = "swagger", derive(ToSchema))]
pub struct Horoscope {
    /// 绘制星盘的时间
    pub date: HoroDateTime,
    /// 绘制星盘的地理位置
    pub geo: GeoPosition,
    /// 星盘的宫位
    pub house_name: HouseName,
    /// 12宫头黄经度数
    pub houses_cups: Vec<f64>,
    //     @field:Schema(description = "上升点")
    pub asc: Planet,
    //     @field:Schema(description = "中天")
    pub mc: Planet,
    //     @field:Schema(description = "下降点")
    pub dsc: Planet,
    //     @field:Schema(description = "天底")
    pub ic: Planet,
    //     @field:Schema(description = "七颗行星")
    pub planets: Vec<Planet>,
    //     @field:Schema(description = "白天盘:true,夜间盘:false")
    pub is_diurnal: bool,
    //     @field:Schema(description = "日主星，值为行星id，与瑞士星历表同")
    pub planetary_day: PlanetName,
    //     @field:Schema(description = "时主星，值为行星id，与瑞士星历表同")
    pub planetary_hours: PlanetName,
    //     @field:Schema(description = "行星相位，仅包含四轴、行星间的相位")
    pub aspects: Vec<Aspect>,
}
impl Horoscope {
    pub fn new(
        date: HoroDateTime,
        geo: GeoPosition,
        house_name: HouseName,
        planets_config: &[PlanetConfig],
        ephe_path: &str,
    ) -> Result<Self, Error> {
        // 计算宫位
        let (cups, ascmc) =
            if let Ok(v) = swe_houses(date.jd_ut1, geo.lat, geo.long, &(&house_name).into()) {
                v
            } else {
                return Err(Error::Function("swe_houses()调用失败".to_owned()));
            };

        // 计算四轴
        // 计算四轴的赤经，赤纬
        // 先计算黄赤倾角
        let eps = calc_eps(date.jd_utc, ephe_path)?;
        // 0: ASC, 1: MC
        // 计算asc的赤经、赤纬
        let default_planet_config = PlanetConfig::default_config(&PlanetName::ASC);
        let planet_config = planets_config
            .iter()
            .find(|p| p.name == PlanetName::ASC)
            .unwrap_or(&default_planet_config);
        let asc_equator = swe_cotrans(ascmc[0], 0.0, 1.0, -eps);
        let asc = Planet::new(
            PlanetName::ASC,
            ascmc[0],
            0.0,
            0.0,
            asc_equator[0],
            asc_equator[1],
            planet_config,
        );

        let default_planet_config = PlanetConfig::default_config(&PlanetName::MC);
        let planet_config = planets_config
            .iter()
            .find(|p| p.name == PlanetName::MC)
            .unwrap_or(&default_planet_config);
        let mc_equator = swe_cotrans(ascmc[1], 0.0, 1.0, -eps);
        let mc = Planet::new(
            PlanetName::MC,
            ascmc[1],
            0.0,
            0.0,
            mc_equator[0],
            mc_equator[1],
            planet_config,
        );

        let default_planet_config = PlanetConfig::default_config(&PlanetName::DSC);
        let planet_config = planets_config
            .iter()
            .find(|p| p.name == PlanetName::DSC)
            .unwrap_or(&default_planet_config);
        let dsc_long = swe_degnorm(ascmc[0] + 180.0);
        let dsc_equator = swe_cotrans(dsc_long, 0.0, 1.0, -eps);
        let dsc = Planet::new(
            PlanetName::DSC,
            dsc_long,
            0.0,
            0.0,
            dsc_equator[0],
            dsc_equator[1],
            planet_config,
        );

        let default_planet_config = PlanetConfig::default_config(&PlanetName::IC);
        let planet_config = planets_config
            .iter()
            .find(|p| p.name == PlanetName::IC)
            .unwrap_or(&default_planet_config);
        let ic_long = swe_degnorm(ascmc[1] + 180.0);
        let ic_equator = swe_cotrans(ic_long, 0.0, 1.0, -eps);
        let ic = Planet::new(
            PlanetName::IC,
            ic_long,
            0.0,
            0.0,
            ic_equator[0],
            ic_equator[1],
            planet_config,
        );

        // 计算行星
        let planets = calc_planets(date.jd_utc, planets_config, ephe_path)?;

        // 星盘昼夜
        let sun = planets.iter().find(|p| p.name == PlanetName::Sun).unwrap();

        let diff = swe_degnorm(asc.long - sun.long);
        let is_diurnal = diff <= 180.0;

        // 计算时主星
        //月、火、水、木、金、土、日
        // 一、二、三、四、五、六、日
        // 1  2   3  4   5  6   0
        // 迦勒底序
        let chaldean_order = [
            PlanetName::Sun,
            PlanetName::Moon,
            PlanetName::Mars,
            PlanetName::Mercury,
            PlanetName::Jupiter,
            PlanetName::Venus,
            PlanetName::Saturn,
        ];

        let sun_on_asc_time = sun_on_asc(&date, &geo, ephe_path)?;
        let sun_on_dsc_time = sun_on_dsc(&date, &geo, ephe_path)?;
        // 计算第二天太阳在asc的时刻，以大约中午时刻迭代，即sunOnDscTime+1.25天
        let sun_on_asc_time_next = sun_on_asc(&sun_on_asc_time.plus_days(1.25)?, &geo, ephe_path)?;

        // 计算星期
        // 儒略历，公元前4713年1月1日，是星期一
        // 此日，jd = 0
        let t = HoroDateTime::new(
            sun_on_asc_time.year,
            sun_on_asc_time.month,
            sun_on_asc_time.day,
            12,
            0,
            0,
            0.0,
        )?;
        // jd:0,1,2,3 ,4 ,5 ,6
        //    7,8,9,10,11,12,13
        //    1,2,3,4 ,5 ,6, 0
        // jd % 7 得到0, 6，对应1到7，因此需要加1
        // 加1后，得到1到7，因此需要%7，因为7对应0，是星期日
        let n = (t.jd_utc as u32 % 7 + 1) % 7;

        // 日主星
        let planetary_day = chaldean_order[n as usize].clone();

        // 时主星序，土、木、火、日、金、水、月
        let planetary_hours_list = [
            PlanetName::Saturn,
            PlanetName::Jupiter,
            PlanetName::Mars,
            PlanetName::Sun,
            PlanetName::Venus,
            PlanetName::Mercury,
            PlanetName::Moon,
        ];
        // 第一个时主星
        let first_planetary_hours_index = planetary_hours_list
            .iter()
            .position(|p| *p == planetary_day)
            .unwrap();
        let planetary_hours = if date.jd_utc < sun_on_dsc_time.jd_utc {
            let m = (12.0 * (date.jd_utc - sun_on_asc_time.jd_utc)
                / (sun_on_dsc_time.jd_utc - sun_on_asc_time.jd_utc)) as usize;
            planetary_hours_list[(first_planetary_hours_index + m) % 7].clone()
        } else {
            let m = (12.0 * (date.jd_utc - sun_on_dsc_time.jd_utc)
                / (sun_on_asc_time_next.jd_utc - sun_on_dsc_time.jd_utc))
                as usize;
            planetary_hours_list[(first_planetary_hours_index + m + 12) % 7].clone()
        };

        // 计算相位
        let mut aspects: Vec<Aspect> = vec![];
        // let asm_and_planets = [
        //     planets.clone(),
        //     vec![asc.clone(), mc.clone(), dsc.clone(), ic.clone()],
        // ]
        // .concat();
        let mut asm_and_planets: Vec<_> = planets.iter().collect();
        asm_and_planets.push(&asc);
        asm_and_planets.push(&mc);
        asm_and_planets.push(&dsc);
        asm_and_planets.push(&ic);
        for i in 0..asm_and_planets.len() {
            for j in i..asm_and_planets.len() {
                let aspect = asm_and_planets[i].has_aspect(asm_and_planets[j], false);
                if let Some(aspect) = aspect {
                    aspects.push(aspect)
                }
            }
        }

        Ok(Self {
            date,
            geo,
            house_name,
            houses_cups: cups[1..13].to_vec(),
            asc,
            mc,
            dsc,
            ic,
            planets,
            is_diurnal,
            planetary_day,
            planetary_hours,
            aspects,
        })
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
#[cfg_attr(feature = "swagger", derive(ToSchema))]
pub struct HoroscopeCompare {
    /// 原星盘的时间
    pub date: HoroDateTime,
    /// 比较盘时间
    pub date_compare: HoroDateTime,
    /// 绘制星盘的地理位置
    pub geo: GeoPosition,
    /// 星盘的宫位
    pub house_name: HouseName,
    /// 12宫头黄经度数
    pub houses_cups: Vec<f64>,
    // 比较盘12宫头黄经度数
    // pub houses_cups_compare: Vec<f64>,
    // 上升点
    pub asc: Planet,
    pub asc_compare: Planet,
    // 中天
    pub mc: Planet,
    pub mc_compare: Planet,
    // 下降点
    pub dsc: Planet,
    pub dsc_compare: Planet,
    //     天底
    pub ic: Planet,
    pub ic_compare: Planet,
    // 七颗行星
    pub planets: Vec<Planet>,
    pub planets_compare: Vec<Planet>,

    // 行星相位，仅包含四轴、行星间的相位
    pub aspects: Vec<Aspect>,
}

impl HoroscopeCompare {
    pub fn new(
        date: HoroDateTime,
        date_compare: HoroDateTime,
        geo: GeoPosition,
        house_name: HouseName,
        planets_config: &[PlanetConfig],
        ephe_path: &str,
    ) -> Result<Self, Error> {
        // 计算原星盘
        let horo = Horoscope::new(
            date.clone(),
            geo.clone(),
            house_name.clone(),
            planets_config,
            ephe_path,
        )?;

        let horo_compare = Horoscope::new(
            date_compare.clone(),
            geo.clone(),
            house_name.clone(),
            planets_config,
            ephe_path,
        )?;

        // 计算相位
        let mut aspects: Vec<Aspect> = vec![];

        let mut asm_and_planets: Vec<_> = horo.planets.iter().collect();
        asm_and_planets.push(&horo.asc);
        asm_and_planets.push(&horo.mc);
        asm_and_planets.push(&horo.dsc);
        asm_and_planets.push(&horo.ic);

        let mut asm_and_planets_compare: Vec<_> = horo_compare.planets.iter().collect();
        asm_and_planets_compare.push(&horo_compare.asc);
        asm_and_planets_compare.push(&horo_compare.mc);
        asm_and_planets_compare.push(&horo_compare.dsc);
        asm_and_planets_compare.push(&horo_compare.ic);

        for i in 0..asm_and_planets.len() {
            for j in 0..asm_and_planets_compare.len() {
                let aspect = asm_and_planets[i].has_aspect(asm_and_planets_compare[j], true);
                if let Some(aspect) = aspect {
                    aspects.push(aspect)
                }
            }
        }

        Ok(Self {
            date,

            date_compare,

            geo,

            house_name,

            houses_cups: horo.houses_cups,

            // houses_cups_compare: horo_compare.houses_cups,
            asc: horo.asc,
            asc_compare: horo_compare.asc,

            mc: horo.mc,
            mc_compare: horo_compare.mc,

            dsc: horo.dsc,
            dsc_compare: horo_compare.dsc,

            ic: horo.ic,
            ic_compare: horo_compare.ic,

            planets: horo.planets,
            planets_compare: horo_compare.planets,

            aspects,
        })
    }
}

//   计算行星
fn calc_planets(
    jd_utc: f64,
    planets_config: &[PlanetConfig],
    ephe_path: &str,
) -> Result<Vec<Planet>, Error> {
    swe_set_ephe_path(ephe_path);

    let mut planets = vec![];
    let planet_names = [
        PlanetName::Sun,
        PlanetName::Moon,
        PlanetName::Mercury,
        PlanetName::Venus,
        PlanetName::Mars,
        PlanetName::Jupiter,
        PlanetName::Saturn,
        PlanetName::NorthNode,
    ];
    for planet_name in planet_names {
        let body = match planet_name {
            PlanetName::Sun => Body::SeSun,
            PlanetName::Moon => Body::SeMoon,
            PlanetName::Mercury => Body::SeMercury,
            PlanetName::Venus => Body::SeVenus,
            PlanetName::Mars => Body::SeMars,
            PlanetName::Jupiter => Body::SeJupiter,
            PlanetName::Saturn => Body::SeSaturn,
            _ => Body::SeMeanNode,
        };
        let xx = swe_calc_ut(jd_utc, &body, &[Flag::SeflgSpeed])
            .map_err(|e| Error::Function(format!("计算行星错误:{e}")))?;
        //计算赤经和赤纬
        let yy = swe_calc_ut(jd_utc, &body, &[Flag::SeflgEquatorial])
            .map_err(|e| Error::Function(format!("计算行星错误:{e}")))?;

        swe_close();

        let default_planet_config = PlanetConfig::default_config(&planet_name);
        let planet_config = planets_config
            .iter()
            .find(|p| p.name == planet_name)
            .unwrap_or(&default_planet_config);
        let p = Planet::new(
            planet_name.clone(),
            xx[0],
            xx[1],
            xx[3],
            yy[0],
            yy[1],
            planet_config,
        );
        planets.push(p);
        if planet_name == PlanetName::NorthNode {
            let p = Planet::new(
                PlanetName::SouthNode,
                swe_degnorm(xx[0] + 180.0),
                xx[1],
                xx[3],
                swe_degnorm(yy[0] + 180.0),
                -yy[1],
                planet_config,
            );
            planets.push(p);
        }
    }

    Ok(planets)
}

/// 计算给定时刻之前，太阳在东方地平线上的时刻
fn sun_on_asc(t: &HoroDateTime, geo: &GeoPosition, ephe_path: &str) -> Result<HoroDateTime, Error> {
    swe_set_ephe_path(ephe_path);
    let xx = swe_calc_ut(t.jd_utc, &Body::SeSun, &[])
        .map_err(|e| Error::Function(format!("函数sun_on_asc计算太阳位置错误:{e}")))?;
    swe_close();

    let sun_long = xx[0];

    let (_, ascmc) = swe_houses(t.jd_ut1, geo.lat, geo.long, &HouseSystem::B)
        .map_err(|_e| Error::Function("函数sun_on_asc()，计算asc错误".to_string()))?;

    let asc_long = ascmc[0];

    // 为牛顿迭代设置初值，太阳在东方地平线时刻大约等于jd0
    let d = swe_degnorm(asc_long - sun_long);
    // 太阳周日运动，1天走过360度，走1度需要1/360天，走d度，需要1/360*d天
    // 因此jd0 = jd_utc - d/360.0
    let jd0 = t.jd_utc - d / 360.0;

    let jd = newton_iteration(jd0, |jd| {
        let t0 = HoroDateTime::from_jd_zone(jd, t.tz)?;
        swe_set_ephe_path(ephe_path);
        let xx = swe_calc_ut(t0.jd_utc, &Body::SeSun, &[]).map_err(|e| {
            Error::Function(format!("函数sun_on_asc()，牛顿迭代计算太阳位置错误:{e}"))
        })?;
        let (_, ascmc) = swe_houses(t0.jd_ut1, geo.lat, geo.long, &HouseSystem::B)
            .map_err(|_e| Error::Function("函数sun_on_asc()，牛顿迭代计算asc错误".to_string()))?;
        swe_close();

        // 将结果转换到[0, 360)，再将结果转换到[-180, 180]
        // 这样可以确保曲线上太阳与asc黄经度数相等的点是连续的，
        Ok(mod180(swe_degnorm(ascmc[0] - xx[0])))
    })?;

    HoroDateTime::from_jd_zone(jd, t.tz)
}

/// 计算太阳在西方地平线上的时刻
/// 如果太阳在地平线上，则计算之后的时刻
/// 如果太阳在地平线下，则计算之前的时刻
fn sun_on_dsc(t: &HoroDateTime, geo: &GeoPosition, ephe_path: &str) -> Result<HoroDateTime, Error> {
    swe_set_ephe_path(ephe_path);
    let xx = swe_calc_ut(t.jd_utc, &Body::SeSun, &[])
        .map_err(|e| Error::Function(format!("函数sun_on_dsc计算太阳位置错误:{e}")))?;
    swe_close();

    let sun_long = xx[0];

    let (_, ascmc) = swe_houses(t.jd_ut1, geo.lat, geo.long, &HouseSystem::B)
        .map_err(|_e| Error::Function("函数sun_on_dsc()，计算asc错误".to_string()))?;

    let asc_long = ascmc[0];

    let d = swe_degnorm(asc_long - sun_long);
    // 假定白天长度为0.5天
    let jd0 = t.jd_utc - d / 360.0 + 0.5;

    let jd = newton_iteration(jd0, |jd| {
        let t0 = HoroDateTime::from_jd_zone(jd, t.tz)?;
        swe_set_ephe_path(ephe_path);

        let xx = swe_calc_ut(t0.jd_utc, &Body::SeSun, &[]).map_err(|e| {
            Error::Function(format!("函数sun_on_dsc()，牛顿迭代计算太阳位置错误:{e}"))
        })?;
        let (_, ascmc) = swe_houses(t0.jd_ut1, geo.lat, geo.long, &HouseSystem::B)
            .map_err(|_e| Error::Function("函数sun_on_dsc()，牛顿迭代计算asc错误".to_string()))?;

        swe_close();

        // 将结果转换到[0, 360)，再将结果转换到[-180, 180]
        // 这样可以确保曲线上太阳与asc黄经度数相等的点是连续的，
        Ok(mod180(swe_degnorm(swe_degnorm(ascmc[0] + 180.0) - xx[0])))
    })?;

    HoroDateTime::from_jd_zone(jd, t.tz)
}

#[cfg(test)]
mod tests {
    use std::env;

    use swe::{
        swe_calc_ut, swe_close, swe_cotrans, swe_degnorm, swe_houses, swe_set_ephe_path, Body, Flag,
    };

    use crate::{
        config::PlanetConfig, geo_position::GeoPosition, horo_date_time::HoroDateTime,
        house::HouseName, planet::PlanetSpeedState::*, utils::calc_eps, Horoscope,
        HoroscopeCompare, PlanetName::*,
    };

    #[test]
    fn test_horoscope_new() {
        dotenvy::dotenv().ok();
        let ephe_path = env::var("EPHE_PATH")
            .expect("没设置 EPHE_PATH 环境变量，可在.env文件中设置或export EPHE_PATH=...");

        let t = HoroDateTime::new(2021, 9, 14, 10, 30, 20, 8.0);
        assert!(t.is_ok());
        let t = t.unwrap();

        let geo = GeoPosition::new(
            102.0 + 41.0 / 60.0 + 59.0 / 3600.0,
            25.0 + 1.0 / 60.0 + 53.0 / 3600.0,
        );
        assert!(geo.is_ok());
        let geo = geo.unwrap();

        let house = HouseName::Alcabitus;
        let planet_configs = PlanetConfig::default_all_configs();

        let horo = Horoscope::new(
            t.clone(),
            geo.clone(),
            house.clone(),
            &planet_configs,
            &ephe_path,
        );
        assert!(horo.is_ok());
        let horo = horo.unwrap();

        // 时间
        assert_eq!(t.year, horo.date.year);
        assert_eq!(t.month, horo.date.month);
        assert_eq!(t.hour, horo.date.hour);
        assert_eq!(t.minute, horo.date.minute);
        assert_eq!(t.second, horo.date.second);
        assert_eq!(t.tz, horo.date.tz);

        // 大地经纬度
        assert_eq!(geo.long, horo.geo.long); //, this.doubleDelta)
        assert_eq!(geo.lat, horo.geo.lat); //, this.doubleDelta)

        // 宫位系统
        match horo.house_name {
            HouseName::Alcabitus => assert!(true),
            _ => assert!(false),
        }

        // 12宫
        let yy = swe_houses(t.jd_ut1, geo.lat, geo.long, &(&house).into());
        assert!(yy.is_ok(), "swe_houses()调用失败");
        let (houses_cups, ascmc) = yy.unwrap();
        let houses_cups = &houses_cups[1..13];

        assert_eq!(12, horo.houses_cups.len());

        for i in 0..12 {
            assert_eq!(houses_cups[i], horo.houses_cups[i]);
        }

        // 四轴
        let eps = calc_eps(t.jd_utc, &ephe_path);
        assert!(eps.is_ok());
        let eps = eps.unwrap();
        // 0: ASC, 1: MC
        let asc_equator = swe_cotrans(ascmc[0], 0.0, 1.0, -eps);
        assert_eq!(ASC, horo.asc.name, "asc name");
        assert_eq!(ascmc[0], horo.asc.long, "asc 黄道经度");
        assert_eq!(0.0, horo.asc.lat, "asc 黄纬");
        assert_eq!(asc_equator[0], horo.asc.ra, "asc 赤经");
        assert_eq!(asc_equator[1], horo.asc.dec, "asc 赤纬");
        assert_eq!(0, horo.asc.orb, "asc 容许度");
        assert_eq!(均, horo.asc.speed_state, "asc速度是“均”");

        // mc
        let mc_equator = swe_cotrans(ascmc[1], 0.0, 1.0, -eps);
        assert_eq!(MC, horo.mc.name, "mc name");
        assert_eq!(ascmc[1], horo.mc.long, "mc 黄道经度");
        assert_eq!(0.0, horo.mc.lat, "mc 黄纬");
        assert_eq!(mc_equator[0], horo.mc.ra, "mc 赤经");
        assert_eq!(mc_equator[1], horo.mc.dec, "mc 赤纬");
        assert_eq!(0, horo.mc.orb, "mc 容许度");
        assert_eq!(均, horo.mc.speed_state, "mc速度是均");

        // DSC
        let dsc_equator = swe_cotrans(swe_degnorm(ascmc[0] + 180.0), 0.0, 1.0, -eps);
        assert_eq!(DSC, horo.dsc.name, "dsc name");
        assert_eq!(swe_degnorm(ascmc[0] + 180.0), horo.dsc.long, "dsc 黄道经度");
        assert_eq!(0.0, horo.dsc.lat, "dsc 黄纬");
        assert_eq!(dsc_equator[0], horo.dsc.ra, "dsc 赤经");
        assert_eq!(dsc_equator[1], horo.dsc.dec, "dsc 赤纬");
        assert_eq!(0, horo.dsc.orb, "dsc 容许度");
        assert_eq!(均, horo.dsc.speed_state, "dsc速冻是均");

        // IC
        let ic_equator = swe_cotrans(swe_degnorm(ascmc[1] + 180.0), 0.0, 1.0, -eps);
        assert_eq!(IC, horo.ic.name, "ic name");
        assert_eq!(swe_degnorm(ascmc[1] + 180.0), horo.ic.long, "ic 黄道经度");
        assert_eq!(0.0, horo.ic.lat, "ic 黄纬");
        assert_eq!(ic_equator[0], horo.ic.ra, "ic 赤经");
        assert_eq!(ic_equator[1], horo.ic.dec, "ic 赤纬");
        assert_eq!(0, horo.ic.orb, "ic 容许度");
        assert_eq!(均, horo.ic.speed_state, "IC速度是均");

        // 七颗正星
        for planet_name in [Sun, Moon, Mercury, Venus, Mars, Jupiter, Saturn] {
            let p = horo.planets.iter().find(|p| p.name == planet_name);
            assert!(p.is_some());
            let p = p.unwrap();

            let body = match planet_name {
                Sun => Body::SeSun,
                Moon => Body::SeMoon,
                Mercury => Body::SeMercury,
                Venus => Body::SeVenus,
                Mars => Body::SeMars,
                Jupiter => Body::SeJupiter,
                _ => Body::SeSaturn, // Saturn
            };

            swe_set_ephe_path(&ephe_path);
            let xx = swe_calc_ut(t.jd_utc, &body, &[Flag::SeflgSpeed]);
            let yy = swe_calc_ut(t.jd_utc, &body, &[Flag::SeflgEquatorial]); //计算赤经和赤纬

            assert!(xx.is_ok(), "计算行星错误");
            assert!(yy.is_ok(), "计算行星错误");
            swe_close();

            let xx = xx.unwrap();
            let yy = yy.unwrap();

            let config = PlanetConfig::default_config(&planet_name);
            let speed_state = if config.max > config.min {
                if xx[3].abs() > config.max {
                    快
                } else if xx[3].abs() < config.min {
                    慢
                } else {
                    均
                }
            } else {
                均
            };
            assert_eq!(planet_name, p.name);
            assert_eq!(xx[0], p.long, "{:?}", planet_name);
            assert_eq!(xx[1], p.lat, "{:?}黄纬", planet_name);
            assert_eq!(xx[3], p.speed, "{:?}黄道上每日速度", planet_name);
            assert_eq!(yy[0], p.ra, "{:?}赤经", planet_name);
            assert_eq!(yy[1], p.dec, "{:?}赤纬", planet_name);
            assert_eq!(config.orb, p.orb, "{:?}容许度", planet_name);
            assert_eq!(speed_state, p.speed_state, "{:?}迟疾", planet_name);
        }

        // 月交点
        let north_node = horo.planets.iter().find(|p| p.name == NorthNode);
        let south_node = horo.planets.iter().find(|p| p.name == SouthNode);

        assert!(north_node.is_some());
        assert!(south_node.is_some());

        let north_node = north_node.unwrap();
        let south_node = south_node.unwrap();

        swe_set_ephe_path(&ephe_path);
        let xx = swe_calc_ut(t.jd_utc, &Body::SeMeanNode, &[Flag::SeflgSpeed]);
        let yy = swe_calc_ut(t.jd_utc, &Body::SeMeanNode, &[Flag::SeflgEquatorial]); //计算赤经和赤纬

        assert!(xx.is_ok(), "计算行星错误");
        assert!(yy.is_ok(), "计算行星错误");
        swe_close();

        let xx = xx.unwrap();
        let yy = yy.unwrap();

        assert_eq!(NorthNode, north_node.name, "北交点");
        assert_eq!(xx[0], north_node.long, "黄经，北交点");
        assert_eq!(0.0, north_node.lat, "黄纬, 北交点");
        assert_eq!(xx[3], north_node.speed, "黄道上每日速度, 北交点");
        assert_eq!(yy[0], north_node.ra, "赤经, 北交点");
        assert_eq!(yy[1], north_node.dec, "赤纬, 北交点");
        assert_eq!(0, north_node.orb, "容许度, 北交点");
        assert_eq!(均, north_node.speed_state, "迟疾, 北交点");

        assert_eq!(SouthNode, south_node.name, "南交点");
        assert_eq!(swe_degnorm(xx[0] + 180.0), south_node.long, "黃经，南交点");
        assert_eq!(0.0, south_node.lat, "黄纬, 南交点");
        assert_eq!(xx[3], south_node.speed, "黄道上每日速度, 南交点");
        assert_eq!(swe_degnorm(yy[0] + 180.0), south_node.ra, "赤经, 南交点");
        assert_eq!(-yy[1], south_node.dec, "赤纬, 南交点");
        assert_eq!(0, south_node.orb, "容许度, 南交点");
        assert_eq!(均, south_node.speed_state, "迟疾, 南交点");

        // 相位
        assert_eq!(13, horo.aspects.len());
    }
    // 星盘昼夜
    #[test]
    fn test_diurnal() {
        dotenvy::dotenv().ok();
        let ephe_path = env::var("EPHE_PATH")
            .expect("没设置 EPHE_PATH 环境变量，可在.env文件中设置或export EPHE_PATH=...");

        let geo = GeoPosition::new(
            102.0 + 41.0 / 60.0 + 59.0 / 3600.0,
            25.0 + 1.0 / 60.0 + 53.0 / 3600.0,
        );
        assert!(geo.is_ok());
        let geo = geo.unwrap();

        let house = HouseName::Alcabitus;
        let planet_configs = PlanetConfig::default_all_configs();

        let diurnal = HoroDateTime::new(2021, 9, 14, 10, 30, 20, 8.0);
        let nocturnal = HoroDateTime::new(2021, 9, 14, 22, 30, 20, 8.0);

        assert!(diurnal.is_ok());
        assert!(nocturnal.is_ok());

        let diurnal = diurnal.unwrap();
        let nocturnal = nocturnal.unwrap();

        let horo_diurnal = Horoscope::new(
            diurnal,
            geo.clone(),
            house.clone(),
            &planet_configs,
            &ephe_path,
        );
        let horo_nocturnal =
            Horoscope::new(nocturnal, geo, house.clone(), &planet_configs, &ephe_path);

        assert!(horo_diurnal.is_ok());
        assert!(horo_nocturnal.is_ok());

        let horo_diurnal = horo_diurnal.unwrap();
        let horo_nocturnal = horo_nocturnal.unwrap();

        assert!(horo_diurnal.is_diurnal, "白天盘");
        assert!(!horo_nocturnal.is_diurnal, "夜间盘");

        // 此时刻太阳在地平线上，前一分钟，太阳在地平线下
        let t0 = HoroDateTime::new(2021, 9, 16, 7, 3, 0, 8.0);
        let t1 = HoroDateTime::new(2021, 9, 16, 7, 2, 0, 8.0);
        assert!(t0.is_ok());
        assert!(t1.is_ok());
        let t0 = t0.unwrap();
        let t1 = t1.unwrap();

        let geo = GeoPosition::new(102.0, 25.0);
        assert!(geo.is_ok());
        let geo = geo.unwrap();

        let h0 = Horoscope::new(t0, geo.clone(), house.clone(), &planet_configs, &ephe_path);
        let h1 = Horoscope::new(t1, geo, house, &planet_configs, &ephe_path);

        assert!(h0.is_ok());
        assert!(h1.is_ok());

        let h0 = h0.unwrap();
        let h1 = h1.unwrap();

        assert!(h0.is_diurnal, "白天盘");
        assert!(!h1.is_diurnal, "夜间盘");
    }

    // 日主星
    #[test]
    fn test_planetary_day() {
        dotenvy::dotenv().ok();
        let ephe_path = env::var("EPHE_PATH")
            .expect("没设置 EPHE_PATH 环境变量，可在.env文件中设置或export EPHE_PATH=...");
        //月、火、水、木、金、土、日
        // 一、二、三、四、五、六、日
        // 迦勒底序
        let chaldean_order = [Sun, Moon, Mars, Mercury, Jupiter, Venus, Saturn];

        // 2021-9-12 星期日，太阳掌管
        let t0 = HoroDateTime::new(2021, 9, 12, 10, 30, 20, 8.0);
        let t1 = HoroDateTime::new(2021, 9, 12, 22, 30, 20, 8.0);
        assert!(t0.is_ok());
        assert!(t1.is_ok());
        let t0 = t0.unwrap();
        let t1 = t1.unwrap();

        let geo = GeoPosition::new(
            102.0 + 41.0 / 60.0 + 59.0 / 3600.0,
            25.0 + 1.0 / 60.0 + 53.0 / 3600.0,
        );
        assert!(geo.is_ok());
        let geo = geo.unwrap();
        let house = HouseName::Alcabitus;
        let planet_configs = PlanetConfig::default_all_configs();

        for i in 0u8..7 {
            let horo0 = Horoscope::new(
                t0.plus_days(i.into()).unwrap(),
                geo.clone(),
                house.clone(),
                &planet_configs,
                &ephe_path,
            )
            .unwrap();
            let horo1 = Horoscope::new(
                t1.plus_days(i.into()).unwrap(),
                geo.clone(),
                house.clone(),
                &planet_configs,
                &ephe_path,
            )
            .unwrap();
            assert_eq!(chaldean_order[i as usize], horo0.planetary_day, "白天盘");
            assert_eq!(chaldean_order[i as usize], horo1.planetary_day, "夜间盘");
        }
    }

    // 时主星
    #[test]
    fn test_planetary_hours() {
        dotenvy::dotenv().ok();
        let ephe_path = env::var("EPHE_PATH")
            .expect("没设置 EPHE_PATH 环境变量，可在.env文件中设置或export ephe_path=...");

        // 2021-9-16 星期四，木星掌管
        // 此时刻，太阳在地平线上，前一分钟，太阳在地平线下
        let t0 = HoroDateTime::new(2021, 9, 16, 7, 3, 0, 8.0).unwrap();
        // 此时刻，太阳在地平线下，前一分钟，太阳在地平线上
        let t1 = HoroDateTime::new(2021, 9, 16, 19, 12, 0, 8.0).unwrap();
        //此时刻，太阳在地平线下，下一分钟，太阳在地平线上
        let t2 = HoroDateTime::new(2021, 9, 17, 7, 2, 0, 8.0).unwrap();

        let geo = GeoPosition::new(102.0, 25.0).unwrap();

        let planet_configs = PlanetConfig::default_all_configs();

        let planetary_hours_list = [Saturn, Jupiter, Mars, Sun, Venus, Mercury, Moon];

        assert_eq!(
            Saturn,
            Horoscope::new(
                HoroDateTime::new(2021, 9, 16, 7, 2, 0, 8.0).unwrap(), //t0-1分钟
                geo.clone(),
                HouseName::Alcabitus,
                &planet_configs,
                &ephe_path
            )
            .unwrap()
            .planetary_hours,
            "太阳升起前一分钟的时主星，即2021-9月-15，最后一个时主星"
        );
        for i in 0u8..12 {
            let t = t0
                .plus_days((t1.jd_utc - t0.jd_utc) * f64::from(i) / 12.0)
                .unwrap();
            let h = Horoscope::new(
                t,
                geo.clone(),
                HouseName::Alcabitus,
                &planet_configs,
                &ephe_path,
            )
            .unwrap();
            assert_eq!(
                planetary_hours_list[((1 + i) % 7) as usize],
                h.planetary_hours,
                "白天，第{}个行星小时",
                i + 1
            );
        }

        assert_eq!(
            Moon,
            Horoscope::new(
                t1.clone(),
                geo.clone(),
                HouseName::Alcabitus,
                &planet_configs,
                &ephe_path
            )
            .unwrap()
            .planetary_hours,
            "日落后第1个行星时，2021-9-16日，夜间第一个时主星，月亮"
        );

        for i in 0u8..11 {
            // t2 + 1分钟
            let t = t1
                .plus_days((t2.jd_utc + 1.0 / 2400.0 - t1.jd_utc) * f64::from(i) / 12.0)
                .unwrap();
            let h = Horoscope::new(
                t,
                geo.clone(),
                HouseName::Alcabitus,
                &planet_configs,
                &ephe_path,
            )
            .unwrap();
            // 夜间第1个行星小时的时主星是火星
            assert_eq!(
                planetary_hours_list[((6 + i) % 7) as usize],
                h.planetary_hours,
                "夜间，第{}个行星小时",
                i + 1
            )
        }

        assert_eq!(
            Sun,
            Horoscope::new(t2, geo, HouseName::Alcabitus, &planet_configs, &ephe_path)
                .unwrap()
                .planetary_hours,
            "日出前的行星时"
        )
    }

    #[test]
    fn test_horoscope_compare_new() {
        dotenvy::dotenv().ok();
        let ephe_path = env::var("EPHE_PATH")
            .expect("没设置 EPHE_PATH 环境变量，可在.env文件中设置或export EPHE_PATH=...");

        let t = HoroDateTime::new(2021, 9, 14, 10, 30, 20, 8.0);
        assert!(t.is_ok());
        let t = t.unwrap();

        let t_compare = HoroDateTime::new(2023, 12, 26, 20, 14, 20, 8.0);
        // let t_compare = HoroDateTime::new(2021, 9, 14, 10, 30, 20, 8.0);
        assert!(t_compare.is_ok());
        let t_compare = t_compare.unwrap();

        let geo = GeoPosition::new(
            102.0 + 41.0 / 60.0 + 59.0 / 3600.0,
            25.0 + 1.0 / 60.0 + 53.0 / 3600.0,
        );
        assert!(geo.is_ok());
        let geo = geo.unwrap();

        let house = HouseName::Alcabitus;
        let planet_configs = PlanetConfig::default_all_configs();

        let horo = HoroscopeCompare::new(
            t.clone(),
            t_compare.clone(),
            geo.clone(),
            house.clone(),
            &planet_configs,
            &ephe_path,
        );
        assert!(horo.is_ok());
        let horo = horo.unwrap();

        // 时间
        assert_eq!(t.year, horo.date.year);
        assert_eq!(t.month, horo.date.month);
        assert_eq!(t.hour, horo.date.hour);
        assert_eq!(t.minute, horo.date.minute);
        assert_eq!(t.second, horo.date.second);
        assert_eq!(t.tz, horo.date.tz);

        assert_eq!(t_compare.year, horo.date_compare.year);
        assert_eq!(t_compare.month, horo.date_compare.month);
        assert_eq!(t_compare.hour, horo.date_compare.hour);
        assert_eq!(t_compare.minute, horo.date_compare.minute);
        assert_eq!(t_compare.second, horo.date_compare.second);
        assert_eq!(t_compare.tz, horo.date_compare.tz);

        // 大地经纬度
        assert_eq!(geo.long, horo.geo.long); //, this.doubleDelta)
        assert_eq!(geo.lat, horo.geo.lat); //, this.doubleDelta)

        // 宫位系统
        match horo.house_name {
            HouseName::Alcabitus => assert!(true),
            _ => assert!(false),
        }

        // 12宫
        let yy = swe_houses(t.jd_ut1, geo.lat, geo.long, &(&house).into());
        assert!(yy.is_ok(), "swe_houses()调用失败");
        let (houses_cups, ascmc) = yy.unwrap();
        let houses_cups = &houses_cups[1..13];

        assert_eq!(12, horo.houses_cups.len());

        for i in 0..12 {
            assert_eq!(houses_cups[i], horo.houses_cups[i]);
        }

        // 本盘四轴
        let eps = calc_eps(t.jd_utc, &ephe_path);
        assert!(eps.is_ok());
        let eps = eps.unwrap();

        let asc_equator = swe_cotrans(ascmc[0], 0.0, 1.0, -eps);
        assert_eq!(ASC, horo.asc.name, "asc name");
        assert_eq!(ascmc[0], horo.asc.long, "asc 黄道经度");
        assert_eq!(0.0, horo.asc.lat, "asc 黄纬");
        assert_eq!(asc_equator[0], horo.asc.ra, "asc 赤经");
        assert_eq!(asc_equator[1], horo.asc.dec, "asc 赤纬");
        assert_eq!(0, horo.asc.orb, "asc 容许度");
        assert_eq!(均, horo.asc.speed_state, "asc速度是“均”");

        // mc
        let mc_equator = swe_cotrans(ascmc[1], 0.0, 1.0, -eps);
        assert_eq!(MC, horo.mc.name, "mc name");
        assert_eq!(ascmc[1], horo.mc.long, "mc 黄道经度");
        assert_eq!(0.0, horo.mc.lat, "mc 黄纬");
        assert_eq!(mc_equator[0], horo.mc.ra, "mc 赤经");
        assert_eq!(mc_equator[1], horo.mc.dec, "mc 赤纬");
        assert_eq!(0, horo.mc.orb, "mc 容许度");
        assert_eq!(均, horo.mc.speed_state, "mc速度是均");

        // DSC
        let dsc_equator = swe_cotrans(swe_degnorm(ascmc[0] + 180.0), 0.0, 1.0, -eps);
        assert_eq!(DSC, horo.dsc.name, "dsc name");
        assert_eq!(swe_degnorm(ascmc[0] + 180.0), horo.dsc.long, "dsc 黄道经度");
        assert_eq!(0.0, horo.dsc.lat, "dsc 黄纬");
        assert_eq!(dsc_equator[0], horo.dsc.ra, "dsc 赤经");
        assert_eq!(dsc_equator[1], horo.dsc.dec, "dsc 赤纬");
        assert_eq!(0, horo.dsc.orb, "dsc 容许度");
        assert_eq!(均, horo.dsc.speed_state, "dsc速冻是均");

        // IC
        let ic_equator = swe_cotrans(swe_degnorm(ascmc[1] + 180.0), 0.0, 1.0, -eps);
        assert_eq!(IC, horo.ic.name, "ic name");
        assert_eq!(swe_degnorm(ascmc[1] + 180.0), horo.ic.long, "ic 黄道经度");
        assert_eq!(0.0, horo.ic.lat, "ic 黄纬");
        assert_eq!(ic_equator[0], horo.ic.ra, "ic 赤经");
        assert_eq!(ic_equator[1], horo.ic.dec, "ic 赤纬");
        assert_eq!(0, horo.ic.orb, "ic 容许度");
        assert_eq!(均, horo.ic.speed_state, "IC速度是均");

        // 比较盘12宫
        let yy = swe_houses(t_compare.jd_ut1, geo.lat, geo.long, &(&house).into());
        assert!(yy.is_ok(), "swe_houses()调用失败");
        let (_, ascmc) = yy.unwrap();

        // 比较盘四轴
        let eps = calc_eps(t_compare.jd_utc, &ephe_path);
        assert!(eps.is_ok());
        let eps = eps.unwrap();

        let asc_equator = swe_cotrans(ascmc[0], 0.0, 1.0, -eps);
        assert_eq!(ASC, horo.asc_compare.name, "asc name");
        assert_eq!(ascmc[0], horo.asc_compare.long, "asc 黄道经度");
        assert_eq!(0.0, horo.asc_compare.lat, "asc 黄纬");
        assert_eq!(asc_equator[0], horo.asc_compare.ra, "asc 赤经");
        assert_eq!(asc_equator[1], horo.asc_compare.dec, "asc 赤纬");
        assert_eq!(0, horo.asc_compare.orb, "asc 容许度");
        assert_eq!(均, horo.asc_compare.speed_state, "asc速度是“均”");

        // mc
        let mc_equator = swe_cotrans(ascmc[1], 0.0, 1.0, -eps);
        assert_eq!(MC, horo.mc_compare.name, "mc name");
        assert_eq!(ascmc[1], horo.mc_compare.long, "mc 黄道经度");
        assert_eq!(0.0, horo.mc_compare.lat, "mc 黄纬");
        assert_eq!(mc_equator[0], horo.mc_compare.ra, "mc 赤经");
        assert_eq!(mc_equator[1], horo.mc_compare.dec, "mc 赤纬");
        assert_eq!(0, horo.mc_compare.orb, "mc 容许度");
        assert_eq!(均, horo.mc_compare.speed_state, "mc速度是均");

        // DSC
        let dsc_equator = swe_cotrans(swe_degnorm(ascmc[0] + 180.0), 0.0, 1.0, -eps);
        assert_eq!(DSC, horo.dsc_compare.name, "dsc name");
        assert_eq!(
            swe_degnorm(ascmc[0] + 180.0),
            horo.dsc_compare.long,
            "dsc 黄道经度"
        );
        assert_eq!(0.0, horo.dsc_compare.lat, "dsc 黄纬");
        assert_eq!(dsc_equator[0], horo.dsc_compare.ra, "dsc 赤经");
        assert_eq!(dsc_equator[1], horo.dsc_compare.dec, "dsc 赤纬");
        assert_eq!(0, horo.dsc_compare.orb, "dsc 容许度");
        assert_eq!(均, horo.dsc_compare.speed_state, "dsc速冻是均");

        // IC
        let ic_equator = swe_cotrans(swe_degnorm(ascmc[1] + 180.0), 0.0, 1.0, -eps);
        assert_eq!(IC, horo.ic_compare.name, "ic name");
        assert_eq!(
            swe_degnorm(ascmc[1] + 180.0),
            horo.ic_compare.long,
            "ic 黄道经度"
        );
        assert_eq!(0.0, horo.ic_compare.lat, "ic 黄纬");
        assert_eq!(ic_equator[0], horo.ic_compare.ra, "ic 赤经");
        assert_eq!(ic_equator[1], horo.ic_compare.dec, "ic 赤纬");
        assert_eq!(0, horo.ic_compare.orb, "ic 容许度");
        assert_eq!(均, horo.ic_compare.speed_state, "IC速度是均");

        // 七颗正星
        for planet_name in [Sun, Moon, Mercury, Venus, Mars, Jupiter, Saturn] {
            let p = horo.planets.iter().find(|p| p.name == planet_name);
            assert!(p.is_some());
            let p = p.unwrap();

            let body = match planet_name {
                Sun => Body::SeSun,
                Moon => Body::SeMoon,
                Mercury => Body::SeMercury,
                Venus => Body::SeVenus,
                Mars => Body::SeMars,
                Jupiter => Body::SeJupiter,
                _ => Body::SeSaturn, // Saturn
            };

            swe_set_ephe_path(&ephe_path);
            let xx = swe_calc_ut(t.jd_utc, &body, &[Flag::SeflgSpeed]);
            let yy = swe_calc_ut(t.jd_utc, &body, &[Flag::SeflgEquatorial]); //计算赤经和赤纬

            assert!(xx.is_ok(), "计算行星错误");
            assert!(yy.is_ok(), "计算行星错误");
            swe_close();

            let xx = xx.unwrap();
            let yy = yy.unwrap();

            let config = PlanetConfig::default_config(&planet_name);
            let speed_state = if config.max > config.min {
                if xx[3].abs() > config.max {
                    快
                } else if xx[3].abs() < config.min {
                    慢
                } else {
                    均
                }
            } else {
                均
            };
            assert_eq!(planet_name, p.name);
            assert_eq!(xx[0], p.long, "{:?}", planet_name);
            assert_eq!(xx[1], p.lat, "{:?}黄纬", planet_name);
            assert_eq!(xx[3], p.speed, "{:?}黄道上每日速度", planet_name);
            assert_eq!(yy[0], p.ra, "{:?}赤经", planet_name);
            assert_eq!(yy[1], p.dec, "{:?}赤纬", planet_name);
            assert_eq!(config.orb, p.orb, "{:?}容许度", planet_name);
            assert_eq!(speed_state, p.speed_state, "{:?}迟疾", planet_name);
        }

        // 比较盘七颗正星
        for planet_name in [Sun, Moon, Mercury, Venus, Mars, Jupiter, Saturn] {
            let p = horo.planets_compare.iter().find(|p| p.name == planet_name);
            assert!(p.is_some());
            let p = p.unwrap();

            let body = match planet_name {
                Sun => Body::SeSun,
                Moon => Body::SeMoon,
                Mercury => Body::SeMercury,
                Venus => Body::SeVenus,
                Mars => Body::SeMars,
                Jupiter => Body::SeJupiter,
                _ => Body::SeSaturn, // Saturn
            };

            swe_set_ephe_path(&ephe_path);
            let xx = swe_calc_ut(t_compare.jd_utc, &body, &[Flag::SeflgSpeed]);
            let yy = swe_calc_ut(t_compare.jd_utc, &body, &[Flag::SeflgEquatorial]); //计算赤经和赤纬

            assert!(xx.is_ok(), "计算行星错误");
            assert!(yy.is_ok(), "计算行星错误");
            swe_close();

            let xx = xx.unwrap();
            let yy = yy.unwrap();

            let config = PlanetConfig::default_config(&planet_name);
            let speed_state = if config.max > config.min {
                if xx[3].abs() > config.max {
                    快
                } else if xx[3].abs() < config.min {
                    慢
                } else {
                    均
                }
            } else {
                均
            };
            assert_eq!(planet_name, p.name);
            assert_eq!(xx[0], p.long, "{:?}", planet_name);
            assert_eq!(xx[1], p.lat, "{:?}黄纬", planet_name);
            assert_eq!(xx[3], p.speed, "{:?}黄道上每日速度", planet_name);
            assert_eq!(yy[0], p.ra, "{:?}赤经", planet_name);
            assert_eq!(yy[1], p.dec, "{:?}赤纬", planet_name);
            assert_eq!(config.orb, p.orb, "{:?}容许度", planet_name);
            assert_eq!(speed_state, p.speed_state, "{:?}迟疾", planet_name);
        }

        // 月交点
        let north_node = horo.planets.iter().find(|p| p.name == NorthNode);
        let south_node = horo.planets.iter().find(|p| p.name == SouthNode);

        assert!(north_node.is_some());
        assert!(south_node.is_some());

        let north_node = north_node.unwrap();
        let south_node = south_node.unwrap();

        swe_set_ephe_path(&ephe_path);
        let xx = swe_calc_ut(t.jd_utc, &Body::SeMeanNode, &[Flag::SeflgSpeed]);
        let yy = swe_calc_ut(t.jd_utc, &Body::SeMeanNode, &[Flag::SeflgEquatorial]); //计算赤经和赤纬

        assert!(xx.is_ok(), "计算行星错误");
        assert!(yy.is_ok(), "计算行星错误");
        swe_close();

        let xx = xx.unwrap();
        let yy = yy.unwrap();

        assert_eq!(NorthNode, north_node.name, "北交点");
        assert_eq!(xx[0], north_node.long, "黄经，北交点");
        assert_eq!(0.0, north_node.lat, "黄纬, 北交点");
        assert_eq!(xx[3], north_node.speed, "黄道上每日速度, 北交点");
        assert_eq!(yy[0], north_node.ra, "赤经, 北交点");
        assert_eq!(yy[1], north_node.dec, "赤纬, 北交点");
        assert_eq!(0, north_node.orb, "容许度, 北交点");
        assert_eq!(均, north_node.speed_state, "迟疾, 北交点");

        assert_eq!(SouthNode, south_node.name, "南交点");
        assert_eq!(swe_degnorm(xx[0] + 180.0), south_node.long, "黃经，南交点");
        assert_eq!(0.0, south_node.lat, "黄纬, 南交点");
        assert_eq!(xx[3], south_node.speed, "黄道上每日速度, 南交点");
        assert_eq!(swe_degnorm(yy[0] + 180.0), south_node.ra, "赤经, 南交点");
        assert_eq!(-yy[1], south_node.dec, "赤纬, 南交点");
        assert_eq!(0, south_node.orb, "容许度, 南交点");
        assert_eq!(均, south_node.speed_state, "迟疾, 南交点");

        // 比较盘月交点
        let north_node = horo.planets_compare.iter().find(|p| p.name == NorthNode);
        let south_node = horo.planets_compare.iter().find(|p| p.name == SouthNode);

        assert!(north_node.is_some());
        assert!(south_node.is_some());

        let north_node = north_node.unwrap();
        let south_node = south_node.unwrap();

        swe_set_ephe_path(&ephe_path);
        let xx = swe_calc_ut(t_compare.jd_utc, &Body::SeMeanNode, &[Flag::SeflgSpeed]);
        let yy = swe_calc_ut(
            t_compare.jd_utc,
            &Body::SeMeanNode,
            &[Flag::SeflgEquatorial],
        ); //计算赤经和赤纬

        assert!(xx.is_ok(), "计算行星错误");
        assert!(yy.is_ok(), "计算行星错误");
        swe_close();

        let xx = xx.unwrap();
        let yy = yy.unwrap();

        assert_eq!(NorthNode, north_node.name, "北交点");
        assert_eq!(xx[0], north_node.long, "黄经，北交点");
        assert_eq!(0.0, north_node.lat, "黄纬, 北交点");
        assert_eq!(xx[3], north_node.speed, "黄道上每日速度, 北交点");
        assert_eq!(yy[0], north_node.ra, "赤经, 北交点");
        assert_eq!(yy[1], north_node.dec, "赤纬, 北交点");
        assert_eq!(0, north_node.orb, "容许度, 北交点");
        assert_eq!(均, north_node.speed_state, "迟疾, 北交点");

        assert_eq!(SouthNode, south_node.name, "南交点");
        assert_eq!(swe_degnorm(xx[0] + 180.0), south_node.long, "黃经，南交点");
        assert_eq!(0.0, south_node.lat, "黄纬, 南交点");
        assert_eq!(xx[3], south_node.speed, "黄道上每日速度, 南交点");
        assert_eq!(swe_degnorm(yy[0] + 180.0), south_node.ra, "赤经, 南交点");
        assert_eq!(-yy[1], south_node.dec, "赤纬, 南交点");
        assert_eq!(0, south_node.orb, "容许度, 南交点");
        assert_eq!(均, south_node.speed_state, "迟疾, 南交点");

        // 相位
        assert_eq!(40, horo.aspects.len());
    }
}
