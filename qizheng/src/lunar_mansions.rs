use swe::{Flag, swe_close, swe_degnorm, swe_fixstar2_ut, swe_set_ephe_path};

use crate::{config::DistanceStarConfig, error::Error};

#[cfg(feature = "serde")]
use serde::Serialize;

#[cfg(feature = "swagger")]
use utoipa::ToSchema;

/// 二十八宿名
#[cfg_attr(feature = "serde", derive(Serialize))]
#[cfg_attr(feature = "swagger", derive(ToSchema))]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum LunarMansionsName {
    // 东方七宿
    角,
    亢,
    氐,
    房,
    心,
    尾,
    箕,
    // 北方七宿
    斗,
    牛,
    女,
    虚,
    危,
    室,
    壁,
    // 西方七宿
    奎,
    娄,
    胃,
    昴,
    毕,
    觜,
    参,
    // 南方七宿
    井,
    鬼,
    柳,
    星,
    张,
    翼,
    轸,
}

/// 二十八宿的黄道经度
#[cfg_attr(feature = "serde", derive(Serialize))]
#[cfg_attr(feature = "swagger", derive(ToSchema))]
pub struct DistanceStarLong {
    /// 二十八宿的名称
    pub lunar_mansions: LunarMansionsName,
    /// 二十八宿距星的黄道经度
    pub long: f64,
}

/// 计算二十八宿黄道经度
pub(crate) fn calc_distance_star_long(
    jd_utc: f64,
    distance_star_config: &[DistanceStarConfig],
    ephe_path: &str,
) -> Result<Vec<DistanceStarLong>, Error> {
    let mut distance_star_long = vec![];
    swe_set_ephe_path(ephe_path);

    for distance_star in distance_star_config {
        let star_name = format!(",{}", distance_star.distance_star);
        let (_, xx) = swe_fixstar2_ut(&star_name, jd_utc, &[Flag::SeflgSwieph,Flag::SeflgEquatorial])
            .map_err(|e| Error::Function(format!("计算二十八距星错误:{e}")))?;
        distance_star_long.push(DistanceStarLong {
            lunar_mansions: distance_star.lunar_mansions,
            long: xx[0],
        });
    }
    swe_close();
    Ok(distance_star_long)
}

// 计算入宿度
pub(crate) fn calc_xiu_degree(
    star_long: f64,
    distance_star_long: &[DistanceStarLong],
) -> Result<(LunarMansionsName, f64), Error> {
    distance_star_long
        .iter()
        .enumerate()
        .find_map(|(index, distance_star)| {
            let next_distance_star = &distance_star_long[(index + 1) % distance_star_long.len()];

            let distance = swe_degnorm(next_distance_star.long - distance_star.long);
            let planet_distance = swe_degnorm(star_long - distance_star.long);
            if planet_distance < distance {
                Some((distance_star.lunar_mansions, planet_distance))
            } else {
                None
            }
        })
        .ok_or(Error::Function(
            "找不到行星的入宿度，请检查源代码".to_string(),
        ))
}

#[cfg(test)]
mod tests {
    use swe::swe_degnorm;

    use crate::{
        DistanceStarConfig, DistanceStarLong,
        LunarMansionsName::*,
        lunar_mansions::{calc_distance_star_long, calc_xiu_degree},
    };

    #[test]
    fn test_calc_xiu_degree() {
        let distance_star_long: [DistanceStarLong; 28] = [
            DistanceStarLong {
                lunar_mansions: 角,
                long: swe_degnorm(180.0 + 12.0 + 1.0),
            },
            DistanceStarLong {
                lunar_mansions: 亢,
                long: swe_degnorm(180.0 + 24.0 + 1.0),
            },
            DistanceStarLong {
                lunar_mansions: 氐,
                long: swe_degnorm(180.0 + 36.0 + 1.0),
            },
            DistanceStarLong {
                lunar_mansions: 房,
                long: swe_degnorm(180.0 + 48.0 + 1.0),
            },
            DistanceStarLong {
                lunar_mansions: 心,
                long: swe_degnorm(180.0 + 60.0 + 1.0),
            },
            DistanceStarLong {
                lunar_mansions: 尾,
                long: swe_degnorm(180.0 + 72.0 + 1.0),
            },
            DistanceStarLong {
                lunar_mansions: 箕,
                long: swe_degnorm(180.0 + 84.0 + 1.0),
            },
            //
            DistanceStarLong {
                lunar_mansions: 斗,
                long: swe_degnorm(180.0 + 96.0 + 1.0),
            },
            DistanceStarLong {
                lunar_mansions: 牛,
                long: swe_degnorm(180.0 + 108.0 + 1.0),
            },
            DistanceStarLong {
                lunar_mansions: 女,
                long: swe_degnorm(180.0 + 120.0 + 1.0),
            },
            DistanceStarLong {
                lunar_mansions: 虚,
                long: swe_degnorm(180.0 + 132.0 + 1.0),
            },
            DistanceStarLong {
                lunar_mansions: 危,
                long: swe_degnorm(180.0 + 144.0 + 1.0),
            },
            DistanceStarLong {
                lunar_mansions: 室,
                long: swe_degnorm(180.0 + 156.0 + 1.0),
            },
            DistanceStarLong {
                lunar_mansions: 壁,
                long: swe_degnorm(180.0 + 168.0 + 1.0),
            },
            //
            DistanceStarLong {
                lunar_mansions: 奎,
                long: swe_degnorm(180.0 + 180.0 + 1.0),
            },
            DistanceStarLong {
                lunar_mansions: 娄,
                long: swe_degnorm(180.0 + 192.0 + 1.0),
            },
            DistanceStarLong {
                lunar_mansions: 胃,
                long: swe_degnorm(180.0 + 204.0 + 1.0),
            },
            DistanceStarLong {
                lunar_mansions: 昴,
                long: swe_degnorm(180.0 + 216.0 + 1.0),
            },
            DistanceStarLong {
                lunar_mansions: 毕,
                long: swe_degnorm(180.0 + 228.0 + 1.0),
            },
            DistanceStarLong {
                lunar_mansions: 觜,
                long: swe_degnorm(180.0 + 240.0 + 1.0),
            },
            DistanceStarLong {
                lunar_mansions: 参,
                long: swe_degnorm(180.0 + 252.0 + 1.0),
            },
            //
            DistanceStarLong {
                lunar_mansions: 井,
                long: swe_degnorm(180.0 + 264.0 + 1.0),
            },
            DistanceStarLong {
                lunar_mansions: 鬼,
                long: swe_degnorm(180.0 + 276.0 + 1.0),
            },
            DistanceStarLong {
                lunar_mansions: 柳,
                long: swe_degnorm(180.0 + 288.0 + 1.0),
            },
            DistanceStarLong {
                lunar_mansions: 星,
                long: swe_degnorm(180.0 + 300.0),
            },
            DistanceStarLong {
                lunar_mansions: 张,
                long: swe_degnorm(180.0 + 312.0),
            },
            DistanceStarLong {
                lunar_mansions: 翼,
                long: swe_degnorm(180.0 + 324.0),
            },
            DistanceStarLong {
                lunar_mansions: 轸,
                long: swe_degnorm(180.0 + 336.0),
            },
        ];

        let distance_star_long_sum: f64 = distance_star_long
            .iter()
            .enumerate()
            .map(|(index, star)| {
                let next_star = &distance_star_long[(index + 1) % 28];
                swe_degnorm(next_star.long - star.long)
            })
            .sum();

        assert_eq!(distance_star_long_sum, 360.0);

        // 辰:180, 轸: 24.0
        let xiu_and_degree = calc_xiu_degree(180.0, &distance_star_long);
        assert!(xiu_and_degree.is_ok());
        let (xiu, xiu_degree) = xiu_and_degree.unwrap();
        assert_eq!(xiu, 轸);
        assert_eq!(xiu_degree, 24.0);

        // 卯:210, 亢: 5.0
        let xiu_and_degree = calc_xiu_degree(210.0, &distance_star_long);
        assert!(xiu_and_degree.is_ok());
        let (xiu, xiu_degree) = xiu_and_degree.unwrap();
        assert_eq!(xiu, 亢);
        assert_eq!(xiu_degree, 5.0);

        // 寅:240, 房: 11.0
        let xiu_and_degree = calc_xiu_degree(240.0, &distance_star_long);
        assert!(xiu_and_degree.is_ok());
        let (xiu, xiu_degree) = xiu_and_degree.unwrap();
        assert_eq!(xiu, 房);
        assert_eq!(xiu_degree, 11.0);

        // 丑:270, 箕: 5.0
        let xiu_and_degree = calc_xiu_degree(270.0, &distance_star_long);
        assert!(xiu_and_degree.is_ok());
        let (xiu, xiu_degree) = xiu_and_degree.unwrap();
        assert_eq!(xiu, 箕);
        assert_eq!(xiu_degree, 5.0);

        // 子:300, 牛: 11.0
        let xiu_and_degree = calc_xiu_degree(300.0, &distance_star_long);
        assert!(xiu_and_degree.is_ok());
        let (xiu, xiu_degree) = xiu_and_degree.unwrap();
        assert_eq!(xiu, 牛);
        assert_eq!(xiu_degree, 11.0);

        // 亥:330, 危: 5.0
        let xiu_and_degree = calc_xiu_degree(330.0, &distance_star_long);
        assert!(xiu_and_degree.is_ok());
        let (xiu, xiu_degree) = xiu_and_degree.unwrap();
        assert_eq!(xiu, 危);
        assert_eq!(xiu_degree, 5.0);

        // 戌:0, 壁: 11.0
        let xiu_and_degree = calc_xiu_degree(0.0, &distance_star_long);
        assert!(xiu_and_degree.is_ok());
        let (xiu, xiu_degree) = xiu_and_degree.unwrap();
        assert_eq!(xiu, 壁);
        assert_eq!(xiu_degree, 11.0);

        // 酉:30, 胃: 5.0
        let xiu_and_degree = calc_xiu_degree(30.0, &distance_star_long);
        assert!(xiu_and_degree.is_ok());
        let (xiu, xiu_degree) = xiu_and_degree.unwrap();
        assert_eq!(xiu, 胃);
        assert_eq!(xiu_degree, 5.0);

        // 申:60, 毕: 11.0
        let xiu_and_degree = calc_xiu_degree(60.0, &distance_star_long);
        assert!(xiu_and_degree.is_ok());
        let (xiu, xiu_degree) = xiu_and_degree.unwrap();
        assert_eq!(xiu, 毕);
        assert_eq!(xiu_degree, 11.0);

        // 未:90, 井: 5.0
        let xiu_and_degree = calc_xiu_degree(90.0, &distance_star_long);
        assert!(xiu_and_degree.is_ok());
        let (xiu, xiu_degree) = xiu_and_degree.unwrap();
        assert_eq!(xiu, 井);
        assert_eq!(xiu_degree, 5.0);

        // 午:120, 星: 0.0
        let xiu_and_degree = calc_xiu_degree(120.0, &distance_star_long);
        assert!(xiu_and_degree.is_ok());
        let (xiu, xiu_degree) = xiu_and_degree.unwrap();
        assert_eq!(xiu, 星);
        assert_eq!(xiu_degree, 0.0);

        // 巳:150, 翼: 6.0
        let xiu_and_degree = calc_xiu_degree(150.0, &distance_star_long);
        assert!(xiu_and_degree.is_ok());
        let (xiu, xiu_degree) = xiu_and_degree.unwrap();
        assert_eq!(xiu, 翼);
        assert_eq!(xiu_degree, 6.0);
    }

    #[test]
    fn test_calc_xiu_degree_full_coverage() {
        let distance_star_long: [DistanceStarLong; 28] = [
            DistanceStarLong {
                lunar_mansions: 角,
                long: swe_degnorm(180.0 + 12.0 + 1.0),
            },
            DistanceStarLong {
                lunar_mansions: 亢,
                long: swe_degnorm(180.0 + 24.0 + 1.0),
            },
            DistanceStarLong {
                lunar_mansions: 氐,
                long: swe_degnorm(180.0 + 36.0 + 1.0),
            },
            DistanceStarLong {
                lunar_mansions: 房,
                long: swe_degnorm(180.0 + 48.0 + 1.0),
            },
            DistanceStarLong {
                lunar_mansions: 心,
                long: swe_degnorm(180.0 + 60.0 + 1.0),
            },
            DistanceStarLong {
                lunar_mansions: 尾,
                long: swe_degnorm(180.0 + 72.0 + 1.0),
            },
            DistanceStarLong {
                lunar_mansions: 箕,
                long: swe_degnorm(180.0 + 84.0 + 1.0),
            },
            //
            DistanceStarLong {
                lunar_mansions: 斗,
                long: swe_degnorm(180.0 + 96.0 + 1.0),
            },
            DistanceStarLong {
                lunar_mansions: 牛,
                long: swe_degnorm(180.0 + 108.0 + 1.0),
            },
            DistanceStarLong {
                lunar_mansions: 女,
                long: swe_degnorm(180.0 + 120.0 + 1.0),
            },
            DistanceStarLong {
                lunar_mansions: 虚,
                long: swe_degnorm(180.0 + 132.0 + 1.0),
            },
            DistanceStarLong {
                lunar_mansions: 危,
                long: swe_degnorm(180.0 + 144.0 + 1.0),
            },
            DistanceStarLong {
                lunar_mansions: 室,
                long: swe_degnorm(180.0 + 156.0 + 1.0),
            },
            DistanceStarLong {
                lunar_mansions: 壁,
                long: swe_degnorm(180.0 + 168.0 + 1.0),
            },
            //
            DistanceStarLong {
                lunar_mansions: 奎,
                long: swe_degnorm(180.0 + 180.0 + 1.0),
            },
            DistanceStarLong {
                lunar_mansions: 娄,
                long: swe_degnorm(180.0 + 192.0 + 1.0),
            },
            DistanceStarLong {
                lunar_mansions: 胃,
                long: swe_degnorm(180.0 + 204.0 + 1.0),
            },
            DistanceStarLong {
                lunar_mansions: 昴,
                long: swe_degnorm(180.0 + 216.0 + 1.0),
            },
            DistanceStarLong {
                lunar_mansions: 毕,
                long: swe_degnorm(180.0 + 228.0 + 1.0),
            },
            DistanceStarLong {
                lunar_mansions: 觜,
                long: swe_degnorm(180.0 + 240.0 + 1.0),
            },
            DistanceStarLong {
                lunar_mansions: 参,
                long: swe_degnorm(180.0 + 252.0 + 1.0),
            },
            //
            DistanceStarLong {
                lunar_mansions: 井,
                long: swe_degnorm(180.0 + 264.0 + 1.0),
            },
            DistanceStarLong {
                lunar_mansions: 鬼,
                long: swe_degnorm(180.0 + 276.0 + 1.0),
            },
            DistanceStarLong {
                lunar_mansions: 柳,
                long: swe_degnorm(180.0 + 288.0 + 1.0),
            },
            DistanceStarLong {
                lunar_mansions: 星,
                long: swe_degnorm(180.0 + 300.0),
            },
            DistanceStarLong {
                lunar_mansions: 张,
                long: swe_degnorm(180.0 + 312.0),
            },
            DistanceStarLong {
                lunar_mansions: 翼,
                long: swe_degnorm(180.0 + 324.0),
            },
            DistanceStarLong {
                lunar_mansions: 轸,
                long: swe_degnorm(180.0 + 336.0),
            },
        ];

        // Test the boundary of each mansion
        for (index, star) in distance_star_long.iter().enumerate() {
            // Test the exact start of the mansion
            let (xiu, xiu_degree) = calc_xiu_degree(star.long, &distance_star_long).unwrap();
            assert_eq!(
                xiu, star.lunar_mansions,
                "Failed at mansion {:?}",
                star.lunar_mansions
            );
            // Due to f64 precision, we check if it's very close to 0
            assert!(
                xiu_degree < 1e-9,
                "Failed at mansion {:?} with degree {}",
                star.lunar_mansions,
                xiu_degree
            );

            // Test the point just before the start of the mansion
            let prev_star_index = (index + 28 - 1) % 28;
            let prev_star = &distance_star_long[prev_star_index];
            let point_before = swe_degnorm(star.long - 1e-9);
            let (xiu, xiu_degree) = calc_xiu_degree(point_before, &distance_star_long).unwrap();

            let prev_star_span = swe_degnorm(star.long - prev_star.long);

            assert_eq!(
                xiu, prev_star.lunar_mansions,
                "Failed at point before mansion {:?}",
                star.lunar_mansions
            );
            assert!(
                // 由于f64精度问题，这里需要放宽判断，
                // 实际计算结果，这个绝对值大于1e-9
                (xiu_degree - prev_star_span).abs() < 1e-8,
                "Failed at point before mansion {:?} with degree {}, expected {}",
                star.lunar_mansions,
                xiu_degree,
                prev_star_span
            );
        }
    }

    #[test]
    fn test_calc_distance_star_long_snapshot() {
        dotenvy::dotenv().ok();
        let ephe_path = std::env::var("EPHE_PATH")
            .expect("没设置 EPHE_PATH 环境变量，可在.env文件中设置或export EPHE_PATH=...");

        let distance_star_config = DistanceStarConfig::default_all_configs();
        let jd_utc = 2451545.0; // J2000.0

        let result = calc_distance_star_long(jd_utc, &distance_star_config, &ephe_path).unwrap();
        insta::assert_yaml_snapshot!(result);
    }
}
