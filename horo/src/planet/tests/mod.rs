use crate::{
    config::PlanetConfig,
    planet::{Planet, PlanetName::*, PlanetSpeedState::*},
};

// 非比较盘相位
mod no_compare_has_aspect;
// 比较盘相位
mod compare_has_aspect;

// 映点
mod antiscoin;

// 反映点
mod contraantiscia;

// 构造函数正确存储输入参数
#[test]
fn test_new() {
    let p = Planet::new(
        ASC,
        1.0,
        1.0,
        1.0,
        1.0,
        1.0,
        &PlanetConfig::new(ASC, 1, 1.0, 2.0),
    );

    assert_eq!(ASC, p.name, "name");
    assert_eq!(1.0, p.long, "黄经");
    assert_eq!(1.0, p.lat, "黄纬");
    assert_eq!(1.0, p.speed, "speed");
    assert_eq!(1.0, p.ra, "赤经");
    assert_eq!(1.0, p.dec, "赤纬");
    assert_eq!(1, p.orb, "容许度");
}

// 快
#[test]
fn test_faster() {
    // 逆行
    let p0 = Planet::new(
        ASC,
        1.0,
        1.0,
        -3.1,
        1.0,
        1.0,
        &PlanetConfig::new(ASC, 1, 1.0, 2.0),
    );
    assert_eq!(快, p0.speed_state, "逆行，快");

    // 顺行
    let p1 = Planet::new(
        ASC,
        1.0,
        1.0,
        3.1,
        1.0,
        1.0,
        &PlanetConfig::new(ASC, 1, 1.0, 2.0),
    );
    assert_eq!(快, p1.speed_state, "顺行，快")
}

// 慢
#[test]
fn test_slower() {
    // 逆行
    let p0 = Planet::new(
        ASC,
        1.0,
        1.0,
        -0.1,
        1.0,
        1.0,
        &PlanetConfig::new(ASC, 1, 1.0, 2.0),
    );
    assert_eq!(慢, p0.speed_state, "逆行，慢");

    // 顺行
    let p1 = Planet::new(
        ASC,
        1.0,
        1.0,
        0.1,
        1.0,
        1.0,
        &PlanetConfig::new(ASC, 1, 1.0, 2.0),
    );
    assert_eq!(慢, p1.speed_state, "顺行，慢");
}

// 平均
#[test]
fn test_average() {
    let p0 = Planet::new(
        ASC,
        1.0,
        1.0,
        -0.1,
        1.0,
        1.0,
        &PlanetConfig::new(ASC, 1, 2.0, 2.0),
    );
    assert_eq!(均, p0.speed_state, "逆行，均");

    let p1 = Planet::new(
        ASC,
        1.0,
        1.0,
        0.1,
        1.0,
        1.0,
        &PlanetConfig::new(ASC, 1, 2.0, 2.0),
    );
    assert_eq!(均, p1.speed_state, "顺行，均");
}
