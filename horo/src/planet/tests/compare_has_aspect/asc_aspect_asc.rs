use crate::{config::PlanetConfig, Planet, PlanetName::*};
use parameterized::parameterized;
use swe::swe_degnorm;

// 四轴与四轴的相位
#[parameterized(aspect_value = {
    0, 60, 90, 120, 180}
)]
fn test_ascmc_aspect_ascmc(aspect_value: u8) {
    for p0_name in [ASC, MC, DSC, IC] {
        for p1_name in [ASC, MC, DSC, IC] {
            let p0 = Planet::new(
                p0_name.clone(),
                1.0,
                1.0,
                0.0,
                1.0,
                1.0,
                &PlanetConfig::new(p0_name.clone(), 0, 1.0, 2.0),
            );
            let p1_longs = [
                1.0 + f64::from(aspect_value),
                swe_degnorm(360.0 - f64::from(aspect_value) + 1.0),
            ];

            let p1_longs = if aspect_value == 0 || aspect_value == 180 {
                &p1_longs[..1]
            } else {
                &p1_longs
            };
            for p1_long in p1_longs {
                let p1 = Planet::new(
                    p1_name.clone(),
                    *p1_long,
                    1.0,
                    0.0,
                    1.0,
                    1.0,
                    &PlanetConfig::new(p1_name.clone(), 0, 1.0, 2.0),
                );
                let aspect = p0.has_aspect(&p1, true);
                assert!(aspect.is_some());
                let aspect = aspect.unwrap();
                assert_eq!(
                    aspect_value, aspect.aspect_value,
                    "{:?}{}度相位{:?}",
                    p0.name, aspect_value, p1.name
                );
                assert_eq!(false, aspect.apply, "{:?}与{:?}出相位", p0.name, p1.name);
                assert_eq!(0.0, aspect.d, "{:?}与{:?}出相位0.0度", p0.name, p1.name);
                assert_eq!(p0.name, aspect.p0);
                assert_eq!(p1.name, aspect.p1);
            }
        }
    }
}
