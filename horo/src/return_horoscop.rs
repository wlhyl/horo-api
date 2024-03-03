use swe::{swe_calc_ut, swe_close, swe_degnorm, swe_set_ephe_path, Body};

use crate::{
    utils::{mod180, newton_iteration},
    Aspect, Error, GeoPosition, Horoscope, HouseName, Planet, PlanetConfig,
};

use horo_date_time::HoroDateTime;

#[cfg(feature = "serde")]
use serde::Serialize;

#[cfg(feature = "swagger")]
use utoipa::ToSchema;

#[cfg_attr(feature = "serde", derive(Serialize))]
#[cfg_attr(feature = "swagger", derive(ToSchema))]
pub struct ReturnHoroscop {
    /// 原星盘的时间
    pub native_date: HoroDateTime,
    /// 推运时间
    pub process_date: HoroDateTime,
    /// 返照时间
    pub return_date: HoroDateTime,
    /// 绘制星盘的地理位置
    pub geo: GeoPosition,
    /// 星盘的宫位
    pub house_name: HouseName,
    /// 12宫头黄经度数
    pub houses_cups: Vec<f64>,

    /// 上升点
    pub asc: Planet,

    /// 中天
    pub mc: Planet,

    /// 下降点
    pub dsc: Planet,

    /// 天底
    pub ic: Planet,

    /// 七颗行星
    pub planets: Vec<Planet>,

    /// 行星相位，仅包含四轴、行星间的相位
    pub aspects: Vec<Aspect>,
}

/// 计算太阳返照盘
pub fn solar_return(
    native_date: HoroDateTime,
    process_date: HoroDateTime,
    geo: GeoPosition,
    house_name: HouseName,
    planets_config: &[PlanetConfig],
    ephe_path: &str,
) -> Result<ReturnHoroscop, Error> {
    // 计算本命星盘太阳黄道经度
    swe_set_ephe_path(ephe_path);
    let xx = swe_calc_ut(native_date.jd_utc, &Body::SeSun, &[])
        .map_err(|e| Error::Function(format!("计算本命星盘太阳黄道经度错误:{e}")))?;
    swe_close();
    let sun_long = xx[0];

    // 计算推运时刻黄道经度
    let xx = swe_calc_ut(process_date.jd_utc, &Body::SeSun, &[])
        .map_err(|e| Error::Function(format!("计算推运时刻太阳黄道经度错误:{e}")))?;
    let process_sun_long = xx[0];

    // 计算迭代初值
    let jd0 = process_date.jd_utc - swe_degnorm(process_sun_long - sun_long);

    // 计算返照时间
    let jd = newton_iteration(jd0, |jd| {
        let t0 = HoroDateTime::from_jd_zone(jd, process_date.tz)?;
        swe_set_ephe_path(ephe_path);
        let xx = swe_calc_ut(t0.jd_utc, &Body::SeSun, &[]).map_err(|e| {
            Error::Function(format!("函数sun_on_asc()，牛顿迭代计算太阳位置错误:{e}"))
        })?;

        swe_close();

        Ok(mod180(xx[0] - sun_long))
    })?;

    let solar_return_date = HoroDateTime::from_jd_zone(jd, process_date.tz)?;
    // 计算返照星盘
    let horo = Horoscope::new(
        solar_return_date.clone(),
        geo.clone(),
        house_name.clone(),
        planets_config,
        ephe_path,
    )?;
    Ok(ReturnHoroscop {
        native_date,
        process_date,
        return_date: solar_return_date,
        geo,
        house_name,
        houses_cups: horo.houses_cups,
        asc: horo.asc,
        mc: horo.mc,
        dsc: horo.dsc,
        ic: horo.ic,
        planets: horo.planets,
        aspects: horo.aspects,
    })
}

/// 计算月亮返照盘
pub fn lunar_return(
    native_date: HoroDateTime,
    process_date: HoroDateTime,
    geo: GeoPosition,
    house_name: HouseName,
    planets_config: &[PlanetConfig],
    ephe_path: &str,
) -> Result<ReturnHoroscop, Error> {
    // 计算本命星盘月亮黄道经度
    swe_set_ephe_path(ephe_path);
    let xx = swe_calc_ut(native_date.jd_utc, &Body::SeMoon, &[])
        .map_err(|e| Error::Function(format!("计算本命星盘月亮黄道经度错误:{e}")))?;
    swe_close();
    let moon_long = xx[0];

    // 计算推运时刻黄道经度
    let xx = swe_calc_ut(process_date.jd_utc, &Body::SeMoon, &[])
        .map_err(|e| Error::Function(format!("计算推运时刻月亮黄道经度错误:{e}")))?;
    let process_moon_long = xx[0];

    // 计算迭代初值
    let jd0 = process_date.jd_utc - swe_degnorm(process_moon_long - moon_long) / 13.0;

    // 计算返照时间
    let jd = newton_iteration(jd0, |jd| {
        let t0 = HoroDateTime::from_jd_zone(jd, process_date.tz)?;
        swe_set_ephe_path(ephe_path);
        let xx = swe_calc_ut(t0.jd_utc, &Body::SeMoon, &[]).map_err(|e| {
            Error::Function(format!("函数sun_on_asc()，牛顿迭代计算月亮位置错误:{e}"))
        })?;

        swe_close();

        Ok(mod180(xx[0] - moon_long))
    })?;

    let lunar_return_date = HoroDateTime::from_jd_zone(jd, process_date.tz)?;
    // 计算返照星盘
    let horo = Horoscope::new(
        lunar_return_date.clone(),
        geo.clone(),
        house_name.clone(),
        planets_config,
        ephe_path,
    )?;
    Ok(ReturnHoroscop {
        native_date,
        process_date,
        return_date: lunar_return_date,
        geo,
        house_name,
        houses_cups: horo.houses_cups,
        asc: horo.asc,
        mc: horo.mc,
        dsc: horo.dsc,
        ic: horo.ic,
        planets: horo.planets,
        aspects: horo.aspects,
    })
}
