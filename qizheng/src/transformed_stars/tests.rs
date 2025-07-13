use std::env;

use crate::{
    planet::name::PlanetName,
    ten_gods::TenGods,
    transformed_stars::{TransformedStar, calculate_tianlu, transformed_stars},
};
use lunar_calendar::lunar_calendar;

// Test for `calculate_tianlu`
// 2024 is 甲辰
// 2025 is 乙巳
// 2026 is 丙午
// 2027 is 丁未
// 2028 is 戊申
// 2029 is 己酉
// 2030 is 庚戌
// 2031 is 辛亥
// 2032 is 壬子
// 2033 is 癸丑

const MONTH: u8 = 7;
const DAY: u8 = 13;
const HOUR: u8 = 20;
const MINUTE: u8 = 0;
const SECOND: u8 = 0;

#[test]
fn test_calculate_tianlu() {
    dotenvy::dotenv().ok();
    let ephe_path = env::var("EPHE_PATH")
        .expect("没设置 EPHE_PATH 环境变量，可在.env文件中设置或export EPHE_PATH=...");
    // 甲
    let l = lunar_calendar(2024, MONTH, DAY, HOUR, MINUTE, SECOND, &ephe_path).unwrap();
    assert_eq!(calculate_tianlu(&l), PlanetName::火);

    // 乙
    let l = lunar_calendar(2025, MONTH, DAY, HOUR, MINUTE, SECOND, &ephe_path).unwrap();
    assert_eq!(calculate_tianlu(&l), PlanetName::孛);

    // 丙
    let l = lunar_calendar(2026, MONTH, DAY, HOUR, MINUTE, SECOND, &ephe_path).unwrap();
    assert_eq!(calculate_tianlu(&l), PlanetName::木);

    // 丁
    let l = lunar_calendar(2027, MONTH, DAY, HOUR, MINUTE, SECOND, &ephe_path).unwrap();
    assert_eq!(calculate_tianlu(&l), PlanetName::金);

    // 戊
    let l = lunar_calendar(2028, MONTH, DAY, HOUR, MINUTE, SECOND, &ephe_path).unwrap();
    assert_eq!(calculate_tianlu(&l), PlanetName::土);

    // 己
    let l = lunar_calendar(2029, MONTH, DAY, HOUR, MINUTE, SECOND, &ephe_path).unwrap();
    assert_eq!(calculate_tianlu(&l), PlanetName::月);

    // 庚
    let l = lunar_calendar(2030, MONTH, DAY, HOUR, MINUTE, SECOND, &ephe_path).unwrap();
    assert_eq!(calculate_tianlu(&l), PlanetName::水);

    // 辛
    let l = lunar_calendar(2031, MONTH, DAY, HOUR, MINUTE, SECOND, &ephe_path).unwrap();
    assert_eq!(calculate_tianlu(&l), PlanetName::气);

    // 壬
    let l = lunar_calendar(2032, MONTH, DAY, HOUR, MINUTE, SECOND, &ephe_path).unwrap();
    assert_eq!(calculate_tianlu(&l), PlanetName::计);

    // 癸
    let l = lunar_calendar(2033, MONTH, DAY, HOUR, MINUTE, SECOND, &ephe_path).unwrap();
    assert_eq!(calculate_tianlu(&l), PlanetName::罗);
}

#[test]
fn test_transformed_star_to_planet() {
    dotenvy::dotenv().ok();
    let ephe_path = env::var("EPHE_PATH")
        .expect("没设置 EPHE_PATH 环境变量，可在.env文件中设置或export EPHE_PATH=...");

    let l = lunar_calendar(2025, MONTH, DAY, HOUR, MINUTE, SECOND, &ephe_path).unwrap();

    assert_eq!(TransformedStar::天禄.planet(&l), PlanetName::孛);
    assert_eq!(TransformedStar::天暗.planet(&l), PlanetName::木);
    assert_eq!(TransformedStar::天福.planet(&l), PlanetName::金);
    assert_eq!(TransformedStar::天耗.planet(&l), PlanetName::土);
    assert_eq!(TransformedStar::天荫.planet(&l), PlanetName::月);
    assert_eq!(TransformedStar::天贵.planet(&l), PlanetName::水);
    assert_eq!(TransformedStar::天刑.planet(&l), PlanetName::气);
    assert_eq!(TransformedStar::天印.planet(&l), PlanetName::计);
    assert_eq!(TransformedStar::天囚.planet(&l), PlanetName::罗);
    assert_eq!(TransformedStar::天权.planet(&l), PlanetName::火);
}

#[test]
fn test_transformed_star_house() {
    assert_eq!(TransformedStar::天禄.house(), "官禄");
    assert_eq!(TransformedStar::天暗.house(), "相貌",);
    assert_eq!(TransformedStar::天福.house(), "财帛、福德、迁移",);
    assert_eq!(TransformedStar::天耗.house(), "兄弟",);
    assert_eq!(TransformedStar::天荫.house(), "妻妾",);
    assert_eq!(TransformedStar::天贵.house(), "男女",);
    assert_eq!(TransformedStar::天刑.house(), "奴仆",);
    assert_eq!(TransformedStar::天印.house(), "田宅",);
    assert_eq!(TransformedStar::天囚.house(), "疾厄",);
    assert_eq!(TransformedStar::天权.house(), "命宫",);
}

#[test]
fn test_transformed_stars() {
    dotenvy::dotenv().ok();
    let ephe_path = env::var("EPHE_PATH")
        .expect("没设置 EPHE_PATH 环境变量，可在.env文件中设置或export EPHE_PATH=...");

    let l = lunar_calendar(2025, MONTH, DAY, HOUR, MINUTE, SECOND, &ephe_path).unwrap();
    let result = transformed_stars(&l);
    assert_eq!(result.len(), 10);

    assert_eq!(result[0].star, PlanetName::孛);
    assert_eq!(result[1].star, PlanetName::木);
    assert_eq!(result[2].star, PlanetName::金);
    assert_eq!(result[3].star, PlanetName::土);
    assert_eq!(result[4].star, PlanetName::月);
    assert_eq!(result[5].star, PlanetName::水);
    assert_eq!(result[6].star, PlanetName::气);
    assert_eq!(result[7].star, PlanetName::计);
    assert_eq!(result[8].star, PlanetName::罗);

    assert_eq!(result[0].transformed_star, TransformedStar::天禄);
    assert_eq!(result[1].transformed_star, TransformedStar::天暗);
    assert_eq!(result[2].transformed_star, TransformedStar::天福);
    assert_eq!(result[3].transformed_star, TransformedStar::天耗);
    assert_eq!(result[4].transformed_star, TransformedStar::天荫);
    assert_eq!(result[5].transformed_star, TransformedStar::天贵);
    assert_eq!(result[6].transformed_star, TransformedStar::天刑);
    assert_eq!(result[7].transformed_star, TransformedStar::天印);
    assert_eq!(result[8].transformed_star, TransformedStar::天囚);
    assert_eq!(result[9].transformed_star, TransformedStar::天权);

    assert_eq!(result[0].ten_gods, TenGods::比肩);
    assert_eq!(result[1].ten_gods, TenGods::伤官);
    assert_eq!(result[2].ten_gods, TenGods::食神);
    assert_eq!(result[3].ten_gods, TenGods::正财);
    assert_eq!(result[4].ten_gods, TenGods::偏财);
    assert_eq!(result[5].ten_gods, TenGods::正官);
    assert_eq!(result[6].ten_gods, TenGods::七杀);
    assert_eq!(result[7].ten_gods, TenGods::正印);
    assert_eq!(result[8].ten_gods, TenGods::偏印);
    assert_eq!(result[9].ten_gods, TenGods::劫财);
}
