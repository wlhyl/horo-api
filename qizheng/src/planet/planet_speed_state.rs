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
    // 留，已由字段is_stationary提供
    // 伏，未实现
    // 逆，由前端根据行星速度为负计算
}
