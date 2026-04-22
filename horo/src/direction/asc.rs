// ASC 方向弧的计算公式为：
// * 正向弧：arc=promittor_OA - ASC_OA
// * 反向弧：arc=ASC_OA - promittor_OA

// * ASC_OA = MC的赤经 + 90
// * MC的赤经已经计算完成，在mc这个数据结构中

// promittor_OA 的计算公式为：
// * promittor_OA = promittor的赤经 - position的AD
// * position的AD的计算公式为：
//   * AD= Arcsin (tan D tan ϕ)
//   其中D 是position的赤纬，ϕ 是地平纬度

// 在Horoscope.planets保存了每个行星的赤纬
// 相位点的赤纬需要自行计算,可参看horo/src/horoscope/mod.rs中计算mc的代码
// 计算相位的的赤经步骤：
// * 根据行星的黄道经度计算对应相位的黄道经度，参看horo/src/direction/mc.rs
// * 根据相位点的黄道，相位的黄道纬度为0，计算相位的赤经与赤纬

use swe::swe_degnorm;

use crate::{
    Direction, Error, Horoscope, Planet, PlanetName, Promittor,
    direction::{
        MAX_ARC, arc_to_date,
        utils::{calc_asc_oa, calc_promittor_oa, calc_promittor_od},
    },
};

/// 计算ASC的主向推运方向
/// * horo: 星盘数据
/// * ephe_path: 星历表文件路径
pub(crate) fn asc_direction(
    horo: &Horoscope,
    promittors: &[(Promittor, Planet)],
) -> Result<Vec<Direction>, Error> {
    let asc_oa = calc_asc_oa(horo.mc.ra);

    // 找出所有 Term 的索引
    let term_indices: Vec<usize> = promittors
        .iter()
        .enumerate()
        .filter(|(_, (promittor, _))| matches!(promittor, Promittor::Term(_, _)))
        .map(|(i, _)| i)
        .collect();

    let mut directions = vec![];

    for (index, &(promittor, planet)) in promittors.iter().enumerate() {
        // 正向弧
        let promittor_oa = calc_promittor_oa(planet.ra, planet.dec, horo.geo.lat)?;
        let arc = swe_degnorm(promittor_oa - asc_oa);

        if arc < MAX_ARC {
            let t = arc_to_date(arc, &horo.date)?;
            let direction = Direction::new(PlanetName::ASC, promittor, arc, t);
            directions.push(direction);
        }

        // 反向弧
        // 界是逆时针推运，调整为前一个界（首尾相连）
        let promittor = if matches!(promittor, Promittor::Term(_, _)) {
            let term_pos = term_indices.iter().position(|&i| i == index).unwrap();
            let prev_term_index = term_indices[(term_pos + term_indices.len() - 1) % term_indices.len()];
            promittors[prev_term_index].0
        } else {
            promittor
        };

        let arc = swe_degnorm(-arc);
        if arc < MAX_ARC {
            let t = arc_to_date(arc, &horo.date)?;
            let direction = Direction::new(PlanetName::ASC, promittor, -arc, t);
            directions.push(direction);
        }
    }

    // directions.sort_by(|a, b| a.arc.abs().total_cmp(&b.arc.abs()));
    Ok(directions)
}

pub(crate) fn dsc_direction(
    horo: &Horoscope,
    promittors: &[(Promittor, Planet)],
) -> Result<Vec<Direction>, Error> {
    let dsc_od = swe_degnorm(horo.ic.ra + 90.0);

    // 找出所有 Term 的索引
    let term_indices: Vec<usize> = promittors
        .iter()
        .enumerate()
        .filter(|(_, (promittor, _))| matches!(promittor, Promittor::Term(_, _)))
        .map(|(i, _)| i)
        .collect();

    let mut directions = vec![];

    for (index, &(promittor, planet)) in promittors.iter().enumerate() {
        // 正向弧
        let promittor_od = calc_promittor_od(planet.ra, planet.dec, horo.geo.lat)?;
        let arc = swe_degnorm(promittor_od - dsc_od);

        if arc < MAX_ARC {
            let t = arc_to_date(arc, &horo.date)?;
            let direction = Direction::new(PlanetName::DSC, promittor, arc, t);
            directions.push(direction);
        }

        // 反向弧
        // 界是逆时针推运，调整为前一个界（首尾相连）
        let promittor = if matches!(promittor, Promittor::Term(_, _)) {
            let term_pos = term_indices.iter().position(|&i| i == index).unwrap();
            let prev_term_index = term_indices[(term_pos + term_indices.len() - 1) % term_indices.len()];
            promittors[prev_term_index].0
        } else {
            promittor
        };

        let arc = swe_degnorm(-arc);
        if arc < MAX_ARC {
            let t = arc_to_date(arc, &horo.date)?;
            let direction = Direction::new(PlanetName::DSC, promittor, -arc, t);
            directions.push(direction);
        }
    }

    Ok(directions)
}
