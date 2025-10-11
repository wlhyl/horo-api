use std::env;

use geo_position::GeoPosition;
use horo_date_time::HoroDateTime;
use swe::{
    Body, Flag, swe_calc_ut, swe_close, swe_cotrans, swe_degnorm, swe_houses, swe_set_ephe_path,
};

use crate::{
    HoroscopeComparison, PlanetName::*, config::PlanetConfig, house::HouseName,
    planet::PlanetSpeedState::*, utils::calc_eps,
};

#[test]
fn çtest_horoscope_compare_new() {
    dotenvy::dotenv().ok();
    let ephe_path = env::var("EPHE_PATH")
        .expect("没设置 EPHE_PATH 环境变量，可在.env文件中设置或export EPHE_PATH=...");

    let native_date = HoroDateTime::new(2021, 9, 14, 10, 30, 20, 8.0).unwrap();

    let compare_date = HoroDateTime::new(2023, 12, 26, 20, 14, 20, 8.0).unwrap();

    let geo = GeoPosition::new(
        102.0 + 41.0 / 60.0 + 59.0 / 3600.0,
        25.0 + 1.0 / 60.0 + 53.0 / 3600.0,
    )
    .unwrap();

    let process_geo = GeoPosition::new(
        120.0 + 41.0 / 60.0 + 59.0 / 3600.0,
        30.0 + 1.0 / 60.0 + 53.0 / 3600.0,
    )
    .unwrap();

    let house = HouseName::Alcabitus;
    let planet_configs = PlanetConfig::default_all_configs();

    let horo = HoroscopeComparison::new(
        native_date,
        compare_date,
        geo,
        process_geo,
        house,
        &planet_configs,
        &ephe_path,
    )
    .unwrap();

    // 时间
    assert_eq!(native_date.year, horo.original_date.year);
    assert_eq!(native_date.month, horo.original_date.month);
    assert_eq!(native_date.hour, horo.original_date.hour);
    assert_eq!(native_date.minute, horo.original_date.minute);
    assert_eq!(native_date.second, horo.original_date.second);
    assert_eq!(native_date.tz, horo.original_date.tz);

    assert_eq!(compare_date.year, horo.comparison_date.year);
    assert_eq!(compare_date.month, horo.comparison_date.month);
    assert_eq!(compare_date.hour, horo.comparison_date.hour);
    assert_eq!(compare_date.minute, horo.comparison_date.minute);
    assert_eq!(compare_date.second, horo.comparison_date.second);
    assert_eq!(compare_date.tz, horo.comparison_date.tz);

    // 大地经纬度
    assert_eq!(geo.long, horo.original_geo.long);
    assert_eq!(geo.lat, horo.original_geo.lat);

    assert_eq!(process_geo.long, horo.comparison_geo.long);
    assert_eq!(process_geo.lat, horo.comparison_geo.lat);

    // 宫位系统
    match horo.house_name {
        HouseName::Alcabitus => assert!(true),
        _ => assert!(false),
    }

    // 12宫
    let yy = swe_houses(native_date.jd_ut1, geo.lat, geo.long, &(&house).into());
    assert!(yy.is_ok(), "swe_houses()调用失败");
    let (houses_cups, ascmc) = yy.unwrap();
    let houses_cups = &houses_cups[1..13];

    assert_eq!(12, horo.houses_cups.len());

    for i in 0..12 {
        assert_eq!(houses_cups[i], horo.houses_cups[i]);
    }

    // 本盘四轴
    let eps = calc_eps(native_date.jd_utc, &ephe_path).unwrap();
    let asc_equator = swe_cotrans(ascmc[0], 0.0, 1.0, -eps);

    assert_eq!(ASC, horo.original_asc.name, "asc name");
    assert_eq!(ascmc[0], horo.original_asc.long, "asc 黄道经度");
    assert_eq!(0.0, horo.original_asc.lat, "asc 黄纬");
    assert_eq!(asc_equator[0], horo.original_asc.ra, "asc 赤经");
    assert_eq!(asc_equator[1], horo.original_asc.dec, "asc 赤纬");
    assert_eq!(0, horo.original_asc.orb, "asc 容许度");
    assert_eq!(均, horo.original_asc.speed_state, "asc速度是“均”");

    // mc
    let mc_equator = swe_cotrans(ascmc[1], 0.0, 1.0, -eps);
    assert_eq!(MC, horo.original_mc.name, "mc name");
    assert_eq!(ascmc[1], horo.original_mc.long, "mc 黄道经度");
    assert_eq!(0.0, horo.original_mc.lat, "mc 黄纬");
    assert_eq!(mc_equator[0], horo.original_mc.ra, "mc 赤经");
    assert_eq!(mc_equator[1], horo.original_mc.dec, "mc 赤纬");
    assert_eq!(0, horo.original_mc.orb, "mc 容许度");
    assert_eq!(均, horo.original_mc.speed_state, "mc速度是均");

    // DSC
    let dsc_equator = swe_cotrans(swe_degnorm(ascmc[0] + 180.0), 0.0, 1.0, -eps);
    assert_eq!(DSC, horo.original_dsc.name, "dsc name");
    assert_eq!(
        swe_degnorm(ascmc[0] + 180.0),
        horo.original_dsc.long,
        "dsc 黄道经度"
    );
    assert_eq!(0.0, horo.original_dsc.lat, "dsc 黄纬");
    assert_eq!(dsc_equator[0], horo.original_dsc.ra, "dsc 赤经");
    assert_eq!(dsc_equator[1], horo.original_dsc.dec, "dsc 赤纬");
    assert_eq!(0, horo.original_dsc.orb, "dsc 容许度");
    assert_eq!(均, horo.original_dsc.speed_state, "dsc速冻是均");

    // IC
    let ic_equator = swe_cotrans(swe_degnorm(ascmc[1] + 180.0), 0.0, 1.0, -eps);
    assert_eq!(IC, horo.original_ic.name, "ic name");
    assert_eq!(
        swe_degnorm(ascmc[1] + 180.0),
        horo.original_ic.long,
        "ic 黄道经度"
    );
    assert_eq!(0.0, horo.original_ic.lat, "ic 黄纬");
    assert_eq!(ic_equator[0], horo.original_ic.ra, "ic 赤经");
    assert_eq!(ic_equator[1], horo.original_ic.dec, "ic 赤纬");
    assert_eq!(0, horo.original_ic.orb, "ic 容许度");
    assert_eq!(均, horo.original_ic.speed_state, "IC速度是均");

    // 比较盘12宫
    let yy = swe_houses(
        compare_date.jd_ut1,
        process_geo.lat,
        process_geo.long,
        &(&house).into(),
    );
    assert!(yy.is_ok(), "swe_houses()调用失败");
    let (_, ascmc) = yy.unwrap();

    // 比较盘四轴
    let eps = calc_eps(compare_date.jd_utc, &ephe_path).unwrap();

    let asc_equator = swe_cotrans(ascmc[0], 0.0, 1.0, -eps);
    assert_eq!(ASC, horo.comparison_asc.name, "asc name");
    assert_eq!(ascmc[0], horo.comparison_asc.long, "asc 黄道经度");
    assert_eq!(0.0, horo.comparison_asc.lat, "asc 黄纬");
    assert_eq!(asc_equator[0], horo.comparison_asc.ra, "asc 赤经");
    assert_eq!(asc_equator[1], horo.comparison_asc.dec, "asc 赤纬");
    assert_eq!(0, horo.comparison_asc.orb, "asc 容许度");
    assert_eq!(均, horo.comparison_asc.speed_state, "asc速度是“均”");

    // mc
    let mc_equator = swe_cotrans(ascmc[1], 0.0, 1.0, -eps);
    assert_eq!(MC, horo.comparison_mc.name, "mc name");
    assert_eq!(ascmc[1], horo.comparison_mc.long, "mc 黄道经度");
    assert_eq!(0.0, horo.comparison_mc.lat, "mc 黄纬");
    assert_eq!(mc_equator[0], horo.comparison_mc.ra, "mc 赤经");
    assert_eq!(mc_equator[1], horo.comparison_mc.dec, "mc 赤纬");
    assert_eq!(0, horo.comparison_mc.orb, "mc 容许度");
    assert_eq!(均, horo.comparison_mc.speed_state, "mc速度是均");

    // DSC
    let dsc_equator = swe_cotrans(swe_degnorm(ascmc[0] + 180.0), 0.0, 1.0, -eps);
    assert_eq!(DSC, horo.comparison_dsc.name, "dsc name");
    assert_eq!(
        swe_degnorm(ascmc[0] + 180.0),
        horo.comparison_dsc.long,
        "dsc 黄道经度"
    );
    assert_eq!(0.0, horo.comparison_dsc.lat, "dsc 黄纬");
    assert_eq!(dsc_equator[0], horo.comparison_dsc.ra, "dsc 赤经");
    assert_eq!(dsc_equator[1], horo.comparison_dsc.dec, "dsc 赤纬");
    assert_eq!(0, horo.comparison_dsc.orb, "dsc 容许度");
    assert_eq!(均, horo.comparison_dsc.speed_state, "dsc速冻是均");

    // IC
    let ic_equator = swe_cotrans(swe_degnorm(ascmc[1] + 180.0), 0.0, 1.0, -eps);
    assert_eq!(IC, horo.comparison_ic.name, "ic name");
    assert_eq!(
        swe_degnorm(ascmc[1] + 180.0),
        horo.comparison_ic.long,
        "ic 黄道经度"
    );
    assert_eq!(0.0, horo.comparison_ic.lat, "ic 黄纬");
    assert_eq!(ic_equator[0], horo.comparison_ic.ra, "ic 赤经");
    assert_eq!(ic_equator[1], horo.comparison_ic.dec, "ic 赤纬");
    assert_eq!(0, horo.comparison_ic.orb, "ic 容许度");
    assert_eq!(均, horo.comparison_ic.speed_state, "IC速度是均");

    // 七颗正星
    for planet_name in [Sun, Moon, Mercury, Venus, Mars, Jupiter, Saturn] {
        let p = horo
            .original_planets
            .iter()
            .find(|p| p.name == planet_name)
            .unwrap();

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
        let xx = swe_calc_ut(native_date.jd_utc, &body, &[Flag::SeflgSpeed]);
        let yy = swe_calc_ut(native_date.jd_utc, &body, &[Flag::SeflgEquatorial]); //计算赤经和赤纬
        swe_close();

        assert!(xx.is_ok(), "计算行星错误");
        assert!(yy.is_ok(), "计算行星错误");

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
        assert_eq!(xx[0], p.long, "{:?}黄经度", planet_name);
        assert_eq!(xx[1], p.lat, "{:?}黄纬度", planet_name);
        assert_eq!(xx[3], p.speed, "{:?}黄道上每日速度", planet_name);
        assert_eq!(yy[0], p.ra, "{:?}赤经度", planet_name);
        assert_eq!(yy[1], p.dec, "{:?}赤纬度", planet_name);
        assert_eq!(config.orb, p.orb, "{:?}容许度", planet_name);
        assert_eq!(speed_state, p.speed_state, "{:?}迟疾", planet_name);
    }

    // 比较盘七颗正星
    for planet_name in [Sun, Moon, Mercury, Venus, Mars, Jupiter, Saturn] {
        let p = horo
            .comparison_planets
            .iter()
            .find(|p| p.name == planet_name)
            .unwrap();

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
        let xx = swe_calc_ut(compare_date.jd_utc, &body, &[Flag::SeflgSpeed]);
        let yy = swe_calc_ut(compare_date.jd_utc, &body, &[Flag::SeflgEquatorial]); //计算赤经和赤纬
        swe_close();

        assert!(xx.is_ok(), "计算行星错误");
        assert!(yy.is_ok(), "计算行星错误");

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
        assert_eq!(xx[0], p.long, "{:?}黄经度", planet_name);
        assert_eq!(xx[1], p.lat, "{:?}黄纬度", planet_name);
        assert_eq!(xx[3], p.speed, "{:?}黄道上每日速度", planet_name);
        assert_eq!(yy[0], p.ra, "{:?}赤经度", planet_name);
        assert_eq!(yy[1], p.dec, "{:?}赤纬度", planet_name);
        assert_eq!(config.orb, p.orb, "{:?}容许度", planet_name);
        assert_eq!(speed_state, p.speed_state, "{:?}迟疾", planet_name);
    }

    // 月交点
    let north_node = horo
        .original_planets
        .iter()
        .find(|p| p.name == NorthNode)
        .unwrap();
    let south_node = horo
        .original_planets
        .iter()
        .find(|p| p.name == SouthNode)
        .unwrap();

    swe_set_ephe_path(&ephe_path);
    let xx = swe_calc_ut(native_date.jd_utc, &Body::SeMeanNode, &[Flag::SeflgSpeed]);
    let yy = swe_calc_ut(
        native_date.jd_utc,
        &Body::SeMeanNode,
        &[Flag::SeflgEquatorial],
    ); //计算赤经和赤纬
    swe_close();

    assert!(xx.is_ok(), "计算行星错误");
    assert!(yy.is_ok(), "计算行星错误");

    let xx = xx.unwrap();
    let yy = yy.unwrap();

    assert_eq!(NorthNode, north_node.name, "北交点");
    assert_eq!(xx[0], north_node.long, "北交点黄经度");
    assert_eq!(0.0, north_node.lat, "北交点黄纬度");
    assert_eq!(xx[3], north_node.speed, "黄道上每日速度, 北交点");
    assert_eq!(yy[0], north_node.ra, "北交点赤经度");
    assert_eq!(yy[1], north_node.dec, "北交点赤纬度");
    assert_eq!(0, north_node.orb, "容许度, 北交点");
    assert_eq!(均, north_node.speed_state, "迟疾, 北交点");

    assert_eq!(SouthNode, south_node.name, "南交点");
    assert_eq!(swe_degnorm(xx[0] + 180.0), south_node.long, "南交点黄经度");
    assert_eq!(0.0, south_node.lat, "南交点黄纬度");
    assert_eq!(xx[3], south_node.speed, "黄道上每日速度, 南交点");
    assert_eq!(swe_degnorm(yy[0] + 180.0), south_node.ra, "南交点赤经度");
    assert_eq!(-yy[1], south_node.dec, "南交点赤纬度");
    assert_eq!(0, south_node.orb, "容许度, 南交点");
    assert_eq!(均, south_node.speed_state, "迟疾, 南交点");

    // 比较盘月交点
    let north_node = horo
        .comparison_planets
        .iter()
        .find(|p| p.name == NorthNode)
        .unwrap();
    let south_node = horo
        .comparison_planets
        .iter()
        .find(|p| p.name == SouthNode)
        .unwrap();

    swe_set_ephe_path(&ephe_path);
    let xx = swe_calc_ut(compare_date.jd_utc, &Body::SeMeanNode, &[Flag::SeflgSpeed]);
    let yy = swe_calc_ut(
        compare_date.jd_utc,
        &Body::SeMeanNode,
        &[Flag::SeflgEquatorial],
    ); //计算赤经和赤纬
    swe_close();

    assert!(xx.is_ok(), "计算行星错误");
    assert!(yy.is_ok(), "计算行星错误");

    let xx = xx.unwrap();
    let yy = yy.unwrap();

    assert_eq!(NorthNode, north_node.name, "北交点");
    assert_eq!(xx[0], north_node.long, "北交点黄经度");
    assert_eq!(0.0, north_node.lat, "北交点黄纬度");
    assert_eq!(xx[3], north_node.speed, "黄道上每日速度, 北交点");
    assert_eq!(yy[0], north_node.ra, "北交点赤经度");
    assert_eq!(yy[1], north_node.dec, "北交点赤纬度");
    assert_eq!(0, north_node.orb, "容许度, 北交点");
    assert_eq!(均, north_node.speed_state, "迟疾, 北交点");

    assert_eq!(SouthNode, south_node.name, "南交点");
    assert_eq!(swe_degnorm(xx[0] + 180.0), south_node.long, "南交点黄经度");
    assert_eq!(0.0, south_node.lat, "南交点黄纬度");
    assert_eq!(xx[3], south_node.speed, "黄道上每日速度, 南交点");
    assert_eq!(swe_degnorm(yy[0] + 180.0), south_node.ra, "南交点赤经度");
    assert_eq!(-yy[1], south_node.dec, "南交点赤纬度");
    assert_eq!(0, south_node.orb, "容许度, 南交点");
    assert_eq!(均, south_node.speed_state, "迟疾, 南交点");

    // 福点
    let part_of_fortune = horo.original_part_of_fortune;
    let asc = horo.original_asc;
    let sun = horo
        .original_planets
        .iter()
        .find(|p| p.name == Sun)
        .unwrap();
    let moon = horo
        .original_planets
        .iter()
        .find(|p| p.name == Moon)
        .unwrap();

    let eps = calc_eps(native_date.jd_utc, &ephe_path).unwrap();

    let part_of_fortune_long = swe_degnorm(asc.long + moon.long - sun.long);
    let part_of_fortune_equator = swe_cotrans(part_of_fortune_long, 0.0, 1.0, -eps);

    assert_eq!(PartOfFortune, part_of_fortune.name, "原星盘福点名称");
    assert_eq!(
        part_of_fortune_long, part_of_fortune.long,
        "原星盘福点黄道经度"
    );
    assert_eq!(0.0, part_of_fortune.lat, "原星盘福点黄纬");
    assert_eq!(
        part_of_fortune_equator[0], part_of_fortune.ra,
        "原星盘福点赤经"
    );
    assert_eq!(
        part_of_fortune_equator[1], part_of_fortune.dec,
        "原星盘福点赤纬"
    );
    assert_eq!(0, part_of_fortune.orb, "原星盘福点容许度");
    assert_eq!(均, part_of_fortune.speed_state, "原星盘福点速度是“均”");

    let comparison_part_of_fortune = horo.comparison_part_of_fortune;
    let compa_asc = horo.comparison_asc;
    let compa_sun = horo
        .comparison_planets
        .iter()
        .find(|p| p.name == Sun)
        .unwrap();
    let compa_moon = horo
        .comparison_planets
        .iter()
        .find(|p| p.name == Moon)
        .unwrap();

    let eps = calc_eps(compare_date.jd_utc, &ephe_path).unwrap();

    let compa_part_of_fortune_long = swe_degnorm(compa_asc.long + compa_sun.long - compa_moon.long);
    let compa_part_of_fortune_equator = swe_cotrans(compa_part_of_fortune_long, 0.0, 1.0, -eps);

    assert_eq!(
        PartOfFortune, comparison_part_of_fortune.name,
        "比较盘福点名称"
    );
    assert_eq!(
        compa_part_of_fortune_long, comparison_part_of_fortune.long,
        "比较盘福点黄道经度"
    );
    assert_eq!(0.0, comparison_part_of_fortune.lat, "比较盘福点黄纬");
    assert_eq!(
        compa_part_of_fortune_equator[0], comparison_part_of_fortune.ra,
        "比较盘福点赤经"
    );
    assert_eq!(
        compa_part_of_fortune_equator[1], comparison_part_of_fortune.dec,
        "比较盘福点赤纬"
    );
    assert_eq!(0, comparison_part_of_fortune.orb, "比较盘福点容许度");
    assert_eq!(
        均, comparison_part_of_fortune.speed_state,
        "比较盘福点速度是“均”"
    );

    // 相位
    assert_eq!(38, horo.aspects.len());
}
