use std::env;

use geo_position::GeoPosition;
use horo_date_time::HoroDateTime;

use crate::{
    config::PlanetConfig,
    house::HouseName,
    return_horoscop::{lunar_return, solar_return},
};

#[test]
fn test_solar_return() {
    dotenvy::dotenv().ok();
    let ephe_path = env::var("EPHE_PATH")
        .expect("没设置 EPHE_PATH 环境变量，可在.env文件中设置或export EPHE_PATH=...");

    // 设置本命时间
    let native_date = HoroDateTime::new(1990, 6, 15, 10, 30, 0, 8.0).unwrap();

    // 设置推运时间
    let process_date = HoroDateTime::new(2023, 12, 26, 12, 0, 0, 7.0).unwrap();

    // 设置地理位置
    let geo = GeoPosition::new(116.4, 39.9).unwrap(); // 北京

    // 设置宫位系统
    let house_name = HouseName::Alcabitus;

    // 设置行星配置
    let planet_configs = PlanetConfig::default_all_configs();

    // 测试太阳返照函数
    let result = solar_return(
        native_date.clone(),
        process_date.clone(),
        geo,
        house_name.clone(),
        &planet_configs,
        &ephe_path,
    );

    assert!(result.is_ok(), "太阳返照计算应该成功");

    let solar_return_horo = result.unwrap();

    // 验证基本属性
    assert_eq!(solar_return_horo.native_date.jd_utc, native_date.jd_utc);
    assert_eq!(solar_return_horo.process_date.jd_utc, process_date.jd_utc);
    assert_eq!(solar_return_horo.geo.long, geo.long);
    assert_eq!(solar_return_horo.geo.lat, geo.lat);
    assert!(matches!(solar_return_horo.house_name, HouseName::Alcabitus));

    // 验证返照时间
    assert_eq!(solar_return_horo.return_date.year, 2023);
    assert_eq!(solar_return_horo.return_date.month, 6);
    assert_eq!(solar_return_horo.return_date.day, 15);
    assert_eq!(solar_return_horo.return_date.hour, 8);
    assert_eq!(solar_return_horo.return_date.minute, 57);
    assert_eq!(solar_return_horo.return_date.second, 21);
    assert_eq!(solar_return_horo.return_date.tz, 7.0);

    // 验证星盘数据存在
    assert!(!solar_return_horo.houses_cups.is_empty(), "应该有宫位数据");
    assert_eq!(solar_return_horo.houses_cups.len(), 12, "应该有12个宫位");
    assert!(!solar_return_horo.planets.is_empty(), "应该有行星数据");
    assert!(!solar_return_horo.aspects.is_empty(), "应该有相位数据");
}

#[test]
fn test_lunar_return() {
    dotenvy::dotenv().ok();
    let ephe_path = env::var("EPHE_PATH")
        .expect("没设置 EPHE_PATH 环境变量，可在.env文件中设置或export EPHE_PATH=...");

    // 设置本命时间
    let native_date = HoroDateTime::new(1990, 6, 15, 10, 30, 0, 8.0).unwrap();

    // 设置推运时间
    let process_date = HoroDateTime::new(2023, 12, 26, 12, 0, 0, 7.0).unwrap();

    // 设置地理位置
    let geo = GeoPosition::new(116.4, 39.9).unwrap(); // 北京

    // 设置宫位系统
    let house_name = HouseName::Alcabitus;

    // 设置行星配置
    let planet_configs = PlanetConfig::default_all_configs();

    // 测试月亮返照函数
    let result = lunar_return(
        native_date.clone(),
        process_date.clone(),
        geo.clone(),
        house_name.clone(),
        &planet_configs,
        &ephe_path,
    );

    assert!(result.is_ok(), "月亮返照计算应该成功");

    let lunar_return_horo = result.unwrap();

    // 验证基本属性
    assert_eq!(lunar_return_horo.native_date.jd_utc, native_date.jd_utc);
    assert_eq!(lunar_return_horo.process_date.jd_utc, process_date.jd_utc);
    assert_eq!(lunar_return_horo.geo.long, geo.long);
    assert_eq!(lunar_return_horo.geo.lat, geo.lat);
    assert!(matches!(lunar_return_horo.house_name, HouseName::Alcabitus));

    // 验证返照时间
    assert_eq!(lunar_return_horo.return_date.year, 2023);
    assert_eq!(lunar_return_horo.return_date.month, 12);
    assert_eq!(lunar_return_horo.return_date.day, 18);
    assert_eq!(lunar_return_horo.return_date.hour, 19);
    assert_eq!(lunar_return_horo.return_date.minute, 56);
    assert_eq!(lunar_return_horo.return_date.second, 51);
    assert_eq!(lunar_return_horo.return_date.tz, 7.0);

    // 验证星盘数据存在
    assert!(!lunar_return_horo.houses_cups.is_empty(), "应该有宫位数据");
    assert_eq!(lunar_return_horo.houses_cups.len(), 12, "应该有12个宫位");
    assert!(!lunar_return_horo.planets.is_empty(), "应该有行星数据");
    assert!(!lunar_return_horo.aspects.is_empty(), "应该有相位数据");
}
