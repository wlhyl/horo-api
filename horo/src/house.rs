#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};
use swe::HouseSystem;
#[cfg(feature = "swagger")]
use utoipa::ToSchema;

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[cfg_attr(feature = "swagger", derive(ToSchema))]
pub enum HouseName {
    Alcabitus,
    Placidus,
    Regiomontanus,
    WholeSign,
}

impl HouseName {
    pub fn all_house_names() -> Vec<HouseName> {
        vec![
            HouseName::Alcabitus,
            HouseName::Placidus,
            HouseName::Regiomontanus,
            HouseName::WholeSign,
        ]
    }
}

impl From<&HouseName> for HouseSystem {
    fn from(value: &HouseName) -> Self {
        match value {
            HouseName::Alcabitus => HouseSystem::B,
            HouseName::Placidus => HouseSystem::P,
            HouseName::WholeSign => HouseSystem::W,
            HouseName::Regiomontanus => HouseSystem::R,
        }
    }
}

#[cfg(test)]
mod tests {
    use swe::HouseSystem;

    use crate::HouseName;

    #[test]
    fn test_all_house_names() {
        let house_names = HouseName::all_house_names();
        assert_eq!(house_names.len(), 4);
        assert!(matches!(house_names[0], HouseName::Alcabitus));
        assert!(matches!(house_names[1], HouseName::Placidus));
        assert!(matches!(house_names[2], HouseName::Regiomontanus));
        assert!(matches!(house_names[3], HouseName::WholeSign));
    }

    #[test]
    fn test_house_name_to_house_system() {
        let house_system: HouseSystem = (&HouseName::Alcabitus).into();
        assert!(matches!(house_system, HouseSystem::B));

        let house_system: HouseSystem = (&HouseName::Placidus).into();
        assert!(matches!(house_system, HouseSystem::P));

        let house_system: HouseSystem = (&HouseName::Regiomontanus).into();
        assert!(matches!(house_system, HouseSystem::R));

        let house_system: HouseSystem = (&HouseName::WholeSign).into();
        assert!(matches!(house_system, HouseSystem::W));
    }
}
