use ganzhiwuxing::TianGan;

use TenGods::*;

#[cfg(feature = "serde")]
use serde::Serialize;

#[cfg(feature = "swagger")]
use utoipa::ToSchema;

#[cfg_attr(feature = "serde", derive(Serialize))]
#[cfg_attr(feature = "swagger", derive(ToSchema))]
#[derive(Clone, Copy)]
/// 十神
pub(crate) enum TenGods {
    比肩,
    劫财,
    食神,
    伤官,
    正财,
    偏财,
    正官,
    七杀,
    正印,
    偏印,
}

pub(crate) trait ToTenGods {
    fn to_ten_gods(self, gan: TianGan) -> TenGods;
}

impl ToTenGods for TianGan {
    fn to_ten_gods(self, gan: TianGan) -> TenGods {
        if self == gan {
            return 比肩;
        }

        if self.wu_xing() == gan.wu_xing() {
            return 劫财;
        }

        if self.sheng(&gan) {
            if self.masculine() == gan.masculine() {
                return 食神;
            } else {
                return 伤官;
            }
        }

        if self.ke(&gan) {
            if self.masculine() == gan.masculine() {
                return 偏财;
            } else {
                return 正财;
            }
        }

        if gan.ke(&self) {
            if gan.masculine() == self.masculine() {
                return 七杀;
            } else {
                return 正官;
            }
        }

        if self.masculine() == gan.masculine() {
            return 偏印;
        } else {
            return 正印;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ganzhiwuxing::TianGan;

    #[test]
    fn test_to_ten_gods() {
        // 甲
        assert!(matches!(TianGan::甲.to_ten_gods(TianGan::甲), TenGods::比肩));
        assert!(matches!(TianGan::甲.to_ten_gods(TianGan::乙), TenGods::劫财));
        assert!(matches!(TianGan::甲.to_ten_gods(TianGan::丙), TenGods::食神));
        assert!(matches!(TianGan::甲.to_ten_gods(TianGan::丁), TenGods::伤官));
        assert!(matches!(TianGan::甲.to_ten_gods(TianGan::戊), TenGods::偏财));
        assert!(matches!(TianGan::甲.to_ten_gods(TianGan::己), TenGods::正财));
        assert!(matches!(TianGan::甲.to_ten_gods(TianGan::庚), TenGods::七杀));
        assert!(matches!(TianGan::甲.to_ten_gods(TianGan::辛), TenGods::正官));
        assert!(matches!(TianGan::甲.to_ten_gods(TianGan::壬), TenGods::偏印));
        assert!(matches!(TianGan::甲.to_ten_gods(TianGan::癸), TenGods::正印));

        // 乙
        assert!(matches!(TianGan::乙.to_ten_gods(TianGan::甲), TenGods::劫财));
        assert!(matches!(TianGan::乙.to_ten_gods(TianGan::乙), TenGods::比肩));
        assert!(matches!(TianGan::乙.to_ten_gods(TianGan::丙), TenGods::伤官));
        assert!(matches!(TianGan::乙.to_ten_gods(TianGan::丁), TenGods::食神));
        assert!(matches!(TianGan::乙.to_ten_gods(TianGan::戊), TenGods::正财));
        assert!(matches!(TianGan::乙.to_ten_gods(TianGan::己), TenGods::偏财));
        assert!(matches!(TianGan::乙.to_ten_gods(TianGan::庚), TenGods::正官));
        assert!(matches!(TianGan::乙.to_ten_gods(TianGan::辛), TenGods::七杀));
        assert!(matches!(TianGan::乙.to_ten_gods(TianGan::壬), TenGods::正印));
        assert!(matches!(TianGan::乙.to_ten_gods(TianGan::癸), TenGods::偏印));

        // 丙
        assert!(matches!(TianGan::丙.to_ten_gods(TianGan::甲), TenGods::偏印));
        assert!(matches!(TianGan::丙.to_ten_gods(TianGan::乙), TenGods::正印));
        assert!(matches!(TianGan::丙.to_ten_gods(TianGan::丙), TenGods::比肩));
        assert!(matches!(TianGan::丙.to_ten_gods(TianGan::丁), TenGods::劫财));
        assert!(matches!(TianGan::丙.to_ten_gods(TianGan::戊), TenGods::食神));
        assert!(matches!(TianGan::丙.to_ten_gods(TianGan::己), TenGods::伤官));
        assert!(matches!(TianGan::丙.to_ten_gods(TianGan::庚), TenGods::偏财));
        assert!(matches!(TianGan::丙.to_ten_gods(TianGan::辛), TenGods::正财));
        assert!(matches!(TianGan::丙.to_ten_gods(TianGan::壬), TenGods::七杀));
        assert!(matches!(TianGan::丙.to_ten_gods(TianGan::癸), TenGods::正官));

        // 丁
        assert!(matches!(TianGan::丁.to_ten_gods(TianGan::甲), TenGods::正印));
        assert!(matches!(TianGan::丁.to_ten_gods(TianGan::乙), TenGods::偏印));
        assert!(matches!(TianGan::丁.to_ten_gods(TianGan::丙), TenGods::劫财));
        assert!(matches!(TianGan::丁.to_ten_gods(TianGan::丁), TenGods::比肩));
        assert!(matches!(TianGan::丁.to_ten_gods(TianGan::戊), TenGods::伤官));
        assert!(matches!(TianGan::丁.to_ten_gods(TianGan::己), TenGods::食神));
        assert!(matches!(TianGan::丁.to_ten_gods(TianGan::庚), TenGods::正财));
        assert!(matches!(TianGan::丁.to_ten_gods(TianGan::辛), TenGods::偏财));
        assert!(matches!(TianGan::丁.to_ten_gods(TianGan::壬), TenGods::正官));
        assert!(matches!(TianGan::丁.to_ten_gods(TianGan::癸), TenGods::七杀));

        // 戊
        assert!(matches!(TianGan::戊.to_ten_gods(TianGan::甲), TenGods::七杀));
        assert!(matches!(TianGan::戊.to_ten_gods(TianGan::乙), TenGods::正官));
        assert!(matches!(TianGan::戊.to_ten_gods(TianGan::丙), TenGods::偏印));
        assert!(matches!(TianGan::戊.to_ten_gods(TianGan::丁), TenGods::正印));
        assert!(matches!(TianGan::戊.to_ten_gods(TianGan::戊), TenGods::比肩));
        assert!(matches!(TianGan::戊.to_ten_gods(TianGan::己), TenGods::劫财));
        assert!(matches!(TianGan::戊.to_ten_gods(TianGan::庚), TenGods::食神));
        assert!(matches!(TianGan::戊.to_ten_gods(TianGan::辛), TenGods::伤官));
        assert!(matches!(TianGan::戊.to_ten_gods(TianGan::壬), TenGods::偏财));
        assert!(matches!(TianGan::戊.to_ten_gods(TianGan::癸), TenGods::正财));

        // 己
        assert!(matches!(TianGan::己.to_ten_gods(TianGan::甲), TenGods::正官));
        assert!(matches!(TianGan::己.to_ten_gods(TianGan::乙), TenGods::七杀));
        assert!(matches!(TianGan::己.to_ten_gods(TianGan::丙), TenGods::正印));
        assert!(matches!(TianGan::己.to_ten_gods(TianGan::丁), TenGods::偏印));
        assert!(matches!(TianGan::己.to_ten_gods(TianGan::戊), TenGods::劫财));
        assert!(matches!(TianGan::己.to_ten_gods(TianGan::己), TenGods::比肩));
        assert!(matches!(TianGan::己.to_ten_gods(TianGan::庚), TenGods::伤官));
        assert!(matches!(TianGan::己.to_ten_gods(TianGan::辛), TenGods::食神));
        assert!(matches!(TianGan::己.to_ten_gods(TianGan::壬), TenGods::正财));
        assert!(matches!(TianGan::己.to_ten_gods(TianGan::癸), TenGods::偏财));

        // 庚
        assert!(matches!(TianGan::庚.to_ten_gods(TianGan::甲), TenGods::偏财));
        assert!(matches!(TianGan::庚.to_ten_gods(TianGan::乙), TenGods::正财));
        assert!(matches!(TianGan::庚.to_ten_gods(TianGan::丙), TenGods::七杀));
        assert!(matches!(TianGan::庚.to_ten_gods(TianGan::丁), TenGods::正官));
        assert!(matches!(TianGan::庚.to_ten_gods(TianGan::戊), TenGods::偏印));
        assert!(matches!(TianGan::庚.to_ten_gods(TianGan::己), TenGods::正印));
        assert!(matches!(TianGan::庚.to_ten_gods(TianGan::庚), TenGods::比肩));
        assert!(matches!(TianGan::庚.to_ten_gods(TianGan::辛), TenGods::劫财));
        assert!(matches!(TianGan::庚.to_ten_gods(TianGan::壬), TenGods::食神));
        assert!(matches!(TianGan::庚.to_ten_gods(TianGan::癸), TenGods::伤官));

        // 辛
        assert!(matches!(TianGan::辛.to_ten_gods(TianGan::甲), TenGods::正财));
        assert!(matches!(TianGan::辛.to_ten_gods(TianGan::乙), TenGods::偏财));
        assert!(matches!(TianGan::辛.to_ten_gods(TianGan::丙), TenGods::正官));
        assert!(matches!(TianGan::辛.to_ten_gods(TianGan::丁), TenGods::七杀));
        assert!(matches!(TianGan::辛.to_ten_gods(TianGan::戊), TenGods::正印));
        assert!(matches!(TianGan::辛.to_ten_gods(TianGan::己), TenGods::偏印));
        assert!(matches!(TianGan::辛.to_ten_gods(TianGan::庚), TenGods::劫财));
        assert!(matches!(TianGan::辛.to_ten_gods(TianGan::辛), TenGods::比肩));
        assert!(matches!(TianGan::辛.to_ten_gods(TianGan::壬), TenGods::伤官));
        assert!(matches!(TianGan::辛.to_ten_gods(TianGan::癸), TenGods::食神));

        // 壬
        assert!(matches!(TianGan::壬.to_ten_gods(TianGan::甲), TenGods::食神));
        assert!(matches!(TianGan::壬.to_ten_gods(TianGan::乙), TenGods::伤官));
        assert!(matches!(TianGan::壬.to_ten_gods(TianGan::丙), TenGods::偏财));
        assert!(matches!(TianGan::壬.to_ten_gods(TianGan::丁), TenGods::正财));
        assert!(matches!(TianGan::壬.to_ten_gods(TianGan::戊), TenGods::七杀));
        assert!(matches!(TianGan::壬.to_ten_gods(TianGan::己), TenGods::正官));
        assert!(matches!(TianGan::壬.to_ten_gods(TianGan::庚), TenGods::偏印));
        assert!(matches!(TianGan::壬.to_ten_gods(TianGan::辛), TenGods::正印));
        assert!(matches!(TianGan::壬.to_ten_gods(TianGan::壬), TenGods::比肩));
        assert!(matches!(TianGan::壬.to_ten_gods(TianGan::癸), TenGods::劫财));

        // 癸
        assert!(matches!(TianGan::癸.to_ten_gods(TianGan::甲), TenGods::伤官));
        assert!(matches!(TianGan::癸.to_ten_gods(TianGan::乙), TenGods::食神));
        assert!(matches!(TianGan::癸.to_ten_gods(TianGan::丙), TenGods::正财));
        assert!(matches!(TianGan::癸.to_ten_gods(TianGan::丁), TenGods::偏财));
        assert!(matches!(TianGan::癸.to_ten_gods(TianGan::戊), TenGods::正官));
        assert!(matches!(TianGan::癸.to_ten_gods(TianGan::己), TenGods::七杀));
        assert!(matches!(TianGan::癸.to_ten_gods(TianGan::庚), TenGods::正印));
        assert!(matches!(TianGan::癸.to_ten_gods(TianGan::辛), TenGods::偏印));
        assert!(matches!(TianGan::癸.to_ten_gods(TianGan::壬), TenGods::劫财));
        assert!(matches!(TianGan::癸.to_ten_gods(TianGan::癸), TenGods::比肩));
    }
}
