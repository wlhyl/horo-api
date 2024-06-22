use crate::{Planet, PlanetConfig, PlanetName::*};

// 不再逐度测试，因为逐度测试已由both_orb_0完成

// 白羊座2度，容许度3度，加上容许度跨星座
#[test]
fn aries_2() {
    let p0 = Planet::new(
        Sun,
        2.0,
        1.0,
        1.0,
        1.0,
        1.0,
        &PlanetConfig::new(Sun, 4, 1.0, 2.0),
    );

    // 映点在178

    // 还没有映点
    let p1 = crate::Planet::new(
        Moon,
        f64::from(178 - 4),
        1.0,
        1.0,
        1.0,
        1.0,
        &PlanetConfig::new(Moon, 2, 1.0, 2.0),
    );

    let antiscoin = p0.has_antiscoin(&p1);
    assert!(antiscoin.is_none());

    // 正好有映点
    let p1 = crate::Planet::new(
        Moon,
        f64::from(178 - 3),
        1.0,
        1.0,
        1.0,
        1.0,
        &PlanetConfig::new(Moon, 2, 1.0, 2.0),
    );

    let antiscoin = p0.has_antiscoin(&p1);
    assert!(antiscoin.is_some());
    let antiscoin = antiscoin.unwrap();
    assert_eq!(antiscoin.d, 3.0);

    // 已经有映点
    let p1 = crate::Planet::new(
        Moon,
        f64::from(178 - 2),
        1.0,
        1.0,
        1.0,
        1.0,
        &PlanetConfig::new(Moon, 2, 1.0, 2.0),
    );

    let antiscoin = p0.has_antiscoin(&p1);
    assert!(antiscoin.is_some());
    let antiscoin = antiscoin.unwrap();
    assert_eq!(antiscoin.d, 2.0);

    // 正好映点
    let p1 = crate::Planet::new(
        Moon,
        f64::from(178),
        1.0,
        1.0,
        1.0,
        1.0,
        &PlanetConfig::new(Moon, 2, 1.0, 2.0),
    );

    let antiscoin = p0.has_antiscoin(&p1);
    assert!(antiscoin.is_some());
    let antiscoin = antiscoin.unwrap();
    assert_eq!(antiscoin.d, 0.0);

    // 还有映点
    let p1 = crate::Planet::new(
        Moon,
        f64::from(178 + 1),
        1.0,
        1.0,
        1.0,
        1.0,
        &PlanetConfig::new(Moon, 2, 1.0, 2.0),
    );

    let antiscoin = p0.has_antiscoin(&p1);
    assert!(antiscoin.is_some());
    let antiscoin = antiscoin.unwrap();
    assert_eq!(antiscoin.d, 1.0);

    // 度数上有映点，但跨星座，不形成映点
    let p1 = crate::Planet::new(
        Moon,
        f64::from(178 + 2),
        1.0,
        1.0,
        1.0,
        1.0,
        &PlanetConfig::new(Moon, 2, 1.0, 2.0),
    );

    let antiscoin = p0.has_antiscoin(&p1);
    assert!(antiscoin.is_none());

    // 度数上即将无帅点，但跨星座，不形成映点
    let p1 = crate::Planet::new(
        Moon,
        f64::from(178 + 3),
        1.0,
        1.0,
        1.0,
        1.0,
        &PlanetConfig::new(Moon, 2, 1.0, 2.0),
    );

    let antiscoin = p0.has_antiscoin(&p1);
    assert!(antiscoin.is_none());

    // 无映点
    let p1 = crate::Planet::new(
        Moon,
        f64::from(178 + 4),
        1.0,
        1.0,
        1.0,
        1.0,
        &PlanetConfig::new(Moon, 2, 1.0, 2.0),
    );

    let antiscoin = p0.has_antiscoin(&p1);
    assert!(antiscoin.is_none());
}

// 白羊座28度，容许度3度，加上容许度跨星座
#[test]
fn aries_28() {
    let p0 = Planet::new(
        Sun,
        28.0,
        1.0,
        1.0,
        1.0,
        1.0,
        &PlanetConfig::new(Sun, 4, 1.0, 2.0),
    );

    // 映点在152

    // 还没有映点
    let p1 = crate::Planet::new(
        Moon,
        f64::from(152 - 4),
        1.0,
        1.0,
        1.0,
        1.0,
        &PlanetConfig::new(Moon, 2, 1.0, 2.0),
    );

    let antiscoin = p0.has_antiscoin(&p1);
    assert!(antiscoin.is_none());

    // 正好有映点,但跨星座，不能成映点
    let p1 = crate::Planet::new(
        Moon,
        f64::from(152 - 3),
        1.0,
        1.0,
        1.0,
        1.0,
        &PlanetConfig::new(Moon, 2, 1.0, 2.0),
    );

    let antiscoin = p0.has_antiscoin(&p1);
    assert!(antiscoin.is_none());

    // 已经有映点
    let p1 = crate::Planet::new(
        Moon,
        f64::from(152 - 2),
        1.0,
        1.0,
        1.0,
        1.0,
        &PlanetConfig::new(Moon, 2, 1.0, 2.0),
    );

    let antiscoin = p0.has_antiscoin(&p1);
    assert!(antiscoin.is_some());
    let antiscoin = antiscoin.unwrap();
    assert_eq!(antiscoin.d, 2.0);

    // 正好映点
    let p1 = crate::Planet::new(
        Moon,
        f64::from(152),
        1.0,
        1.0,
        1.0,
        1.0,
        &PlanetConfig::new(Moon, 2, 1.0, 2.0),
    );

    let antiscoin = p0.has_antiscoin(&p1);
    assert!(antiscoin.is_some());
    let antiscoin = antiscoin.unwrap();
    assert_eq!(antiscoin.d, 0.0);

    // 还有映点
    let p1 = crate::Planet::new(
        Moon,
        f64::from(152 + 1),
        1.0,
        1.0,
        1.0,
        1.0,
        &PlanetConfig::new(Moon, 2, 1.0, 2.0),
    );

    let antiscoin = p0.has_antiscoin(&p1);
    assert!(antiscoin.is_some());
    let antiscoin = antiscoin.unwrap();
    assert_eq!(antiscoin.d, 1.0);

    // 还有映点
    let p1 = crate::Planet::new(
        Moon,
        f64::from(152 + 2),
        1.0,
        1.0,
        1.0,
        1.0,
        &PlanetConfig::new(Moon, 2, 1.0, 2.0),
    );

    let antiscoin = p0.has_antiscoin(&p1);
    assert!(antiscoin.is_some());
    let antiscoin = antiscoin.unwrap();
    assert_eq!(antiscoin.d, 2.0);

    // 即将无映点
    let p1 = crate::Planet::new(
        Moon,
        f64::from(152 + 3),
        1.0,
        1.0,
        1.0,
        1.0,
        &PlanetConfig::new(Moon, 2, 1.0, 2.0),
    );

    let antiscoin = p0.has_antiscoin(&p1);
    assert!(antiscoin.is_some());
    let antiscoin = antiscoin.unwrap();
    assert_eq!(antiscoin.d, 3.0);

    // 无映点
    let p1 = crate::Planet::new(
        Moon,
        f64::from(152 + 4),
        1.0,
        1.0,
        1.0,
        1.0,
        &PlanetConfig::new(Moon, 2, 1.0, 2.0),
    );

    let antiscoin = p0.has_antiscoin(&p1);
    assert!(antiscoin.is_none());
}




// 白羊座10度，容许度3度，不跨星座
#[test]
fn aries_10() {
    let p0 = Planet::new(
        Sun,
        10.0,
        1.0,
        1.0,
        1.0,
        1.0,
        &PlanetConfig::new(Sun, 4, 1.0, 2.0),
    );

    // 映点在170

    // 还没有映点
    let p1 = crate::Planet::new(
        Moon,
        f64::from(170 - 4),
        1.0,
        1.0,
        1.0,
        1.0,
        &PlanetConfig::new(Moon, 2, 1.0, 2.0),
    );

    let antiscoin = p0.has_antiscoin(&p1);
    assert!(antiscoin.is_none());

    // 正好有映点
    let p1 = crate::Planet::new(
        Moon,
        f64::from(170 - 3),
        1.0,
        1.0,
        1.0,
        1.0,
        &PlanetConfig::new(Moon, 2, 1.0, 2.0),
    );

    let antiscoin = p0.has_antiscoin(&p1);
    assert!(antiscoin.is_some());
    let antiscoin = antiscoin.unwrap();
    assert_eq!(antiscoin.d, 3.0);

    // 已经有映点
    let p1 = crate::Planet::new(
        Moon,
        f64::from(170 - 2),
        1.0,
        1.0,
        1.0,
        1.0,
        &PlanetConfig::new(Moon, 2, 1.0, 2.0),
    );

    let antiscoin = p0.has_antiscoin(&p1);
    assert!(antiscoin.is_some());
    let antiscoin = antiscoin.unwrap();
    assert_eq!(antiscoin.d, 2.0);

    // 正好映点
    let p1 = crate::Planet::new(
        Moon,
        f64::from(170),
        1.0,
        1.0,
        1.0,
        1.0,
        &PlanetConfig::new(Moon, 2, 1.0, 2.0),
    );

    let antiscoin = p0.has_antiscoin(&p1);
    assert!(antiscoin.is_some());
    let antiscoin = antiscoin.unwrap();
    assert_eq!(antiscoin.d, 0.0);

    // 还有映点
    let p1 = crate::Planet::new(
        Moon,
        f64::from(170 + 1),
        1.0,
        1.0,
        1.0,
        1.0,
        &PlanetConfig::new(Moon, 2, 1.0, 2.0),
    );

    let antiscoin = p0.has_antiscoin(&p1);
    assert!(antiscoin.is_some());
    let antiscoin = antiscoin.unwrap();
    assert_eq!(antiscoin.d, 1.0);

    // 还有映点
    let p1 = crate::Planet::new(
        Moon,
        f64::from(170 + 2),
        1.0,
        1.0,
        1.0,
        1.0,
        &PlanetConfig::new(Moon, 2, 1.0, 2.0),
    );

    let antiscoin = p0.has_antiscoin(&p1);
    assert!(antiscoin.is_some());
    let antiscoin = antiscoin.unwrap();
    assert_eq!(antiscoin.d, 2.0);

    // 即将无映点
    let p1 = crate::Planet::new(
        Moon,
        f64::from(170 + 3),
        1.0,
        1.0,
        1.0,
        1.0,
        &PlanetConfig::new(Moon, 2, 1.0, 2.0),
    );

    let antiscoin = p0.has_antiscoin(&p1);
    assert!(antiscoin.is_some());
    let antiscoin = antiscoin.unwrap();
    assert_eq!(antiscoin.d, 3.0);

    // 无映点
    let p1 = crate::Planet::new(
        Moon,
        f64::from(170 + 4),
        1.0,
        1.0,
        1.0,
        1.0,
        &PlanetConfig::new(Moon, 2, 1.0, 2.0),
    );

    let antiscoin = p0.has_antiscoin(&p1);
    assert!(antiscoin.is_none());
}
