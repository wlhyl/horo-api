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
