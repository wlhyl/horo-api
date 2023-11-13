use crate::planet::PlanetName;
#[cfg(feature = "serde")]
use serde::Serialize;

#[cfg(feature = "swagger")]
use utoipa::ToSchema;

/// 相位
/// @param aspectValue
/// 相位值: 0, 60, 90, 120, 180, null
/// @param apply
/// true: 入相位, false: 出相位
/// @param d
/// 入或出相位度数
/// @param p0
/// 第一颗行星
/// @param p1
/// 第二颗行星
#[derive(Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
#[cfg_attr(feature = "swagger", derive(ToSchema))]
pub struct Aspect {
    /// 行星p0与p1的相位值
    pub aspect_value: u8,
    /// 入相位: true, 出相位: false
    pub apply: bool,
    /// 入相位或出相位多少度
    pub d: f64,
    /// 行星p0的name
    pub p0: PlanetName,
    /// 行星p1的name
    pub p1: PlanetName,
}

impl Aspect {
    pub fn new(aspect_value: u8, apply: bool, d: f64, p0: PlanetName, p1: PlanetName) -> Self {
        Self {
            aspect_value,
            apply,
            d,
            p0,
            p1,
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::planet::PlanetName;

    use super::Aspect;

    #[test]
    fn test_new() {
        let aspect_value = 120;
        let apply = true;
        let d = 3.0;
        let p0 = PlanetName::Sun;
        let p1 = PlanetName::Moon;

        let aspect = Aspect::new(aspect_value, apply, d, p0.clone(), p1.clone());

        assert_eq!(aspect.aspect_value, 120);
        assert!(aspect.apply);
        assert_eq!(aspect.d, d);
        assert_eq!(aspect.p0, p0);
        assert_eq!(aspect.p1, p1);
    }
}
