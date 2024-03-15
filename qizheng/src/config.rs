use crate::{
    lunar_mansions::LunarMansionsName::{self, *},
    planet::PlanetName::{self, *},
};

pub struct PlanetConfig {
    pub name: PlanetName,

    /// 每日速度
    /// 如果行星每日速度平均，可以设置max与min相等
    // 太阳、南北交点、四轴每日移动速度平均，可以两值设置为一样，如0.0
    pub min: f64,
    pub max: f64,
}

impl PlanetConfig {
    pub fn new(name: PlanetName, min: f64, max: f64) -> Self {
        Self { name, min, max }
    }

    /// 默认配置
    /// 行星每日的最大速度、最小速度, 单位度/每天, 日、计、罗、孛、气没有快、慢
    pub fn default_config(planet: &PlanetName) -> Self {
        match planet {
            日 => PlanetConfig::new(
                日, 0.0, //如果行星每日速度平均，可以设置max与min相等
                0.0,
            ),
            月 => PlanetConfig::new(
                月, 12.5, // 12 + 30/60
                13.5, // 13 + 30 / 60
            ),
            水 => PlanetConfig::new(
                水, 1.0, // 1度
                1.5, // 1+ 30/60
            ),
            金 => PlanetConfig::new(金, 50.0 / 60.0, 1.0 + 10.0 / 60.0),
            火 => PlanetConfig::new(火, 30.0 / 60.0, 40.0 / 60.0),
            木 => PlanetConfig::new(木, 5.0 / 60.0, 10.0 / 60.0),
            土 => PlanetConfig::new(土, 2.0 / 60.0, 5.0 / 60.0),
            计 => PlanetConfig::new(计, 0.0, 0.0),
            罗 => PlanetConfig::new(罗, 0.0, 0.0),
            孛 => PlanetConfig::new(孛, 0.0, 0.0),
            气 => PlanetConfig::new(气, 0.0, 0.0),
        }
    }

    // 所有行星的默认配置
    pub fn default_all_configs() -> [PlanetConfig; 11] {
        [
            PlanetConfig::default_config(&日),
            PlanetConfig::default_config(&月),
            PlanetConfig::default_config(&水),
            PlanetConfig::default_config(&金),
            PlanetConfig::default_config(&火),
            PlanetConfig::default_config(&木),
            PlanetConfig::default_config(&土),
            PlanetConfig::default_config(&计),
            PlanetConfig::default_config(&罗),
            PlanetConfig::default_config(&孛),
            PlanetConfig::default_config(&气),
        ]
    }
}

/// 二十八宿距星配置
pub struct DistanceStarConfig {
    pub lunar_mansions: LunarMansionsName,
    pub distance_star: String,
}

impl DistanceStarConfig {
    pub fn new(lunar_mansions: LunarMansionsName, distance_star: String) -> Self {
        Self {
            lunar_mansions,
            distance_star,
        }
    }

    /// 默认配置
    pub fn default_config(star: &LunarMansionsName) -> Self {
        match star {
            角 => Self {
                lunar_mansions: 角,
                distance_star: "alVir".to_owned(),
            },
            亢 => Self {
                lunar_mansions: 亢,
                distance_star: "kaVir".to_owned(),
            },
            氐 => Self {
                lunar_mansions: 氐,
                distance_star: "al-2Lib".to_owned(),
            },
            房 => Self {
                lunar_mansions: 房,
                distance_star: "piSco".to_owned(),
            },
            心 => Self {
                lunar_mansions: 心,
                distance_star: "siSco".to_owned(),
            },
            尾 => Self {
                lunar_mansions: 尾,
                distance_star: "mu-1Sco".to_owned(),
            },
            箕 => Self {
                lunar_mansions: 箕,
                distance_star: "gaSgr".to_owned(),
            },
            // 北方七宿
            斗 => Self {
                lunar_mansions: 斗,
                distance_star: "phSgr".to_owned(),
            },
            牛 => Self {
                lunar_mansions: 牛,
                distance_star: "beCap".to_owned(),
            },
            女 => Self {
                lunar_mansions: 女,
                distance_star: "epAqr".to_owned(),
            },
            虚 => Self {
                lunar_mansions: 虚,
                distance_star: "beAqr".to_owned(),
            }, // 虚宿一
            危 => Self {
                lunar_mansions: 危,
                distance_star: "alAqr".to_owned(),
            }, // 危宿一
            室 => Self {
                lunar_mansions: 室,
                distance_star: "alPeg".to_owned(),
            }, // 室宿一（α Peg、飞马座α）
            壁 => Self {
                lunar_mansions: 壁,
                distance_star: "gaPeg".to_owned(),
            }, //壁宿一（γ Peg / 飞马座γ）
            // 西方七宿
            奎 => Self {
                lunar_mansions: 奎,
                distance_star: "38And".to_owned(),
            }, // 奎宿二（仙女座ζ, ζ And）， 此汉宋距星，明清距星：奎宿一 (η And / 仙女座η, 38And)，瑞士星历表无此星数据
            娄 => Self {
                lunar_mansions: 娄,
                distance_star: "beAri".to_owned(),
            }, //娄宿一
            胃 => Self {
                lunar_mansions: 胃,
                distance_star: "35Ari".to_owned(),
            }, //胃宿一
            昴 => Self {
                lunar_mansions: 昴,
                distance_star: "17Tau".to_owned(),
            }, // 昴宿一，即金牛座17（17 Tau，17 Tauri）
            毕 => Self {
                lunar_mansions: 毕,
                distance_star: "epTau".to_owned(),
            }, // 毕宿一，即金牛座ε（ε Tau，ε Tauri）
            觜 => Self {
                lunar_mansions: 觜,
                distance_star: "laOri".to_owned(),
            }, // 清距星：猎户座λ（,laOri） 觜宿一
            // 觜： turtle_beak: ',ph-1Ori' # 汉唐宋距星：猎户座φ¹（,ph-1Ori） 觜宿二
            参 => Self {
                lunar_mansions: 参,
                distance_star: "zeOri".to_owned(),
            }, // 清距星：猎户座ζ 参宿一
            //three_stars: ',deOri' # 汉唐宋距星：猎户座δ 参宿三

            // 南方七宿
            井 => Self {
                lunar_mansions: 井,
                distance_star: "muGem".to_owned(),
            }, // 井宿一（μ Gem/双子座μ）
            鬼 => Self {
                lunar_mansions: 鬼,
                distance_star: "31Cnc".to_owned(),
            }, //鬼宿一       巨蟹座θ
            柳 => Self {
                lunar_mansions: 柳,
                distance_star: "deHya".to_owned(),
            }, //柳宿一 （δ Hya / 长蛇座δ）
            星 => Self {
                lunar_mansions: 星,
                distance_star: "alHya".to_owned(),
            }, // 星宿一（Alphard，也称为Alpha Hydrae，缩写为α Hydrae或Alpha Hya、α Hya）
            张 => Self {
                lunar_mansions: 张,
                distance_star: "up-1Hya".to_owned(),
            }, // 张宿一（υ¹Hyd / 长蛇座υ¹）
            翼 => Self {
                lunar_mansions: 翼,
                distance_star: "alCrt".to_owned(),
            }, //翼宿一       巨爵座α
            轸 => Self {
                lunar_mansions: 轸,
                distance_star: "gaCrv".to_owned(),
            }, // 轸宿一（γ Crv、乌鸦座γ）
        }
    }

    // 所有星宿的默认配置
    pub fn default_all_configs() -> [DistanceStarConfig; 28] {
        [
            // 东方七宿
            DistanceStarConfig::default_config(&角),
            DistanceStarConfig::default_config(&亢),
            DistanceStarConfig::default_config(&氐),
            DistanceStarConfig::default_config(&房),
            DistanceStarConfig::default_config(&心),
            DistanceStarConfig::default_config(&尾),
            DistanceStarConfig::default_config(&箕),
            // 北方七宿
            DistanceStarConfig::default_config(&斗),
            DistanceStarConfig::default_config(&牛),
            DistanceStarConfig::default_config(&女),
            DistanceStarConfig::default_config(&虚),
            DistanceStarConfig::default_config(&危),
            DistanceStarConfig::default_config(&室),
            DistanceStarConfig::default_config(&壁),
            // 西方七宿
            DistanceStarConfig::default_config(&奎),
            DistanceStarConfig::default_config(&娄),
            DistanceStarConfig::default_config(&胃),
            DistanceStarConfig::default_config(&昴),
            DistanceStarConfig::default_config(&毕),
            DistanceStarConfig::default_config(&觜),
            DistanceStarConfig::default_config(&参),
            // 南方七宿
            DistanceStarConfig::default_config(&井),
            DistanceStarConfig::default_config(&鬼),
            DistanceStarConfig::default_config(&柳),
            DistanceStarConfig::default_config(&星),
            DistanceStarConfig::default_config(&张),
            DistanceStarConfig::default_config(&翼),
            DistanceStarConfig::default_config(&轸),
        ]
    }
}

#[cfg(test)]
mod tests {

    mod planet_config {
        use parameterized::parameterized;

        use crate::{
            config::PlanetConfig,
            planet::PlanetName::{self, *},
        };

        #[test]
        fn test_new() {
            let name = 日;
            let min = 1.0;
            let max = 2.0;

            let planet_config = PlanetConfig::new(name, min, max);

            assert_eq!(planet_config.name, name);
            assert_eq!(planet_config.min, min);
            assert_eq!(planet_config.max, max);
        }

        #[parameterized(config = {
        (日, 0.0, 0.0),
        (月, 12.5, 13.5),
        (水, 1.0, 1.5),
        (金, 50.0 / 60.0, 1.0 + 10.0 / 60.0),
        (火, 30.0 / 60.0, 40.0 / 60.0),
        (木, 5.0 / 60.0, 10.0 / 60.0),
        (土, 2.0 / 60.0, 5.0 / 60.0),
        (计, 0.0, 0.0),
        (罗, 0.0, 0.0),
        (孛, 0.0, 0.0),
        (气, 0.0, 0.0),
    }
    )]
        fn test_default_config(config: (PlanetName, f64, f64)) {
            let planet_config = PlanetConfig::default_config(&config.0);
            assert_eq!(planet_config.name, config.0);
            assert_eq!(planet_config.min, config.1);
            assert_eq!(planet_config.max, config.2);
        }

        #[parameterized(config = {
        (日, 0.0, 0.0),
        (月, 12.5, 13.5),
        (水, 1.0, 1.5),
        (金, 50.0 / 60.0, 1.0 + 10.0 / 60.0),
        (火, 30.0 / 60.0, 40.0 / 60.0),
        (木, 5.0 / 60.0, 10.0 / 60.0),
        (土, 2.0 / 60.0, 5.0 / 60.0),
        (计, 0.0, 0.0),
        (罗, 0.0, 0.0),
        (孛, 0.0, 0.0),
        (气, 0.0, 0.0),
    }
    )]
        fn test_default_all_configs(config: (PlanetName, f64, f64)) {
            let configs = PlanetConfig::default_all_configs();

            let planet_config = configs.iter().find(|c| c.name == config.0);
            assert!(planet_config.is_some());
            let planet_config = planet_config.unwrap();

            assert_eq!(planet_config.name, config.0);
            assert_eq!(planet_config.min, config.1);
            assert_eq!(planet_config.max, config.2);
        }
    }

    mod distance_star_config {
        use parameterized::parameterized;

        use crate::{
            config::DistanceStarConfig,
            LunarMansionsName::{self, *},
        };

        #[test]
        fn test_new() {
            let name = 角;
            let star = "距星";

            let distance_star_config = DistanceStarConfig::new(name, star.to_string());

            assert_eq!(distance_star_config.lunar_mansions, name);
            assert_eq!(distance_star_config.distance_star, star);
        }

        #[parameterized(config = {
        // 东方七宿
        (角, "alVir",), 
        (亢, "kaVir",),
        (氐, "al-2Lib",),
        (房, "piSco",),
        (心, "siSco",),
        (尾, "mu-1Sco",),
        (箕, "gaSgr",),
        // 北方七宿
        (斗, "phSgr",),
        (牛, "beCap",),
        (女, "epAqr",),
        (虚, "beAqr",),
        (危, "alAqr",),
        (室, "alPeg",),
        (壁, "gaPeg",),
          // 西方七宿
        (奎, "38And",),
        (娄, "beAri",),
        (胃, "35Ari",),
        (昴, "17Tau",),
        (毕, "epTau",),
        (觜, "laOri",),
        (参, "zeOri",),
        // 南方七宿
        (井, "muGem",),
        (鬼, "31Cnc", ),
        (柳, "deHya", ),
        (星, "alHya", ),
        (张, "up-1Hya",),
        (翼, "alCrt", ),
        (轸, "gaCrv",),
    }
    )]
        fn test_default_config(config: (LunarMansionsName, &str)) {
            let distance_star_config = DistanceStarConfig::default_config(&config.0);
            assert_eq!(distance_star_config.lunar_mansions, config.0);
            assert_eq!(distance_star_config.distance_star, config.1);
        }

        #[parameterized(config = {
         // 东方七宿
         (角, "alVir",), 
         (亢, "kaVir",),
         (氐, "al-2Lib",),
         (房, "piSco",),
         (心, "siSco",),
         (尾, "mu-1Sco",),
         (箕, "gaSgr",),
         // 北方七宿
         (斗, "phSgr",),
         (牛, "beCap",),
         (女, "epAqr",),
         (虚, "beAqr",),
         (危, "alAqr",),
         (室, "alPeg",),
         (壁, "gaPeg",),
           // 西方七宿
         (奎, "38And",),
         (娄, "beAri",),
         (胃, "35Ari",),
         (昴, "17Tau",),
         (毕, "epTau",),
         (觜, "laOri",),
         (参, "zeOri",),
         // 南方七宿
         (井, "muGem",),
         (鬼, "31Cnc", ),
         (柳, "deHya", ),
         (星, "alHya", ),
         (张, "up-1Hya",),
         (翼, "alCrt", ),
         (轸, "gaCrv",),
    }
    )]
        fn test_default_all_configs(config: (LunarMansionsName, &str)) {
            let configs = DistanceStarConfig::default_all_configs();

            let distance_star_config = configs.iter().find(|c| c.lunar_mansions == config.0);
            assert!(distance_star_config.is_some());
            let distance_star_config = distance_star_config.unwrap();

            assert_eq!(distance_star_config.lunar_mansions, config.0);
            assert_eq!(distance_star_config.distance_star, config.1);
        }
    }
}
