use horo_date_time::HoroDateTime;
use swe::{Body, Flag, swe_calc_ut, swe_close, swe_degnorm, swe_set_ephe_path};

use crate::{
    DistanceStarLong,
    LunarMansionsName::*,
    config::{DistanceStarConfig, PlanetConfig},
    lunar_mansions::calc_distance_star_long,
};

use super::{Planet, PlanetName::*, planet_speed_state::*};

#[test]
fn test_new() {
    let p = Planet::new(
        日,
        1.0,
        1.0,
        角,
        1.0,
        false,
        &PlanetConfig::new(日, 1.0, 2.0),
    );

    assert_eq!(日, p.name, "name");
    assert_eq!(1.0, p.long, "黄经");
    assert_eq!(1.0, p.speed, "speed");
    assert_eq!(角, p.xiu, "二十八宿");
    assert_eq!(1.0, p.xiu_degree, "speed");
    assert!(!p.is_stationary);
}

// 快
#[test]
fn test_faster() {
    // 逆行
    let p0 = Planet::new(
        日,
        1.0,
        -3.1,
        角,
        1.0,
        false,
        &PlanetConfig::new(日, 1.0, 2.0),
    );
    assert_eq!(PlanetSpeedState::疾, p0.speed_state, "逆行，快");

    // 顺行
    let p1 = Planet::new(
        日,
        1.0,
        3.1,
        角,
        1.0,
        false,
        &PlanetConfig::new(日, 1.0, 2.0),
    );
    assert_eq!(PlanetSpeedState::疾, p1.speed_state, "顺行，快")
}

// 慢
#[test]
fn test_slower() {
    // 逆行
    let p0 = Planet::new(
        日,
        1.0,
        -0.1,
        角,
        1.0,
        false,
        &PlanetConfig::new(日, 1.0, 2.0),
    );
    assert_eq!(PlanetSpeedState::迟, p0.speed_state, "逆行，慢");

    // 顺行
    let p1 = Planet::new(
        日,
        1.0,
        0.1,
        角,
        1.0,
        false,
        &PlanetConfig::new(日, 1.0, 2.0),
    );
    assert_eq!(PlanetSpeedState::迟, p1.speed_state, "顺行，慢");
}

// 平均
#[test]
fn test_average() {
    let p0 = Planet::new(
        日,
        1.0,
        -0.1,
        角,
        1.0,
        false,
        &PlanetConfig::new(日, 2.0, 2.0),
    );
    assert_eq!(PlanetSpeedState::均, p0.speed_state, "逆行，均");

    let p1 = Planet::new(
        日,
        1.0,
        0.1,
        角,
        1.0,
        false,
        &PlanetConfig::new(日, 2.0, 2.0),
    );
    assert_eq!(PlanetSpeedState::均, p1.speed_state, "顺行，均");
}

// 停滞
#[test]
fn test_stationary() {
    // 日、月、计、罗、孛、气,无停滞
    for planet_name in [日, 月, 计, 罗, 孛, 气] {
        // 顺行，停滞
        let p = Planet::new(
            planet_name,
            1.0,
            0.1,
            角,
            1.0,
            true,
            &PlanetConfig::new(日, 2.0, 2.0),
        );
        assert!(!p.is_stationary);

        // 逆行，停滞
        let p = Planet::new(
            planet_name,
            1.0,
            -0.1,
            角,
            1.0,
            true,
            &PlanetConfig::new(日, 2.0, 2.0),
        );
        assert!(!p.is_stationary);

        // 顺行，非停滞
        let p = Planet::new(
            planet_name,
            1.0,
            2.1,
            角,
            1.0,
            false,
            &PlanetConfig::new(日, 2.0, 2.0),
        );
        assert!(!p.is_stationary);

        // 逆行，非停滞
        let p = Planet::new(
            planet_name,
            1.0,
            -2.1,
            角,
            1.0,
            false,
            &PlanetConfig::new(日, 2.0, 2.0),
        );
        assert!(!p.is_stationary);
    }

    // 水、金、火、木、土,有停滞
    for planet_name in [水, 金, 火, 木, 土] {
        // 顺行，停滞
        let p = Planet::new(
            planet_name,
            1.0,
            0.1,
            角,
            1.0,
            true,
            &PlanetConfig::new(日, 2.0, 2.0),
        );
        assert!(p.is_stationary);

        // 逆行，停滞
        let p = Planet::new(
            planet_name,
            1.0,
            -0.1,
            角,
            1.0,
            true,
            &PlanetConfig::new(日, 2.0, 2.0),
        );
        assert!(p.is_stationary);

        // 顺行，非停滞
        let p = Planet::new(
            planet_name,
            1.0,
            2.1,
            角,
            1.0,
            false,
            &PlanetConfig::new(日, 2.0, 2.0),
        );
        assert!(!p.is_stationary);

        // 逆行，非停滞
        let p = Planet::new(
            planet_name,
            1.0,
            -2.1,
            角,
            1.0,
            false,
            &PlanetConfig::new(日, 2.0, 2.0),
        );
        assert!(!p.is_stationary);
    }
}

#[test]
fn test_calc_planets_long() {
    // Load environment variables from .env file
    dotenvy::dotenv().ok();

    // 2. 准备 ephe_path
    // 首先检查环境变量 EPHE_PATH
    // let  ephe_path = std::env::var("EPHE_PATH").unwrap_or_default();
    let ephe_path = std::env::var("EPHE_PATH")
        .expect("没设置 EPHE_PATH 环境变量，可在.env文件中设置或export EPHE_PATH=...");

    // 1. 设置一个固定的测试时间
    let dt = HoroDateTime::new(2025, 7, 13, 12, 0, 0, 8.0).unwrap();
    let jd_utc = dt.jd_utc;

    // 3. 生成 distance_star_long
    let distance_star_configs = DistanceStarConfig::default_all_configs();
    let distance_star_long =
        calc_distance_star_long(jd_utc, &distance_star_configs, &ephe_path).unwrap();

    // 4. 生成 planets_config
    let planets_config = PlanetConfig::default_all_configs();

    // 5. 调用 calc_planets 函数
    let planets_result =
        super::calc_planets(jd_utc, &distance_star_long, &planets_config, &ephe_path);

    // 6. 验证结果
    assert!(planets_result.is_ok(), "calc_planets should return Ok");
    let planets = planets_result.unwrap();

    // 7. 断言行星数量
    // 应该是11个: 日, 月, 水, 金, 火, 木, 土, 计, 孛, 气, 罗
    assert_eq!(planets.len(), 11, "There should be 11 planets");

    // 8. 对一些行星数据进行基本检查
    for p in &planets {
        assert!(
            p.long >= 0.0 && p.long < 360.0,
            "Longitude for {:?} is out of range",
            p.name
        );
        assert!(
            p.xiu_degree >= 0.0,
            "Xiu degree for {:?} is negative",
            p.name
        );
    }

    for (planet_name, planet_body) in [
        (日, Body::SeSun),
        (月, Body::SeMoon),
        (水, Body::SeMercury),
        (金, Body::SeVenus),
        (火, Body::SeMars),
        (木, Body::SeJupiter),
        (土, Body::SeSaturn),
        (孛, Body::SeMeanApog),
        (计, Body::SeMeanNode),
    ] {
        swe_set_ephe_path(&ephe_path);
        let xx = swe_calc_ut(jd_utc, &planet_body, &[Flag::SeflgSpeed]).unwrap();
        let long = xx[0];
        swe_close();

        let planet = planets
            .iter()
            .find(|p| p.name == planet_name)
            .expect(format!("Planet {:?} not found", planet_name).as_str());
        assert_eq!(planet.long, long);
    }

    // 检查停滞
    let saturn = planets
        .iter()
        .find(|p| p.name == 土)
        .expect("SeSaturn not found");
    assert!(saturn.is_stationary, "SeSaturn should be stationary");

    // 检查计都和罗喉的黄经是否相差180度
    let ketu = planets
        .iter()
        .find(|p| p.name == 计)
        .expect("Ketu not found");
    assert!(ketu.speed < 0.0);
    let rahu = planets
        .iter()
        .find(|p| p.name == 罗)
        .expect("Rahu not found");
    assert!(rahu.speed < 0.0);

    let diff = (ketu.long - rahu.long).abs();
    assert!(
        (diff - 180.0).abs() < 1e-9,
        "Ketu and Rahu should be 180 degrees apart"
    );
}

// 测试行星的宿度
#[test]
fn test_calc_planets_xiu_degree() {
    // Load environment variables from .env file
    dotenvy::dotenv().ok();

    // 2. 准备 ephe_path
    // 首先检查环境变量 EPHE_PATH
    // let  ephe_path = std::env::var("EPHE_PATH").unwrap_or_default();
    let ephe_path = std::env::var("EPHE_PATH")
        .expect("没设置 EPHE_PATH 环境变量，可在.env文件中设置或export EPHE_PATH=...");

    // 1. 设置一个固定的测试时间
    let dt = HoroDateTime::new(2025, 7, 13, 12, 0, 0, 8.0).unwrap();
    let jd_utc = dt.jd_utc;

    // 3. 生成 distance_star_long
    // 每宿宽13度，轸宿宽9度
    // 角在天秤0度
    let distance_star_long: [DistanceStarLong; 28] = [
        DistanceStarLong {
            lunar_mansions: 角, // 辰 0 度
            long: 180.0,
        },
        DistanceStarLong {
            lunar_mansions: 亢, // 辰 13 度
            long: swe_degnorm(180.0 + 13.0),
        },
        DistanceStarLong {
            lunar_mansions: 氐, // 辰 26 度
            long: swe_degnorm(180.0 + 13.0 * 2.0),
        },
        DistanceStarLong {
            lunar_mansions: 房, // 卯 9 度
            long: swe_degnorm(180.0 + 13.0 * 3.0),
        },
        DistanceStarLong {
            lunar_mansions: 心, // 卯 22 度
            long: swe_degnorm(180.0 + 13.0 * 4.0),
        },
        DistanceStarLong {
            lunar_mansions: 尾, // 寅 5 度
            long: swe_degnorm(180.0 + 13.0 * 5.0),
        },
        DistanceStarLong {
            lunar_mansions: 箕, // 寅 18 度
            long: swe_degnorm(180.0 + 13.0 * 6.0),
        },
        DistanceStarLong {
            lunar_mansions: 斗, // 丑 1 度
            long: swe_degnorm(180.0 + 13.0 * 7.0),
        },
        DistanceStarLong {
            lunar_mansions: 牛, // 丑 14 度
            long: swe_degnorm(180.0 + 13.0 * 8.0),
        },
        DistanceStarLong {
            lunar_mansions: 女, // 丑 27 度
            long: swe_degnorm(180.0 + 13.0 * 9.0),
        },
        DistanceStarLong {
            lunar_mansions: 虚, // 子 10 度
            long: swe_degnorm(180.0 + 13.0 * 10.0),
        },
        DistanceStarLong {
            lunar_mansions: 危, // 子 23 度
            long: swe_degnorm(180.0 + 13.0 * 11.0),
        },
        DistanceStarLong {
            lunar_mansions: 室, // 亥 6 度
            long: swe_degnorm(180.0 + 13.0 * 12.0),
        },
        DistanceStarLong {
            lunar_mansions: 壁, // 亥 19 度
            long: swe_degnorm(180.0 + 13.0 * 13.0),
        },
        DistanceStarLong {
            lunar_mansions: 奎, // 戌 2 度
            long: swe_degnorm(180.0 + 13.0 * 14.0),
        },
        DistanceStarLong {
            lunar_mansions: 娄, // 戌 15 度
            long: swe_degnorm(180.0 + 13.0 * 15.0),
        },
        DistanceStarLong {
            lunar_mansions: 胃, // 戌 28 度
            long: swe_degnorm(180.0 + 13.0 * 16.0),
        },
        DistanceStarLong {
            lunar_mansions: 昴, // 酉 11 度
            long: swe_degnorm(180.0 + 13.0 * 17.0),
        },
        DistanceStarLong {
            lunar_mansions: 毕, // 酉 24 度
            long: swe_degnorm(180.0 + 13.0 * 18.0),
        },
        DistanceStarLong {
            lunar_mansions: 觜, // 申 7 度
            long: swe_degnorm(180.0 + 13.0 * 19.0),
        },
        DistanceStarLong {
            lunar_mansions: 参, // 申 20 度
            long: swe_degnorm(180.0 + 13.0 * 20.0),
        },
        DistanceStarLong {
            lunar_mansions: 井, // 未 3 度
            long: swe_degnorm(180.0 + 13.0 * 21.0),
        },
        DistanceStarLong {
            lunar_mansions: 鬼, // 未 16 度
            long: swe_degnorm(180.0 + 13.0 * 22.0),
        },
        DistanceStarLong {
            lunar_mansions: 柳, // 未 29 度
            long: swe_degnorm(180.0 + 13.0 * 23.0),
        },
        DistanceStarLong {
            lunar_mansions: 星, // 午 12 度
            long: swe_degnorm(180.0 + 13.0 * 24.0),
        },
        DistanceStarLong {
            lunar_mansions: 张, // 午 25 度
            long: swe_degnorm(180.0 + 13.0 * 25.0),
        },
        DistanceStarLong {
            lunar_mansions: 翼, // 巳 8 度
            long: swe_degnorm(180.0 + 13.0 * 26.0),
        },
        DistanceStarLong {
            lunar_mansions: 轸, // 巳 21 度
            long: swe_degnorm(180.0 + 13.0 * 27.0),
        },
    ];

    // 4. 生成 planets_config
    let planets_config = PlanetConfig::default_all_configs();

    // 5. 调用 calc_planets 函数
    let planets_result =
        super::calc_planets(jd_utc, &distance_star_long, &planets_config, &ephe_path);

    // 6. 验证结果
    assert!(planets_result.is_ok(), "calc_planets should return Ok");
    let planets = planets_result.unwrap();

    // 以下测试不再计算行星的度数，因为test_calc_planets_long测试了度数的计算
    // 以下只测试宿度

    // 太阳
    let sun = planets
        .iter()
        .find(|p| p.name == 日)
        .expect("Sun not found");
    assert_eq!(sun.xiu, 鬼);
    assert_eq!(sun.xiu_degree, sun.long - 90.0 - 16.0);

    // 月亮
    let moon = planets
        .iter()
        .find(|p| p.name == 月)
        .expect("Moon not found");
    assert_eq!(moon.xiu, 虚);
    assert_eq!(moon.xiu_degree, moon.long - 300.0 - 10.0);

    // 水星
    let mercury = planets
        .iter()
        .find(|p| p.name == 水)
        .expect("Mercury not found");
    assert_eq!(mercury.xiu, 星);
    assert_eq!(mercury.xiu_degree, mercury.long - 120.0 - 12.0);

    // 金星
    let venus = planets
        .iter()
        .find(|p| p.name == 金)
        .expect("Venus not found");
    assert_eq!(venus.xiu, 觜);
    assert_eq!(venus.xiu_degree, venus.long - 60.0 - 7.0);

    // 火星
    let mars = planets
        .iter()
        .find(|p| p.name == 火)
        .expect("Mars not found");
    assert_eq!(mars.xiu, 翼);
    assert_eq!(mars.xiu_degree, mars.long - 150.0 - 8.0);

    // 木星
    let jupiter = planets
        .iter()
        .find(|p| p.name == 木)
        .expect("Jupiter not found");
    assert_eq!(jupiter.xiu, 井);
    assert_eq!(jupiter.xiu_degree, jupiter.long - 90.0 - 3.0);

    // 土星
    // 戌宫1度56分
    // 壁：亥19-戌2
    let saturn = planets
        .iter()
        .find(|p| p.name == 土)
        .expect("Saturn not found");
    assert_eq!(saturn.xiu, 壁);
    assert!((saturn.long + 11.0 - saturn.xiu_degree) * 3600.0 < 1.0);

    // 计
    let ketu = planets
        .iter()
        .find(|p| p.name == 计)
        .expect("ketu not found");
    assert_eq!(ketu.xiu, 壁);
    assert_eq!(ketu.xiu_degree, ketu.long - 330.0 - 19.0);

    // 罗
    let rahu = planets
        .iter()
        .find(|p| p.name == 罗)
        .expect("rahu not found");
    assert_eq!(rahu.xiu, 轸);
    assert_eq!(rahu.xiu_degree, rahu.long - 150.0 - 21.0);

    // 孛

    let mean_apog = planets
        .iter()
        .find(|p| p.name == 孛)
        .expect("MeanApog not found");
    assert_eq!(mean_apog.xiu, 房);
    assert_eq!(mean_apog.xiu_degree, mean_apog.long - 210.0 - 9.0);
}

// 测试行星速度状态
#[test]
fn test_calc_planets_speed_state() {
    // Load environment variables from .env file
    dotenvy::dotenv().ok();

    // 2. 准备 ephe_path
    // 首先检查环境变量 EPHE_PATH
    // let  ephe_path = std::env::var("EPHE_PATH").unwrap_or_default();
    let ephe_path = std::env::var("EPHE_PATH")
        .expect("没设置 EPHE_PATH 环境变量，可在.env文件中设置或export EPHE_PATH=...");

    // 1. 设置一个固定的测试时间
    let dt = HoroDateTime::new(2025, 7, 13, 12, 0, 0, 8.0).unwrap();
    let jd_utc = dt.jd_utc;

    // 3. 生成 distance_star_long
    let distance_star_configs = DistanceStarConfig::default_all_configs();
    let distance_star_long =
        calc_distance_star_long(jd_utc, &distance_star_configs, &ephe_path).unwrap();

    // 4. 生成 planets_config
    let planets_config = PlanetConfig::default_all_configs();

    // 5. 调用 calc_planets 函数
    let planets_result =
        super::calc_planets(jd_utc, &distance_star_long, &planets_config, &ephe_path);

    // 6. 验证结果
    assert!(planets_result.is_ok(), "calc_planets should return Ok");
    let planets = planets_result.unwrap();

    // 以下测试不再计算行星的度数，因为test_calc_planets_long测试了度数的计算
    // 以下只测试宿度

    // 太阳
    let sun = planets
        .iter()
        .find(|p| p.name == 日)
        .expect("Sun not found");
    assert_eq!(sun.speed_state, PlanetSpeedState::均);

    // 月亮
    let moon = planets
        .iter()
        .find(|p| p.name == 月)
        .expect("Moon not found");
    assert_eq!(moon.speed_state, PlanetSpeedState::均);

    // 计
    let qi = planets.iter().find(|p| p.name == 气).expect("qi not found");
    assert_eq!(qi.speed_state, PlanetSpeedState::均);

    // 计
    let ketu = planets
        .iter()
        .find(|p| p.name == 计)
        .expect("ketu not found");
    assert_eq!(ketu.speed_state, PlanetSpeedState::均);

    // 罗
    let rahu = planets
        .iter()
        .find(|p| p.name == 罗)
        .expect("rahu not found");
    assert_eq!(rahu.speed_state, PlanetSpeedState::均);

    // 孛
    let mean_apog = planets
        .iter()
        .find(|p| p.name == 孛)
        .expect("MeanApog not found");
    assert_eq!(mean_apog.speed_state, PlanetSpeedState::均);

    // 水星
    let mercury = planets
        .iter()
        .find(|p| p.name == 水)
        .expect("Mercury not found");
    assert_eq!(mercury.speed_state, PlanetSpeedState::迟);

    // 金星
    let venus = planets
        .iter()
        .find(|p| p.name == 金)
        .expect("Venus not found");
    assert_eq!(venus.speed_state, PlanetSpeedState::均);

    // 火星
    let mars = planets
        .iter()
        .find(|p| p.name == 火)
        .expect("Mars not found");
    assert_eq!(mars.speed_state, PlanetSpeedState::均);

    // 木星
    let jupiter = planets
        .iter()
        .find(|p| p.name == 木)
        .expect("Jupiter not found");
    assert_eq!(jupiter.speed_state, PlanetSpeedState::疾);

    // 土星
    let saturn = planets
        .iter()
        .find(|p| p.name == 土)
        .expect("Saturn not found");
    assert_eq!(saturn.speed_state, PlanetSpeedState::迟);
}
