use swe::swe_degnorm;

use crate::{
    Direction, Error, Horoscope, Planet, Promittor,
    direction::{MAX_ARC, arc_to_date, planet_to_planet_direction},
};

pub(super) fn planet_direction(
    horo: &Horoscope,
    significator: &Planet,
    promittors: &[(Promittor, Planet)],
) -> Result<Vec<Direction>, Error> {
    // 找出所有 Term 的索引
    let term_indices: Vec<usize> = promittors
        .iter()
        .enumerate()
        .filter(|(_, (promittor, _))| matches!(promittor, Promittor::Term(_, _)))
        .map(|(i, _)| i)
        .collect();

    let mut directions = Vec::new();

    for (index, &(promittor, promittor_planet)) in promittors.iter().enumerate() {
        if promittor == Promittor::Conjunction(significator.name) {
            continue;
        }

        let arc = planet_to_planet_direction(horo, significator, &promittor_planet)?;

        // 正向弧度
        if arc < MAX_ARC {
            let t = arc_to_date(arc, &horo.date)?;
            let direction = Direction::new(significator.name, promittor, arc, t);
            directions.push(direction);
        }

        // 反向弧度
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
            let direction = Direction::new(significator.name, promittor, -arc, t);
            directions.push(direction);
        }
    }
    Ok(directions)
}
