#[cfg(feature = "serde")]
use serde::Serialize;

#[cfg(feature = "swagger")]
use utoipa::ToSchema;

#[cfg(test)]
mod tests;

use swe::swe_degnorm;

use crate::aspect::Aspect;
use crate::config::PlanetConfig;
use crate::utils::included_angle;
use PlanetSpeedState::*;

#[derive(PartialEq, Eq, Debug, Clone)]
#[cfg_attr(feature = "serde", derive(Serialize))]
#[cfg_attr(feature = "swagger", derive(ToSchema))]
pub enum PlanetSpeedState {
    快,
    均,
    慢,
}

#[derive(PartialEq, Eq, Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
#[cfg_attr(feature = "swagger", derive(ToSchema))]
pub enum PlanetName {
    ASC,
    MC,
    DSC,
    IC,
    Sun,
    Moon,
    Mercury,
    Venus,
    Mars,
    Jupiter,
    Saturn,
    // MeanNode,
    NorthNode,
    SouthNode,
}
#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
#[cfg_attr(feature = "swagger", derive(ToSchema))]
pub struct Planet {
    // @field:Schema(description = "七颗行星，北交，莉莉丝按瑞士星历表的行星编号。" +
    //         "ASC: -1, MC: -2, DES: -3, IC: -4。" +
    //         "南交点：北交点*-1")
    pub name: PlanetName,
    /// 行星的黄经
    pub long: f64,
    /// 行星的黄纬
    pub lat: f64,
    /// 行星在黄道上每日的移动速度
    pub speed: f64,
    /// 行星的赤经
    pub ra: f64,
    /// 行星的赤纬
    pub dec: f64,
    /// 行星的容许度
    pub orb: u8,
    // 太阳、南北交点、四轴每日移动速度平均，可以两值设置为一样，如0.0
    // private val minSpeed :Double,
    // private val maxSpeed :Double,
    /// 行星速度状态：快、平均、慢
    pub speed_state: PlanetSpeedState,
}

impl Planet {
    pub fn new(
        name: PlanetName,
        long: f64,
        lat: f64,
        speed: f64,
        ra: f64,
        dec: f64,
        config: &PlanetConfig,
    ) -> Self {
        let speed_state = if config.min < config.max {
            if speed.abs() > config.max {
                快
            } else if speed.abs() < config.min {
                慢
            } else {
                均
            }
        } else {
            均
        };
        Self {
            name,
            long,
            lat,
            speed,
            ra,
            dec,
            orb: config.orb,
            speed_state,
        }
    }

    /**
     * 这颗行星与给定的行星有相位否
     * 比较盘中，self为本盘星体
     * @param p
     * 另一颗行星
     * @param compare
     * 比较盘，此值为True
     * @return
     * 有相位：true
     */
    pub fn has_aspect(&self, p: &Planet, compare: bool) -> Option<Aspect> {
        let aspect = p0_has_aspect_p1(self, p, compare);

        let aspect = if let Some(aspect) = aspect {
            aspect
        } else {
            return None;
        };

        // 计算行星所落星座有无相位
        // d = self行星所落星座-p行星所落星座，此值是p行星逆时针到self行星的度数
        // d可能的值<0, 0<=d<=180, 180<d<360
        // d<0，d=d+360转换到[0,360)
        // 180<d<360, d=360-d
        let s0 = self.long as i32 / 30 * 30;
        let s1 = p.long as i32 / 30 * 30;
        let mut d = s0 - s1;
        if d < 0 {
            d += 360
        }
        if d > 180 {
            d = 360 - d
        }
        if d as u8 == aspect.aspect_value {
            Some(aspect)
        } else {
            None
        }
    }
}

/// 本行星与另一颗行星有相位否，只计算行星间度数，未计算行星所落星座有相位
/// 比较盘，此值为True
/// 比较盘中，p0为本盘星体
/// @param p
/// 另一个行星
/// @return
/// 此行星与行星p有相位，返回true
fn p0_has_aspect_p1(p0: &Planet, p1: &Planet, compare: bool) -> Option<Aspect> {
    let f = if compare {
        compare_has_aspect_0
    } else {
        no_compare_has_aspect_0
    };
    let aspect_values = [0.0, 60.0, 90.0, 120.0, 180.0];
    let long_diff = swe_degnorm(p0.long - p1.long);

    let aspect = aspect_values.iter().find_map(|&aspect_value| {
        let aspect = if long_diff <= 180.0 {
            let mut p = p1.clone();
            p.long = swe_degnorm(p1.long + aspect_value);
            f(p0, &p)
        } else {
            let mut p = p0.clone();
            p.long = swe_degnorm(p0.long + aspect_value);
            f(&p, p1)
        };

        if let Some(mut aspect) = aspect {
            aspect.aspect_value += aspect_value as u8;
            Some(aspect)
        } else {
            None
        }
    });

    aspect
}

/// 非比较盘0度相位，本行星与另一颗行星有相位否，只计算行星间度数，未计算行星所落星座有相位
/// @param p
/// 另一个行星或虚点
/// @return
/// 此行星与行星p有相位，返回true
fn no_compare_has_aspect_0(p0: &Planet, p1: &Planet) -> Option<Aspect> {
    //本命不考虑以下两种情况的相位
    if p0.name == p1.name {
        return None;
    }

    //北交与南交不能看相位
    let nodes = [PlanetName::NorthNode, PlanetName::SouthNode];
    if nodes.contains(&p0.name) && nodes.contains(&p1.name) {
        return None;
    }

    // ASC与DSC永远对冲， MC与IC永远对冲，无相位
    let asc_dsc = [PlanetName::ASC, PlanetName::DSC];
    if asc_dsc.contains(&p0.name) && asc_dsc.contains(&p1.name) {
        return None;
    }

    let mc_ic = [PlanetName::MC, PlanetName::IC];
    if mc_ic.contains(&p0.name) && mc_ic.contains(&p1.name) {
        return None;
    }

    // 以下计算相位

    // ASC与MC、IC有相位，DSC与MC、IC有相位
    let ascmc = [
        PlanetName::ASC,
        PlanetName::MC,
        PlanetName::DSC,
        PlanetName::IC,
    ];
    if ascmc.contains(&p0.name) && ascmc.contains(&p1.name) {
        let d = included_angle(p0.long, p1.long);
        if d == 0.0 {
            return Some(Aspect::new(0, false, 0.0, p0.name.clone(), p1.name.clone()));
        } else {
            return None;
        }
    }
    // 行星与ASC 、MC、DSC、IC的相位
    // if ascmc.contains(&p0.name) || ascmc.contains(&p1.name) {
    let aspect_diff = included_angle(p0.long, p1.long);
    let orb_half = f64::from(p0.orb + p1.orb) / 2.0;

    // 没有在容许度内，无相位
    if aspect_diff > orb_half {
        return None;
    }

    //p_0为asc, mc, dsc, ic, 快速行星
    //p_1为慢速行星
    let (p_0, p_1) = if ascmc.contains(&p1.name) {
        (p1, p0)
    } else if ascmc.contains(&p0.name) {
        (p0, p1)
    } else if p1.speed.abs() > p0.speed.abs() {
        (p1, p0)
    } else {
        (p0, p1)
    };

    // 二星度数相等，为出相位
    // 正相位，不必作判断了
    if aspect_diff == 0.0 {
        return Some(Aspect::new(0, false, 0.0, p0.name.clone(), p1.name.clone()));
    }

    // 二星度数不相等
    // 假定慢行星不动，给虚点或快速行星一个增量:aspectDiff
    // 二星顺行时
    // p_0:4.7,p_1:5.0
    // 二星相距0.3
    // 增量为0.3
    // 0.3+4.7-5.0>0.0
    //判定为入相位
    // 在合相时，p_0:5.3,p_1:5.0
    // aspect_diff=0.3
    // 0.3+5.3-5.0==0.0
    // 判定为出相位

    // 二星逆行时
    // p_0:4.7,p_1:5.0
    // 二星相距0.3
    // 增量为0.3
    // -0.3+4.7-5.0>0.0
    //判定为出相位
    // 在合相时，p_0:5.3,p_1:5.0
    // aspect_diff=0.3
    // -0.3+5.3-5.0==0.0
    // 判定为入相位

    // 快速行星顺行，慢速行星逆行，依二得顺行判断
    // 快速行星逆行，慢速行星顺，依二星逆行判断

    // 综上
    // delta = p_0.speed/p_0.spped.abs() * aspectDiff
    // 以p_0.long + delta - p_1.long >0为出相，否则为入相

    // 增量,需要考虑行星顺逆
    let delta = if p_0.speed < 0.0 {
        -aspect_diff
    } else {
        aspect_diff
    };

    let aspect_diff1 = included_angle(p_0.long + delta, p_1.long);

    if aspect_diff1 > 0.0 {
        return Some(Aspect::new(
            0,
            false,
            aspect_diff,
            p0.name.clone(),
            p1.name.clone(),
        ));
    } else {
        return Some(Aspect::new(
            0,
            true,
            aspect_diff,
            p0.name.clone(),
            p1.name.clone(),
        ));
    }
}

// 比较盘0度相位，只计算行星间度数，未计算行星所落星座有相位
/// p0,本命盘行星，不动
/// p1，比较盘行星，动
fn compare_has_aspect_0(p0: &Planet, p1: &Planet) -> Option<Aspect> {
    let orb_half = f64::from(p0.orb + p1.orb) / 2.0;
    let aspect_diff = included_angle(p0.long, p1.long);
    // 对p0,p1都是四轴，也适用，因为都是四轴，orb_half==0
    if aspect_diff > orb_half {
        return None;
    }
    // p0,p1都是四轴，p0.long==p1.long有相位

    if aspect_diff == 0.0 {
        return Some(Aspect::new(0, false, 0.0, p0.name.clone(), p1.name.clone()));
    }

    // 以上条件已经排除二星都是四轴的情况，剩余以下情况
    // 本命：行星，行运：四轴
    // 本命：四轴，行运：行星
    // 本命：行星，行运：行星
    // 可采用非比较盘算法的

    let delta = if p1.speed < 0.0 {
        -aspect_diff
    } else {
        aspect_diff
    };

    let aspect_diff1 = included_angle(p1.long + delta, p0.long);

    if aspect_diff1 > 0.0 {
        return Some(Aspect::new(
            0,
            false,
            aspect_diff,
            p0.name.clone(),
            p1.name.clone(),
        ));
    } else {
        return Some(Aspect::new(
            0,
            true,
            aspect_diff,
            p0.name.clone(),
            p1.name.clone(),
        ));
    }
}
