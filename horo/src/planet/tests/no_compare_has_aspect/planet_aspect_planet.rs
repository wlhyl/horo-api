use crate::{
    config::PlanetConfig,
    planet::{Planet, PlanetName::*},
};
use parameterized::parameterized;
use swe::swe_degnorm;

// 行星与行星相位，行星容许度为0
#[parameterized(aspect_value = {
    0, 60, 90, 120, 180}
)]
fn planet_aspect_planet_plant_orb_is_0(aspect_value: u8) {
    // 二星移动速度相同
    let p0 = Planet::new(
        Sun,
        1.0,
        1.0,
        1.0,
        1.0,
        1.0,
        &PlanetConfig::new(Sun, 0, 1.0, 2.0),
    );
    let p1 = Planet::new(
        Moon,
        1.0 + f64::from(aspect_value),
        1.0,
        1.0,
        1.0,
        1.0,
        &PlanetConfig::new(Moon, 0, 1.0, 2.0),
    );
    let aspect = p0.has_aspect(&p1, false);
    assert!(aspect.is_some());
    let aspect = aspect.unwrap();
    assert_eq!(
        aspect_value, aspect.aspect_value,
        "行星{aspect_value}度相位行星"
    );
    assert_eq!(false, aspect.apply, "行星与行星出相位");
    assert_eq!(0.0, aspect.d, "行星与行星出相位0.0度");
    assert_eq!(p0.name, aspect.p0);
    assert_eq!(p1.name, aspect.p1);

    let aspect = p1.has_aspect(&p0, false);
    assert!(aspect.is_some());
    let aspect = aspect.unwrap();
    assert_eq!(
        aspect_value, aspect.aspect_value,
        "行星{aspect_value}度相位行星"
    );
    assert_eq!(false, aspect.apply, "行星与行星出相位");
    assert_eq!(0.0, aspect.d, "行星与行星出相位0.0度");
    assert_eq!(p1.name, aspect.p0);
    assert_eq!(p0.name, aspect.p1);

    // 二星移动速度不同
    let p0 = Planet::new(
        Sun,
        1.0,
        1.0,
        1.0,
        1.0,
        1.0,
        &PlanetConfig::new(Sun, 0, 1.0, 2.0),
    );
    let p1 = Planet::new(
        Moon,
        1.0 + f64::from(aspect_value),
        1.0,
        2.0,
        1.0,
        1.0,
        &PlanetConfig::new(Moon, 0, 1.0, 2.0),
    );
    let aspect = p0.has_aspect(&p1, false);
    assert!(aspect.is_some());
    let aspect = aspect.unwrap();
    assert_eq!(
        aspect_value, aspect.aspect_value,
        "行星{aspect_value}度相位行星"
    );
    assert_eq!(false, aspect.apply, "行星与行星出相位");
    assert_eq!(0.0, aspect.d, "行星与行星出相位0.0度");
    assert_eq!(p0.name, aspect.p0);
    assert_eq!(p1.name, aspect.p1);

    let aspect = p1.has_aspect(&p0, false);
    assert!(aspect.is_some());
    let aspect = aspect.unwrap();
    assert_eq!(
        aspect_value, aspect.aspect_value,
        "行星{aspect_value}度相位行星"
    );
    assert_eq!(false, aspect.apply, "行星与行星出相位");
    assert_eq!(0.0, aspect.d, "行星与行星出相位0.0度");
    assert_eq!(p1.name, aspect.p0);
    assert_eq!(p0.name, aspect.p1);
}

// 行星与行星相位，行星容许度不为0，二星顺行，快行星左/右相位
#[parameterized(aspect_value = {
    0, 60, 90, 120, 180}
)]
fn planet_sinister_or_dexter_planet_plant_orb_is_not_0_both_forward(aspect_value: u8) {
    // 相位点在白羊座10度

    // sun 在-0, -60, -90, -120, -180
    // -0, -180不存在右相位，但测试中仍保留，这两种相位的测试与左相位是相同的
    // swe_degnorm(360.0 - f64::from(aspect_value) + 10.0)
    let p0_longs = [
        10.0 + f64::from(aspect_value),
        swe_degnorm(360.0 - f64::from(aspect_value) + 10.0),
    ];

    let p0_longs = if aspect_value == 0 || aspect_value == 180 {
        &p0_longs[..1]
    } else {
        &p0_longs
    };

    for p0_long in p0_longs {
        // 没有相位
        let p0 = Planet::new(
            Sun,
            *p0_long,
            1.0,
            1.0,
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
        let aspect = p0.has_aspect(&p1, false);
        assert!(aspect.is_none(), "还没入相位，行星{aspect_value}度相位行星");
        let aspect = p1.has_aspect(&p0, false);
        assert!(aspect.is_none(), "还没入相位，行星{aspect_value}度相位asc");

        // 刚入相位
        p1.long = 5.0;
        let aspect = p0.has_aspect(&p1, false);
        assert!(aspect.is_some());
        let aspect = aspect.unwrap();
        assert_eq!(
            aspect_value, aspect.aspect_value,
            "刚形成相位，行星{aspect_value}度相位行星"
        );
        assert_eq!(true, aspect.apply, "刚形成相位，行星入相位行星");
        assert_eq!(5.0, aspect.d, "刚形成相位，行星与行星距离");
        assert_eq!(p0.name, aspect.p0);
        assert_eq!(p1.name, aspect.p1);

        let aspect = p1.has_aspect(&p0, false);
        assert!(aspect.is_some());
        let aspect = aspect.unwrap();
        assert_eq!(
            aspect_value, aspect.aspect_value,
            "刚形成相位，行星{aspect_value}度相位行星"
        );
        assert_eq!(true, aspect.apply, "刚形成相位，行星入相位行星");
        assert_eq!(5.0, aspect.d, "刚形成相位，行星与行星入距离");
        assert_eq!(p1.name, aspect.p0,);
        assert_eq!(p0.name, aspect.p1,);

        // 入相位中
        p1.long = 6.0;
        let aspect = p0.has_aspect(&p1, false);
        assert!(aspect.is_some());
        let aspect = aspect.unwrap();
        assert_eq!(
            aspect_value, aspect.aspect_value,
            "入相位中，行星{aspect_value}度相位行星"
        );
        assert_eq!(
            true, aspect.apply,
            "入相位中，行星{aspect_value}度入相位行星"
        );
        assert_eq!(4.0, aspect.d, "入相位中，行星与行星距离");
        assert_eq!(p0.name, aspect.p0);
        assert_eq!(p1.name, aspect.p1);

        let aspect = p1.has_aspect(&p0, false);
        assert!(aspect.is_some());
        let aspect = aspect.unwrap();
        assert_eq!(
            aspect_value, aspect.aspect_value,
            "入相位中，行星{aspect_value}度相位行星"
        );
        assert_eq!(
            true, aspect.apply,
            "入相位中，行星{aspect_value}度入相位行星"
        );
        assert_eq!(4.0, aspect.d, "入相位中，行星与行星入距离");
        assert_eq!(p1.name, aspect.p0);
        assert_eq!(p0.name, aspect.p1);

        // 形成相位
        p1.long = 10.0;
        let aspect = p0.has_aspect(&p1, false);
        assert!(aspect.is_some());
        let aspect = aspect.unwrap();
        assert_eq!(
            aspect_value, aspect.aspect_value,
            "形成相位，行星{aspect_value}度相位行星"
        );
        assert_eq!(
            false, aspect.apply,
            "形成相位，行星{aspect_value}度入相位行星"
        );
        assert_eq!(0.0, aspect.d, "形成相位，行星与行星距离");
        assert_eq!(p0.name, aspect.p0);
        assert_eq!(p1.name, aspect.p1);

        let aspect = p1.has_aspect(&p0, false);
        assert!(aspect.is_some());
        let aspect = aspect.unwrap();
        assert_eq!(
            aspect_value, aspect.aspect_value,
            "形成相位，行星{aspect_value}度相位行星"
        );
        assert_eq!(
            false, aspect.apply,
            "形成相位，行星{aspect_value}度入相位行星"
        );
        assert_eq!(0.0, aspect.d, "形成相位，行星与行星入距离");
        assert_eq!(p1.name, aspect.p0);
        assert_eq!(p0.name, aspect.p1);

        // 离相位中
        p1.long = 11.0;
        let aspect = p0.has_aspect(&p1, false);
        assert!(aspect.is_some());
        let aspect = aspect.unwrap();
        assert_eq!(
            aspect_value, aspect.aspect_value,
            "离相位中，行星{aspect_value}度相位行星"
        );
        assert_eq!(
            false, aspect.apply,
            "离相位中，行星{aspect_value}度离相位行星"
        );
        assert_eq!(1.0, aspect.d, "离相位中，行星与行星距离");
        assert_eq!(p0.name, aspect.p0);
        assert_eq!(p1.name, aspect.p1);

        let aspect = p1.has_aspect(&p0, false);
        assert!(aspect.is_some());
        let aspect = aspect.unwrap();
        assert_eq!(
            aspect_value, aspect.aspect_value,
            "离相位中，行星{aspect_value}度相位行星"
        );
        assert_eq!(
            false, aspect.apply,
            "离相位中，行星{aspect_value}度离相位行星"
        );
        assert_eq!(1.0, aspect.d, "离相位中，行星与行星入距离");
        assert_eq!(p1.name, aspect.p0);
        assert_eq!(p0.name, aspect.p1);

        // 即将无相位
        p1.long = 15.0;
        let aspect = p0.has_aspect(&p1, false);
        assert!(aspect.is_some());
        let aspect = aspect.unwrap();
        assert_eq!(
            aspect_value, aspect.aspect_value,
            "离相位中，行星{aspect_value}度相位行星"
        );
        assert_eq!(
            false, aspect.apply,
            "离相位中，行星{aspect_value}度离相位行星"
        );
        assert_eq!(5.0, aspect.d, "离相位中，行星与行星距离");
        assert_eq!(p0.name, aspect.p0);
        assert_eq!(p1.name, aspect.p1);

        let aspect = p1.has_aspect(&p0, false);
        assert!(aspect.is_some());
        let aspect = aspect.unwrap();
        assert_eq!(
            aspect_value, aspect.aspect_value,
            "离相位中，行星{aspect_value}度相位行星"
        );
        assert_eq!(
            false, aspect.apply,
            "离相位中，行星{aspect_value}离相位行星"
        );
        assert_eq!(5.0, aspect.d, "离相位中，行星与行星距离");
        assert_eq!(p1.name, aspect.p0);
        assert_eq!(p0.name, aspect.p1);

        // 已经无相位
        p1.long = 16.0;
        assert!(
            p0.has_aspect(&p1, false).is_none(),
            "已经无相位，行星{aspect_value}度相位行星"
        );
        assert!(
            p1.has_aspect(&p0, false).is_none(),
            "已经无相位，行星{aspect_value}度相位行星"
        );
    }
}

// 行星与行星相位，行星容许度不为0，二星顺行，快行星跨星座左/右入相位
#[parameterized(aspect_value = {
    0, 60, 90, 120, 180}
)]
fn planet_sinister_or_dexter_planet_plant_orb_is_not_0_both_forward_cross_sign_apply(
    aspect_value: u8,
) {
    // 相位点在白羊座2度

    // sun 在-0, -60, -90, -120, -180
    // -0, -180不存在右相位，但测试中仍保留，这两种相位的测试与左相位是相同的
    // swe_degnorm(360.0 - f64::from(aspect_value) + 10.0)
    let p0_longs = [
        2.0 + f64::from(aspect_value),
        swe_degnorm(360.0 - f64::from(aspect_value) + 2.0),
    ];

    let p0_longs = if aspect_value == 0 || aspect_value == 180 {
        &p0_longs[..1]
    } else {
        &p0_longs
    };

    for p0_long in p0_longs {
        // 没有相位
        let p0 = Planet::new(
            Sun,
            *p0_long,
            1.0,
            1.0,
            1.0,
            1.0,
            &PlanetConfig::new(Sun, 6, 1.0, 2.0),
        );
        let mut p1 = Planet::new(
            Moon,
            356.0,
            1.0,
            2.0,
            1.0,
            1.0,
            &PlanetConfig::new(Moon, 4, 1.0, 2.0),
        );
        let aspect = p0.has_aspect(&p1, false);
        assert!(aspect.is_none(), "还没入相位，行星{aspect_value}度相位行星");
        let aspect = p1.has_aspect(&p0, false);
        assert!(aspect.is_none(), "还没入相位，行星{aspect_value}度相位asc");

        // 刚入相位
        p1.long = 355.0;
        let aspect = p0.has_aspect(&p1, false);
        assert!(
            aspect.is_none(),
            "无相位，度数上刚形成相位，行星{aspect_value}度相位行星"
        );

        let aspect = p1.has_aspect(&p0, false);
        assert!(
            aspect.is_none(),
            "无相位，度数上刚形成相位，行星{aspect_value}度相位行星"
        );

        // 入相位中
        p1.long = 359.0;
        let aspect = p0.has_aspect(&p1, false);
        assert!(
            aspect.is_none(),
            "无相位，度数上入相位中，行星{aspect_value}度相位行星"
        );

        let aspect = p1.has_aspect(&p0, false);
        assert!(
            aspect.is_none(),
            "无相位，度数上入相位中，行星{aspect_value}度相位行星"
        );

        // 入相位中，在白羊座0度
        p1.long = 0.0;
        let aspect = p0.has_aspect(&p1, false);
        assert!(aspect.is_some());
        let aspect = aspect.unwrap();
        assert_eq!(
            aspect_value, aspect.aspect_value,
            "入相位中，行星{aspect_value}度相位行星"
        );
        assert_eq!(
            true, aspect.apply,
            "入相位中，行星{aspect_value}度入相位行星"
        );
        assert_eq!(2.0, aspect.d, "形成相位，行星与行星距离");
        assert_eq!(p0.name, aspect.p0);
        assert_eq!(p1.name, aspect.p1);

        let aspect = p1.has_aspect(&p0, false);
        assert!(aspect.is_some());
        let aspect = aspect.unwrap();
        assert_eq!(
            aspect_value, aspect.aspect_value,
            "入相位中，行星{aspect_value}度相位行星"
        );
        assert_eq!(
            true, aspect.apply,
            "形成相位，行星{aspect_value}度入相位行星"
        );
        assert_eq!(2.0, aspect.d, "形成相位，行星与行星入距离");
        assert_eq!(p1.name, aspect.p0);
        assert_eq!(p0.name, aspect.p1);

        // 形成相位
        p1.long = 2.0;
        let aspect = p0.has_aspect(&p1, false);
        assert!(aspect.is_some());
        let aspect = aspect.unwrap();
        assert_eq!(
            aspect_value, aspect.aspect_value,
            "形成相位，行星{aspect_value}度相位行星"
        );
        assert_eq!(
            false, aspect.apply,
            "形成相位，行星{aspect_value}度入相位行星"
        );
        assert_eq!(0.0, aspect.d, "形成相位，行星与行星距离");
        assert_eq!(p0.name, aspect.p0);
        assert_eq!(p1.name, aspect.p1);

        let aspect = p1.has_aspect(&p0, false);
        assert!(aspect.is_some());
        let aspect = aspect.unwrap();
        assert_eq!(
            aspect_value, aspect.aspect_value,
            "形成相位，行星{aspect_value}度相位行星"
        );
        assert_eq!(
            false, aspect.apply,
            "形成相位，行星{aspect_value}度入相位行星"
        );
        assert_eq!(0.0, aspect.d, "形成相位，行星与行星入距离");
        assert_eq!(p1.name, aspect.p0);
        assert_eq!(p0.name, aspect.p1);

        // 离相位中
        p1.long = 3.0;
        let aspect = p0.has_aspect(&p1, false);
        assert!(aspect.is_some());
        let aspect = aspect.unwrap();
        assert_eq!(
            aspect_value, aspect.aspect_value,
            "离相位中，行星{aspect_value}度相位行星"
        );
        assert_eq!(
            false, aspect.apply,
            "离相位中，行星{aspect_value}度离相位行星"
        );
        assert_eq!(1.0, aspect.d, "离相位中，行星与行星距离");
        assert_eq!(p0.name, aspect.p0);
        assert_eq!(p1.name, aspect.p1);

        let aspect = p1.has_aspect(&p0, false);
        assert!(aspect.is_some());
        let aspect = aspect.unwrap();
        assert_eq!(
            aspect_value, aspect.aspect_value,
            "离相位中，行星{aspect_value}度相位行星"
        );
        assert_eq!(
            false, aspect.apply,
            "离相位中，行星{aspect_value}度离相位行星"
        );
        assert_eq!(1.0, aspect.d, "离相位中，行星与行星入距离");
        assert_eq!(p1.name, aspect.p0);
        assert_eq!(p0.name, aspect.p1);

        // 即将无相位
        p1.long = 7.0;
        let aspect = p0.has_aspect(&p1, false);
        assert!(aspect.is_some());
        let aspect = aspect.unwrap();
        assert_eq!(
            aspect_value, aspect.aspect_value,
            "离相位中，行星{aspect_value}度相位行星"
        );
        assert_eq!(
            false, aspect.apply,
            "离相位中，行星{aspect_value}度离相位行星"
        );
        assert_eq!(5.0, aspect.d, "离相位中，行星与行星距离");
        assert_eq!(p0.name, aspect.p0);
        assert_eq!(p1.name, aspect.p1);

        let aspect = p1.has_aspect(&p0, false);
        assert!(aspect.is_some());
        let aspect = aspect.unwrap();
        assert_eq!(
            aspect_value, aspect.aspect_value,
            "离相位中，行星{aspect_value}度相位行星"
        );
        assert_eq!(
            false, aspect.apply,
            "离相位中，行星{aspect_value}离相位行星"
        );
        assert_eq!(5.0, aspect.d, "离相位中，行星与行星距离");
        assert_eq!(p1.name, aspect.p0);
        assert_eq!(p0.name, aspect.p1);

        // 已经无相位
        p1.long = 8.0;
        assert!(
            p0.has_aspect(&p1, false).is_none(),
            "已经无相位，行星{aspect_value}度相位行星"
        );
        assert!(
            p1.has_aspect(&p0, false).is_none(),
            "已经无相位，行星{aspect_value}度相位行星"
        );
    }
}

// 行星与行星相位，行星容许度不为0，二星顺行，快行星跨星左/右离相位
#[parameterized(aspect_value = {
    0, 60, 90, 120, 180}
)]
fn planet_sinister_or_dexter_planet_plant_orb_is_not_0_both_forward_cross_sign_separated(
    aspect_value: u8,
) {
    // 相位点在白羊座28度

    // sun 在-0, -60, -90, -120, -180
    // -0, -180不存在右相位，但测试中仍保留，这两种相位的测试与左相位是相同的
    // swe_degnorm(360.0 - f64::from(aspect_value) + 10.0)
    let p0_longs = [
        28.0 + f64::from(aspect_value),
        swe_degnorm(360.0 - f64::from(aspect_value) + 28.0),
    ];

    let p0_longs = if aspect_value == 0 || aspect_value == 180 {
        &p0_longs[..1]
    } else {
        &p0_longs
    };

    for p0_long in p0_longs {
        // 没有相位
        let p0 = Planet::new(
            Sun,
            *p0_long,
            1.0,
            1.0,
            1.0,
            1.0,
            &PlanetConfig::new(Sun, 6, 1.0, 2.0),
        );
        let mut p1 = Planet::new(
            Moon,
            22.0,
            1.0,
            2.0,
            1.0,
            1.0,
            &PlanetConfig::new(Moon, 4, 1.0, 2.0),
        );
        let aspect = p0.has_aspect(&p1, false);
        assert!(aspect.is_none(), "还没入相位，行星{aspect_value}度相位行星");
        let aspect = p1.has_aspect(&p0, false);
        assert!(aspect.is_none(), "还没入相位，行星{aspect_value}度相位asc");

        // 刚入相位
        p1.long = 23.0;
        let aspect = p0.has_aspect(&p1, false);
        assert!(aspect.is_some());
        let aspect = aspect.unwrap();
        assert_eq!(
            aspect_value, aspect.aspect_value,
            "刚入相位，行星{aspect_value}度相位行星"
        );
        assert_eq!(true, aspect.apply, "刚入相位，行星入相位行星");
        assert_eq!(5.0, aspect.d, "行星与行星距离");
        assert_eq!(p0.name, aspect.p0);
        assert_eq!(p1.name, aspect.p1);

        let aspect = p1.has_aspect(&p0, false);
        assert!(aspect.is_some());
        let aspect = aspect.unwrap();
        assert_eq!(
            aspect_value, aspect.aspect_value,
            "刚入相位，行星{aspect_value}度相位行星"
        );
        assert_eq!(true, aspect.apply, "刚入相位，行星入相位行星");
        assert_eq!(5.0, aspect.d, "行星与行星距离");
        assert_eq!(p1.name, aspect.p0);
        assert_eq!(p0.name, aspect.p1);

        // 入相位中
        p1.long = 24.0;
        let aspect = p0.has_aspect(&p1, false);
        assert!(aspect.is_some());
        let aspect = aspect.unwrap();
        assert_eq!(
            aspect_value, aspect.aspect_value,
            "入相位中，行星{aspect_value}度相位行星"
        );
        assert_eq!(true, aspect.apply, "入相位中，行星入相位行星");
        assert_eq!(4.0, aspect.d, "行星与行星距离");
        assert_eq!(p0.name, aspect.p0);
        assert_eq!(p1.name, aspect.p1);

        let aspect = p1.has_aspect(&p0, false);
        assert!(aspect.is_some());
        let aspect = aspect.unwrap();
        assert_eq!(
            aspect_value, aspect.aspect_value,
            "入相位中，行星{aspect_value}度相位行星"
        );
        assert_eq!(true, aspect.apply, "入相位中，行星入相位行星");
        assert_eq!(4.0, aspect.d, "行星与行星距离");
        assert_eq!(p1.name, aspect.p0);
        assert_eq!(p0.name, aspect.p1);

        // 形成相位
        p1.long = 28.0;
        let aspect = p0.has_aspect(&p1, false);
        assert!(aspect.is_some());
        let aspect = aspect.unwrap();
        assert_eq!(
            aspect_value, aspect.aspect_value,
            "形成相位，行星{aspect_value}度相位行星"
        );
        assert_eq!(
            false, aspect.apply,
            "形成相位，行星{aspect_value}度入相位行星"
        );
        assert_eq!(0.0, aspect.d, "形成相位，行星与行星距离");
        assert_eq!(p0.name, aspect.p0);
        assert_eq!(p1.name, aspect.p1);

        let aspect = p1.has_aspect(&p0, false);
        assert!(aspect.is_some());
        let aspect = aspect.unwrap();
        assert_eq!(
            aspect_value, aspect.aspect_value,
            "形成相位，行星{aspect_value}度相位行星"
        );
        assert_eq!(
            false, aspect.apply,
            "形成相位，行星{aspect_value}度入相位行星"
        );
        assert_eq!(0.0, aspect.d, "形成相位，行星与行星入距离");
        assert_eq!(p1.name, aspect.p0);
        assert_eq!(p0.name, aspect.p1);

        // 离相位中
        p1.long = 29.0;
        let aspect = p0.has_aspect(&p1, false);
        assert!(aspect.is_some());
        let aspect = aspect.unwrap();
        assert_eq!(
            aspect_value, aspect.aspect_value,
            "离相位中，行星{aspect_value}度相位行星"
        );
        assert_eq!(
            false, aspect.apply,
            "离相位中，行星{aspect_value}度离相位行星"
        );
        assert_eq!(1.0, aspect.d, "离相位中，行星与行星距离");
        assert_eq!(p0.name, aspect.p0);
        assert_eq!(p1.name, aspect.p1);

        let aspect = p1.has_aspect(&p0, false);
        assert!(aspect.is_some());
        let aspect = aspect.unwrap();
        assert_eq!(
            aspect_value, aspect.aspect_value,
            "离相位中，行星{aspect_value}度相位行星"
        );
        assert_eq!(
            false, aspect.apply,
            "离相位中，行星{aspect_value}度离相位行星"
        );
        assert_eq!(1.0, aspect.d, "离相位中，行星与行星入距离");
        assert_eq!(p1.name, aspect.p0);
        assert_eq!(p0.name, aspect.p1);

        // 离相位中，在金牛座0度
        p1.long = 30.0;
        let aspect = p0.has_aspect(&p1, false);
        assert!(
            aspect.is_none(),
            "无相位，度数上离相位中，行星{aspect_value}度相位行星"
        );

        let aspect = p1.has_aspect(&p0, false);
        assert!(
            aspect.is_none(),
            "无相位，度数上离相位中，行星{aspect_value}度相位行星"
        );

        // 即将无相位
        p1.long = 33.0;
        assert!(
            aspect.is_none(),
            "无相位，度数上离相位中，行星{aspect_value}度相位行星"
        );

        let aspect = p1.has_aspect(&p0, false);
        assert!(
            aspect.is_none(),
            "无相位，度数离相位中，行星{aspect_value}度相位行星"
        );

        // 已经无相位
        p1.long = 34.0;
        assert!(
            p0.has_aspect(&p1, false).is_none(),
            "已经无相位，行星{aspect_value}度相位行星"
        );
        assert!(
            p1.has_aspect(&p0, false).is_none(),
            "已经无相位，行星{aspect_value}度相位行星"
        );
    }
}

// 行星与行星度相位，行星容许度不为0，二星逆行，快行星左/右相位
#[parameterized(aspect_value = {
    0, 60, 90, 120, 180}
)]
fn planet_sinister_or_dexter_planet_plant_orb_is_not_0_both_retrograde(aspect_value: u8) {
    // 相位点在白羊座10度

    // sun 在-0, -60, -90, -120, -180
    // -0, -180不存在右相位，但测试中仍保留，这两种相位的测试与左相位是相同的
    // swe_degnorm(360.0 - f64::from(aspect_value) + 10.0)
    let p0_longs = [
        10.0 + f64::from(aspect_value),
        swe_degnorm(360.0 - f64::from(aspect_value) + 10.0),
    ];

    let p0_longs = if aspect_value == 0 || aspect_value == 180 {
        &p0_longs[..1]
    } else {
        &p0_longs
    };

    for p0_long in p0_longs {
        // 没有相位
        let p0 = Planet::new(
            Sun,
            *p0_long,
            1.0,
            1.0,
            -1.0,
            1.0,
            &PlanetConfig::new(Sun, 6, 1.0, 2.0),
        );
        let mut p1 = Planet::new(
            Moon,
            16.0,
            1.0,
            -2.0,
            1.0,
            1.0,
            &PlanetConfig::new(Moon, 4, 1.0, 2.0),
        );
        let aspect = p0.has_aspect(&p1, false);
        assert!(aspect.is_none(), "还没入相位，行星{aspect_value}度相位行星");
        let aspect = p1.has_aspect(&p0, false);
        assert!(aspect.is_none(), "还没入相位，行星{aspect_value}度相位asc");

        // 刚入相位
        p1.long = 15.0;
        let aspect = p0.has_aspect(&p1, false);
        assert!(aspect.is_some());
        let aspect = aspect.unwrap();
        assert_eq!(
            aspect_value, aspect.aspect_value,
            "刚形成相位，行星{aspect_value}度相位行星"
        );
        assert_eq!(true, aspect.apply, "刚形成相位，行星入相位行星");
        assert_eq!(5.0, aspect.d, "刚形成相位，行星与行星距离");
        assert_eq!(p0.name, aspect.p0);
        assert_eq!(p1.name, aspect.p1);

        let aspect = p1.has_aspect(&p0, false);
        assert!(aspect.is_some());
        let aspect = aspect.unwrap();
        assert_eq!(
            aspect_value, aspect.aspect_value,
            "刚形成相位，行星{aspect_value}度相位行星"
        );
        assert_eq!(true, aspect.apply, "刚形成相位，行星入相位行星");
        assert_eq!(5.0, aspect.d, "刚形成相位，行星与行星入距离");
        assert_eq!(p1.name, aspect.p0,);
        assert_eq!(p0.name, aspect.p1,);

        // 入相位中
        p1.long = 14.0;
        let aspect = p0.has_aspect(&p1, false);
        assert!(aspect.is_some());
        let aspect = aspect.unwrap();
        assert_eq!(
            aspect_value, aspect.aspect_value,
            "入相位中，行星{aspect_value}度相位行星"
        );
        assert_eq!(
            true, aspect.apply,
            "入相位中，行星{aspect_value}度入相位行星"
        );
        assert_eq!(4.0, aspect.d, "入相位中，行星与行星距离");
        assert_eq!(p0.name, aspect.p0);
        assert_eq!(p1.name, aspect.p1);

        let aspect = p1.has_aspect(&p0, false);
        assert!(aspect.is_some());
        let aspect = aspect.unwrap();
        assert_eq!(
            aspect_value, aspect.aspect_value,
            "入相位中，行星{aspect_value}度相位行星"
        );
        assert_eq!(
            true, aspect.apply,
            "入相位中，行星{aspect_value}度入相位行星"
        );
        assert_eq!(4.0, aspect.d, "入相位中，行星与行星入距离");
        assert_eq!(p1.name, aspect.p0);
        assert_eq!(p0.name, aspect.p1);

        // 形成相位
        p1.long = 10.0;
        let aspect = p0.has_aspect(&p1, false);
        assert!(aspect.is_some());
        let aspect = aspect.unwrap();
        assert_eq!(
            aspect_value, aspect.aspect_value,
            "形成相位，行星{aspect_value}度相位行星"
        );
        assert_eq!(
            false, aspect.apply,
            "形成相位，行星{aspect_value}度入相位行星"
        );
        assert_eq!(0.0, aspect.d, "形成相位，行星与行星距离");
        assert_eq!(p0.name, aspect.p0);
        assert_eq!(p1.name, aspect.p1);

        let aspect = p1.has_aspect(&p0, false);
        assert!(aspect.is_some());
        let aspect = aspect.unwrap();
        assert_eq!(
            aspect_value, aspect.aspect_value,
            "形成相位，行星{aspect_value}度相位行星"
        );
        assert_eq!(
            false, aspect.apply,
            "形成相位，行星{aspect_value}度入相位行星"
        );
        assert_eq!(0.0, aspect.d, "形成相位，行星与行星入距离");
        assert_eq!(p1.name, aspect.p0);
        assert_eq!(p0.name, aspect.p1);

        // 离相位中
        p1.long = 9.0;
        let aspect = p0.has_aspect(&p1, false);
        assert!(aspect.is_some());
        let aspect = aspect.unwrap();
        assert_eq!(
            aspect_value, aspect.aspect_value,
            "离相位中，行星{aspect_value}度相位行星"
        );
        assert_eq!(
            false, aspect.apply,
            "离相位中，行星{aspect_value}度离相位行星"
        );
        assert_eq!(1.0, aspect.d, "离相位中，行星与行星距离");
        assert_eq!(p0.name, aspect.p0);
        assert_eq!(p1.name, aspect.p1);

        let aspect = p1.has_aspect(&p0, false);
        assert!(aspect.is_some());
        let aspect = aspect.unwrap();
        assert_eq!(
            aspect_value, aspect.aspect_value,
            "离相位中，行星{aspect_value}度相位行星"
        );
        assert_eq!(
            false, aspect.apply,
            "离相位中，行星{aspect_value}度离相位行星"
        );
        assert_eq!(1.0, aspect.d, "离相位中，行星与行星入距离");
        assert_eq!(p1.name, aspect.p0);
        assert_eq!(p0.name, aspect.p1);

        // 即将无相位
        p1.long = 5.0;
        let aspect = p0.has_aspect(&p1, false);
        assert!(aspect.is_some());
        let aspect = aspect.unwrap();
        assert_eq!(
            aspect_value, aspect.aspect_value,
            "离相位中，行星{aspect_value}度相位行星"
        );
        assert_eq!(
            false, aspect.apply,
            "离相位中，行星{aspect_value}度离相位行星"
        );
        assert_eq!(5.0, aspect.d, "离相位中，行星与行星距离");
        assert_eq!(p0.name, aspect.p0);
        assert_eq!(p1.name, aspect.p1);

        let aspect = p1.has_aspect(&p0, false);
        assert!(aspect.is_some());
        let aspect = aspect.unwrap();
        assert_eq!(
            aspect_value, aspect.aspect_value,
            "离相位中，行星{aspect_value}度相位行星"
        );
        assert_eq!(
            false, aspect.apply,
            "离相位中，行星{aspect_value}离相位行星"
        );
        assert_eq!(5.0, aspect.d, "离相位中，行星与行星距离");
        assert_eq!(p1.name, aspect.p0);
        assert_eq!(p0.name, aspect.p1);

        // 已经无相位
        p1.long = 4.0;
        assert!(
            p0.has_aspect(&p1, false).is_none(),
            "已经无相位，行星{aspect_value}度相位行星"
        );
        assert!(
            p1.has_aspect(&p0, false).is_none(),
            "已经无相位，行星{aspect_value}度相位行星"
        );
    }
}

// 行星与行星相位，行星容许度不为0，二星逆行，快行星跨星左/右入相位
#[parameterized(aspect_value = {
    0, 60, 90, 120, 180}
)]
fn planet_sinister_or_dexter_planet_plant_orb_is_not_0_both_retrograde_cross_sign_apply(
    aspect_value: u8,
) {
    // 相位点在白羊座2度

    // sun 在-0, -60, -90, -120, -180
    // -0, -180不存在右相位，但测试中仍保留，这两种相位的测试与左相位是相同的
    // swe_degnorm(360.0 - f64::from(aspect_value) + 10.0)
    let p0_longs = [
        28.0 + f64::from(aspect_value),
        swe_degnorm(360.0 - f64::from(aspect_value) + 28.0),
    ];

    let p0_longs = if aspect_value == 0 || aspect_value == 180 {
        &p0_longs[..1]
    } else {
        &p0_longs
    };

    for p0_long in p0_longs {
        // 没有相位
        let p0 = Planet::new(
            Sun,
            *p0_long,
            1.0,
            -1.0,
            1.0,
            1.0,
            &PlanetConfig::new(Sun, 6, 1.0, 2.0),
        );
        let mut p1 = Planet::new(
            Moon,
            34.0,
            1.0,
            -2.0,
            1.0,
            1.0,
            &PlanetConfig::new(Moon, 4, 1.0, 2.0),
        );
        let aspect = p0.has_aspect(&p1, false);
        assert!(aspect.is_none(), "还没入相位，行星{aspect_value}度相位行星");
        let aspect = p1.has_aspect(&p0, false);
        assert!(aspect.is_none(), "还没入相位，行星{aspect_value}度相位asc");

        // 刚入相位
        p1.long = 33.0;
        let aspect = p0.has_aspect(&p1, false);
        assert!(
            aspect.is_none(),
            "无相位，度数上刚形成相位，行星{aspect_value}度相位行星"
        );

        let aspect = p1.has_aspect(&p0, false);
        assert!(
            aspect.is_none(),
            "无相位，度数上刚形成相位，行星{aspect_value}度相位行星"
        );

        // 入相位中
        p1.long = 32.0;
        let aspect = p0.has_aspect(&p1, false);
        assert!(
            aspect.is_none(),
            "无相位，度数上入相位中，行星{aspect_value}度相位行星"
        );

        let aspect = p1.has_aspect(&p0, false);
        assert!(
            aspect.is_none(),
            "无相位，度数上入相位中，行星{aspect_value}度相位行星"
        );

        // 入相位中，在金牛座0度
        p1.long = 30.0;
        let aspect = p0.has_aspect(&p1, false);
        assert!(
            aspect.is_none(),
            "无相位，度数上入相位中，行星{aspect_value}度相位行星"
        );

        let aspect = p1.has_aspect(&p0, false);
        assert!(
            aspect.is_none(),
            "无相位，度数上入相位中，行星{aspect_value}度相位行星"
        );

        // 入相位中
        p1.long = 29.0;
        let aspect = p0.has_aspect(&p1, false);
        assert!(aspect.is_some());
        let aspect = aspect.unwrap();
        assert_eq!(
            aspect_value, aspect.aspect_value,
            "入相位中，行星{aspect_value}度相位行星"
        );
        assert_eq!(
            true, aspect.apply,
            "入相位中，行星{aspect_value}度入相位行星"
        );
        assert_eq!(1.0, aspect.d, "形成相位，行星与行星距离");
        assert_eq!(p0.name, aspect.p0);
        assert_eq!(p1.name, aspect.p1);

        let aspect = p1.has_aspect(&p0, false);
        assert!(aspect.is_some());
        let aspect = aspect.unwrap();
        assert_eq!(
            aspect_value, aspect.aspect_value,
            "入相位中，行星{aspect_value}度相位行星"
        );
        assert_eq!(
            true, aspect.apply,
            "形成相位，行星{aspect_value}度入相位行星"
        );
        assert_eq!(1.0, aspect.d, "形成相位，行星与行星入距离");
        assert_eq!(p1.name, aspect.p0);
        assert_eq!(p0.name, aspect.p1);

        // 形成相位
        p1.long = 28.0;
        let aspect = p0.has_aspect(&p1, false);
        assert!(aspect.is_some());
        let aspect = aspect.unwrap();
        assert_eq!(
            aspect_value, aspect.aspect_value,
            "形成相位，行星{aspect_value}度相位行星"
        );
        assert_eq!(
            false, aspect.apply,
            "形成相位，行星{aspect_value}度入相位行星"
        );
        assert_eq!(0.0, aspect.d, "形成相位，行星与行星距离");
        assert_eq!(p0.name, aspect.p0);
        assert_eq!(p1.name, aspect.p1);

        let aspect = p1.has_aspect(&p0, false);
        assert!(aspect.is_some());
        let aspect = aspect.unwrap();
        assert_eq!(
            aspect_value, aspect.aspect_value,
            "形成相位，行星{aspect_value}度相位行星"
        );
        assert_eq!(
            false, aspect.apply,
            "形成相位，行星{aspect_value}度入相位行星"
        );
        assert_eq!(0.0, aspect.d, "形成相位，行星与行星入距离");
        assert_eq!(p1.name, aspect.p0);
        assert_eq!(p0.name, aspect.p1);

        // 离相位中
        p1.long = 27.0;
        let aspect = p0.has_aspect(&p1, false);
        assert!(aspect.is_some());
        let aspect = aspect.unwrap();
        assert_eq!(
            aspect_value, aspect.aspect_value,
            "离相位中，行星{aspect_value}度相位行星"
        );
        assert_eq!(
            false, aspect.apply,
            "离相位中，行星{aspect_value}度离相位行星"
        );
        assert_eq!(1.0, aspect.d, "离相位中，行星与行星距离");
        assert_eq!(p0.name, aspect.p0);
        assert_eq!(p1.name, aspect.p1);

        let aspect = p1.has_aspect(&p0, false);
        assert!(aspect.is_some());
        let aspect = aspect.unwrap();
        assert_eq!(
            aspect_value, aspect.aspect_value,
            "离相位中，行星{aspect_value}度相位行星"
        );
        assert_eq!(
            false, aspect.apply,
            "离相位中，行星{aspect_value}度离相位行星"
        );
        assert_eq!(1.0, aspect.d, "离相位中，行星与行星入距离");
        assert_eq!(p1.name, aspect.p0);
        assert_eq!(p0.name, aspect.p1);

        // 即将无相位
        p1.long = 23.0;
        let aspect = p0.has_aspect(&p1, false);
        assert!(aspect.is_some());
        let aspect = aspect.unwrap();
        assert_eq!(
            aspect_value, aspect.aspect_value,
            "离相位中，行星{aspect_value}度相位行星"
        );
        assert_eq!(
            false, aspect.apply,
            "离相位中，行星{aspect_value}度离相位行星"
        );
        assert_eq!(5.0, aspect.d, "离相位中，行星与行星距离");
        assert_eq!(p0.name, aspect.p0);
        assert_eq!(p1.name, aspect.p1);

        let aspect = p1.has_aspect(&p0, false);
        assert!(aspect.is_some());
        let aspect = aspect.unwrap();
        assert_eq!(
            aspect_value, aspect.aspect_value,
            "离相位中，行星{aspect_value}度相位行星"
        );
        assert_eq!(
            false, aspect.apply,
            "离相位中，行星{aspect_value}离相位行星"
        );
        assert_eq!(5.0, aspect.d, "离相位中，行星与行星距离");
        assert_eq!(p1.name, aspect.p0);
        assert_eq!(p0.name, aspect.p1);

        // 已经无相位
        p1.long = 22.0;
        assert!(
            p0.has_aspect(&p1, false).is_none(),
            "已经无相位，行星{aspect_value}度相位行星"
        );
        assert!(
            p1.has_aspect(&p0, false).is_none(),
            "已经无相位，行星{aspect_value}度相位行星"
        );
    }
}

// 行星与行星相位，行星容许度不为0，二星逆行，快行星跨星左/右离相位
#[parameterized(aspect_value = {
    0, 60, 90, 120, 180}
)]
fn planet_sinister_or_dexter_planet_plant_orb_is_not_0_both_retrograde_cross_sign_separated(
    aspect_value: u8,
) {
    // 相位点在白羊座28度

    // sun 在-0, -60, -90, -120, -180
    // -0, -180不存在右相位，但测试中仍保留，这两种相位的测试与左相位是相同的
    // swe_degnorm(360.0 - f64::from(aspect_value) + 10.0)
    let p0_longs = [
        2.0 + f64::from(aspect_value),
        swe_degnorm(360.0 - f64::from(aspect_value) + 2.0),
    ];

    let p0_longs = if aspect_value == 0 || aspect_value == 180 {
        &p0_longs[..1]
    } else {
        &p0_longs
    };

    for p0_long in p0_longs {
        // 没有相位
        let p0 = Planet::new(
            Sun,
            *p0_long,
            1.0,
            -1.0,
            1.0,
            1.0,
            &PlanetConfig::new(Sun, 6, 1.0, 2.0),
        );
        let mut p1 = Planet::new(
            Moon,
            8.0,
            1.0,
            -2.0,
            1.0,
            1.0,
            &PlanetConfig::new(Moon, 4, 1.0, 2.0),
        );
        let aspect = p0.has_aspect(&p1, false);
        assert!(aspect.is_none(), "还没入相位，行星{aspect_value}度相位行星");
        let aspect = p1.has_aspect(&p0, false);
        assert!(aspect.is_none(), "还没入相位，行星{aspect_value}度相位asc");

        // 刚入相位
        p1.long = 7.0;
        let aspect = p0.has_aspect(&p1, false);
        assert!(aspect.is_some());
        let aspect = aspect.unwrap();
        assert_eq!(
            aspect_value, aspect.aspect_value,
            "刚入相位，行星{aspect_value}度相位行星"
        );
        assert_eq!(true, aspect.apply, "刚入相位，行星入相位行星");
        assert_eq!(5.0, aspect.d, "行星与行星距离");
        assert_eq!(p0.name, aspect.p0);
        assert_eq!(p1.name, aspect.p1);

        let aspect = p1.has_aspect(&p0, false);
        assert!(aspect.is_some());
        let aspect = aspect.unwrap();
        assert_eq!(
            aspect_value, aspect.aspect_value,
            "刚入相位，行星{aspect_value}度相位行星"
        );
        assert_eq!(true, aspect.apply, "刚入相位，行星入相位行星");
        assert_eq!(5.0, aspect.d, "行星与行星距离");
        assert_eq!(p1.name, aspect.p0);
        assert_eq!(p0.name, aspect.p1);

        // 入相位中
        p1.long = 6.0;
        let aspect = p0.has_aspect(&p1, false);
        assert!(aspect.is_some());
        let aspect = aspect.unwrap();
        assert_eq!(
            aspect_value, aspect.aspect_value,
            "入相位中，行星{aspect_value}度相位行星"
        );
        assert_eq!(true, aspect.apply, "入相位中，行星入相位行星");
        assert_eq!(4.0, aspect.d, "行星与行星距离");
        assert_eq!(p0.name, aspect.p0);
        assert_eq!(p1.name, aspect.p1);

        let aspect = p1.has_aspect(&p0, false);
        assert!(aspect.is_some());
        let aspect = aspect.unwrap();
        assert_eq!(
            aspect_value, aspect.aspect_value,
            "入相位中，行星{aspect_value}度相位行星"
        );
        assert_eq!(true, aspect.apply, "入相位中，行星入相位行星");
        assert_eq!(4.0, aspect.d, "行星与行星距离");
        assert_eq!(p1.name, aspect.p0);
        assert_eq!(p0.name, aspect.p1);

        // 形成相位
        p1.long = 2.0;
        let aspect = p0.has_aspect(&p1, false);
        assert!(aspect.is_some());
        let aspect = aspect.unwrap();
        assert_eq!(
            aspect_value, aspect.aspect_value,
            "形成相位，行星{aspect_value}度相位行星"
        );
        assert_eq!(
            false, aspect.apply,
            "形成相位，行星{aspect_value}度入相位行星"
        );
        assert_eq!(0.0, aspect.d, "形成相位，行星与行星距离");
        assert_eq!(p0.name, aspect.p0);
        assert_eq!(p1.name, aspect.p1);

        let aspect = p1.has_aspect(&p0, false);
        assert!(aspect.is_some());
        let aspect = aspect.unwrap();
        assert_eq!(
            aspect_value, aspect.aspect_value,
            "形成相位，行星{aspect_value}度相位行星"
        );
        assert_eq!(
            false, aspect.apply,
            "形成相位，行星{aspect_value}度入相位行星"
        );
        assert_eq!(0.0, aspect.d, "形成相位，行星与行星入距离");
        assert_eq!(p1.name, aspect.p0);
        assert_eq!(p0.name, aspect.p1);

        // 离相位中
        p1.long = 0.0;
        let aspect = p0.has_aspect(&p1, false);
        assert!(aspect.is_some());
        let aspect = aspect.unwrap();
        assert_eq!(
            aspect_value, aspect.aspect_value,
            "离相位中，行星{aspect_value}度相位行星"
        );
        assert_eq!(
            false, aspect.apply,
            "离相位中，行星{aspect_value}度离相位行星"
        );
        assert_eq!(2.0, aspect.d, "离相位中，行星与行星距离");
        assert_eq!(p0.name, aspect.p0);
        assert_eq!(p1.name, aspect.p1);

        let aspect = p1.has_aspect(&p0, false);
        assert!(aspect.is_some());
        let aspect = aspect.unwrap();
        assert_eq!(
            aspect_value, aspect.aspect_value,
            "离相位中，行星{aspect_value}度相位行星"
        );
        assert_eq!(
            false, aspect.apply,
            "离相位中，行星{aspect_value}度离相位行星"
        );
        assert_eq!(2.0, aspect.d, "离相位中，行星与行星入距离");
        assert_eq!(p1.name, aspect.p0);
        assert_eq!(p0.name, aspect.p1);

        // 离相位中，在双鱼座29度
        p1.long = 359.0;
        let aspect = p0.has_aspect(&p1, false);
        assert!(
            aspect.is_none(),
            "无相位，度数上离相位中，行星{aspect_value}度相位行星"
        );

        let aspect = p1.has_aspect(&p0, false);
        assert!(
            aspect.is_none(),
            "无相位，度数上离相位中，行星{aspect_value}度相位行星"
        );

        // 即将无相位
        p1.long = 357.0;
        assert!(
            aspect.is_none(),
            "无相位，度数上离相位中，行星{aspect_value}度相位行星"
        );

        let aspect = p1.has_aspect(&p0, false);
        assert!(
            aspect.is_none(),
            "无相位，度数离相位中，行星{aspect_value}度相位行星"
        );

        // 已经无相位
        p1.long = 356.0;
        assert!(
            p0.has_aspect(&p1, false).is_none(),
            "已经无相位，行星{aspect_value}度相位行星"
        );
        assert!(
            p1.has_aspect(&p0, false).is_none(),
            "已经无相位，行星{aspect_value}度相位行星"
        );
    }
}

// 行星与行星度相位，行星容许度不为0，快行星逆行，慢行星顺行，快行星左/右相位
#[parameterized(aspect_value = {
    0, 60, 90, 120, 180}
)]
fn planet_sinister_or_dexter_planet_plant_orb_is_not_0_faster_retrograde_slower_forward(
    aspect_value: u8,
) {
    // 相位点在白羊座10度

    // sun 在-0, -60, -90, -120, -180
    // -0, -180不存在右相位，但测试中仍保留，这两种相位的测试与左相位是相同的
    // swe_degnorm(360.0 - f64::from(aspect_value) + 10.0)
    let p0_longs = [
        10.0 + f64::from(aspect_value),
        swe_degnorm(360.0 - f64::from(aspect_value) + 10.0),
    ];

    let p0_longs = if aspect_value == 0 || aspect_value == 180 {
        &p0_longs[..1]
    } else {
        &p0_longs
    };

    for p0_long in p0_longs {
        // 没有相位
        let p0 = Planet::new(
            Sun,
            *p0_long,
            1.0,
            1.0,
            1.0,
            1.0,
            &PlanetConfig::new(Sun, 6, 1.0, 2.0),
        );
        let mut p1 = Planet::new(
            Moon,
            16.0,
            1.0,
            -2.0,
            1.0,
            1.0,
            &PlanetConfig::new(Moon, 4, 1.0, 2.0),
        );
        let aspect = p0.has_aspect(&p1, false);
        assert!(aspect.is_none(), "还没入相位，行星{aspect_value}度相位行星");
        let aspect = p1.has_aspect(&p0, false);
        assert!(aspect.is_none(), "还没入相位，行星{aspect_value}度相位asc");

        // 刚入相位
        p1.long = 15.0;
        let aspect = p0.has_aspect(&p1, false);
        assert!(aspect.is_some());
        let aspect = aspect.unwrap();
        assert_eq!(
            aspect_value, aspect.aspect_value,
            "刚形成相位，行星{aspect_value}度相位行星"
        );
        assert_eq!(true, aspect.apply, "刚形成相位，行星入相位行星");
        assert_eq!(5.0, aspect.d, "刚形成相位，行星与行星距离");
        assert_eq!(p0.name, aspect.p0);
        assert_eq!(p1.name, aspect.p1);

        let aspect = p1.has_aspect(&p0, false);
        assert!(aspect.is_some());
        let aspect = aspect.unwrap();
        assert_eq!(
            aspect_value, aspect.aspect_value,
            "刚形成相位，行星{aspect_value}度相位行星"
        );
        assert_eq!(true, aspect.apply, "刚形成相位，行星入相位行星");
        assert_eq!(5.0, aspect.d, "刚形成相位，行星与行星入距离");
        assert_eq!(p1.name, aspect.p0,);
        assert_eq!(p0.name, aspect.p1,);

        // 入相位中
        p1.long = 14.0;
        let aspect = p0.has_aspect(&p1, false);
        assert!(aspect.is_some());
        let aspect = aspect.unwrap();
        assert_eq!(
            aspect_value, aspect.aspect_value,
            "入相位中，行星{aspect_value}度相位行星"
        );
        assert_eq!(
            true, aspect.apply,
            "入相位中，行星{aspect_value}度入相位行星"
        );
        assert_eq!(4.0, aspect.d, "入相位中，行星与行星距离");
        assert_eq!(p0.name, aspect.p0);
        assert_eq!(p1.name, aspect.p1);

        let aspect = p1.has_aspect(&p0, false);
        assert!(aspect.is_some());
        let aspect = aspect.unwrap();
        assert_eq!(
            aspect_value, aspect.aspect_value,
            "入相位中，行星{aspect_value}度相位行星"
        );
        assert_eq!(
            true, aspect.apply,
            "入相位中，行星{aspect_value}度入相位行星"
        );
        assert_eq!(4.0, aspect.d, "入相位中，行星与行星入距离");
        assert_eq!(p1.name, aspect.p0);
        assert_eq!(p0.name, aspect.p1);

        // 形成相位
        p1.long = 10.0;
        let aspect = p0.has_aspect(&p1, false);
        assert!(aspect.is_some());
        let aspect = aspect.unwrap();
        assert_eq!(
            aspect_value, aspect.aspect_value,
            "形成相位，行星{aspect_value}度相位行星"
        );
        assert_eq!(
            false, aspect.apply,
            "形成相位，行星{aspect_value}度入相位行星"
        );
        assert_eq!(0.0, aspect.d, "形成相位，行星与行星距离");
        assert_eq!(p0.name, aspect.p0);
        assert_eq!(p1.name, aspect.p1);

        let aspect = p1.has_aspect(&p0, false);
        assert!(aspect.is_some());
        let aspect = aspect.unwrap();
        assert_eq!(
            aspect_value, aspect.aspect_value,
            "形成相位，行星{aspect_value}度相位行星"
        );
        assert_eq!(
            false, aspect.apply,
            "形成相位，行星{aspect_value}度入相位行星"
        );
        assert_eq!(0.0, aspect.d, "形成相位，行星与行星入距离");
        assert_eq!(p1.name, aspect.p0);
        assert_eq!(p0.name, aspect.p1);

        // 离相位中
        p1.long = 9.0;
        let aspect = p0.has_aspect(&p1, false);
        assert!(aspect.is_some());
        let aspect = aspect.unwrap();
        assert_eq!(
            aspect_value, aspect.aspect_value,
            "离相位中，行星{aspect_value}度相位行星"
        );
        assert_eq!(
            false, aspect.apply,
            "离相位中，行星{aspect_value}度离相位行星"
        );
        assert_eq!(1.0, aspect.d, "离相位中，行星与行星距离");
        assert_eq!(p0.name, aspect.p0);
        assert_eq!(p1.name, aspect.p1);

        let aspect = p1.has_aspect(&p0, false);
        assert!(aspect.is_some());
        let aspect = aspect.unwrap();
        assert_eq!(
            aspect_value, aspect.aspect_value,
            "离相位中，行星{aspect_value}度相位行星"
        );
        assert_eq!(
            false, aspect.apply,
            "离相位中，行星{aspect_value}度离相位行星"
        );
        assert_eq!(1.0, aspect.d, "离相位中，行星与行星入距离");
        assert_eq!(p1.name, aspect.p0);
        assert_eq!(p0.name, aspect.p1);

        // 即将无相位
        p1.long = 5.0;
        let aspect = p0.has_aspect(&p1, false);
        assert!(aspect.is_some());
        let aspect = aspect.unwrap();
        assert_eq!(
            aspect_value, aspect.aspect_value,
            "离相位中，行星{aspect_value}度相位行星"
        );
        assert_eq!(
            false, aspect.apply,
            "离相位中，行星{aspect_value}度离相位行星"
        );
        assert_eq!(5.0, aspect.d, "离相位中，行星与行星距离");
        assert_eq!(p0.name, aspect.p0);
        assert_eq!(p1.name, aspect.p1);

        let aspect = p1.has_aspect(&p0, false);
        assert!(aspect.is_some());
        let aspect = aspect.unwrap();
        assert_eq!(
            aspect_value, aspect.aspect_value,
            "离相位中，行星{aspect_value}度相位行星"
        );
        assert_eq!(
            false, aspect.apply,
            "离相位中，行星{aspect_value}离相位行星"
        );
        assert_eq!(5.0, aspect.d, "离相位中，行星与行星距离");
        assert_eq!(p1.name, aspect.p0);
        assert_eq!(p0.name, aspect.p1);

        // 已经无相位
        p1.long = 4.0;
        assert!(
            p0.has_aspect(&p1, false).is_none(),
            "已经无相位，行星{aspect_value}度相位行星"
        );
        assert!(
            p1.has_aspect(&p0, false).is_none(),
            "已经无相位，行星{aspect_value}度相位行星"
        );
    }
}

// 行星与行星相位，行星容许度不为0，快行星逆行，慢行星顺行，快行星跨星左/右入相位
#[parameterized(aspect_value = {
    0, 60, 90, 120, 180}
)]
fn planet_sinister_or_dexter_planet_plant_orb_is_not_0_faster_retrograde_slower_forward_cross_sign_apply(
    aspect_value: u8,
) {
    // 相位点在白羊座28度

    // sun 在-0, -60, -90, -120, -180
    // -0, -180不存在右相位，但测试中仍保留，这两种相位的测试与左相位是相同的
    // swe_degnorm(360.0 - f64::from(aspect_value) + 10.0)
    let p0_longs = [
        28.0 + f64::from(aspect_value),
        swe_degnorm(360.0 - f64::from(aspect_value) + 28.0),
    ];

    let p0_longs = if aspect_value == 0 || aspect_value == 180 {
        &p0_longs[..1]
    } else {
        &p0_longs
    };

    for p0_long in p0_longs {
        // 没有相位
        let p0 = Planet::new(
            Sun,
            *p0_long,
            1.0,
            1.0,
            1.0,
            1.0,
            &PlanetConfig::new(Sun, 6, 1.0, 2.0),
        );
        let mut p1 = Planet::new(
            Moon,
            34.0,
            1.0,
            -2.0,
            1.0,
            1.0,
            &PlanetConfig::new(Moon, 4, 1.0, 2.0),
        );
        let aspect = p0.has_aspect(&p1, false);
        assert!(aspect.is_none(), "还没入相位，行星{aspect_value}度相位行星");
        let aspect = p1.has_aspect(&p0, false);
        assert!(aspect.is_none(), "还没入相位，行星{aspect_value}度相位asc");

        // 刚入相位
        p1.long = 33.0;
        let aspect = p0.has_aspect(&p1, false);
        assert!(
            aspect.is_none(),
            "无相位，度数上刚形成相位，行星{aspect_value}度相位行星"
        );

        let aspect = p1.has_aspect(&p0, false);
        assert!(
            aspect.is_none(),
            "无相位，度数上刚形成相位，行星{aspect_value}度相位行星"
        );

        // 入相位中
        p1.long = 32.0;
        let aspect = p0.has_aspect(&p1, false);
        assert!(
            aspect.is_none(),
            "无相位，度数上入相位中，行星{aspect_value}度相位行星"
        );

        let aspect = p1.has_aspect(&p0, false);
        assert!(
            aspect.is_none(),
            "无相位，度数上入相位中，行星{aspect_value}度相位行星"
        );

        // 入相位中，在金牛座0度
        p1.long = 30.0;
        let aspect = p0.has_aspect(&p1, false);
        assert!(
            aspect.is_none(),
            "无相位，度数上入相位中，行星{aspect_value}度相位行星"
        );

        let aspect = p1.has_aspect(&p0, false);
        assert!(
            aspect.is_none(),
            "无相位，度数上入相位中，行星{aspect_value}度相位行星"
        );

        // 入相位中
        p1.long = 29.0;
        let aspect = p0.has_aspect(&p1, false);
        assert!(aspect.is_some());
        let aspect = aspect.unwrap();
        assert_eq!(
            aspect_value, aspect.aspect_value,
            "入相位中，行星{aspect_value}度相位行星"
        );
        assert_eq!(
            true, aspect.apply,
            "入相位中，行星{aspect_value}度入相位行星"
        );
        assert_eq!(1.0, aspect.d, "形成相位，行星与行星距离");
        assert_eq!(p0.name, aspect.p0);
        assert_eq!(p1.name, aspect.p1);

        let aspect = p1.has_aspect(&p0, false);
        assert!(aspect.is_some());
        let aspect = aspect.unwrap();
        assert_eq!(
            aspect_value, aspect.aspect_value,
            "入相位中，行星{aspect_value}度相位行星"
        );
        assert_eq!(
            true, aspect.apply,
            "形成相位，行星{aspect_value}度入相位行星"
        );
        assert_eq!(1.0, aspect.d, "形成相位，行星与行星入距离");
        assert_eq!(p1.name, aspect.p0);
        assert_eq!(p0.name, aspect.p1);

        // 形成相位
        p1.long = 28.0;
        let aspect = p0.has_aspect(&p1, false);
        assert!(aspect.is_some());
        let aspect = aspect.unwrap();
        assert_eq!(
            aspect_value, aspect.aspect_value,
            "形成相位，行星{aspect_value}度相位行星"
        );
        assert_eq!(
            false, aspect.apply,
            "形成相位，行星{aspect_value}度入相位行星"
        );
        assert_eq!(0.0, aspect.d, "形成相位，行星与行星距离");
        assert_eq!(p0.name, aspect.p0);
        assert_eq!(p1.name, aspect.p1);

        let aspect = p1.has_aspect(&p0, false);
        assert!(aspect.is_some());
        let aspect = aspect.unwrap();
        assert_eq!(
            aspect_value, aspect.aspect_value,
            "形成相位，行星{aspect_value}度相位行星"
        );
        assert_eq!(
            false, aspect.apply,
            "形成相位，行星{aspect_value}度入相位行星"
        );
        assert_eq!(0.0, aspect.d, "形成相位，行星与行星入距离");
        assert_eq!(p1.name, aspect.p0);
        assert_eq!(p0.name, aspect.p1);

        // 离相位中
        p1.long = 27.0;
        let aspect = p0.has_aspect(&p1, false);
        assert!(aspect.is_some());
        let aspect = aspect.unwrap();
        assert_eq!(
            aspect_value, aspect.aspect_value,
            "离相位中，行星{aspect_value}度相位行星"
        );
        assert_eq!(
            false, aspect.apply,
            "离相位中，行星{aspect_value}度离相位行星"
        );
        assert_eq!(1.0, aspect.d, "离相位中，行星与行星距离");
        assert_eq!(p0.name, aspect.p0);
        assert_eq!(p1.name, aspect.p1);

        let aspect = p1.has_aspect(&p0, false);
        assert!(aspect.is_some());
        let aspect = aspect.unwrap();
        assert_eq!(
            aspect_value, aspect.aspect_value,
            "离相位中，行星{aspect_value}度相位行星"
        );
        assert_eq!(
            false, aspect.apply,
            "离相位中，行星{aspect_value}度离相位行星"
        );
        assert_eq!(1.0, aspect.d, "离相位中，行星与行星入距离");
        assert_eq!(p1.name, aspect.p0);
        assert_eq!(p0.name, aspect.p1);

        // 即将无相位
        p1.long = 23.0;
        let aspect = p0.has_aspect(&p1, false);
        assert!(aspect.is_some());
        let aspect = aspect.unwrap();
        assert_eq!(
            aspect_value, aspect.aspect_value,
            "离相位中，行星{aspect_value}度相位行星"
        );
        assert_eq!(
            false, aspect.apply,
            "离相位中，行星{aspect_value}度离相位行星"
        );
        assert_eq!(5.0, aspect.d, "离相位中，行星与行星距离");
        assert_eq!(p0.name, aspect.p0);
        assert_eq!(p1.name, aspect.p1);

        let aspect = p1.has_aspect(&p0, false);
        assert!(aspect.is_some());
        let aspect = aspect.unwrap();
        assert_eq!(
            aspect_value, aspect.aspect_value,
            "离相位中，行星{aspect_value}度相位行星"
        );
        assert_eq!(
            false, aspect.apply,
            "离相位中，行星{aspect_value}离相位行星"
        );
        assert_eq!(5.0, aspect.d, "离相位中，行星与行星距离");
        assert_eq!(p1.name, aspect.p0);
        assert_eq!(p0.name, aspect.p1);

        // 已经无相位
        p1.long = 22.0;
        assert!(
            p0.has_aspect(&p1, false).is_none(),
            "已经无相位，行星{aspect_value}度相位行星"
        );
        assert!(
            p1.has_aspect(&p0, false).is_none(),
            "已经无相位，行星{aspect_value}度相位行星"
        );
    }
}

// 行星与行星相位，行星容许度不为0，快行星逆行，慢行星顺行，快行星跨星左/右离相位
#[parameterized(aspect_value = {
    0, 60, 90, 120, 180}
)]
fn planet_sinister_or_dexter_planet_plant_orb_is_not_0_faster_retrograde_slower_forward_cross_sign_separated(
    aspect_value: u8,
) {
    // 相位点在白羊座2度

    // sun 在-0, -60, -90, -120, -180
    // -0, -180不存在右相位，但测试中仍保留，这两种相位的测试与左相位是相同的
    // swe_degnorm(360.0 - f64::from(aspect_value) + 10.0)
    let p0_longs = [
        2.0 + f64::from(aspect_value),
        swe_degnorm(360.0 - f64::from(aspect_value) + 2.0),
    ];

    let p0_longs = if aspect_value == 0 || aspect_value == 180 {
        &p0_longs[..1]
    } else {
        &p0_longs
    };

    for p0_long in p0_longs {
        // 没有相位
        let p0 = Planet::new(
            Sun,
            *p0_long,
            1.0,
            1.0,
            1.0,
            1.0,
            &PlanetConfig::new(Sun, 6, 1.0, 2.0),
        );
        let mut p1 = Planet::new(
            Moon,
            8.0,
            1.0,
            -2.0,
            1.0,
            1.0,
            &PlanetConfig::new(Moon, 4, 1.0, 2.0),
        );
        let aspect = p0.has_aspect(&p1, false);
        assert!(aspect.is_none(), "还没入相位，行星{aspect_value}度相位行星");
        let aspect = p1.has_aspect(&p0, false);
        assert!(aspect.is_none(), "还没入相位，行星{aspect_value}度相位asc");

        // 刚入相位
        p1.long = 7.0;
        let aspect = p0.has_aspect(&p1, false);
        assert!(aspect.is_some());
        let aspect = aspect.unwrap();
        assert_eq!(
            aspect_value, aspect.aspect_value,
            "刚入相位，行星{aspect_value}度相位行星"
        );
        assert_eq!(true, aspect.apply, "刚入相位，行星入相位行星");
        assert_eq!(5.0, aspect.d, "行星与行星距离");
        assert_eq!(p0.name, aspect.p0);
        assert_eq!(p1.name, aspect.p1);

        let aspect = p1.has_aspect(&p0, false);
        assert!(aspect.is_some());
        let aspect = aspect.unwrap();
        assert_eq!(
            aspect_value, aspect.aspect_value,
            "刚入相位，行星{aspect_value}度相位行星"
        );
        assert_eq!(true, aspect.apply, "刚入相位，行星入相位行星");
        assert_eq!(5.0, aspect.d, "行星与行星距离");
        assert_eq!(p1.name, aspect.p0);
        assert_eq!(p0.name, aspect.p1);

        // 入相位中
        p1.long = 6.0;
        let aspect = p0.has_aspect(&p1, false);
        assert!(aspect.is_some());
        let aspect = aspect.unwrap();
        assert_eq!(
            aspect_value, aspect.aspect_value,
            "入相位中，行星{aspect_value}度相位行星"
        );
        assert_eq!(true, aspect.apply, "入相位中，行星入相位行星");
        assert_eq!(4.0, aspect.d, "行星与行星距离");
        assert_eq!(p0.name, aspect.p0);
        assert_eq!(p1.name, aspect.p1);

        let aspect = p1.has_aspect(&p0, false);
        assert!(aspect.is_some());
        let aspect = aspect.unwrap();
        assert_eq!(
            aspect_value, aspect.aspect_value,
            "入相位中，行星{aspect_value}度相位行星"
        );
        assert_eq!(true, aspect.apply, "入相位中，行星入相位行星");
        assert_eq!(4.0, aspect.d, "行星与行星距离");
        assert_eq!(p1.name, aspect.p0);
        assert_eq!(p0.name, aspect.p1);

        // 形成相位
        p1.long = 2.0;
        let aspect = p0.has_aspect(&p1, false);
        assert!(aspect.is_some());
        let aspect = aspect.unwrap();
        assert_eq!(
            aspect_value, aspect.aspect_value,
            "形成相位，行星{aspect_value}度相位行星"
        );
        assert_eq!(
            false, aspect.apply,
            "形成相位，行星{aspect_value}度入相位行星"
        );
        assert_eq!(0.0, aspect.d, "形成相位，行星与行星距离");
        assert_eq!(p0.name, aspect.p0);
        assert_eq!(p1.name, aspect.p1);

        let aspect = p1.has_aspect(&p0, false);
        assert!(aspect.is_some());
        let aspect = aspect.unwrap();
        assert_eq!(
            aspect_value, aspect.aspect_value,
            "形成相位，行星{aspect_value}度相位行星"
        );
        assert_eq!(
            false, aspect.apply,
            "形成相位，行星{aspect_value}度入相位行星"
        );
        assert_eq!(0.0, aspect.d, "形成相位，行星与行星入距离");
        assert_eq!(p1.name, aspect.p0);
        assert_eq!(p0.name, aspect.p1);

        // 离相位中
        p1.long = 0.0;
        let aspect = p0.has_aspect(&p1, false);
        assert!(aspect.is_some());
        let aspect = aspect.unwrap();
        assert_eq!(
            aspect_value, aspect.aspect_value,
            "离相位中，行星{aspect_value}度相位行星"
        );
        assert_eq!(
            false, aspect.apply,
            "离相位中，行星{aspect_value}度离相位行星"
        );
        assert_eq!(2.0, aspect.d, "离相位中，行星与行星距离");
        assert_eq!(p0.name, aspect.p0);
        assert_eq!(p1.name, aspect.p1);

        let aspect = p1.has_aspect(&p0, false);
        assert!(aspect.is_some());
        let aspect = aspect.unwrap();
        assert_eq!(
            aspect_value, aspect.aspect_value,
            "离相位中，行星{aspect_value}度相位行星"
        );
        assert_eq!(
            false, aspect.apply,
            "离相位中，行星{aspect_value}度离相位行星"
        );
        assert_eq!(2.0, aspect.d, "离相位中，行星与行星入距离");
        assert_eq!(p1.name, aspect.p0);
        assert_eq!(p0.name, aspect.p1);

        // 离相位中，在双鱼座29度
        p1.long = 359.0;
        let aspect = p0.has_aspect(&p1, false);
        assert!(
            aspect.is_none(),
            "无相位，度数上离相位中，行星{aspect_value}度相位行星"
        );

        let aspect = p1.has_aspect(&p0, false);
        assert!(
            aspect.is_none(),
            "无相位，度数上离相位中，行星{aspect_value}度相位行星"
        );

        // 即将无相位
        p1.long = 357.0;
        assert!(
            aspect.is_none(),
            "无相位，度数上离相位中，行星{aspect_value}度相位行星"
        );

        let aspect = p1.has_aspect(&p0, false);
        assert!(
            aspect.is_none(),
            "无相位，度数离相位中，行星{aspect_value}度相位行星"
        );

        // 已经无相位
        p1.long = 356.0;
        assert!(
            p0.has_aspect(&p1, false).is_none(),
            "已经无相位，行星{aspect_value}度相位行星"
        );
        assert!(
            p1.has_aspect(&p0, false).is_none(),
            "已经无相位，行星{aspect_value}度相位行星"
        );
    }
}

// 行星与行星度相位，行星容许度不为0，快行星顺行，慢行星逆行，快行星左/右相位
#[parameterized(aspect_value = {
    0, 60, 90, 120, 180}
)]
fn planet_sinister_or_dexter_planet_plant_orb_is_not_0_faster_forward_slower_retrograde(
    aspect_value: u8,
) {
    // 相位点在白羊座10度

    // sun 在-0, -60, -90, -120, -180
    // -0, -180不存在右相位，但测试中仍保留，这两种相位的测试与左相位是相同的
    // swe_degnorm(360.0 - f64::from(aspect_value) + 10.0)
    let p0_longs = [
        10.0 + f64::from(aspect_value),
        swe_degnorm(360.0 - f64::from(aspect_value) + 10.0),
    ];

    let p0_longs = if aspect_value == 0 || aspect_value == 180 {
        &p0_longs[..1]
    } else {
        &p0_longs
    };

    for p0_long in p0_longs {
        // 没有相位
        let p0 = Planet::new(
            Sun,
            *p0_long,
            1.0,
            -1.0,
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
        let aspect = p0.has_aspect(&p1, false);
        assert!(aspect.is_none(), "还没入相位，行星{aspect_value}度相位行星");
        let aspect = p1.has_aspect(&p0, false);
        assert!(aspect.is_none(), "还没入相位，行星{aspect_value}度相位asc");

        // 刚入相位
        p1.long = 5.0;
        let aspect = p0.has_aspect(&p1, false);
        assert!(aspect.is_some());
        let aspect = aspect.unwrap();
        assert_eq!(
            aspect_value, aspect.aspect_value,
            "刚形成相位，行星{aspect_value}度相位行星"
        );
        assert_eq!(true, aspect.apply, "刚形成相位，行星入相位行星");
        assert_eq!(5.0, aspect.d, "刚形成相位，行星与行星距离");
        assert_eq!(p0.name, aspect.p0);
        assert_eq!(p1.name, aspect.p1);

        let aspect = p1.has_aspect(&p0, false);
        assert!(aspect.is_some());
        let aspect = aspect.unwrap();
        assert_eq!(
            aspect_value, aspect.aspect_value,
            "刚形成相位，行星{aspect_value}度相位行星"
        );
        assert_eq!(true, aspect.apply, "刚形成相位，行星入相位行星");
        assert_eq!(5.0, aspect.d, "刚形成相位，行星与行星入距离");
        assert_eq!(p1.name, aspect.p0,);
        assert_eq!(p0.name, aspect.p1,);

        // 入相位中
        p1.long = 6.0;
        let aspect = p0.has_aspect(&p1, false);
        assert!(aspect.is_some());
        let aspect = aspect.unwrap();
        assert_eq!(
            aspect_value, aspect.aspect_value,
            "入相位中，行星{aspect_value}度相位行星"
        );
        assert_eq!(
            true, aspect.apply,
            "入相位中，行星{aspect_value}度入相位行星"
        );
        assert_eq!(4.0, aspect.d, "入相位中，行星与行星距离");
        assert_eq!(p0.name, aspect.p0);
        assert_eq!(p1.name, aspect.p1);

        let aspect = p1.has_aspect(&p0, false);
        assert!(aspect.is_some());
        let aspect = aspect.unwrap();
        assert_eq!(
            aspect_value, aspect.aspect_value,
            "入相位中，行星{aspect_value}度相位行星"
        );
        assert_eq!(
            true, aspect.apply,
            "入相位中，行星{aspect_value}度入相位行星"
        );
        assert_eq!(4.0, aspect.d, "入相位中，行星与行星入距离");
        assert_eq!(p1.name, aspect.p0);
        assert_eq!(p0.name, aspect.p1);

        // 形成相位
        p1.long = 10.0;
        let aspect = p0.has_aspect(&p1, false);
        assert!(aspect.is_some());
        let aspect = aspect.unwrap();
        assert_eq!(
            aspect_value, aspect.aspect_value,
            "形成相位，行星{aspect_value}度相位行星"
        );
        assert_eq!(
            false, aspect.apply,
            "形成相位，行星{aspect_value}度入相位行星"
        );
        assert_eq!(0.0, aspect.d, "形成相位，行星与行星距离");
        assert_eq!(p0.name, aspect.p0);
        assert_eq!(p1.name, aspect.p1);

        let aspect = p1.has_aspect(&p0, false);
        assert!(aspect.is_some());
        let aspect = aspect.unwrap();
        assert_eq!(
            aspect_value, aspect.aspect_value,
            "形成相位，行星{aspect_value}度相位行星"
        );
        assert_eq!(
            false, aspect.apply,
            "形成相位，行星{aspect_value}度入相位行星"
        );
        assert_eq!(0.0, aspect.d, "形成相位，行星与行星入距离");
        assert_eq!(p1.name, aspect.p0);
        assert_eq!(p0.name, aspect.p1);

        // 离相位中
        p1.long = 11.0;
        let aspect = p0.has_aspect(&p1, false);
        assert!(aspect.is_some());
        let aspect = aspect.unwrap();
        assert_eq!(
            aspect_value, aspect.aspect_value,
            "离相位中，行星{aspect_value}度相位行星"
        );
        assert_eq!(
            false, aspect.apply,
            "离相位中，行星{aspect_value}度离相位行星"
        );
        assert_eq!(1.0, aspect.d, "离相位中，行星与行星距离");
        assert_eq!(p0.name, aspect.p0);
        assert_eq!(p1.name, aspect.p1);

        let aspect = p1.has_aspect(&p0, false);
        assert!(aspect.is_some());
        let aspect = aspect.unwrap();
        assert_eq!(
            aspect_value, aspect.aspect_value,
            "离相位中，行星{aspect_value}度相位行星"
        );
        assert_eq!(
            false, aspect.apply,
            "离相位中，行星{aspect_value}度离相位行星"
        );
        assert_eq!(1.0, aspect.d, "离相位中，行星与行星入距离");
        assert_eq!(p1.name, aspect.p0);
        assert_eq!(p0.name, aspect.p1);

        // 即将无相位
        p1.long = 15.0;
        let aspect = p0.has_aspect(&p1, false);
        assert!(aspect.is_some());
        let aspect = aspect.unwrap();
        assert_eq!(
            aspect_value, aspect.aspect_value,
            "离相位中，行星{aspect_value}度相位行星"
        );
        assert_eq!(
            false, aspect.apply,
            "离相位中，行星{aspect_value}度离相位行星"
        );
        assert_eq!(5.0, aspect.d, "离相位中，行星与行星距离");
        assert_eq!(p0.name, aspect.p0);
        assert_eq!(p1.name, aspect.p1);

        let aspect = p1.has_aspect(&p0, false);
        assert!(aspect.is_some());
        let aspect = aspect.unwrap();
        assert_eq!(
            aspect_value, aspect.aspect_value,
            "离相位中，行星{aspect_value}度相位行星"
        );
        assert_eq!(
            false, aspect.apply,
            "离相位中，行星{aspect_value}离相位行星"
        );
        assert_eq!(5.0, aspect.d, "离相位中，行星与行星距离");
        assert_eq!(p1.name, aspect.p0);
        assert_eq!(p0.name, aspect.p1);

        // 已经无相位
        p1.long = 16.0;
        assert!(
            p0.has_aspect(&p1, false).is_none(),
            "已经无相位，行星{aspect_value}度相位行星"
        );
        assert!(
            p1.has_aspect(&p0, false).is_none(),
            "已经无相位，行星{aspect_value}度相位行星"
        );
    }
}

// 行星与行星相位，行星容许度不为0，快行星顺行，慢行星逆行，快行星跨星左/右入相位
#[parameterized(aspect_value = {
    0, 60, 90, 120, 180}
)]
fn planet_sinister_or_dexter_planet_plant_orb_is_not_0_aster_forward_slower_retrograde_cross_sign_apply(
    aspect_value: u8,
) {
    // 相位点在白羊座2度

    // sun 在-0, -60, -90, -120, -180
    // -0, -180不存在右相位，但测试中仍保留，这两种相位的测试与左相位是相同的
    // swe_degnorm(360.0 - f64::from(aspect_value) + 10.0)
    let p0_longs = [
        2.0 + f64::from(aspect_value),
        swe_degnorm(360.0 - f64::from(aspect_value) + 2.0),
    ];

    let p0_longs = if aspect_value == 0 || aspect_value == 180 {
        &p0_longs[..1]
    } else {
        &p0_longs
    };

    for p0_long in p0_longs {
        // 没有相位
        let p0 = Planet::new(
            Sun,
            *p0_long,
            1.0,
            -1.0,
            1.0,
            1.0,
            &PlanetConfig::new(Sun, 6, 1.0, 2.0),
        );
        let mut p1 = Planet::new(
            Moon,
            356.0,
            1.0,
            2.0,
            1.0,
            1.0,
            &PlanetConfig::new(Moon, 4, 1.0, 2.0),
        );
        let aspect = p0.has_aspect(&p1, false);
        assert!(aspect.is_none(), "还没入相位，行星{aspect_value}度相位行星");
        let aspect = p1.has_aspect(&p0, false);
        assert!(aspect.is_none(), "还没入相位，行星{aspect_value}度相位asc");

        // 刚入相位
        p1.long = 357.0;
        let aspect = p0.has_aspect(&p1, false);
        assert!(
            aspect.is_none(),
            "无相位，度数上刚形成相位，行星{aspect_value}度相位行星"
        );

        let aspect = p1.has_aspect(&p0, false);
        assert!(
            aspect.is_none(),
            "无相位，度数上刚形成相位，行星{aspect_value}度相位行星"
        );

        // 入相位中
        p1.long = 358.0;
        let aspect = p0.has_aspect(&p1, false);
        assert!(
            aspect.is_none(),
            "无相位，度数上入相位中，行星{aspect_value}度相位行星"
        );

        let aspect = p1.has_aspect(&p0, false);
        assert!(
            aspect.is_none(),
            "无相位，度数上入相位中，行星{aspect_value}度相位行星"
        );

        // 入相位中，在双鱼座29度
        p1.long = 359.0;
        let aspect = p0.has_aspect(&p1, false);
        assert!(
            aspect.is_none(),
            "无相位，度数上入相位中，行星{aspect_value}度相位行星"
        );

        let aspect = p1.has_aspect(&p0, false);
        assert!(
            aspect.is_none(),
            "无相位，度数上入相位中，行星{aspect_value}度相位行星"
        );

        // 入相位中
        p1.long = 0.0;
        let aspect = p0.has_aspect(&p1, false);
        assert!(aspect.is_some());
        let aspect = aspect.unwrap();
        assert_eq!(
            aspect_value, aspect.aspect_value,
            "入相位中，行星{aspect_value}度相位行星"
        );
        assert_eq!(
            true, aspect.apply,
            "入相位中，行星{aspect_value}度入相位行星"
        );
        assert_eq!(2.0, aspect.d, "形成相位，行星与行星距离");
        assert_eq!(p0.name, aspect.p0);
        assert_eq!(p1.name, aspect.p1);

        let aspect = p1.has_aspect(&p0, false);
        assert!(aspect.is_some());
        let aspect = aspect.unwrap();
        assert_eq!(
            aspect_value, aspect.aspect_value,
            "入相位中，行星{aspect_value}度相位行星"
        );
        assert_eq!(
            true, aspect.apply,
            "形成相位，行星{aspect_value}度入相位行星"
        );
        assert_eq!(2.0, aspect.d, "形成相位，行星与行星入距离");
        assert_eq!(p1.name, aspect.p0);
        assert_eq!(p0.name, aspect.p1);

        // 形成相位
        p1.long = 2.0;
        let aspect = p0.has_aspect(&p1, false);
        assert!(aspect.is_some());
        let aspect = aspect.unwrap();
        assert_eq!(
            aspect_value, aspect.aspect_value,
            "形成相位，行星{aspect_value}度相位行星"
        );
        assert_eq!(
            false, aspect.apply,
            "形成相位，行星{aspect_value}度入相位行星"
        );
        assert_eq!(0.0, aspect.d, "形成相位，行星与行星距离");
        assert_eq!(p0.name, aspect.p0);
        assert_eq!(p1.name, aspect.p1);

        let aspect = p1.has_aspect(&p0, false);
        assert!(aspect.is_some());
        let aspect = aspect.unwrap();
        assert_eq!(
            aspect_value, aspect.aspect_value,
            "形成相位，行星{aspect_value}度相位行星"
        );
        assert_eq!(
            false, aspect.apply,
            "形成相位，行星{aspect_value}度入相位行星"
        );
        assert_eq!(0.0, aspect.d, "形成相位，行星与行星入距离");
        assert_eq!(p1.name, aspect.p0);
        assert_eq!(p0.name, aspect.p1);

        // 离相位中
        p1.long = 3.0;
        let aspect = p0.has_aspect(&p1, false);
        assert!(aspect.is_some());
        let aspect = aspect.unwrap();
        assert_eq!(
            aspect_value, aspect.aspect_value,
            "离相位中，行星{aspect_value}度相位行星"
        );
        assert_eq!(
            false, aspect.apply,
            "离相位中，行星{aspect_value}度离相位行星"
        );
        assert_eq!(1.0, aspect.d, "离相位中，行星与行星距离");
        assert_eq!(p0.name, aspect.p0);
        assert_eq!(p1.name, aspect.p1);

        let aspect = p1.has_aspect(&p0, false);
        assert!(aspect.is_some());
        let aspect = aspect.unwrap();
        assert_eq!(
            aspect_value, aspect.aspect_value,
            "离相位中，行星{aspect_value}度相位行星"
        );
        assert_eq!(
            false, aspect.apply,
            "离相位中，行星{aspect_value}度离相位行星"
        );
        assert_eq!(1.0, aspect.d, "离相位中，行星与行星入距离");
        assert_eq!(p1.name, aspect.p0);
        assert_eq!(p0.name, aspect.p1);

        // 即将无相位
        p1.long = 7.0;
        let aspect = p0.has_aspect(&p1, false);
        assert!(aspect.is_some());
        let aspect = aspect.unwrap();
        assert_eq!(
            aspect_value, aspect.aspect_value,
            "离相位中，行星{aspect_value}度相位行星"
        );
        assert_eq!(
            false, aspect.apply,
            "离相位中，行星{aspect_value}度离相位行星"
        );
        assert_eq!(5.0, aspect.d, "离相位中，行星与行星距离");
        assert_eq!(p0.name, aspect.p0);
        assert_eq!(p1.name, aspect.p1);

        let aspect = p1.has_aspect(&p0, false);
        assert!(aspect.is_some());
        let aspect = aspect.unwrap();
        assert_eq!(
            aspect_value, aspect.aspect_value,
            "离相位中，行星{aspect_value}度相位行星"
        );
        assert_eq!(
            false, aspect.apply,
            "离相位中，行星{aspect_value}离相位行星"
        );
        assert_eq!(5.0, aspect.d, "离相位中，行星与行星距离");
        assert_eq!(p1.name, aspect.p0);
        assert_eq!(p0.name, aspect.p1);

        // 已经无相位
        p1.long = 8.0;
        assert!(
            p0.has_aspect(&p1, false).is_none(),
            "已经无相位，行星{aspect_value}度相位行星"
        );
        assert!(
            p1.has_aspect(&p0, false).is_none(),
            "已经无相位，行星{aspect_value}度相位行星"
        );
    }
}

// 行星与行星相位，行星容许度不为0，快行星顺行，慢行星逆行，快行星跨星左/右离相位
#[parameterized(aspect_value = {
    0, 60, 90, 120, 180}
)]
fn planet_sinister_or_dexter_planet_plant_orb_is_not_0_aster_forward_slower_retrograde_cross_sign_separated(
    aspect_value: u8,
) {
    // 相位点在白羊座2度

    // sun 在-0, -60, -90, -120, -180
    // -0, -180不存在右相位，但测试中仍保留，这两种相位的测试与左相位是相同的
    // swe_degnorm(360.0 - f64::from(aspect_value) + 10.0)
    let p0_longs = [
        28.0 + f64::from(aspect_value),
        swe_degnorm(360.0 - f64::from(aspect_value) + 28.0),
    ];

    let p0_longs = if aspect_value == 0 || aspect_value == 180 {
        &p0_longs[..1]
    } else {
        &p0_longs
    };

    for p0_long in p0_longs {
        // 没有相位
        let p0 = Planet::new(
            Sun,
            *p0_long,
            1.0,
            -1.0,
            1.0,
            1.0,
            &PlanetConfig::new(Sun, 6, 1.0, 2.0),
        );
        let mut p1 = Planet::new(
            Moon,
            22.0,
            1.0,
            2.0,
            1.0,
            1.0,
            &PlanetConfig::new(Moon, 4, 1.0, 2.0),
        );
        let aspect = p0.has_aspect(&p1, false);
        assert!(aspect.is_none(), "还没入相位，行星{aspect_value}度相位行星");
        let aspect = p1.has_aspect(&p0, false);
        assert!(aspect.is_none(), "还没入相位，行星{aspect_value}度相位asc");

        // 刚入相位
        p1.long = 23.0;
        let aspect = p0.has_aspect(&p1, false);
        assert!(aspect.is_some());
        let aspect = aspect.unwrap();
        assert_eq!(
            aspect_value, aspect.aspect_value,
            "刚入相位，行星{aspect_value}度相位行星"
        );
        assert_eq!(true, aspect.apply, "刚入相位，行星入相位行星");
        assert_eq!(5.0, aspect.d, "行星与行星距离");
        assert_eq!(p0.name, aspect.p0);
        assert_eq!(p1.name, aspect.p1);

        let aspect = p1.has_aspect(&p0, false);
        assert!(aspect.is_some());
        let aspect = aspect.unwrap();
        assert_eq!(
            aspect_value, aspect.aspect_value,
            "刚入相位，行星{aspect_value}度相位行星"
        );
        assert_eq!(true, aspect.apply, "刚入相位，行星入相位行星");
        assert_eq!(5.0, aspect.d, "行星与行星距离");
        assert_eq!(p1.name, aspect.p0);
        assert_eq!(p0.name, aspect.p1);

        // 入相位中
        p1.long = 24.0;
        let aspect = p0.has_aspect(&p1, false);
        assert!(aspect.is_some());
        let aspect = aspect.unwrap();
        assert_eq!(
            aspect_value, aspect.aspect_value,
            "入相位中，行星{aspect_value}度相位行星"
        );
        assert_eq!(true, aspect.apply, "入相位中，行星入相位行星");
        assert_eq!(4.0, aspect.d, "行星与行星距离");
        assert_eq!(p0.name, aspect.p0);
        assert_eq!(p1.name, aspect.p1);

        let aspect = p1.has_aspect(&p0, false);
        assert!(aspect.is_some());
        let aspect = aspect.unwrap();
        assert_eq!(
            aspect_value, aspect.aspect_value,
            "入相位中，行星{aspect_value}度相位行星"
        );
        assert_eq!(true, aspect.apply, "入相位中，行星入相位行星");
        assert_eq!(4.0, aspect.d, "行星与行星距离");
        assert_eq!(p1.name, aspect.p0);
        assert_eq!(p0.name, aspect.p1);

        // 形成相位
        p1.long = 28.0;
        let aspect = p0.has_aspect(&p1, false);
        assert!(aspect.is_some());
        let aspect = aspect.unwrap();
        assert_eq!(
            aspect_value, aspect.aspect_value,
            "形成相位，行星{aspect_value}度相位行星"
        );
        assert_eq!(
            false, aspect.apply,
            "形成相位，行星{aspect_value}度入相位行星"
        );
        assert_eq!(0.0, aspect.d, "形成相位，行星与行星距离");
        assert_eq!(p0.name, aspect.p0);
        assert_eq!(p1.name, aspect.p1);

        let aspect = p1.has_aspect(&p0, false);
        assert!(aspect.is_some());
        let aspect = aspect.unwrap();
        assert_eq!(
            aspect_value, aspect.aspect_value,
            "形成相位，行星{aspect_value}度相位行星"
        );
        assert_eq!(
            false, aspect.apply,
            "形成相位，行星{aspect_value}度入相位行星"
        );
        assert_eq!(0.0, aspect.d, "形成相位，行星与行星入距离");
        assert_eq!(p1.name, aspect.p0);
        assert_eq!(p0.name, aspect.p1);

        // 离相位中
        p1.long = 29.0;
        let aspect = p0.has_aspect(&p1, false);
        assert!(aspect.is_some());
        let aspect = aspect.unwrap();
        assert_eq!(
            aspect_value, aspect.aspect_value,
            "离相位中，行星{aspect_value}度相位行星"
        );
        assert_eq!(
            false, aspect.apply,
            "离相位中，行星{aspect_value}度离相位行星"
        );
        assert_eq!(1.0, aspect.d, "离相位中，行星与行星距离");
        assert_eq!(p0.name, aspect.p0);
        assert_eq!(p1.name, aspect.p1);

        let aspect = p1.has_aspect(&p0, false);
        assert!(aspect.is_some());
        let aspect = aspect.unwrap();
        assert_eq!(
            aspect_value, aspect.aspect_value,
            "离相位中，行星{aspect_value}度相位行星"
        );
        assert_eq!(
            false, aspect.apply,
            "离相位中，行星{aspect_value}度离相位行星"
        );
        assert_eq!(1.0, aspect.d, "离相位中，行星与行星入距离");
        assert_eq!(p1.name, aspect.p0);
        assert_eq!(p0.name, aspect.p1);

        // 离相位中，在金牛座0度
        p1.long = 30.0;
        let aspect = p0.has_aspect(&p1, false);
        assert!(
            aspect.is_none(),
            "无相位，度数上离相位中，行星{aspect_value}度相位行星"
        );

        let aspect = p1.has_aspect(&p0, false);
        assert!(
            aspect.is_none(),
            "无相位，度数上离相位中，行星{aspect_value}度相位行星"
        );

        // 即将无相位
        p1.long = 33.0;
        assert!(
            aspect.is_none(),
            "无相位，度数上离相位中，行星{aspect_value}度相位行星"
        );

        let aspect = p1.has_aspect(&p0, false);
        assert!(
            aspect.is_none(),
            "无相位，度数离相位中，行星{aspect_value}度相位行星"
        );

        // 已经无相位
        p1.long = 34.0;
        assert!(
            p0.has_aspect(&p1, false).is_none(),
            "已经无相位，行星{aspect_value}度相位行星"
        );
        assert!(
            p1.has_aspect(&p0, false).is_none(),
            "已经无相位，行星{aspect_value}度相位行星"
        );
    }
}
