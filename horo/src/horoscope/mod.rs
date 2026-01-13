#[cfg(test)]
mod tests;

use crate::{
    Aspect, Error, HouseName, Planet, PlanetConfig, PlanetName,
    fixed_star::{FixedStar, calc_fixed_star_long},
    utils::{calc_eps, mod180, newton_iteration},
};
use geo_position::GeoPosition;
use swe::{
    Body, Flag, HouseSystem, swe_calc_ut, swe_close, swe_cotrans, swe_degnorm, swe_houses,
    swe_set_ephe_path,
};

use horo_date_time::HoroDateTime;

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
    /// 福点
    pub part_of_fortune: Planet,
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
    // 映点
    pub antiscoins: Vec<Aspect>,
    // 反映点
    pub contraantiscias: Vec<Aspect>,
    // 恒星
    pub fixed_stars: Vec<FixedStar>,
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

        // 计算福点
        let moon = planets.iter().find(|p| p.name == PlanetName::Moon).unwrap();
        let default_planet_config = PlanetConfig::default_config(&PlanetName::PartOfFortune);
        let planet_config = planets_config
            .iter()
            .find(|p| p.name == PlanetName::PartOfFortune)
            .unwrap_or(&default_planet_config);
        let part_of_fortune_long = if is_diurnal {
            // 昼生
            // 福点=上升点+月亮−太阳
            swe_degnorm(ascmc[0] + moon.long - sun.long)
        } else {
            // 夜生
            // 福点=上升点+太阳−月亮
            swe_degnorm(ascmc[0] + sun.long - moon.long)
        };
        let part_of_fortune_equator = swe_cotrans(part_of_fortune_long, 0.0, 1.0, -eps);
        let part_of_fortune = Planet::new(
            PlanetName::PartOfFortune,
            part_of_fortune_long,
            0.0,
            0.0,
            part_of_fortune_equator[0],
            part_of_fortune_equator[1],
            planet_config,
        );

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

        // 计算相位和映点
        let mut aspects: Vec<Aspect> = vec![];
        let mut antiscoins: Vec<Aspect> = vec![];
        let mut contraantiscias: Vec<Aspect> = vec![];
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
        asm_and_planets.push(&part_of_fortune);
        for i in 0..asm_and_planets.len() {
            for j in i..asm_and_planets.len() {
                let aspect = asm_and_planets[i].has_aspect(asm_and_planets[j], false);
                if let Some(aspect) = aspect {
                    aspects.push(aspect)
                }

                let antiscoin = asm_and_planets[i].has_antiscoin(asm_and_planets[j]);
                if let Some(aspect) = antiscoin {
                    antiscoins.push(aspect)
                }

                let contraantiscia = asm_and_planets[i].has_contraantiscia(asm_and_planets[j]);
                if let Some(aspect) = contraantiscia {
                    contraantiscias.push(aspect)
                }
            }
        }

        // 计算恒星
        let fixed_stars = calc_fixed_star_long(date.jd_utc, ephe_path)?;

        Ok(Self {
            date,
            geo,
            house_name,
            houses_cups: cups[1..13].to_vec(),
            asc,
            mc,
            dsc,
            ic,
            part_of_fortune,
            planets,
            is_diurnal,
            planetary_day,
            planetary_hours,
            aspects,
            antiscoins,
            contraantiscias,
            fixed_stars,
        })
    }
}

#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
#[cfg_attr(feature = "swagger", derive(ToSchema))]
pub struct HoroscopeComparison {
    /// 原星盘的时间
    pub original_date: HoroDateTime,
    /// 比较盘时间
    pub comparison_date: HoroDateTime,
    /// 原星盘的地理位置
    pub original_geo: GeoPosition,
    /// 比较星盘的地理位置
    pub comparison_geo: GeoPosition,
    /// 星盘的宫位
    pub house_name: HouseName,
    /// 原盘12宫头黄经度数
    pub houses_cups: Vec<f64>,
    /// 比较盘12宫头黄经度数
    comparison_cups: Vec<f64>,

    /// 上升点
    pub original_asc: Planet,
    pub comparison_asc: Planet,
    /// 中天
    pub original_mc: Planet,
    pub comparison_mc: Planet,
    /// 下降点
    pub original_dsc: Planet,
    pub comparison_dsc: Planet,
    /// 天底
    pub original_ic: Planet,
    pub comparison_ic: Planet,
    /// 福点
    pub original_part_of_fortune: Planet,
    pub comparison_part_of_fortune: Planet,
    /// 七颗行星
    pub original_planets: Vec<Planet>,
    pub comparison_planets: Vec<Planet>,

    /// 行星相位，仅包含四轴、行星间的相位
    pub aspects: Vec<Aspect>,
    /// 映点
    pub antiscoins: Vec<Aspect>,
    /// 反映点
    pub contraantiscias: Vec<Aspect>,
}

impl HoroscopeComparison {
    pub fn new(
        date: HoroDateTime,
        date_compare: HoroDateTime,
        geo: GeoPosition,
        process_geo: GeoPosition,
        house_name: HouseName,
        planets_config: &[PlanetConfig],
        ephe_path: &str,
    ) -> Result<Self, Error> {
        // 计算原星盘
        let horo = Horoscope::new(date, geo, house_name, planets_config, ephe_path)?;

        let horo_compare = Horoscope::new(
            date_compare,
            process_geo,
            house_name,
            planets_config,
            ephe_path,
        )?;

        // 计算相位和映点
        let (aspects, antiscoins, contraantiscias) = Self::calculate_aspects(&horo, &horo_compare);

        Ok(Self {
            original_date: date,
            comparison_date: date_compare,
            original_geo: geo,
            comparison_geo: process_geo,
            house_name,
            houses_cups: horo.houses_cups,
            comparison_cups: horo_compare.houses_cups,
            original_asc: horo.asc,
            comparison_asc: horo_compare.asc,
            original_mc: horo.mc,
            comparison_mc: horo_compare.mc,
            original_dsc: horo.dsc,
            comparison_dsc: horo_compare.dsc,
            original_ic: horo.ic,
            comparison_ic: horo_compare.ic,
            // 福点
            original_part_of_fortune: horo.part_of_fortune,
            comparison_part_of_fortune: horo_compare.part_of_fortune,
            // 行星
            original_planets: horo.planets,
            comparison_planets: horo_compare.planets,
            aspects,
            antiscoins,
            contraantiscias,
        })
    }

    fn calculate_aspects(
        horo: &Horoscope,
        horo_compare: &Horoscope,
    ) -> (Vec<Aspect>, Vec<Aspect>, Vec<Aspect>) {
        let mut aspects: Vec<Aspect> = vec![];
        let mut antiscoins: Vec<Aspect> = vec![];
        let mut contraantiscias: Vec<Aspect> = vec![];

        let mut asm_and_planets: Vec<_> = horo.planets.iter().collect();
        asm_and_planets.push(&horo.asc);
        asm_and_planets.push(&horo.mc);
        asm_and_planets.push(&horo.dsc);
        asm_and_planets.push(&horo.ic);
        // 福点
        asm_and_planets.push(&horo.part_of_fortune);

        let mut asm_and_planets_compare: Vec<_> = horo_compare.planets.iter().collect();
        asm_and_planets_compare.push(&horo_compare.asc);
        asm_and_planets_compare.push(&horo_compare.mc);
        asm_and_planets_compare.push(&horo_compare.dsc);
        asm_and_planets_compare.push(&horo_compare.ic);
        // 福点
        asm_and_planets_compare.push(&horo_compare.part_of_fortune);

        for i in 0..asm_and_planets.len() {
            for j in 0..asm_and_planets_compare.len() {
                let aspect = asm_and_planets[i].has_aspect(asm_and_planets_compare[j], true);
                if let Some(aspect) = aspect {
                    aspects.push(aspect)
                }

                let antiscoin = asm_and_planets[i].has_antiscoin(asm_and_planets_compare[j]);
                if let Some(aspect) = antiscoin {
                    antiscoins.push(aspect)
                }

                let contraantiscia =
                    asm_and_planets[i].has_contraantiscia(asm_and_planets_compare[j]);
                if let Some(aspect) = contraantiscia {
                    contraantiscias.push(aspect)
                }
            }
        }

        (aspects, antiscoins, contraantiscias)
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

    let date = HoroDateTime::from_jd_zone(jd, t.tz)?;
    Ok(date)
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

    let date = HoroDateTime::from_jd_zone(jd, t.tz)?;
    Ok(date)
}
