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
        let promittor = if matches!(promittor, Promittor::Term(_, _)) {
            promittors[(index + promittors.len() - 1) % promittors.len()].0
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
        let promittor = if matches!(promittor, Promittor::Term(_, _)) {
            promittors[(index + promittors.len() - 1) % promittors.len()].0
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
