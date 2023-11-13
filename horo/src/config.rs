use crate::planet::PlanetName::{self, *};

pub struct PlanetConfig {
    pub name: PlanetName,
    pub orb: u8,
    /// 每日速度
    /// 如果行星每日速度平均，可以设置max与min相等
    // 太阳、南北交点、四轴每日移动速度平均，可以两值设置为一样，如0.0
    pub min: f64,
    pub max: f64,
}

impl PlanetConfig {
    pub fn new(name: PlanetName, orb: u8, min: f64, max: f64) -> Self {
        Self {
            name,
            orb,
            min,
            max,
        }
    }

    /// 默认配置
    /// 行星每日的最大速度、最小速度, 单位度/每天, 日、计、罗、孛、气没有快、慢
    pub fn default_config(planet: &PlanetName) -> Self {
        match planet {
            ASC => PlanetConfig::new(ASC, 0, 0.0, 0.0),
            MC => PlanetConfig::new(MC, 0, 0.0, 0.0),
            DSC => PlanetConfig::new(DSC, 0, 0.0, 0.0),
            IC => PlanetConfig::new(IC, 0, 0.0, 0.0),
            Sun => PlanetConfig::new(
                Sun, 15, 0.0, //如果行星每日速度平均，可以设置max与min相等
                0.0,
            ),
            Moon => PlanetConfig::new(
                Moon, 12, 12.5, // 12 + 30/60
                13.5, // 13 + 30 / 60
            ),
            Mercury => PlanetConfig::new(
                Mercury, 7, 1.0, // 1度
                1.5, // 1+ 30/60
            ),
            Venus => PlanetConfig::new(Venus, 7, 50.0 / 60.0, 1.0 + 10.0 / 60.0),
            Mars => PlanetConfig::new(Mars, 8, 30.0 / 60.0, 40.0 / 60.0),
            Jupiter => PlanetConfig::new(Jupiter, 9, 5.0 / 60.0, 10.0 / 60.0),
            Saturn => PlanetConfig::new(Saturn, 9, 2.0 / 60.0, 5.0 / 60.0),
            NorthNode => PlanetConfig::new(NorthNode, 0, 0.0, 0.0),
            SouthNode => PlanetConfig::new(SouthNode, 0, 0.0, 0.0),
        }
    }

    // 所有行星的默认配置
    pub fn default_all_configs() -> [PlanetConfig; 13] {
        [
            PlanetConfig::default_config(&ASC),
            PlanetConfig::default_config(&MC),
            PlanetConfig::default_config(&DSC),
            PlanetConfig::default_config(&IC),
            PlanetConfig::default_config(&Sun),
            PlanetConfig::default_config(&Moon),
            PlanetConfig::default_config(&Mercury),
            PlanetConfig::default_config(&Venus),
            PlanetConfig::default_config(&Mars),
            PlanetConfig::default_config(&Jupiter),
            PlanetConfig::default_config(&Saturn),
            PlanetConfig::default_config(&NorthNode),
            PlanetConfig::default_config(&SouthNode),
        ]
    }
}

#[cfg(test)]
mod tests {
    use parameterized::parameterized;

    use crate::{
        config::PlanetConfig,
        planet::PlanetName::{self, *},
    };

    #[test]
    fn test_new() {
        let name = Sun;
        let orb = 2;
        let min = 1.0;
        let max = 2.0;

        let aspect = PlanetConfig::new(name.clone(), orb, min, max);

        assert_eq!(aspect.name, name);
        assert_eq!(aspect.orb, orb);
        assert_eq!(aspect.min, min);
        assert_eq!(aspect.max, max);
    }

    #[parameterized(config = {
        (ASC, 0, 0.0, 0.0),
        (MC, 0, 0.0, 0.0),
        (DSC, 0, 0.0, 0.0),
        (IC, 0, 0.0, 0.0),
        (Sun, 15, 0.0, 0.0),
        (Moon, 12, 12.5, 13.5),
        (Mercury, 7, 1.0, 1.5),
        (Venus, 7, 50.0 / 60.0, 1.0 + 10.0 / 60.0),
        (Mars, 8, 30.0 / 60.0, 40.0 / 60.0),
        (Jupiter, 9, 5.0 / 60.0, 10.0 / 60.0),
        (Saturn, 9, 2.0 / 60.0, 5.0 / 60.0),
        (NorthNode, 0, 0.0, 0.0),
        (SouthNode, 0, 0.0, 0.0),
    }
    )]
    fn test_default_config(config: (PlanetName, u8, f64, f64)) {
        let p = PlanetConfig::default_config(&config.0);
        assert_eq!(p.name, config.0);
        assert_eq!(p.orb, config.1);
        assert_eq!(p.min, config.2);
        assert_eq!(p.max, config.3);
    }

    #[parameterized(config = {
        (ASC, 0, 0.0, 0.0),
        (MC, 0, 0.0, 0.0),
        (DSC, 0, 0.0, 0.0),
        (IC, 0, 0.0, 0.0),
        (Sun, 15, 0.0, 0.0),
        (Moon, 12, 12.5, 13.5),
        (Mercury, 7, 1.0, 1.5),
        (Venus, 7, 50.0 / 60.0, 1.0 + 10.0 / 60.0),
        (Mars, 8, 30.0 / 60.0, 40.0 / 60.0),
        (Jupiter, 9, 5.0 / 60.0, 10.0 / 60.0),
        (Saturn, 9, 2.0 / 60.0, 5.0 / 60.0),
        (NorthNode, 0, 0.0, 0.0),
        (SouthNode, 0, 0.0, 0.0),
    }
    )]
    fn test_default_all_configs(config: (PlanetName, u8, f64, f64)) {
        let configs = PlanetConfig::default_all_configs();

        let planet_config = configs.iter().find(|c| c.name == config.0);
        assert!(planet_config.is_some());
        let planet_config = planet_config.unwrap();

        assert_eq!(planet_config.name, config.0);
        assert_eq!(planet_config.orb, config.1);
        assert_eq!(planet_config.min, config.2);
        assert_eq!(planet_config.max, config.3);
    }
}
