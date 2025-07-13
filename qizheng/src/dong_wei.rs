use crate::LunarMansionsName;

#[cfg(feature = "serde")]
use serde::Serialize;

#[cfg(feature = "swagger")]
use utoipa::ToSchema;

#[cfg_attr(feature = "serde", derive(Serialize))]
#[cfg_attr(feature = "swagger", derive(ToSchema))]
/// 洞微大限
pub struct DongWei {
    /// 洞微大限每一年的黄道经度，从0岁起至洞微大限总年数，洞微大限总年数略去小数部分，起算点为每年的公历生日
    long_of_per_year: Vec<f64>,
    /// 当前推运时间的洞微大限黄道经度
    long: f64,
    /// 当前推运时间的洞微大限黄道经度所在宿名
    xiu: LunarMansionsName,
    /// 当前推运时间的洞微大限道经度的入宿度数
    xiu_degree: f64,
}

impl DongWei {
    pub fn new(
        long_of_per_year: Vec<f64>,
        long: f64,
        xiu: LunarMansionsName,
        xiu_degree: f64,
    ) -> Self {
        Self {
            long_of_per_year,
            long,
            xiu,
            xiu_degree,
        }
    }
}
