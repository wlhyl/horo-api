use crate::{config::PlanetConfig, Planet, PlanetName::*};
use parameterized::parameterized;
use swe::swe_degnorm;

// ASC与行星相位，行星容许度为0
#[parameterized(aspect_value = {
    0, 60, 90, 120, 180}
)]
fn asc_aspect_planet_plant_orb_is_0(aspect_value: u8) {
    // 不必测试大于180的情况，也不用区分穿过白羊座0度，因为容许度都为0，必然是度数相同
    let asc = Planet::new(
        ASC,
        1.0,
        1.0,
        1.0,
        1.0,
        1.0,
        &PlanetConfig::new(ASC, 0, 1.0, 2.0),
    );
    let p = Planet::new(
        Sun,
        1.0 + f64::from(aspect_value),
        1.0,
        1.0,
        1.0,
        1.0,
        &PlanetConfig::new(Sun, 0, 1.0, 2.0),
    );
    let aspect = asc.has_aspect(&p, false);
    assert!(aspect.is_some());
    let aspect = aspect.unwrap();
    assert_eq!(
        aspect_value, aspect.aspect_value,
        "asc{aspect_value}度相位行星"
    );
    assert_eq!(false, aspect.apply, "asc与行星出相位");
    assert_eq!(0.0, aspect.d, "asc与行星出相位0.0度");
    assert_eq!(asc.name, aspect.p0);
    assert_eq!(p.name, aspect.p1);

    let aspect = p.has_aspect(&asc, false);
    assert!(aspect.is_some());
    let aspect = aspect.unwrap();
    assert_eq!(
        aspect_value, aspect.aspect_value,
        "行星{aspect_value}度相位asc"
    );
    assert_eq!(false, aspect.apply, "行星与asc出相位");
    assert_eq!(0.0, aspect.d, "行星与asc出相位0.0度");
    assert_eq!(p.name, aspect.p0);
    assert_eq!(asc.name, aspect.p1);
}

// ASC与行星相位，行星容许度不为0，没有跨星座
#[parameterized(aspect_value = {
    0, 60, 90, 120, 180}
)]
fn asc_aspect_planet_plant_orb_is_not_0_with_no_cross_sign(aspect_value: u8) {
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
        let mut asc = Planet::new(
            ASC,
            4.0,
            1.0,
            1.0,
            1.0,
            1.0,
            &PlanetConfig::new(ASC, 0, 1.0, 2.0),
        );
        let p = Planet::new(
            Sun,
            *p0_long,
            1.0,
            1.0,
            1.0,
            1.0,
            &PlanetConfig::new(Sun, 10, 1.0, 2.0),
        );

        let aspect = asc.has_aspect(&p, false);
        assert!(aspect.is_none(), "没有入相位，asc{aspect_value}度相位行星");
        let aspect = p.has_aspect(&asc, false);
        assert!(aspect.is_none(), "没有入相位，行星{aspect_value}度相位asc");

        // 刚入相位
        asc.long = 5.0;
        let aspect = asc.has_aspect(&p, false);
        assert!(aspect.is_some());
        let aspect = aspect.unwrap();
        assert_eq!(
            aspect_value, aspect.aspect_value,
            "刚形成相位，asc{aspect_value}度相位行星"
        );
        assert_eq!(true, aspect.apply, "刚形成相位，asc入相位行星");
        assert_eq!(5.0, aspect.d, "刚形成相位，asc与行星入相位1.0度");
        assert_eq!(asc.name, aspect.p0);
        assert_eq!(p.name, aspect.p1);

        let aspect = p.has_aspect(&asc, false);
        assert!(aspect.is_some());
        let aspect = aspect.unwrap();
        assert_eq!(
            aspect_value, aspect.aspect_value,
            "刚形成相位，行星{aspect_value}度相位asc"
        );
        assert_eq!(true, aspect.apply, "刚形成相位，asc入相位行星");
        assert_eq!(5.0, aspect.d, "刚形成相位，asc与行星入相位1.0度");
        assert_eq!(p.name, aspect.p0);
        assert_eq!(asc.name, aspect.p1);

        // 入相位中
        asc.long = 6.0;
        let aspect = asc.has_aspect(&p, false);
        assert!(aspect.is_some());
        let aspect = aspect.unwrap();
        assert_eq!(
            aspect_value, aspect.aspect_value,
            "入相位中，asc{aspect_value}度相位行星"
        );
        assert_eq!(true, aspect.apply, "入相位中，asc 入相位行星");
        assert_eq!(4.0, aspect.d, "入相位中，asc与行星距离");
        assert_eq!(asc.name, aspect.p0);
        assert_eq!(p.name, aspect.p1);

        let aspect = p.has_aspect(&asc, false);
        assert!(aspect.is_some());
        let aspect = aspect.unwrap();
        assert_eq!(
            aspect_value, aspect.aspect_value,
            "入相位中，行星{aspect_value}度相位asc"
        );
        assert_eq!(true, aspect.apply, "入相位中，asc入相位行星");
        assert_eq!(4.0, aspect.d, "入相位中，asc与行星距离");
        assert_eq!(p.name, aspect.p0);
        assert_eq!(asc.name, aspect.p1);

        // 形成相位
        asc.long = 10.0;
        let aspect = asc.has_aspect(&p, false);
        assert!(aspect.is_some());
        let aspect = aspect.unwrap();
        assert_eq!(
            aspect_value, aspect.aspect_value,
            "形成相位，asc{aspect_value}度相位行星"
        );
        assert_eq!(false, aspect.apply, "形成相位，asc出相位行星");
        assert_eq!(0.0, aspect.d, "形成相位，asc与行星入相位0.0度");
        assert_eq!(asc.name, aspect.p0);
        assert_eq!(p.name, aspect.p1);

        let aspect = p.has_aspect(&asc, false);
        assert!(aspect.is_some());
        let aspect = aspect.unwrap();
        assert_eq!(
            aspect_value, aspect.aspect_value,
            "形成相位，行星{aspect_value}度相位asc"
        );
        assert_eq!(false, aspect.apply, "形成相位，asc出相位行星");
        assert_eq!(0.0, aspect.d, "形成相位，asc与行星入相位0.0度");
        assert_eq!(p.name, aspect.p0);
        assert_eq!(asc.name, aspect.p1);

        // 离相位中
        asc.long = 11.0;
        let aspect = asc.has_aspect(&p, false);
        assert!(aspect.is_some());
        let aspect = aspect.unwrap();
        assert_eq!(
            aspect_value, aspect.aspect_value,
            "离相位中，asc{aspect_value}度相位行星"
        );
        assert_eq!(false, aspect.apply, "离相位中，asc 离相位行星");
        assert_eq!(1.0, aspect.d, "离相位中，asc与行星离相位1.0度");
        assert_eq!(asc.name, aspect.p0);
        assert_eq!(p.name, aspect.p1);

        let aspect = p.has_aspect(&asc, false);
        assert!(aspect.is_some());
        let aspect = aspect.unwrap();
        assert_eq!(
            aspect_value, aspect.aspect_value,
            "离相位中，行星{aspect_value}度相位asc"
        );
        assert_eq!(false, aspect.apply, "离相位中，asc离相位行星");
        assert_eq!(1.0, aspect.d, "离相位中，asc与行星离相位1.0度");
        assert_eq!(p.name, aspect.p0);
        assert_eq!(asc.name, aspect.p1);

        // 即将无相位
        asc.long = 15.0;
        let aspect = asc.has_aspect(&p, false);
        assert!(aspect.is_some());
        let aspect = aspect.unwrap();
        assert_eq!(
            aspect_value, aspect.aspect_value,
            "即将无相位，asc{aspect_value}度相位行星"
        );
        assert_eq!(false, aspect.apply, "即将无相位，asc 离相位行星");
        assert_eq!(5.0, aspect.d, "即将无相位，asc与行星距离");
        assert_eq!(asc.name, aspect.p0);
        assert_eq!(p.name, aspect.p1);

        let aspect = p.has_aspect(&asc, false);
        assert!(aspect.is_some());
        let aspect = aspect.unwrap();
        assert_eq!(
            aspect_value, aspect.aspect_value,
            "即将无相位，行星{aspect_value}度相位asc"
        );
        assert_eq!(false, aspect.apply, "即将无相位，asc离相位行星");
        assert_eq!(5.0, aspect.d, "即将无相位，asc与行星距离");
        assert_eq!(p.name, aspect.p0);
        assert_eq!(asc.name, aspect.p1);

        // 已经无相位
        asc.long = 16.0;
        assert!(
            asc.has_aspect(&p, false).is_none(),
            "已经无相位，asc 0度相位行星"
        );
        assert!(
            p.has_aspect(&asc, false).is_none(),
            "已经无相位，行星 0度相位asc"
        );
    }
}

// ASC与行星相位，行星容许度不为0，跨星座入相位
#[parameterized(aspect_value = {
    0, 60, 90, 120, 180}
)]
fn asc_aspect_planet_plant_orb_is_not_0_cross_sign_apply(aspect_value: u8) {
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
        let p = Planet::new(
            Sun,
            *p0_long,
            1.0,
            1.0,
            1.0,
            1.0,
            &PlanetConfig::new(Sun, 10, 1.0, 2.0),
        );
        let mut asc = Planet::new(
            ASC,
            356.0,
            1.0,
            1.0,
            1.0,
            1.0,
            &PlanetConfig::new(ASC, 0, 1.0, 2.0),
        );
        let aspect = asc.has_aspect(&p, false);
        assert!(aspect.is_none(), "还没入相位，asc{aspect_value}度相位行星");
        let aspect = p.has_aspect(&asc, false);
        assert!(aspect.is_none(), "还没入相位，行星{aspect_value}度相位asc");

        // 刚入相位
        asc.long = 357.0;
        let aspect = asc.has_aspect(&p, false);
        assert!(
            aspect.is_none(),
            "没有相位，度数上刚形成相位，asc{aspect_value}度相位行星"
        );

        let aspect = p.has_aspect(&asc, false);
        assert!(
            aspect.is_none(),
            "没有相位，度数上刚形成相位，行星{aspect_value}度相位asc"
        );

        // 入相位中，asc还没过白羊座0度
        asc.long = 359.0;
        let aspect = asc.has_aspect(&p, false);
        assert!(
            aspect.is_none(),
            "没有相位，度数上入相位中，asc还没过白羊座0度，asc{aspect_value}度相位行星"
        );

        let aspect = p.has_aspect(&asc, false);
        assert!(
            aspect.is_none(),
            "没有相位，度数上入相位中，asc还没过白羊座0度，行星{aspect_value}度相位asc"
        );

        // 入相位中，asc在白羊座0度
        asc.long = 0.0;
        let aspect = asc.has_aspect(&p, false);
        assert!(aspect.is_some());
        let aspect = aspect.unwrap();
        assert_eq!(
            aspect_value, aspect.aspect_value,
            "入相位中，asc在白羊座0度，asc{aspect_value}度相位行星"
        );
        assert_eq!(
            true, aspect.apply,
            "入相位中，asc在白羊座0度，asc入相位行星"
        );
        assert_eq!(2.0, aspect.d, "入相位中，asc在白羊座0度，asc与行星距离");
        assert_eq!(asc.name, aspect.p0);
        assert_eq!(p.name, aspect.p1);

        let aspect = p.has_aspect(&asc, false);
        assert!(aspect.is_some());
        let aspect = aspect.unwrap();
        assert_eq!(
            aspect_value, aspect.aspect_value,
            "入相位中，asc在白羊座0度，行星{aspect_value}度相位asc"
        );
        assert_eq!(
            true, aspect.apply,
            "入相位中，asc在白羊座0度，asc入相位行星"
        );
        assert_eq!(2.0, aspect.d, "入相位中，asc在白羊座0度，asc与行星距离");
        assert_eq!(p.name, aspect.p0);
        assert_eq!(asc.name, aspect.p1);

        // 入相位中，asc已经过白羊座0度
        asc.long = 1.0;
        let aspect = asc.has_aspect(&p, false);
        assert!(aspect.is_some());
        let aspect = aspect.unwrap();
        assert_eq!(
            aspect_value, aspect.aspect_value,
            "入相位中，asc已经过白羊座0度，asc{aspect_value}度相位行星"
        );
        assert_eq!(
            true, aspect.apply,
            "入相位中，asc已经过白羊座0度，asc入相位行星"
        );
        assert_eq!(1.0, aspect.d, "入相位中，asc已经过白羊座0度，asc与行星距离");
        assert_eq!(asc.name, aspect.p0);
        assert_eq!(p.name, aspect.p1);

        let aspect = p.has_aspect(&asc, false);
        assert!(aspect.is_some());
        let aspect = aspect.unwrap();
        assert_eq!(
            aspect_value, aspect.aspect_value,
            "入相位中，asc已经过白羊座0度，行星{aspect_value}度相位asc"
        );
        assert_eq!(
            true, aspect.apply,
            "入相位中，asc已经过白羊座0度，asc入相位行星"
        );
        assert_eq!(1.0, aspect.d, "入相位中，asc已经过白羊座0度，asc与行星距离");
        assert_eq!(p.name, aspect.p0);
        assert_eq!(asc.name, aspect.p1);

        // 形成相位
        asc.long = 2.0;
        let aspect = asc.has_aspect(&p, false);
        assert!(aspect.is_some());
        let aspect = aspect.unwrap();
        assert_eq!(
            aspect_value, aspect.aspect_value,
            "形成相位，asc{aspect_value} 度相位行星"
        );
        assert_eq!(false, aspect.apply, "形成相位，asc 入相位行星");
        assert_eq!(0.0, aspect.d, "形成相位，asc与行星距离");
        assert_eq!(asc.name, aspect.p0);
        assert_eq!(p.name, aspect.p1);

        let aspect = p.has_aspect(&asc, false);
        assert!(aspect.is_some());
        let aspect = aspect.unwrap();
        assert_eq!(
            aspect_value, aspect.aspect_value,
            "形成相位，行星{aspect_value}度相位asc"
        );
        assert_eq!(false, aspect.apply, "形成相位，asc入相位行星");
        assert_eq!(0.0, aspect.d, "形成相位，asc与行星距离");
        assert_eq!(p.name, aspect.p0);
        assert_eq!(asc.name, aspect.p1);

        // 离相位中
        asc.long = 3.0;
        let aspect = asc.has_aspect(&p, false);
        assert!(aspect.is_some());
        let aspect = aspect.unwrap();
        assert_eq!(
            aspect_value, aspect.aspect_value,
            "离相位中，asc{aspect_value}度相位行星"
        );
        assert_eq!(false, aspect.apply, "离相位中，asc 离相位行星");
        assert_eq!(1.0, aspect.d, "离相位中，asc与行星距离");
        assert_eq!(asc.name, aspect.p0);
        assert_eq!(p.name, aspect.p1);

        let aspect = p.has_aspect(&asc, false);
        assert!(aspect.is_some());
        let aspect = aspect.unwrap();
        assert_eq!(
            aspect_value, aspect.aspect_value,
            "离相位中，行星{aspect_value}度相位asc"
        );
        assert_eq!(false, aspect.apply, "离相位中，asc离相位行星");
        assert_eq!(1.0, aspect.d, "离相位中，asc与行星距离");
        assert_eq!(p.name, aspect.p0);
        assert_eq!(asc.name, aspect.p1);

        // 即将无相位
        asc.long = 7.0;
        let aspect = asc.has_aspect(&p, false);
        assert!(aspect.is_some());
        let aspect = aspect.unwrap();
        assert_eq!(
            aspect_value, aspect.aspect_value,
            "即将无相位，asc已过白羊座0度，asc{aspect_value}度相位行星"
        );
        assert_eq!(
            false, aspect.apply,
            "即将无相位，asc已过白羊座0度，asc 离相位行星"
        );
        assert_eq!(5.0, aspect.d, "即将无相位，asc已过白羊座0度，asc与行星距离");
        assert_eq!(asc.name, aspect.p0);
        assert_eq!(p.name, aspect.p1);

        let aspect = p.has_aspect(&asc, false);
        assert!(aspect.is_some());
        let aspect = aspect.unwrap();
        assert_eq!(
            aspect_value, aspect.aspect_value,
            "即将无相位，asc已过白羊座0度，行星{aspect_value}度相位asc"
        );
        assert_eq!(
            false, aspect.apply,
            "即将无相位，asc已过白羊座0度，asc离相位行星"
        );
        assert_eq!(5.0, aspect.d, "即将无相位，asc已过白羊座0度，asc与行星距离");
        assert_eq!(p.name, aspect.p0);
        assert_eq!(asc.name, aspect.p1);

        // 已经无相位
        asc.long = 8.0;
        let aspect = asc.has_aspect(&p, false);
        assert!(
            aspect.is_none(),
            "已经无相位，asc已过白羊座0度，asc{aspect_value}度相位行星"
        );

        let aspect = p.has_aspect(&asc, false);
        assert!(
            aspect.is_none(),
            "已经无相位，asc已过白羊座0度，行星{aspect_value}度相位asc"
        );
    }
}

// ASC与行星相位，行星容许度不为0，跨星座离相位
#[parameterized(aspect_value = {
    0, 60, 90, 120, 180}
)]
fn asc_aspect_planet_plant_orb_is_not_0_cross_sign_separated(aspect_value: u8) {
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
        let p = Planet::new(
            Sun,
            *p0_long,
            1.0,
            1.0,
            1.0,
            1.0,
            &PlanetConfig::new(Sun, 10, 1.0, 2.0),
        );
        let mut asc = Planet::new(
            ASC,
            22.0,
            1.0,
            1.0,
            1.0,
            1.0,
            &PlanetConfig::new(ASC, 0, 1.0, 2.0),
        );
        let aspect = asc.has_aspect(&p, false);
        assert!(aspect.is_none(), "还没入相位，asc{aspect_value}度相位行星");
        let aspect = p.has_aspect(&asc, false);
        assert!(aspect.is_none(), "还没入相位，行星{aspect_value}度相位asc");

        // 刚入相位
        asc.long = 23.0;
        let aspect = asc.has_aspect(&p, false);
        assert!(aspect.is_some());
        let aspect = aspect.unwrap();
        assert_eq!(
            aspect_value, aspect.aspect_value,
            "刚入相位，asc{aspect_value}度相位行星"
        );
        assert_eq!(true, aspect.apply, "入相位中，asc入相位行星");
        assert_eq!(5.0, aspect.d, "入相位中，asc与行星距离");
        assert_eq!(asc.name, aspect.p0);
        assert_eq!(p.name, aspect.p1);

        let aspect = p.has_aspect(&asc, false);
        assert!(aspect.is_some());
        let aspect = aspect.unwrap();
        assert_eq!(
            aspect_value, aspect.aspect_value,
            "刚入相位，行星{aspect_value}度相位asc"
        );
        assert_eq!(true, aspect.apply, "入相位中，asc入相位行星");
        assert_eq!(5.0, aspect.d, "入相位中，asc与行星距离");
        assert_eq!(p.name, aspect.p0);
        assert_eq!(asc.name, aspect.p1);

        // 形成相位
        asc.long = 28.0;
        let aspect = asc.has_aspect(&p, false);
        assert!(aspect.is_some());
        let aspect = aspect.unwrap();
        assert_eq!(
            aspect_value, aspect.aspect_value,
            "形成相位，asc{aspect_value} 度相位行星"
        );
        assert_eq!(false, aspect.apply, "形成相位，asc 入相位行星");
        assert_eq!(0.0, aspect.d, "形成相位，asc与行星距离");
        assert_eq!(asc.name, aspect.p0);
        assert_eq!(p.name, aspect.p1);

        let aspect = p.has_aspect(&asc, false);
        assert!(aspect.is_some());
        let aspect = aspect.unwrap();
        assert_eq!(
            aspect_value, aspect.aspect_value,
            "形成相位，行星{aspect_value}度相位asc"
        );
        assert_eq!(false, aspect.apply, "形成相位，asc入相位行星");
        assert_eq!(0.0, aspect.d, "形成相位，asc与行星距离");
        assert_eq!(p.name, aspect.p0);
        assert_eq!(asc.name, aspect.p1);

        // 离相位中
        asc.long = 29.0;
        let aspect = asc.has_aspect(&p, false);
        assert!(aspect.is_some());
        let aspect = aspect.unwrap();
        assert_eq!(
            aspect_value, aspect.aspect_value,
            "离相位中，asc{aspect_value}度相位行星"
        );
        assert_eq!(false, aspect.apply, "离相位中，asc离相位行星");
        assert_eq!(1.0, aspect.d, "离相位中，asc与行星距离");
        assert_eq!(asc.name, aspect.p0);
        assert_eq!(p.name, aspect.p1);

        let aspect = p.has_aspect(&asc, false);
        assert!(aspect.is_some());
        let aspect = aspect.unwrap();
        assert_eq!(
            aspect_value, aspect.aspect_value,
            "离相位中，行星{aspect_value}度相位asc"
        );
        assert_eq!(false, aspect.apply, "离相位中，asc离相位行星");
        assert_eq!(1.0, aspect.d, "离相位中，asc与行星距离");
        assert_eq!(p.name, aspect.p0);
        assert_eq!(asc.name, aspect.p1);

        // 离相位中，在金牛座0度
        asc.long = 30.0;
        let aspect = asc.has_aspect(&p, false);
        assert!(
            aspect.is_none(),
            "无相位，度数上离相位中，asc{aspect_value}度相位行星"
        );

        let aspect = p.has_aspect(&asc, false);
        assert!(
            aspect.is_none(),
            "无相位，度数上离相位中，行星{aspect_value}度相位asc"
        );

        // 即将无相位
        asc.long = 33.0;
        let aspect = asc.has_aspect(&p, false);
        assert!(
            aspect.is_none(),
            "无相位，度数上即将无相位，asc{aspect_value}度相位行星"
        );

        let aspect = p.has_aspect(&asc, false);
        assert!(
            aspect.is_none(),
            "无相位，度数上即将无相位，行星{aspect_value}度相位asc"
        );

        // 已经无相位
        asc.long = 34.0;
        let aspect = asc.has_aspect(&p, false);
        assert!(aspect.is_none(), "已经无相位，asc{aspect_value}度相位行星");

        let aspect = p.has_aspect(&asc, false);
        assert!(aspect.is_none(), "已经无相位，行星{aspect_value}度相位asc");
    }
}
