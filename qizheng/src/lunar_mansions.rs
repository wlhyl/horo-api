use swe::{swe_close, swe_fixstar2_ut, swe_set_ephe_path, Flag};

use crate::{config::DistanceStarConfig, error::Error};

#[cfg(feature = "serde")]
use serde::Serialize;

#[cfg(feature = "swagger")]
use utoipa::ToSchema;

/// 二十八宿名
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
#[cfg_attr(feature = "swagger", derive(ToSchema))]
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
        let (_, xx) = swe_fixstar2_ut(&star_name, jd_utc, &[Flag::SeflgSwieph])
            .map_err(|e| Error::Function(format!("计算二十八距星错误:{e}")))?;
        distance_star_long.push(DistanceStarLong {
            lunar_mansions: distance_star.lunar_mansions,
            long: xx[0],
        });
    }
    swe_close();
    Ok(distance_star_long)
}
