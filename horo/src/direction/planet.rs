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
    let mut directions = Vec::new();

    for (index, &(promittor, promittor_planet)) in promittors.iter().enumerate() {
        if promittor == Promittor::Conjunction(significator.name) {
            continue;
        }

        let arc = planet_to_planet_direction(horo, significator, &promittor_planet)?;

        // 正弧度
        if arc < MAX_ARC {
            let t = arc_to_date(arc, &horo.date)?;
            let direction = Direction::new(significator.name, promittor, arc, t);
            directions.push(direction);
        }

        // 反弧度
        let promittor = if matches!(promittor, Promittor::Term(_, _)) {
            promittors[(index + promittors.len() - 1) % promittors.len()].0
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
