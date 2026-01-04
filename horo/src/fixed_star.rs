use qizheng::{DistanceStarConfig, LunarMansionsName, calc_distance_star_long, calc_xiu_degree};
use swe::{Flag, swe_close, swe_fixstar2_ut, swe_set_ephe_path};

use crate::error::Error;

#[cfg(feature = "serde")]
use serde::Serialize;

#[cfg(feature = "swagger")]
use utoipa::ToSchema;

use FixedStarName::*;

/// 恒星名
#[cfg_attr(feature = "serde", derive(Serialize))]
#[cfg_attr(feature = "swagger", derive(ToSchema))]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub enum FixedStarName {
    角宿一,
    大角星,
    氐宿一,
    心宿二,
    北门师落,
    大陵五,
    毕宿五,
    五车二,
    参宿四,
    天狼星,
    南河三,
    轩辕十四,
}

/// 恒星的黄道经度
#[cfg_attr(feature = "serde", derive(Serialize))]
#[cfg_attr(feature = "swagger", derive(ToSchema))]
#[derive(Clone, Copy, Debug)]
pub struct FixedStar {
    /// 恒星的名称
    pub fixed_star: FixedStarName,
    /// 恒星距星的黄道经度
    pub long: f64,
    /// 行星在黄道上的入宿名
    pub xiu: LunarMansionsName,
    /// 行星在黄道上的入宿度
    pub xiu_degree: f64,
}

impl FixedStarName {
    fn to_swe_name(&self) -> &'static str {
        match self {
            氐宿一 => "al-2Lib",
            大陵五 => "bePer",
            毕宿五 => "alTau",
            心宿二 => "alSco",
            角宿一 => "alVir",
            轩辕十四 => "alLeo",
            大角星 => "alBoo",
            参宿四 => "alOri",
            五车二 => "alAur",
            北门师落 => "alPsA",
            天狼星 => "alCMa",
            南河三 => "alCMi",
        }
    }
}

/// 计算恒星的黄道经度
pub fn calc_fixed_star_long(jd_utc: f64, ephe_path: &str) -> Result<Vec<FixedStar>, Error> {
    let distance_star_config = DistanceStarConfig::default_all_configs();

    let distance_star_long = calc_distance_star_long(jd_utc, &distance_star_config, ephe_path)?;

    let mut fixed_star_long = vec![];

    swe_set_ephe_path(ephe_path);

    for fixed_star in [
        角宿一,
        大角星,
        氐宿一,
        心宿二,
        北门师落,
        大陵五,
        毕宿五,
        五车二,
        参宿四,
        天狼星,
        南河三,
        轩辕十四,
    ] {
        let star_name = format!(",{}", fixed_star.to_swe_name());
        let (_, xx) = swe_fixstar2_ut(&star_name, jd_utc, &[Flag::SeflgSwieph])
            .map_err(|e| Error::Function(format!("计算恒星星`{:?}`错误:{}", fixed_star, e)))?;

        let star_long = xx[0];
        let xiu_degree = calc_xiu_degree(star_long, &distance_star_long)?;

        fixed_star_long.push(FixedStar {
            fixed_star,
            long: star_long,
            xiu: xiu_degree.0,
            xiu_degree: xiu_degree.1,
        });
    }
    swe_close();
    Ok(fixed_star_long)
}

#[cfg(test)]
mod tests {

    use super::calc_fixed_star_long;

    #[test]
    fn test_calc_fixed_star_long_snapshot() {
        dotenvy::dotenv().ok();
        let ephe_path = std::env::var("EPHE_PATH")
            .expect("没设置 EPHE_PATH 环境变量，可在.env文件中设置或export EPHE_PATH=...");

        let jd_utc = 2451545.0; // J2000.0

        let result = calc_fixed_star_long(jd_utc, &ephe_path).unwrap();
        insta::assert_yaml_snapshot!(result);
    }
}
