use PlanetName::*;
use ganzhiwuxing::TianGan;

#[cfg(feature = "serde")]
use serde::Serialize;

#[cfg(feature = "swagger")]
use utoipa::ToSchema;

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

impl PlanetName {
    pub(crate) fn to_tian_gan(&self) -> Option<TianGan> {
        match self {
            日 => None,
            月 => Some(TianGan::己),
            水 => Some(TianGan::庚),
            金 => Some(TianGan::丁),
            火 => Some(TianGan::甲),
            木 => Some(TianGan::丙),
            土 => Some(TianGan::戊),
            计 => Some(TianGan::壬),
            罗 => Some(TianGan::癸),
            孛 => Some(TianGan::乙),
            气 => Some(TianGan::辛),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ganzhiwuxing::TianGan;

    #[test]
    fn test_to_tian_gan() {
        assert_eq!(PlanetName::日.to_tian_gan(), None);
        assert_eq!(PlanetName::月.to_tian_gan(), Some(TianGan::己));
        assert_eq!(PlanetName::水.to_tian_gan(), Some(TianGan::庚));
        assert_eq!(PlanetName::金.to_tian_gan(), Some(TianGan::丁));
        assert_eq!(PlanetName::火.to_tian_gan(), Some(TianGan::甲));
        assert_eq!(PlanetName::木.to_tian_gan(), Some(TianGan::丙));
        assert_eq!(PlanetName::土.to_tian_gan(), Some(TianGan::戊));
        assert_eq!(PlanetName::计.to_tian_gan(), Some(TianGan::壬));
        assert_eq!(PlanetName::罗.to_tian_gan(), Some(TianGan::癸));
        assert_eq!(PlanetName::孛.to_tian_gan(), Some(TianGan::乙));
        assert_eq!(PlanetName::气.to_tian_gan(), Some(TianGan::辛));
    }
}
