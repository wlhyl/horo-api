use swe::swe_degnorm;

use crate::{
    Direction, Error, Horoscope, Planet, PlanetName, Promittor,
    direction::{MAX_ARC, arc_to_date},
};

// 计算MC的推运
pub(crate) fn mc_direction(
    horo: &Horoscope,
    promittors: &[(Promittor, Planet)],
) -> Result<Vec<Direction>, Error> {
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
        let arc = swe_degnorm(planet.ra - horo.mc.ra);

        if arc < MAX_ARC {
            let t = arc_to_date(arc, &horo.date)?;
            let direction = Direction::new(PlanetName::MC, promittor, arc, t);

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
            let direction = Direction::new(PlanetName::MC, promittor, -arc, t);
            directions.push(direction);
        }
    }

    Ok(directions)
}

pub(crate) fn ic_direction(
    horo: &Horoscope,
    promittors: &[(Promittor, Planet)],
) -> Result<Vec<Direction>, Error> {
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
        let arc = swe_degnorm(planet.ra - horo.ic.ra);

        if arc < MAX_ARC {
            let t = arc_to_date(arc, &horo.date)?;
            let direction = Direction::new(PlanetName::IC, promittor, arc, t);

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
            let direction = Direction::new(PlanetName::IC, promittor, -arc, t);
            directions.push(direction);
        }
    }

    Ok(directions)
}
