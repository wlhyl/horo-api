use crate::{config::PlanetConfig, lunar_mansions::LunarMansionsName};
use PlanetName::*;
use PlanetSpeedState::*;

#[cfg(feature = "serde")]
use serde::Serialize;

#[cfg(feature = "swagger")]
use utoipa::ToSchema;

#[derive(PartialEq, Eq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
#[cfg_attr(feature = "swagger", derive(ToSchema))]
pub enum PlanetSpeedState {
    疾,
    均,
    迟,
    // 留,伏,逆，此三者由前端计算
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[cfg_attr(feature = "swagger", derive(ToSchema))]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum PlanetName {
    日,
    月,
    水,
    金,
    火,
    木,
    土,
    计, // 北交
    罗, // 南交
    孛,
    气,
}

#[cfg_attr(feature = "serde", derive(Serialize))]
#[cfg_attr(feature = "swagger", derive(ToSchema))]
pub struct Planet {
   pub name: PlanetName,
    /// 行星的黄经
   pub long: f64,
    /// 行星在黄道上每日的移动速度
    speed: f64,

    /// 行星在黄道上的宿
    xiu: LunarMansionsName,

    /// 行星在黄道上的入宿度
    xiu_degree: f64,
    /// 行星速度状态：快、平均、慢
    speed_state: PlanetSpeedState,
    /// 停滞，行星移动速度小于1度，是停滞，只有，水、金、火、木、土，有停滞
    is_stationary: bool,
}

impl Planet {
    pub fn new(
        name: PlanetName,
        long: f64,
        speed: f64,
        xiu: LunarMansionsName,
        xiu_degree: f64,
        config: &PlanetConfig,
    ) -> Self {
        let speed_state = if config.min < config.max {
            if speed.abs() > config.max {
                疾
            } else if speed.abs() < config.min {
                迟
            } else {
                均
            }
        } else {
            均
        };

        let is_stationary = if [水, 金, 火, 木, 土].contains(&name) {
            if speed.abs() < 1.0 {
                true
            } else {
                false
            }
        } else {
            false
        };
        Self {
            name,
            long,
            speed,
            speed_state,
            xiu,
            xiu_degree,
            is_stationary,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::config::PlanetConfig;
    use crate::LunarMansionsName::角;

    use super::Planet;
    use super::PlanetName::*;
    use super::PlanetSpeedState::*;

    #[test]
    fn test_new() {
        let p = Planet::new(日, 1.0, 1.0, 角, 1.0, &PlanetConfig::new(日, 1.0, 2.0));

        assert_eq!(日, p.name, "name");
        assert_eq!(1.0, p.long, "黄经");
        assert_eq!(1.0, p.speed, "speed");
        assert_eq!(角, p.xiu, "二十八宿");
        assert_eq!(1.0, p.xiu_degree, "speed");
    }

    // 快
    #[test]
    fn test_faster() {
        // 逆行
        let p0 = Planet::new(日, 1.0, -3.1, 角, 1.0, &PlanetConfig::new(日, 1.0, 2.0));
        assert_eq!(疾, p0.speed_state, "逆行，快");

        // 顺行
        let p1 = Planet::new(日, 1.0, 3.1, 角, 1.0, &PlanetConfig::new(日, 1.0, 2.0));
        assert_eq!(疾, p1.speed_state, "顺行，快")
    }

    // 慢
    #[test]
    fn test_slower() {
        // 逆行
        let p0 = Planet::new(日, 1.0, -0.1, 角, 1.0, &PlanetConfig::new(日, 1.0, 2.0));
        assert_eq!(迟, p0.speed_state, "逆行，慢");

        // 顺行
        let p1 = Planet::new(日, 1.0, 0.1, 角, 1.0, &PlanetConfig::new(日, 1.0, 2.0));
        assert_eq!(迟, p1.speed_state, "顺行，慢");
    }

    // 平均
    #[test]
    fn test_average() {
        let p0 = Planet::new(日, 1.0, -0.1, 角, 1.0, &PlanetConfig::new(日, 2.0, 2.0));
        assert_eq!(均, p0.speed_state, "逆行，均");

        let p1 = Planet::new(日, 1.0, 0.1, 角, 1.0, &PlanetConfig::new(日, 2.0, 2.0));
        assert_eq!(均, p1.speed_state, "顺行，均");
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
                &PlanetConfig::new(日, 2.0, 2.0),
            );
            assert!(!p.is_stationary);
        }
    }
}
