use crate::{
    config::PlanetConfig,
    planet::{Planet, PlanetName::*},
};

// ASC与行星相位
mod asc_apsect_planet;

// 行星与行星相位
mod planet_aspect_planet;

// 非比较盘相位

// 同一行星无相位
#[test]
fn test_self_no_aspect() {
    let planets = [
        ASC, MC, DSC, IC, Sun, Moon, Mercury, Venus, Mars, Jupiter, Saturn, NorthNode, SouthNode,
    ];

    for planet_name in planets {
        let p = Planet::new(
            planet_name.clone(),
            1.0,
            1.0,
            1.0,
            1.0,
            1.0,
            &PlanetConfig::new(planet_name, 1, 1.0, 2.0),
        );
        let aspect = p.has_aspect(&p, false);
        assert!(aspect.is_none());
    }
}

// 北交点与南交点无相位
#[test]
fn test_north_node_no_aspect_south_node() {
    let n = Planet::new(
        NorthNode,
        1.0,
        1.0,
        1.0,
        1.0,
        1.0,
        &PlanetConfig::new(NorthNode, 1, 1.0, 2.0),
    );
    let s = Planet::new(
        SouthNode,
        1.0,
        1.0,
        1.0,
        1.0,
        1.0,
        &PlanetConfig::new(SouthNode, 1, 1.0, 2.0),
    );

    assert!(n.has_aspect(&s, false).is_none());
    assert!(s.has_aspect(&n, false).is_none());
}

// ASC与DSC无相位
#[test]
fn test_asc_no_aspect_dsc() {
    let asc = Planet::new(
        ASC,
        1.0,
        1.0,
        1.0,
        1.0,
        1.0,
        &PlanetConfig::new(ASC, 1, 1.0, 2.0),
    );
    let dsc = Planet::new(
        DSC,
        1.0,
        1.0,
        1.0,
        1.0,
        1.0,
        &PlanetConfig::new(DSC, 1, 1.0, 2.0),
    );

    assert!(asc.has_aspect(&dsc, false).is_none());
    assert!(dsc.has_aspect(&asc, false).is_none());
}

// MC与IC无相位
#[test]
fn test_mc_no_aspect_ic() {
    let mc = Planet::new(
        MC,
        1.0,
        1.0,
        1.0,
        1.0,
        1.0,
        &PlanetConfig::new(MC, 1, 1.0, 2.0),
    );
    let ic = Planet::new(
        IC,
        1.0,
        1.0,
        1.0,
        1.0,
        1.0,
        &PlanetConfig::new(IC, 1, 1.0, 2.0),
    );

    assert!(mc.has_aspect(&ic, false).is_none());
    assert!(ic.has_aspect(&mc, false).is_none());
}

// 四轴间的相位
#[test]
#[ignore = "四轴相位，不必测试"]
fn aspect_between_asc_mc_dsc_ic() {
    todo!()
}
