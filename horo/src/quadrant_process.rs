use geo_position::GeoPosition;
use horo_date_time::{HoroDateTime, horo_date_time};
use swe::swe_degnorm;

use crate::{
    Error, Horoscope, HouseName, PlanetConfig, PlanetName, Promittor,
    direction::{PTOLEMY_TERM, PtolemyTerm},
};

#[cfg(feature = "serde")]
use serde::Serialize;

#[cfg(feature = "swagger")]
use utoipa::ToSchema;

#[derive(Clone, Debug, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize))]
#[cfg_attr(feature = "swagger", derive(ToSchema))]
pub struct QuadrantProcess {
    pub promittor: Promittor,
    pub date: HoroDateTime,
}

pub fn quadrant_process(
    native_date: HoroDateTime,
    geo: GeoPosition,
    house_name: HouseName,
    planets_config: &[PlanetConfig],
    ephe_path: &str,
) -> Result<Vec<QuadrantProcess>, Error> {
    // 计算原星盘
    let horo = Horoscope::new(native_date, geo, house_name, planets_config, ephe_path)?;

    // 关键时间点：ASC(出生)、MC(20岁)、DSC(40岁)、IC(60岁)、ASC结束(80岁)
    let asc_date = horo.date;
    let mc_date = horo_date_time(
        horo.date.year + 20,
        horo.date.month,
        horo.date.day,
        horo.date.hour,
        horo.date.minute,
        horo.date.second,
        horo.date.tz,
        false,
    )?;
    let dsc_date = horo_date_time(
        horo.date.year + 40,
        horo.date.month,
        horo.date.day,
        horo.date.hour,
        horo.date.minute,
        horo.date.second,
        horo.date.tz,
        false,
    )?;
    let ic_date = horo_date_time(
        horo.date.year + 60,
        horo.date.month,
        horo.date.day,
        horo.date.hour,
        horo.date.minute,
        horo.date.second,
        horo.date.tz,
        false,
    )?;
    let asc_date_end = horo_date_time(
        horo.date.year + 80,
        horo.date.month,
        horo.date.day,
        horo.date.hour,
        horo.date.minute,
        horo.date.second,
        horo.date.tz,
        false,
    )?;

    // 象限弧度
    let arc_asc_mc = swe_degnorm(horo.asc.long - horo.mc.long);
    let arc_mc_dsc = swe_degnorm(horo.mc.long - horo.dsc.long);
    let arc_dsc_ic = swe_degnorm(horo.dsc.long - horo.ic.long);
    let arc_ic_asc = swe_degnorm(horo.ic.long - horo.asc.long);

    // 时间间隔（天）
    let days_asc_mc = mc_date.jd_utc - asc_date.jd_utc;
    let days_mc_dsc = dsc_date.jd_utc - mc_date.jd_utc;
    let days_dsc_ic = ic_date.jd_utc - dsc_date.jd_utc;
    let days_ic_asc = asc_date_end.jd_utc - ic_date.jd_utc;

    let promittors = promittors_of_process(&horo);

    // 找出所有 Term 的索引
    let term_indices: Vec<usize> = promittors
        .iter()
        .enumerate()
        .filter(|(_, (promittor, _))| matches!(promittor, Promittor::Term(_, _)))
        .map(|(i, _)| i)
        .collect();

    let mut quadrant_process: Vec<QuadrantProcess> = Vec::new();

    for (index, &(promittor, long)) in promittors.iter().enumerate() {
        // 计算推运星到 ASC 的弧度
        let delta_long = swe_degnorm(horo.asc.long - long);

        // 根据弧度所在象限计算日期
        let date = if delta_long < arc_asc_mc {
            // 第一象限：ASC -> MC
            let delta_days = days_asc_mc * delta_long / arc_asc_mc;
            asc_date.plus_days(delta_days)?
        } else if delta_long < arc_asc_mc + arc_mc_dsc {
            // 第二象限：MC -> DSC
            let delta_long_in_quadrant = delta_long - arc_asc_mc;
            let delta_days = days_mc_dsc * delta_long_in_quadrant / arc_mc_dsc;
            mc_date.plus_days(delta_days)?
        } else if delta_long < arc_asc_mc + arc_mc_dsc + arc_dsc_ic {
            // 第三象限：DSC -> IC
            let delta_long_in_quadrant = delta_long - arc_asc_mc - arc_mc_dsc;
            let delta_days = days_dsc_ic * delta_long_in_quadrant / arc_dsc_ic;
            dsc_date.plus_days(delta_days)?
        } else {
            // 第四象限：IC -> ASC
            let delta_long_in_quadrant = delta_long - arc_asc_mc - arc_mc_dsc - arc_dsc_ic;
            let delta_days = days_ic_asc * delta_long_in_quadrant / arc_ic_asc;
            ic_date.plus_days(delta_days)?
        };

        // 界是逆时针推运，调整为前一个界（首尾相连）
        let promittor = if matches!(promittor, Promittor::Term(_, _)) {
            let term_pos = term_indices.iter().position(|&i| i == index).unwrap();
            let prev_term_index = term_indices[(term_pos + term_indices.len() - 1) % term_indices.len()];
            promittors[prev_term_index].0
        } else {
            promittor
        };

        quadrant_process.push(QuadrantProcess { promittor, date });
    }

    quadrant_process.sort_by(|a, b| a.date.jd_utc.total_cmp(&b.date.jd_utc));
    Ok(quadrant_process)
}

/// 根据推运时间计算对应的黄道经度
///
/// # 参数
/// - `native_date`: 原生日期（出生时间）
/// - `process_date`: 推运发生的日期
/// - `geo`: 地理位置
/// - `house_name`: 宫位系统
/// - `planets_config`: 行星配置
/// - `ephe_path`: 星历表路径
///
/// # 返回
/// - `long`: 推运黄道经度（度，0-360）
pub fn quadrant_process_longitude(
    native_date: HoroDateTime,
    process_date: HoroDateTime,
    geo: GeoPosition,
    house_name: HouseName,
    planets_config: &[PlanetConfig],
    ephe_path: &str,
) -> Result<f64, Error> {
    // 计算原星盘
    let horo = Horoscope::new(native_date, geo, house_name, planets_config, ephe_path)?;

    // 象限边界时间
    let mc_date = horo_date_time(
        native_date.year + 20,
        native_date.month,
        native_date.day,
        native_date.hour,
        native_date.minute,
        native_date.second,
        native_date.tz,
        false,
    )?;
    let dsc_date = horo_date_time(
        native_date.year + 40,
        native_date.month,
        native_date.day,
        native_date.hour,
        native_date.minute,
        native_date.second,
        native_date.tz,
        false,
    )?;
    let ic_date = horo_date_time(
        native_date.year + 60,
        native_date.month,
        native_date.day,
        native_date.hour,
        native_date.minute,
        native_date.second,
        native_date.tz,
        false,
    )?;

    // 象限弧度
    let arc_asc_mc = swe_degnorm(horo.asc.long - horo.mc.long);
    let arc_mc_dsc = swe_degnorm(horo.mc.long - horo.dsc.long);
    let arc_dsc_ic = swe_degnorm(horo.dsc.long - horo.ic.long);
    let arc_ic_asc = swe_degnorm(horo.ic.long - horo.asc.long);

    // 根据推运时间与各象限开始时间的比较确定象限
    let (quadrant_arc, arc_before_quadrant, quadrant_start_date, quadrant_end_date) =
        if process_date.jd_utc < mc_date.jd_utc {
            // 第一象限：ASC -> MC
            (arc_asc_mc, 0.0, native_date, mc_date)
        } else if process_date.jd_utc < dsc_date.jd_utc {
            // 第二象限：MC -> DSC
            (arc_mc_dsc, arc_asc_mc, mc_date, dsc_date)
        } else if process_date.jd_utc < ic_date.jd_utc {
            // 第三象限：DSC -> IC
            (arc_dsc_ic, arc_asc_mc + arc_mc_dsc, dsc_date, ic_date)
        } else {
            // 第四象限：IC -> ASC
            let end_date = horo_date_time(
                native_date.year + 80,
                native_date.month,
                native_date.day,
                native_date.hour,
                native_date.minute,
                native_date.second,
                native_date.tz,
                false,
            )?;
            (arc_ic_asc, arc_asc_mc + arc_mc_dsc + arc_dsc_ic, ic_date, end_date)
        };

    // 计算该象限内走的比例
    let days_in_quadrant = quadrant_end_date.jd_utc - quadrant_start_date.jd_utc;
    let ratio_in_quadrant = (process_date.jd_utc - quadrant_start_date.jd_utc) / days_in_quadrant;

    // 该位置距离 ASC 的总弧度
    let arc = arc_before_quadrant + ratio_in_quadrant * quadrant_arc;

    // 推运黄经 = ASC 黄经 - 弧度（逆时针推运）
    let process_longitude = swe_degnorm(horo.asc.long - arc);

    Ok(process_longitude)
}

fn promittors_of_process(horo: &Horoscope) -> Vec<(Promittor, f64)> {
    let promittors: Vec<(Promittor, f64)> = horo
        .planets
        .iter()
        .chain(std::iter::once(&horo.part_of_fortune))
        .flat_map(|planet| {
            let mut promittors = vec![];
            // 计算合相
            promittors.push((Promittor::Conjunction(planet.name), planet.long));

            // 映点
            let antiscoins_long = swe_degnorm(180.0 - planet.long);
            promittors.push((Promittor::Antiscoins(planet.name), antiscoins_long));

            // 南北交点不用计算反映点，也不需要计算基它相位
            if planet.name == PlanetName::NorthNode || planet.name == PlanetName::SouthNode {
                return promittors;
            }

            // 反映点
            let contraantiscias_long = swe_degnorm(180.0 + antiscoins_long);
            promittors.push((
                Promittor::Contraantiscias(planet.name),
                contraantiscias_long,
            ));

            let aspect_promittors =
                [-60i16, 60, -120, 120, -90, 90, 180]
                    .into_iter()
                    .map(|aspect| {
                        let aspect_long = swe_degnorm(planet.long + aspect as f64);

                        let promittor = match aspect {
                            60 => Promittor::SinisterSextile(planet.name),
                            -60 => Promittor::DexterSextile(planet.name),
                            120 => Promittor::SinisterTrine(planet.name),
                            -120 => Promittor::DexterTrine(planet.name),
                            90 => Promittor::SinisterSquare(planet.name),
                            -90 => Promittor::DexterSquare(planet.name),
                            180 => Promittor::Opposition(planet.name),
                            _ => unreachable!(),
                        };

                        (promittor, aspect_long)
                    });

            promittors.extend(aspect_promittors);

            promittors
        })
        .collect();

    // 托勒密界
    let ptolemy_term: Vec<(Promittor, f64)> = PTOLEMY_TERM
        .into_iter()
        .map(|PtolemyTerm { planet, long }| (Promittor::Term(planet, long), f64::from(long)))
        .collect();

    let cusp = horo
        .cusps
        .iter()
        .enumerate()
        .map(|(i, long)| (Promittor::Cusp((i + 1) as u8), *long))
        .collect::<Vec<_>>();

    promittors
        .into_iter()
        .chain(ptolemy_term)
        .chain(cusp)
        .collect()
}
