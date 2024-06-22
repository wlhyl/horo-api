use crate::{Planet, PlanetConfig, PlanetName::*};

// 白羊座10度
#[test]
fn aries_10() {
    let p0 = Planet::new(
        Sun,
        10.0,
        1.0,
        1.0,
        1.0,
        1.0,
        &PlanetConfig::new(Sun, 0, 1.0, 2.0),
    );

    for long in 0..360 {
        let p1 = crate::Planet::new(
            Moon,
            f64::from(long),
            1.0,
            1.0,
            1.0,
            1.0,
            &PlanetConfig::new(Moon, 0, 1.0, 2.0),
        );

        let antiscoin = p0.has_antiscoin(&p1);
        if long == 170 {
            assert!(antiscoin.is_some())
        } else {
            assert!(antiscoin.is_none());
        }
    }
}

// 金牛座10度
#[test]
fn taurus_10() {
    let p0 = Planet::new(
        Sun,
        40.0,
        1.0,
        1.0,
        1.0,
        1.0,
        &PlanetConfig::new(Sun, 0, 1.0, 2.0),
    );

    for long in 0..360 {
        let p1 = crate::Planet::new(
            Moon,
            f64::from(long),
            1.0,
            1.0,
            1.0,
            1.0,
            &PlanetConfig::new(Moon, 0, 1.0, 2.0),
        );

        let antiscoin = p0.has_antiscoin(&p1);
        if long == 140 {
            assert!(antiscoin.is_some())
        } else {
            assert!(antiscoin.is_none());
        }
    }
}

// 双子座10度
#[test]
fn gemini_10() {
    let p0 = Planet::new(
        Sun,
        70.0,
        1.0,
        1.0,
        1.0,
        1.0,
        &PlanetConfig::new(Sun, 0, 1.0, 2.0),
    );

    for long in 0..360 {
        let p1 = crate::Planet::new(
            Moon,
            f64::from(long),
            1.0,
            1.0,
            1.0,
            1.0,
            &PlanetConfig::new(Moon, 0, 1.0, 2.0),
        );

        let antiscoin = p0.has_antiscoin(&p1);
        if long == 110 {
            assert!(antiscoin.is_some())
        } else {
            assert!(antiscoin.is_none());
        }
    }
}

// 巨蟹座10度
#[test]
fn cancer_10() {
    let p0 = Planet::new(
        Sun,
        100.0,
        1.0,
        1.0,
        1.0,
        1.0,
        &PlanetConfig::new(Sun, 0, 1.0, 2.0),
    );

    for long in 0..360 {
        let p1 = crate::Planet::new(
            Moon,
            f64::from(long),
            1.0,
            1.0,
            1.0,
            1.0,
            &PlanetConfig::new(Moon, 0, 1.0, 2.0),
        );

        let antiscoin = p0.has_antiscoin(&p1);
        if long == 80 {
            assert!(antiscoin.is_some())
        } else {
            assert!(antiscoin.is_none());
        }
    }
}

// 狮子座10度
#[test]
fn le0_10() {
    let p0 = Planet::new(
        Sun,
        130.0,
        1.0,
        1.0,
        1.0,
        1.0,
        &PlanetConfig::new(Sun, 0, 1.0, 2.0),
    );

    for long in 0..360 {
        let p1 = crate::Planet::new(
            Moon,
            f64::from(long),
            1.0,
            1.0,
            1.0,
            1.0,
            &PlanetConfig::new(Moon, 0, 1.0, 2.0),
        );

        let antiscoin = p0.has_antiscoin(&p1);
        if long == 50 {
            assert!(antiscoin.is_some())
        } else {
            assert!(antiscoin.is_none());
        }
    }
}

// 室女座10度
#[test]
fn virgo_10() {
    let p0 = Planet::new(
        Sun,
        160.0,
        1.0,
        1.0,
        1.0,
        1.0,
        &PlanetConfig::new(Sun, 0, 1.0, 2.0),
    );

    for long in 0..360 {
        let p1 = crate::Planet::new(
            Moon,
            f64::from(long),
            1.0,
            1.0,
            1.0,
            1.0,
            &PlanetConfig::new(Moon, 0, 1.0, 2.0),
        );

        let antiscoin = p0.has_antiscoin(&p1);
        if long == 20 {
            assert!(antiscoin.is_some())
        } else {
            assert!(antiscoin.is_none());
        }
    }
}

// 天秤座10度
#[test]
fn libra_10() {
    let p0 = Planet::new(
        Sun,
        190.0,
        1.0,
        1.0,
        1.0,
        1.0,
        &PlanetConfig::new(Sun, 0, 1.0, 2.0),
    );

    for long in 0..360 {
        let p1 = crate::Planet::new(
            Moon,
            f64::from(long),
            1.0,
            1.0,
            1.0,
            1.0,
            &PlanetConfig::new(Moon, 0, 1.0, 2.0),
        );

        let antiscoin = p0.has_antiscoin(&p1);
        if long == 350 {
            assert!(antiscoin.is_some())
        } else {
            assert!(antiscoin.is_none());
        }
    }
}

// 天蝎座10度
#[test]
fn scorpio_10() {
    let p0 = Planet::new(
        Sun,
        220.0,
        1.0,
        1.0,
        1.0,
        1.0,
        &PlanetConfig::new(Sun, 0, 1.0, 2.0),
    );

    for long in 0..360 {
        let p1 = crate::Planet::new(
            Moon,
            f64::from(long),
            1.0,
            1.0,
            1.0,
            1.0,
            &PlanetConfig::new(Moon, 0, 1.0, 2.0),
        );

        let antiscoin = p0.has_antiscoin(&p1);
        if long == 320 {
            assert!(antiscoin.is_some())
        } else {
            assert!(antiscoin.is_none());
        }
    }
}

// 射手座10度
#[test]
fn sagittarius_10() {
    let p0 = Planet::new(
        Sun,
        250.0,
        1.0,
        1.0,
        1.0,
        1.0,
        &PlanetConfig::new(Sun, 0, 1.0, 2.0),
    );

    for long in 0..360 {
        let p1 = crate::Planet::new(
            Moon,
            f64::from(long),
            1.0,
            1.0,
            1.0,
            1.0,
            &PlanetConfig::new(Moon, 0, 1.0, 2.0),
        );

        let antiscoin = p0.has_antiscoin(&p1);
        if long == 290 {
            assert!(antiscoin.is_some())
        } else {
            assert!(antiscoin.is_none());
        }
    }
}

// 摩羯座10度
#[test]
fn capricorn_10() {
    let p0 = Planet::new(
        Sun,
        280.0,
        1.0,
        1.0,
        1.0,
        1.0,
        &PlanetConfig::new(Sun, 0, 1.0, 2.0),
    );

    for long in 0..360 {
        let p1 = crate::Planet::new(
            Moon,
            f64::from(long),
            1.0,
            1.0,
            1.0,
            1.0,
            &PlanetConfig::new(Moon, 0, 1.0, 2.0),
        );

        let antiscoin = p0.has_antiscoin(&p1);
        if long == 260 {
            assert!(antiscoin.is_some())
        } else {
            assert!(antiscoin.is_none());
        }
    }
}

// 水瓶座10度
#[test]
fn aquarius_10() {
    let p0 = Planet::new(
        Sun,
        310.0,
        1.0,
        1.0,
        1.0,
        1.0,
        &PlanetConfig::new(Sun, 0, 1.0, 2.0),
    );

    for long in 0..360 {
        let p1 = crate::Planet::new(
            Moon,
            f64::from(long),
            1.0,
            1.0,
            1.0,
            1.0,
            &PlanetConfig::new(Moon, 0, 1.0, 2.0),
        );

        let antiscoin = p0.has_antiscoin(&p1);
        if long == 230 {
            assert!(antiscoin.is_some())
        } else {
            assert!(antiscoin.is_none());
        }
    }
}

// 双鱼座10度
#[test]
fn pisces_10() {
    let p0 = Planet::new(
        Sun,
        340.0,
        1.0,
        1.0,
        1.0,
        1.0,
        &PlanetConfig::new(Sun, 0, 1.0, 2.0),
    );

    for long in 0..360 {
        let p1 = crate::Planet::new(
            Moon,
            f64::from(long),
            1.0,
            1.0,
            1.0,
            1.0,
            &PlanetConfig::new(Moon, 0, 1.0, 2.0),
        );

        let antiscoin = p0.has_antiscoin(&p1);
        if long == 200 {
            assert!(antiscoin.is_some())
        } else {
            assert!(antiscoin.is_none());
        }
    }
}
