use crate::{config::PlanetConfig, Planet, PlanetName::*};
use parameterized::parameterized;
use swe::swe_degnorm;

// 行星相位行星，行星容许度为0
#[parameterized(aspect_value = {
    0, 60, 90, 120, 180}
)]
fn planet_aspect_planet_plant_orb_is_0(aspect_value: u8) {
    // p0是本盘行星
    let p0 = Planet::new(
        Sun,
        1.0,
        1.0,
        0.0,
        1.0,
        1.0,
        &PlanetConfig::new(Sun, 0, 1.0, 2.0),
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
            Moon,
            *p1_long,
            1.0,
            1.0,
            1.0,
            1.0,
            &PlanetConfig::new(Moon, 0, 1.0, 2.0),
        );
        let aspect = p0.has_aspect(&p1, true);
        assert!(aspect.is_some());
        let aspect = aspect.unwrap();
        assert_eq!(
            aspect_value, aspect.aspect_value,
            "{:?}{}度相位{:?}",
            p1.name, aspect_value, p0.name
        );
        assert_eq!(false, aspect.apply, "{:?}与{:?}出相位", p1.name, p0.name);
        assert_eq!(0.0, aspect.d, "{:?}与{:?}出相位0.0度", p1.name, p0.name);
        assert_eq!(p0.name, aspect.p0);
        assert_eq!(p1.name, aspect.p1);
    }
}

// 行星相位行星，行星容许度不为0，行运行星顺行，没有跨星座
#[parameterized(aspect_value = {
    0, 60, 90, 120, 180},
)]
fn planet_aspect_planet_plant_orb_is_not_0_planet_forward_no_cross_sign(aspect_value: u8) {
    let p0_longs = [
        10.0 + f64::from(aspect_value),
        swe_degnorm(360.0 - f64::from(aspect_value) + 10.0),
    ];

    let p0_longs = if aspect_value == 0 || aspect_value == 180 {
        &p0_longs[..1]
    } else {
        &p0_longs
    };

    // p0是本盘行星
    for speed in [-1.0, 1.0] {
        for p0_long in p0_longs {
            // 没有相位
            let p0 = Planet::new(
                Sun,
                *p0_long,
                1.0,
                speed,
                1.0,
                1.0,
                &PlanetConfig::new(Sun, 6, 1.0, 2.0),
            );
            let mut p1 = Planet::new(
                Moon,
                4.0,
                1.0,
                2.0,
                1.0,
                1.0,
                &PlanetConfig::new(Moon, 4, 1.0, 2.0),
            );
            let aspect = p0.has_aspect(&p1, true);
            assert!(aspect.is_none(), "没有入相位，行星{aspect_value}度相位asc");

            // 刚有相位
            p1.long = 5.0;
            let aspect = p0.has_aspect(&p1, true);
            assert!(aspect.is_some());
            let aspect = aspect.unwrap();
            assert_eq!(
                aspect_value, aspect.aspect_value,
                "刚形成相位，行星{aspect_value}度相位asc"
            );
            assert_eq!(true, aspect.apply, "刚形成相位，asc入相位行星");
            assert_eq!(5.0, aspect.d, "刚形成相位，asc与行星入相位1.0度");
            assert_eq!(p0.name, aspect.p0);
            assert_eq!(p1.name, aspect.p1);

            // 入相位中
            p1.long = 6.0;
            let aspect = p0.has_aspect(&p1, true);
            assert!(aspect.is_some());
            let aspect = aspect.unwrap();
            assert_eq!(
                aspect_value, aspect.aspect_value,
                "入相位中，行星{aspect_value}度相位asc"
            );
            assert_eq!(true, aspect.apply, "入相位中，行星入相位asc");
            assert_eq!(4.0, aspect.d, "入相位中，asc与行星距离");
            assert_eq!(p0.name, aspect.p0);
            assert_eq!(p1.name, aspect.p1);

            // 形成相位
            p1.long = 10.0;
            let aspect = p0.has_aspect(&p1, true);
            assert!(aspect.is_some());
            let aspect = aspect.unwrap();
            assert_eq!(
                aspect_value, aspect.aspect_value,
                "形成相位，行星{aspect_value}度相位asc"
            );
            assert_eq!(false, aspect.apply, "形成相位，行星出相位asc");
            assert_eq!(0.0, aspect.d, "形成相位，asc与行星入相位0.0度");
            assert_eq!(p0.name, aspect.p0);
            assert_eq!(p1.name, aspect.p1);

            // 离相位中
            p1.long = 11.0;
            let aspect = p0.has_aspect(&p1, true);
            assert!(aspect.is_some());
            let aspect = aspect.unwrap();
            assert_eq!(
                aspect_value, aspect.aspect_value,
                "离相位中，行星{aspect_value}度相位asc"
            );
            assert_eq!(false, aspect.apply, "离相位中，行星离相位asc");
            assert_eq!(1.0, aspect.d, "离相位中，asc与行星离相位1.0度");
            assert_eq!(p0.name, aspect.p0);
            assert_eq!(p1.name, aspect.p1);

            // 即将无相位
            p1.long = 15.0;
            let aspect = p0.has_aspect(&p1, true);
            assert!(aspect.is_some());
            let aspect = aspect.unwrap();
            assert_eq!(
                aspect_value, aspect.aspect_value,
                "即将无相位，行星{aspect_value}度相位asc"
            );
            assert_eq!(false, aspect.apply, "即将无相位，行星离相位asc");
            assert_eq!(5.0, aspect.d, "即将无相位，asc与行星距离");
            assert_eq!(p0.name, aspect.p0);
            assert_eq!(p1.name, aspect.p1);

            // 已经无相位
            p1.long = 16.0;
            assert!(
                p0.has_aspect(&p1, true).is_none(),
                "已经无相位，行星{aspect_value}度相位asc"
            );
        }
    }
}

// 行星相位行星，行星容许度不为0，行运行星逆行，没有跨星座
#[parameterized(aspect_value = {
    0, 60, 90, 120, 180},
)]
fn planet_aspect_planet_plant_orb_is_not_0_planet_retrograde_no_cross_sign(aspect_value: u8) {
    let p0_longs = [
        10.0 + f64::from(aspect_value),
        swe_degnorm(360.0 - f64::from(aspect_value) + 10.0),
    ];

    let p0_longs = if aspect_value == 0 || aspect_value == 180 {
        &p0_longs[..1]
    } else {
        &p0_longs
    };
    for speed in [-1.0, 1.0] {
        for p0_long in p0_longs {
            // 没有相位
            let asc = Planet::new(
                Sun,
                *p0_long,
                1.0,
                speed,
                1.0,
                1.0,
                &PlanetConfig::new(Sun, 6, 1.0, 2.0),
            );
            let mut p = Planet::new(
                Sun,
                16.0,
                1.0,
                -2.0,
                1.0,
                1.0,
                &PlanetConfig::new(Sun, 4, 1.0, 2.0),
            );
            let aspect = asc.has_aspect(&p, true);
            assert!(aspect.is_none(), "没有入相位，行星{aspect_value}度相位asc");

            // 刚有相位
            p.long = 15.0;
            let aspect = asc.has_aspect(&p, true);
            assert!(aspect.is_some());
            let aspect = aspect.unwrap();
            assert_eq!(
                aspect_value, aspect.aspect_value,
                "刚形成相位，行星{aspect_value}度相位asc"
            );
            assert_eq!(true, aspect.apply, "刚形成相位，asc入相位行星");
            assert_eq!(5.0, aspect.d, "刚形成相位，asc与行星入相位1.0度");
            assert_eq!(asc.name, aspect.p0);
            assert_eq!(p.name, aspect.p1);

            // 入相位中
            p.long = 14.0;
            let aspect = asc.has_aspect(&p, true);
            assert!(aspect.is_some());
            let aspect = aspect.unwrap();
            assert_eq!(
                aspect_value, aspect.aspect_value,
                "入相位中，行星{aspect_value}度相位asc"
            );
            assert_eq!(true, aspect.apply, "入相位中，行星入相位asc");
            assert_eq!(4.0, aspect.d, "入相位中，asc与行星距离");
            assert_eq!(asc.name, aspect.p0);
            assert_eq!(p.name, aspect.p1);

            // 形成相位
            p.long = 10.0;
            let aspect = asc.has_aspect(&p, true);
            assert!(aspect.is_some());
            let aspect = aspect.unwrap();
            assert_eq!(
                aspect_value, aspect.aspect_value,
                "形成相位，行星{aspect_value}度相位asc"
            );
            assert_eq!(false, aspect.apply, "形成相位，行星出相位asc");
            assert_eq!(0.0, aspect.d, "形成相位，asc与行星入相位0.0度");
            assert_eq!(asc.name, aspect.p0);
            assert_eq!(p.name, aspect.p1);

            // 离相位中
            p.long = 9.0;
            let aspect = asc.has_aspect(&p, true);
            assert!(aspect.is_some());
            let aspect = aspect.unwrap();
            assert_eq!(
                aspect_value, aspect.aspect_value,
                "离相位中，行星{aspect_value}度相位asc"
            );
            assert_eq!(false, aspect.apply, "离相位中，行星离相位asc");
            assert_eq!(1.0, aspect.d, "离相位中，asc与行星离相位1.0度");
            assert_eq!(asc.name, aspect.p0);
            assert_eq!(p.name, aspect.p1);

            // 即将无相位
            p.long = 5.0;
            let aspect = asc.has_aspect(&p, true);
            assert!(aspect.is_some());
            let aspect = aspect.unwrap();
            assert_eq!(
                aspect_value, aspect.aspect_value,
                "即将无相位，行星{aspect_value}度相位asc"
            );
            assert_eq!(false, aspect.apply, "即将无相位，行星离相位asc");
            assert_eq!(5.0, aspect.d, "即将无相位，asc与行星距离");
            assert_eq!(asc.name, aspect.p0);
            assert_eq!(p.name, aspect.p1);

            // 已经无相位
            p.long = 4.0;
            assert!(
                asc.has_aspect(&p, true).is_none(),
                "已经无相位，行星{aspect_value}度相位asc"
            );
        }
    }
}

// 行星相位行星，行星容许度不为0，行运行星顺行，跨星座入相位
#[parameterized(aspect_value = {
    0, 60, 90, 120, 180}
)]
fn planet_aspect_planet_plant_orb_is_not_0_planet_forward_cross_sign_apply(aspect_value: u8) {
    let p0_longs = [
        2.0 + f64::from(aspect_value),
        swe_degnorm(360.0 - f64::from(aspect_value) + 2.0),
    ];

    let p0_longs = if aspect_value == 0 || aspect_value == 180 {
        &p0_longs[..1]
    } else {
        &p0_longs
    };
    for speed in [-1.0, 1.0] {
        for p0_long in p0_longs {
            // 没有相位
            let asc = Planet::new(
                Sun,
                *p0_long,
                1.0,
                speed,
                1.0,
                1.0,
                &PlanetConfig::new(Sun, 6, 1.0, 2.0),
            );
            let mut p = Planet::new(
                Sun,
                356.0,
                1.0,
                2.0,
                1.0,
                1.0,
                &PlanetConfig::new(Sun, 4, 1.0, 2.0),
            );

            let aspect = asc.has_aspect(&p, true);
            assert!(aspect.is_none(), "没有入相位，行星{aspect_value}度相位asc");

            // 刚有相位
            p.long = 357.0;
            let aspect = asc.has_aspect(&p, true);
            assert!(
                aspect.is_none(),
                "无相位，度数上刚形成相位，行星{aspect_value}度相位asc"
            );

            // 入相位中
            p.long = 359.0;
            let aspect = asc.has_aspect(&p, true);
            assert!(
                aspect.is_none(),
                "无相位，度数上入相位中，行星{aspect_value}度相位asc"
            );

            // 入相位中,白羊座0度
            p.long = 0.0;
            let aspect = asc.has_aspect(&p, true);
            assert!(aspect.is_some());
            let aspect = aspect.unwrap();
            assert_eq!(
                aspect_value, aspect.aspect_value,
                "入相位中，行星{aspect_value}度相位asc"
            );
            assert_eq!(true, aspect.apply, "入相位中，行星入相位asc");
            assert_eq!(2.0, aspect.d, "入相位中，asc与行星距离");
            assert_eq!(asc.name, aspect.p0);
            assert_eq!(p.name, aspect.p1);

            // 形成相位
            p.long = 2.0;
            let aspect = asc.has_aspect(&p, true);
            assert!(aspect.is_some());
            let aspect = aspect.unwrap();
            assert_eq!(
                aspect_value, aspect.aspect_value,
                "形成相位，行星{aspect_value}度相位asc"
            );
            assert_eq!(false, aspect.apply, "形成相位，行星出相位asc");
            assert_eq!(0.0, aspect.d, "形成相位，asc与行星入相位0.0度");
            assert_eq!(asc.name, aspect.p0);
            assert_eq!(p.name, aspect.p1);

            // 离相位中
            p.long = 3.0;
            let aspect = asc.has_aspect(&p, true);
            assert!(aspect.is_some());
            let aspect = aspect.unwrap();
            assert_eq!(
                aspect_value, aspect.aspect_value,
                "离相位中，行星{aspect_value}度相位asc"
            );
            assert_eq!(false, aspect.apply, "离相位中，asc离相位行星");
            assert_eq!(1.0, aspect.d, "离相位中，asc与行星离相位1.0度");
            assert_eq!(asc.name, aspect.p0);
            assert_eq!(p.name, aspect.p1);

            // 即将无相位
            p.long = 7.0;
            let aspect = asc.has_aspect(&p, true);
            assert!(aspect.is_some());
            let aspect = aspect.unwrap();
            assert_eq!(
                aspect_value, aspect.aspect_value,
                "即将无相位，行星{aspect_value}度相位asc"
            );
            assert_eq!(false, aspect.apply, "即将无相位，行星离相位asc");
            assert_eq!(5.0, aspect.d, "即将无相位，asc与行星距离");
            assert_eq!(asc.name, aspect.p0);
            assert_eq!(p.name, aspect.p1);

            // 已经无相位
            p.long = 8.0;
            assert!(
                asc.has_aspect(&p, true).is_none(),
                "已经无相位，行星{aspect_value}度相位asc"
            );
        }
    }
}

// 行星相位行星，行星容许度不为0，行运行星逆行，跨星座入相位
#[parameterized(aspect_value = {
    0, 60, 90, 120, 180}
)]
fn planet_aspect_planet_plant_orb_is_not_0_planet_retrograde_cross_sign_apply(aspect_value: u8) {
    let p0_longs = [
        28.0 + f64::from(aspect_value),
        swe_degnorm(360.0 - f64::from(aspect_value) + 28.0),
    ];

    let p0_longs = if aspect_value == 0 || aspect_value == 180 {
        &p0_longs[..1]
    } else {
        &p0_longs
    };
    for speed in [-1.0, 1.0] {
        for p0_long in p0_longs {
            // 没有相位
            let asc = Planet::new(
                Sun,
                *p0_long,
                1.0,
                speed,
                1.0,
                1.0,
                &PlanetConfig::new(Sun, 6, 1.0, 2.0),
            );
            let mut p = Planet::new(
                Sun,
                34.0,
                1.0,
                -2.0,
                1.0,
                1.0,
                &PlanetConfig::new(Sun, 4, 1.0, 2.0),
            );

            let aspect = asc.has_aspect(&p, true);
            assert!(aspect.is_none(), "没有入相位，行星{aspect_value}度相位asc");

            // 刚有相位
            p.long = 33.0;
            let aspect = asc.has_aspect(&p, true);
            assert!(
                aspect.is_none(),
                "无相位，度数上刚形成相位，行星{aspect_value}度相位asc"
            );

            // 入相位中,在金牛座0度
            p.long = 30.0;
            let aspect = asc.has_aspect(&p, true);
            assert!(
                aspect.is_none(),
                "无相位，度数上入相位中，行星{aspect_value}度相位asc"
            );

            // 入相位中,白羊座29度
            p.long = 29.0;
            let aspect = asc.has_aspect(&p, true);
            assert!(aspect.is_some());
            let aspect = aspect.unwrap();
            assert_eq!(
                aspect_value, aspect.aspect_value,
                "入相位中，行星{aspect_value}度相位asc"
            );
            assert_eq!(true, aspect.apply, "入相位中，行星入相位asc");
            assert_eq!(1.0, aspect.d, "入相位中，asc与行星距离");
            assert_eq!(asc.name, aspect.p0);
            assert_eq!(p.name, aspect.p1);

            // 形成相位
            p.long = 28.0;
            let aspect = asc.has_aspect(&p, true);
            assert!(aspect.is_some());
            let aspect = aspect.unwrap();
            assert_eq!(
                aspect_value, aspect.aspect_value,
                "形成相位，行星{aspect_value}度相位asc"
            );
            assert_eq!(false, aspect.apply, "形成相位，行星出相位asc");
            assert_eq!(0.0, aspect.d, "形成相位，asc与行星入相位0.0度");
            assert_eq!(asc.name, aspect.p0);
            assert_eq!(p.name, aspect.p1);

            // 离相位中
            p.long = 27.0;
            let aspect = asc.has_aspect(&p, true);
            assert!(aspect.is_some());
            let aspect = aspect.unwrap();
            assert_eq!(
                aspect_value, aspect.aspect_value,
                "离相位中，行星{aspect_value}度相位asc"
            );
            assert_eq!(false, aspect.apply, "离相位中，asc离相位行星");
            assert_eq!(1.0, aspect.d, "离相位中，asc与行星离相位1.0度");
            assert_eq!(asc.name, aspect.p0);
            assert_eq!(p.name, aspect.p1);

            // 即将无相位
            p.long = 23.0;
            let aspect = asc.has_aspect(&p, true);
            assert!(aspect.is_some());
            let aspect = aspect.unwrap();
            assert_eq!(
                aspect_value, aspect.aspect_value,
                "即将无相位，行星{aspect_value}度相位asc"
            );
            assert_eq!(false, aspect.apply, "即将无相位，行星离相位asc");
            assert_eq!(5.0, aspect.d, "即将无相位，asc与行星距离");
            assert_eq!(asc.name, aspect.p0);
            assert_eq!(p.name, aspect.p1);

            // 已经无相位
            p.long = 22.0;
            assert!(
                asc.has_aspect(&p, true).is_none(),
                "已经无相位，行星{aspect_value}度相位asc"
            );
        }
    }
}

// 行星相位行星，行星容许度不为0，行星顺行，跨星座离相位
#[parameterized(aspect_value = {
    0, 60, 90, 120, 180}
)]
fn planet_aspect_planet_plant_orb_is_not_0_planet_forward_cross_sign_separated(aspect_value: u8) {
    let p0_longs = [
        28.0 + f64::from(aspect_value),
        swe_degnorm(360.0 - f64::from(aspect_value) + 28.0),
    ];

    let p0_longs = if aspect_value == 0 || aspect_value == 180 {
        &p0_longs[..1]
    } else {
        &p0_longs
    };
    for speed in [-1.0, 1.0] {
        for p0_long in p0_longs {
            // 没有相位
            let asc = Planet::new(
                Sun,
                *p0_long,
                1.0,
                speed,
                1.0,
                1.0,
                &PlanetConfig::new(Sun, 6, 1.0, 2.0),
            );
            let mut p = Planet::new(
                Sun,
                22.0,
                1.0,
                2.0,
                1.0,
                1.0,
                &PlanetConfig::new(Sun, 4, 1.0, 2.0),
            );

            let aspect = asc.has_aspect(&p, true);
            assert!(aspect.is_none(), "没有入相位，行星{aspect_value}度相位asc");

            // 刚有相位
            p.long = 23.0;
            let aspect = asc.has_aspect(&p, true);
            assert!(aspect.is_some());
            let aspect = aspect.unwrap();
            assert_eq!(
                aspect_value, aspect.aspect_value,
                "刚入相位，行星{aspect_value}度相位asc"
            );
            assert_eq!(true, aspect.apply, "刚入相位，行星入相位asc");
            assert_eq!(5.0, aspect.d, "刚入相位，asc与行星距离");
            assert_eq!(asc.name, aspect.p0);
            assert_eq!(p.name, aspect.p1);

            // 入相位中
            p.long = 24.0;
            let aspect = asc.has_aspect(&p, true);
            assert!(aspect.is_some());
            let aspect = aspect.unwrap();
            assert_eq!(
                aspect_value, aspect.aspect_value,
                "入相位中，行星{aspect_value}度相位asc"
            );
            assert_eq!(true, aspect.apply, "入相位中，行星入相位asc");
            assert_eq!(4.0, aspect.d, "入相位中，asc与行星距离");
            assert_eq!(asc.name, aspect.p0);
            assert_eq!(p.name, aspect.p1);

            // 形成相位
            p.long = 28.0;
            let aspect = asc.has_aspect(&p, true);
            assert!(aspect.is_some());
            let aspect = aspect.unwrap();
            assert_eq!(
                aspect_value, aspect.aspect_value,
                "形成相位，行星{aspect_value}度相位asc"
            );
            assert_eq!(false, aspect.apply, "形成相位，行星出相位asc");
            assert_eq!(0.0, aspect.d, "形成相位，asc与行星入相位0.0度");
            assert_eq!(asc.name, aspect.p0);
            assert_eq!(p.name, aspect.p1);

            // 离相位中
            p.long = 29.0;
            let aspect = asc.has_aspect(&p, true);
            assert!(aspect.is_some());
            let aspect = aspect.unwrap();
            assert_eq!(
                aspect_value, aspect.aspect_value,
                "离相位中，行星{aspect_value}度相位asc"
            );
            assert_eq!(false, aspect.apply, "离相位中，行星离相位asc");
            assert_eq!(1.0, aspect.d, "离相位中，asc与行星距离");
            assert_eq!(asc.name, aspect.p0);
            assert_eq!(p.name, aspect.p1);

            // 离相位中，金牛座0度
            p.long = 30.0;
            let aspect = asc.has_aspect(&p, true);
            assert!(
                aspect.is_none(),
                "无相位，度数上离相位中，行星{aspect_value}度相位asc"
            );

            // 即将无相位
            p.long = 33.0;
            let aspect = asc.has_aspect(&p, true);
            assert!(
                aspect.is_none(),
                "无相位，度数上即将无相位，行星{aspect_value}度相位asc"
            );

            // 已经无相位
            p.long = 34.0;
            assert!(
                asc.has_aspect(&p, true).is_none(),
                "已经无相位，行星{aspect_value}度相位asc"
            );
        }
    }
}

// 行星相位行星，行星容许度不为0，行星逆行，跨星座离相位
#[parameterized(aspect_value = {
    0, 60, 90, 120, 180}
)]
fn planet_aspect_planet_plant_orb_is_not_0_planet_retrograde_cross_sign_separated(
    aspect_value: u8,
) {
    let p0_longs = [
        2.0 + f64::from(aspect_value),
        swe_degnorm(360.0 - f64::from(aspect_value) + 2.0),
    ];

    let p0_longs = if aspect_value == 0 || aspect_value == 180 {
        &p0_longs[..1]
    } else {
        &p0_longs
    };
    for speed in [-1.0, 1.0] {
        for p0_long in p0_longs {
            // 没有相位
            let asc = Planet::new(
                Sun,
                *p0_long,
                1.0,
                speed,
                1.0,
                1.0,
                &PlanetConfig::new(Sun, 6, 1.0, 2.0),
            );
            let mut p = Planet::new(
                Sun,
                8.0,
                1.0,
                -2.0,
                1.0,
                1.0,
                &PlanetConfig::new(Sun, 4, 1.0, 2.0),
            );

            let aspect = asc.has_aspect(&p, true);
            assert!(aspect.is_none(), "没有入相位，行星{aspect_value}度相位asc");

            // 刚有相位
            p.long = 7.0;
            let aspect = asc.has_aspect(&p, true);
            assert!(aspect.is_some());
            let aspect = aspect.unwrap();
            assert_eq!(
                aspect_value, aspect.aspect_value,
                "刚入相位，行星{aspect_value}度相位asc"
            );
            assert_eq!(true, aspect.apply, "刚入相位，行星入相位asc");
            assert_eq!(5.0, aspect.d, "入相位中，asc与行星距离");
            assert_eq!(asc.name, aspect.p0);
            assert_eq!(p.name, aspect.p1);

            // 入相位中
            p.long = 6.0;
            let aspect = asc.has_aspect(&p, true);
            assert!(aspect.is_some());
            let aspect = aspect.unwrap();
            assert_eq!(
                aspect_value, aspect.aspect_value,
                "入相位中，行星{aspect_value}度相位asc"
            );
            assert_eq!(true, aspect.apply, "入相位中，行星入相位asc");
            assert_eq!(4.0, aspect.d, "入相位中，asc与行星距离");
            assert_eq!(asc.name, aspect.p0);
            assert_eq!(p.name, aspect.p1);

            // 形成相位
            p.long = 2.0;
            let aspect = asc.has_aspect(&p, true);
            assert!(aspect.is_some());
            let aspect = aspect.unwrap();
            assert_eq!(
                aspect_value, aspect.aspect_value,
                "形成相位，行星{aspect_value}度相位asc"
            );
            assert_eq!(false, aspect.apply, "形成相位，行星出相位asc");
            assert_eq!(0.0, aspect.d, "形成相位，asc与行星入相位0.0度");
            assert_eq!(asc.name, aspect.p0);
            assert_eq!(p.name, aspect.p1);

            // 离相位中
            p.long = 1.0;
            let aspect = asc.has_aspect(&p, true);
            assert!(aspect.is_some());
            let aspect = aspect.unwrap();
            assert_eq!(
                aspect_value, aspect.aspect_value,
                "离相位中，行星{aspect_value}度相位asc"
            );
            assert_eq!(false, aspect.apply, "离相位中，行星离相位asc");
            assert_eq!(1.0, aspect.d, "离相位中，asc与行星离相位1.0度");
            assert_eq!(asc.name, aspect.p0);
            assert_eq!(p.name, aspect.p1);

            // 离相位中，白羊座0度
            p.long = 0.0;
            let aspect = asc.has_aspect(&p, true);
            assert!(aspect.is_some());
            let aspect = aspect.unwrap();
            assert_eq!(
                aspect_value, aspect.aspect_value,
                "离相位中，行星{aspect_value}度相位asc"
            );
            assert_eq!(false, aspect.apply, "离相位中，行星离相位asc");
            assert_eq!(2.0, aspect.d, "离相位中，asc与行星离相位1.0度");
            assert_eq!(asc.name, aspect.p0);
            assert_eq!(p.name, aspect.p1);

            // 离相位中，双鱼座29度
            p.long = 359.0;
            let aspect = asc.has_aspect(&p, true);
            assert!(
                aspect.is_none(),
                "无相位，度数上离相位中，行星{aspect_value}度相位asc"
            );

            // 即将无相位
            p.long = 357.0;
            let aspect = asc.has_aspect(&p, true);
            assert!(
                aspect.is_none(),
                "无相位，度数上即将无相位，行星{aspect_value}度相位asc"
            );

            // 已经无相位
            p.long = 356.0;
            assert!(
                asc.has_aspect(&p, true).is_none(),
                "已经无相位，行星{aspect_value}度相位asc"
            );
        }
    }
}
