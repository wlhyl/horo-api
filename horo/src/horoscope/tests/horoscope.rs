use std::env;

use geo_position::GeoPosition;
use horo_date_time::HoroDateTime;
use swe::{
    Body, Flag, swe_calc_ut, swe_close, swe_cotrans, swe_degnorm, swe_houses, swe_set_ephe_path,
};

use crate::{
    Horoscope,
    PlanetName::{self, *},
    config::PlanetConfig,
    house::HouseName,
    planet::PlanetSpeedState::*,
    utils::calc_eps,
};

#[test]
fn test_horoscope_new() {
    dotenvy::dotenv().ok();
    let ephe_path = env::var("EPHE_PATH")
        .expect("没设置 EPHE_PATH 环境变量，可在.env文件中设置或export EPHE_PATH=...");

    let t = HoroDateTime::new(2021, 9, 14, 10, 30, 20, 8.0);
    assert!(t.is_ok());
    let t = t.unwrap();

    let geo = GeoPosition::new(
        102.0 + 41.0 / 60.0 + 59.0 / 3600.0,
        25.0 + 1.0 / 60.0 + 53.0 / 3600.0,
    );
    assert!(geo.is_ok());
    let geo = geo.unwrap();

    let house = HouseName::Alcabitus;
    let planet_configs = PlanetConfig::default_all_configs();

    let horo = Horoscope::new(
        t.clone(),
        geo.clone(),
        house.clone(),
        &planet_configs,
        &ephe_path,
    );
    assert!(horo.is_ok());
    let horo = horo.unwrap();

    // 时间
    assert_eq!(t.year, horo.date.year);
    assert_eq!(t.month, horo.date.month);
    assert_eq!(t.hour, horo.date.hour);
    assert_eq!(t.minute, horo.date.minute);
    assert_eq!(t.second, horo.date.second);
    assert_eq!(t.tz, horo.date.tz);

    // 大地经纬度
    assert_eq!(geo.long, horo.geo.long); //, this.doubleDelta)
    assert_eq!(geo.lat, horo.geo.lat); //, this.doubleDelta)

    // 宫位系统
    match horo.house_name {
        HouseName::Alcabitus => assert!(true),
        _ => assert!(false),
    }

    // 12宫
    let yy = swe_houses(t.jd_ut1, geo.lat, geo.long, &(&house).into());
    assert!(yy.is_ok(), "swe_houses()调用失败");
    let (houses_cups, ascmc) = yy.unwrap();
    let houses_cups = &houses_cups[1..13];

    assert_eq!(12, horo.houses_cups.len());

    for i in 0..12 {
        assert_eq!(houses_cups[i], horo.houses_cups[i]);
    }

    // 四轴
    let eps = calc_eps(t.jd_utc, &ephe_path);
    assert!(eps.is_ok());
    let eps = eps.unwrap();
    // 0: ASC, 1: MC
    let asc_equator = swe_cotrans(ascmc[0], 0.0, 1.0, -eps);
    assert_eq!(ASC, horo.asc.name, "asc name");
    assert_eq!(ascmc[0], horo.asc.long, "asc 黄道经度");
    assert_eq!(0.0, horo.asc.lat, "asc 黄纬");
    assert_eq!(asc_equator[0], horo.asc.ra, "asc 赤经");
    assert_eq!(asc_equator[1], horo.asc.dec, "asc 赤纬");
    assert_eq!(0, horo.asc.orb, "asc 容许度");
    assert_eq!(均, horo.asc.speed_state, "asc速度是“均”");

    // mc
    let mc_equator = swe_cotrans(ascmc[1], 0.0, 1.0, -eps);
    assert_eq!(MC, horo.mc.name, "mc name");
    assert_eq!(ascmc[1], horo.mc.long, "mc 黄道经度");
    assert_eq!(0.0, horo.mc.lat, "mc 黄纬");
    assert_eq!(mc_equator[0], horo.mc.ra, "mc 赤经");
    assert_eq!(mc_equator[1], horo.mc.dec, "mc 赤纬");
    assert_eq!(0, horo.mc.orb, "mc 容许度");
    assert_eq!(均, horo.mc.speed_state, "mc速度是均");

    // DSC
    let dsc_equator = swe_cotrans(swe_degnorm(ascmc[0] + 180.0), 0.0, 1.0, -eps);
    assert_eq!(DSC, horo.dsc.name, "dsc name");
    assert_eq!(swe_degnorm(ascmc[0] + 180.0), horo.dsc.long, "dsc 黄道经度");
    assert_eq!(0.0, horo.dsc.lat, "dsc 黄纬");
    assert_eq!(dsc_equator[0], horo.dsc.ra, "dsc 赤经");
    assert_eq!(dsc_equator[1], horo.dsc.dec, "dsc 赤纬");
    assert_eq!(0, horo.dsc.orb, "dsc 容许度");
    assert_eq!(均, horo.dsc.speed_state, "dsc速冻是均");

    // IC
    let ic_equator = swe_cotrans(swe_degnorm(ascmc[1] + 180.0), 0.0, 1.0, -eps);
    assert_eq!(IC, horo.ic.name, "ic name");
    assert_eq!(swe_degnorm(ascmc[1] + 180.0), horo.ic.long, "ic 黄道经度");
    assert_eq!(0.0, horo.ic.lat, "ic 黄纬");
    assert_eq!(ic_equator[0], horo.ic.ra, "ic 赤经");
    assert_eq!(ic_equator[1], horo.ic.dec, "ic 赤纬");
    assert_eq!(0, horo.ic.orb, "ic 容许度");
    assert_eq!(均, horo.ic.speed_state, "IC速度是均");

    // 七颗正星
    for planet_name in [Sun, Moon, Mercury, Venus, Mars, Jupiter, Saturn] {
        let p = horo.planets.iter().find(|p| p.name == planet_name);
        assert!(p.is_some());
        let p = p.unwrap();

        let body = match planet_name {
            Sun => Body::SeSun,
            Moon => Body::SeMoon,
            Mercury => Body::SeMercury,
            Venus => Body::SeVenus,
            Mars => Body::SeMars,
            Jupiter => Body::SeJupiter,
            _ => Body::SeSaturn, // Saturn
        };

        swe_set_ephe_path(&ephe_path);
        let xx = swe_calc_ut(t.jd_utc, &body, &[Flag::SeflgSpeed]);
        let yy = swe_calc_ut(t.jd_utc, &body, &[Flag::SeflgEquatorial]); //计算赤经和赤纬

        assert!(xx.is_ok(), "计算行星错误");
        assert!(yy.is_ok(), "计算行星错误");
        swe_close();

        let xx = xx.unwrap();
        let yy = yy.unwrap();

        let config = PlanetConfig::default_config(&planet_name);
        let speed_state = if config.max > config.min {
            if xx[3].abs() > config.max {
                快
            } else if xx[3].abs() < config.min {
                慢
            } else {
                均
            }
        } else {
            均
        };
        assert_eq!(planet_name, p.name);
        assert_eq!(xx[0], p.long, "{:?}", planet_name);
        assert_eq!(xx[1], p.lat, "{:?}黄纬", planet_name);
        assert_eq!(xx[3], p.speed, "{:?}黄道上每日速度", planet_name);
        assert_eq!(yy[0], p.ra, "{:?}赤经", planet_name);
        assert_eq!(yy[1], p.dec, "{:?}赤纬", planet_name);
        assert_eq!(config.orb, p.orb, "{:?}容许度", planet_name);
        assert_eq!(speed_state, p.speed_state, "{:?}迟疾", planet_name);
    }

    // 月交点
    let north_node = horo.planets.iter().find(|p| p.name == NorthNode);
    let south_node = horo.planets.iter().find(|p| p.name == SouthNode);

    assert!(north_node.is_some());
    assert!(south_node.is_some());

    let north_node = north_node.unwrap();
    let south_node = south_node.unwrap();

    swe_set_ephe_path(&ephe_path);
    let xx = swe_calc_ut(t.jd_utc, &Body::SeMeanNode, &[Flag::SeflgSpeed]);
    let yy = swe_calc_ut(t.jd_utc, &Body::SeMeanNode, &[Flag::SeflgEquatorial]); //计算赤经和赤纬

    assert!(xx.is_ok(), "计算行星错误");
    assert!(yy.is_ok(), "计算行星错误");
    swe_close();

    let xx = xx.unwrap();
    let yy = yy.unwrap();

    assert_eq!(NorthNode, north_node.name, "北交点");
    assert_eq!(xx[0], north_node.long, "黄经，北交点");
    assert_eq!(0.0, north_node.lat, "黄纬, 北交点");
    assert_eq!(xx[3], north_node.speed, "黄道上每日速度, 北交点");
    assert_eq!(yy[0], north_node.ra, "赤经, 北交点");
    assert_eq!(yy[1], north_node.dec, "赤纬, 北交点");
    assert_eq!(0, north_node.orb, "容许度, 北交点");
    assert_eq!(均, north_node.speed_state, "迟疾, 北交点");

    assert_eq!(SouthNode, south_node.name, "南交点");
    assert_eq!(swe_degnorm(xx[0] + 180.0), south_node.long, "黃经，南交点");
    assert_eq!(0.0, south_node.lat, "黄纬, 南交点");
    assert_eq!(xx[3], south_node.speed, "黄道上每日速度, 南交点");
    assert_eq!(swe_degnorm(yy[0] + 180.0), south_node.ra, "赤经, 南交点");
    assert_eq!(-yy[1], south_node.dec, "赤纬, 南交点");
    assert_eq!(0, south_node.orb, "容许度, 南交点");
    assert_eq!(均, south_node.speed_state, "迟疾, 南交点");

    // 相位
    assert_eq!(13, horo.aspects.len());

    // 恒星
    assert_eq!(12, horo.fixed_stars.len());
}

// 星盘昼夜
#[test]
fn test_diurnal() {
    dotenvy::dotenv().ok();
    let ephe_path = env::var("EPHE_PATH")
        .expect("没设置 EPHE_PATH 环境变量，可在.env文件中设置或export EPHE_PATH=...");

    let geo = GeoPosition::new(
        102.0 + 41.0 / 60.0 + 59.0 / 3600.0,
        25.0 + 1.0 / 60.0 + 53.0 / 3600.0,
    );
    assert!(geo.is_ok());
    let geo = geo.unwrap();

    let house = HouseName::Alcabitus;
    let planet_configs = PlanetConfig::default_all_configs();

    let diurnal = HoroDateTime::new(2021, 9, 14, 10, 30, 20, 8.0);
    let nocturnal = HoroDateTime::new(2021, 9, 14, 22, 30, 20, 8.0);

    assert!(diurnal.is_ok());
    assert!(nocturnal.is_ok());

    let diurnal = diurnal.unwrap();
    let nocturnal = nocturnal.unwrap();

    let horo_diurnal = Horoscope::new(
        diurnal,
        geo.clone(),
        house.clone(),
        &planet_configs,
        &ephe_path,
    );
    let horo_nocturnal = Horoscope::new(nocturnal, geo, house.clone(), &planet_configs, &ephe_path);

    assert!(horo_diurnal.is_ok());
    assert!(horo_nocturnal.is_ok());

    let horo_diurnal = horo_diurnal.unwrap();
    let horo_nocturnal = horo_nocturnal.unwrap();

    assert!(horo_diurnal.is_diurnal, "白天盘");
    assert!(!horo_nocturnal.is_diurnal, "夜间盘");

    // 此时刻太阳在地平线上，前一分钟，太阳在地平线下
    let t0 = HoroDateTime::new(2021, 9, 16, 7, 3, 0, 8.0);
    let t1 = HoroDateTime::new(2021, 9, 16, 7, 2, 0, 8.0);
    assert!(t0.is_ok());
    assert!(t1.is_ok());
    let t0 = t0.unwrap();
    let t1 = t1.unwrap();

    let geo = GeoPosition::new(102.0, 25.0);
    assert!(geo.is_ok());
    let geo = geo.unwrap();

    let h0 = Horoscope::new(t0, geo.clone(), house.clone(), &planet_configs, &ephe_path);
    let h1 = Horoscope::new(t1, geo, house, &planet_configs, &ephe_path);

    assert!(h0.is_ok());
    assert!(h1.is_ok());

    let h0 = h0.unwrap();
    let h1 = h1.unwrap();

    assert!(h0.is_diurnal, "白天盘");
    assert!(!h1.is_diurnal, "夜间盘");
}

// 日主星
#[test]
fn test_planetary_day() {
    dotenvy::dotenv().ok();
    let ephe_path = env::var("EPHE_PATH")
        .expect("没设置 EPHE_PATH 环境变量，可在.env文件中设置或export EPHE_PATH=...");
    //月、火、水、木、金、土、日
    // 一、二、三、四、五、六、日
    // 迦勒底序
    let chaldean_order = [Sun, Moon, Mars, Mercury, Jupiter, Venus, Saturn];

    // 2021-9-12 星期日，太阳掌管
    let t0 = HoroDateTime::new(2021, 9, 12, 10, 30, 20, 8.0);
    let t1 = HoroDateTime::new(2021, 9, 12, 22, 30, 20, 8.0);
    assert!(t0.is_ok());
    assert!(t1.is_ok());
    let t0 = t0.unwrap();
    let t1 = t1.unwrap();

    let geo = GeoPosition::new(
        102.0 + 41.0 / 60.0 + 59.0 / 3600.0,
        25.0 + 1.0 / 60.0 + 53.0 / 3600.0,
    );
    assert!(geo.is_ok());
    let geo = geo.unwrap();
    let house = HouseName::Alcabitus;
    let planet_configs = PlanetConfig::default_all_configs();

    for i in 0u8..7 {
        let horo0 = Horoscope::new(
            t0.plus_days(i.into()).unwrap(),
            geo.clone(),
            house.clone(),
            &planet_configs,
            &ephe_path,
        )
        .unwrap();
        let horo1 = Horoscope::new(
            t1.plus_days(i.into()).unwrap(),
            geo.clone(),
            house.clone(),
            &planet_configs,
            &ephe_path,
        )
        .unwrap();
        assert_eq!(chaldean_order[i as usize], horo0.planetary_day, "白天盘");
        assert_eq!(chaldean_order[i as usize], horo1.planetary_day, "夜间盘");
    }
}

// 时主星
#[test]
fn test_planetary_hours() {
    dotenvy::dotenv().ok();
    let ephe_path = env::var("EPHE_PATH")
        .expect("没设置 EPHE_PATH 环境变量，可在.env文件中设置或export ephe_path=...");

    // 2021-9-16 星期四，木星掌管
    // 此时刻，太阳在地平线上，前一分钟，太阳在地平线下
    let t0 = HoroDateTime::new(2021, 9, 16, 7, 3, 0, 8.0).unwrap();
    // 此时刻，太阳在地平线下，前一分钟，太阳在地平线上
    let t1 = HoroDateTime::new(2021, 9, 16, 19, 12, 0, 8.0).unwrap();
    //此时刻，太阳在地平线下，下一分钟，太阳在地平线上
    let t2 = HoroDateTime::new(2021, 9, 17, 7, 2, 0, 8.0).unwrap();

    let geo = GeoPosition::new(102.0, 25.0).unwrap();

    let planet_configs = PlanetConfig::default_all_configs();

    let planetary_hours_list = [Saturn, Jupiter, Mars, Sun, Venus, Mercury, Moon];

    assert_eq!(
        Saturn,
        Horoscope::new(
            HoroDateTime::new(2021, 9, 16, 7, 2, 0, 8.0).unwrap(), //t0-1分钟
            geo.clone(),
            HouseName::Alcabitus,
            &planet_configs,
            &ephe_path,
        )
        .unwrap()
        .planetary_hours,
        "太阳升起前一分钟的时主星，即2021-9月-15，最后一个时主星"
    );
    for i in 0u8..12 {
        let t = t0
            .plus_days((t1.jd_utc - t0.jd_utc) * f64::from(i) / 12.0)
            .unwrap();
        let h = Horoscope::new(t, geo, HouseName::Alcabitus, &planet_configs, &ephe_path).unwrap();
        assert_eq!(
            planetary_hours_list[((1 + i) % 7) as usize],
            h.planetary_hours,
            "白天，第{}个行星小时",
            i + 1
        );
    }

    assert_eq!(
        Moon,
        Horoscope::new(
            t1.clone(),
            geo.clone(),
            HouseName::Alcabitus,
            &planet_configs,
            &ephe_path,
        )
        .unwrap()
        .planetary_hours,
        "日落后第1个行星时，2021-9-16日，夜间第一个时主星，月亮"
    );

    for i in 0u8..11 {
        // t2 + 1分钟
        let t = t1
            .plus_days((t2.jd_utc + 1.0 / 2400.0 - t1.jd_utc) * f64::from(i) / 12.0)
            .unwrap();
        let h = Horoscope::new(
            t,
            geo.clone(),
            HouseName::Alcabitus,
            &planet_configs,
            &ephe_path,
        )
        .unwrap();
        // 夜间第1个行星小时的时主星是火星
        assert_eq!(
            planetary_hours_list[((6 + i) % 7) as usize],
            h.planetary_hours,
            "夜间，第{}个行星小时",
            i + 1
        )
    }

    assert_eq!(
        Sun,
        Horoscope::new(t2, geo, HouseName::Alcabitus, &planet_configs, &ephe_path)
            .unwrap()
            .planetary_hours,
        "日出前的行星时"
    )
}

// 福点
// 白天盘的福点
#[test]
fn test_daytime_fortunes() {
    dotenvy::dotenv().ok();
    let ephe_path = env::var("EPHE_PATH")
        .expect("没设置 EPHE_PATH 环境变量，可在.env文件中设置或export EPHE_PATH=...");

    let geo = GeoPosition::new(
        102.0 + 41.0 / 60.0 + 59.0 / 3600.0,
        25.0 + 1.0 / 60.0 + 53.0 / 3600.0,
    );
    let geo = geo.unwrap();

    let house = HouseName::Alcabitus;
    let planet_configs = PlanetConfig::default_all_configs();

    let diurnal = HoroDateTime::new(2021, 9, 14, 10, 30, 20, 8.0);

    let diurnal = diurnal.unwrap();

    let horo_diurnal = Horoscope::new(diurnal, geo, house, &planet_configs, &ephe_path);

    let horo_diurnal = horo_diurnal.unwrap();

    assert!(horo_diurnal.is_diurnal, "白天盘");

    // 计算福点
    let asc = horo_diurnal.asc.long;
    let sun = horo_diurnal
        .planets
        .iter()
        .find(|p| p.name == PlanetName::Sun)
        .unwrap()
        .long;
    let moon = horo_diurnal
        .planets
        .iter()
        .find(|p| p.name == PlanetName::Moon)
        .unwrap()
        .long;
    let part_of_fortune_long = swe_degnorm(asc + moon - sun);

    let eps = calc_eps(diurnal.jd_utc, &ephe_path);
    let eps = eps.unwrap();

    let part_of_fortune_equator = swe_cotrans(part_of_fortune_long, 0.0, 1.0, -eps);
    assert_eq!(
        PartOfFortune, horo_diurnal.part_of_fortune.name,
        "白天盘的福点名称"
    );
    assert_eq!(
        part_of_fortune_long, horo_diurnal.part_of_fortune.long,
        "白天盘的福点黄道经度"
    );
    assert_eq!(0.0, horo_diurnal.part_of_fortune.lat, "白天盘的福点黄纬");
    assert_eq!(
        part_of_fortune_equator[0], horo_diurnal.part_of_fortune.ra,
        "白天盘的福点赤经"
    );
    assert_eq!(
        part_of_fortune_equator[1], horo_diurnal.part_of_fortune.dec,
        "白天盘的福点赤纬"
    );
    assert_eq!(0, horo_diurnal.part_of_fortune.orb, "白天盘的福点容许度");
    assert_eq!(
        均, horo_diurnal.part_of_fortune.speed_state,
        "白天盘的福点速度是“均”"
    );
}

// 夜间盘的福点
#[test]
fn test_nocturnal_fortunes() {
    dotenvy::dotenv().ok();
    let ephe_path = env::var("EPHE_PATH")
        .expect("没设置 EPHE_PATH 环境变量，可在.env文件中设置或export EPHE_PATH=...");

    let geo = GeoPosition::new(
        102.0 + 41.0 / 60.0 + 59.0 / 3600.0,
        25.0 + 1.0 / 60.0 + 53.0 / 3600.0,
    );
    assert!(geo.is_ok());
    let geo = geo.unwrap();

    let house = HouseName::Alcabitus;
    let planet_configs = PlanetConfig::default_all_configs();

    let nocturnal = HoroDateTime::new(2021, 9, 14, 22, 30, 20, 8.0);

    let nocturnal = nocturnal.unwrap();

    let horo_nocturnal = Horoscope::new(nocturnal, geo, house.clone(), &planet_configs, &ephe_path);

    let horo_nocturnal = horo_nocturnal.unwrap();

    assert!(!horo_nocturnal.is_diurnal, "夜间盘");

    // 计算福点
    let asc = horo_nocturnal.asc.long;
    let sun = horo_nocturnal
        .planets
        .iter()
        .find(|p| p.name == PlanetName::Sun)
        .unwrap()
        .long;
    let moon = horo_nocturnal
        .planets
        .iter()
        .find(|p| p.name == PlanetName::Moon)
        .unwrap()
        .long;
    let part_of_fortune_long = swe_degnorm(asc + sun - moon);

    let eps = calc_eps(nocturnal.jd_utc, &ephe_path);
    let eps = eps.unwrap();

    let part_of_fortune_equator = swe_cotrans(part_of_fortune_long, 0.0, 1.0, -eps);
    assert_eq!(
        PartOfFortune, horo_nocturnal.part_of_fortune.name,
        "夜间盘的福点名称"
    );
    assert_eq!(
        part_of_fortune_long, horo_nocturnal.part_of_fortune.long,
        "夜间盘的福点黄道经度"
    );
    assert_eq!(0.0, horo_nocturnal.part_of_fortune.lat, "夜间盘的福点黄纬");
    assert_eq!(
        part_of_fortune_equator[0], horo_nocturnal.part_of_fortune.ra,
        "夜间盘的福点赤经"
    );
    assert_eq!(
        part_of_fortune_equator[1], horo_nocturnal.part_of_fortune.dec,
        "夜间盘的福点赤纬"
    );
    assert_eq!(0, horo_nocturnal.part_of_fortune.orb, "夜间盘的福点容许度");
    assert_eq!(
        均, horo_nocturnal.part_of_fortune.speed_state,
        "夜间盘的福点速度是“均”"
    );
}
