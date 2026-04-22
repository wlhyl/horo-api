use crate::PlanetName;

pub(crate) struct PtolemyTerm {
    pub planet: PlanetName,
    // 界开始的度数
    pub long: u16,
    // pub start_long: u16,
    // 界结束的度数
    // pub end_long: u16,
}

// #[rustfmt::skip]
// pub(crate) fn toptolemy_term(long: f64) -> PtolemyTerm {
//     let long = swe_degnorm(long);
//     let n = (long / 30.0).floor() as u16;
//     let zodiac_degree = long - f64::from(n) * 30.0;

//     match n {
//         0 => {
//             if zodiac_degree < 7.0 {
//                 PtolemyTerm { planet: PlanetName::Jupiter, start_long: 30*n, end_long: 30*n+7 }
//             } else if zodiac_degree < 15.0 {
//                 PtolemyTerm { planet: PlanetName::Venus, start_long: 30*n+7, end_long: 30*n+15 }
//             } else if zodiac_degree < 22.0 {
//                 PtolemyTerm { planet: PlanetName::Mercury, start_long: 30*n+15, end_long: 30*n+22 }
//             } else if zodiac_degree < 27.0 {
//                 PtolemyTerm { planet: PlanetName::Mars, start_long: 30*n+22, end_long: 30*n+27 }
//             } else {
//                 PtolemyTerm { planet: PlanetName::Saturn, start_long: 30*n+27 , end_long: 30*n+30 }
//             }
//         }
//         // 金牛
//         1 => {
//             if zodiac_degree < 9.0 {
//                 PtolemyTerm { planet: PlanetName::Venus, start_long: 30*n, end_long: 30*n+9 }
//             } else if zodiac_degree < 16.0 {
//                 PtolemyTerm { planet: PlanetName::Mercury, start_long: 30*n+9, end_long: 30*n+16 }
//             } else if zodiac_degree < 23.0 {
//                 PtolemyTerm { planet: PlanetName::Jupiter, start_long: 30*n+16, end_long: 30*n+23 }
//             } else if zodiac_degree < 27.0 {
//                 PtolemyTerm { planet: PlanetName::Saturn, start_long: 30*n+23, end_long: 30*n+27 }
//             } else {
//                 PtolemyTerm { planet: PlanetName::Mars, start_long: 30*n+27, end_long: 30*n+30 }
//             }
//         }
//         // 双子
//         2 => {
//             if zodiac_degree < 8.0 {
//                 PtolemyTerm { planet: PlanetName::Mercury, start_long: 30*n, end_long: 30*n+8 }
//             } else if zodiac_degree < 15.0 {
//                 PtolemyTerm { planet: PlanetName::Jupiter, start_long: 30*n+8, end_long: 30*n+15 }
//             } else if zodiac_degree < 22.0 {
//                 PtolemyTerm { planet: PlanetName::Venus, start_long: 30*n+15, end_long: 30*n+22 }
//             } else if zodiac_degree < 26.0 {
//                 PtolemyTerm { planet: PlanetName::Saturn, start_long: 30*n+22, end_long: 30*n+26 }
//             } else {
//                 PtolemyTerm { planet: PlanetName::Mars, start_long: 30*n+26, end_long: 30*n+30 }
//             }
//         }
//         // 巨蟹
//         3 => {
//             if zodiac_degree < 7.0 {
//                 PtolemyTerm { planet: PlanetName::Mars, start_long: 30*n, end_long: 30*n+7 }
//             } else if zodiac_degree < 14.0 {
//                 PtolemyTerm { planet: PlanetName::Jupiter, start_long: 30*n+7, end_long: 30*n+14 }
//             } else if zodiac_degree < 21.0 {
//                 PtolemyTerm { planet: PlanetName::Mercury, start_long: 30*n+14, end_long: 30*n+21 }
//             } else if zodiac_degree < 28.0 {
//                 PtolemyTerm { planet: PlanetName::Venus, start_long: 30*n+21, end_long: 30*n+28 }
//             } else {
//                 PtolemyTerm { planet: PlanetName::Saturn, start_long: 30*n+28, end_long: 30*n+30 }
//             }
//         }
//         // 狮子
//         4 => {
//             if zodiac_degree < 7.0 {
//                 PtolemyTerm { planet: PlanetName::Saturn, start_long: 30*n, end_long: 30*n+7 }
//             } else if zodiac_degree < 14.0 {
//                 PtolemyTerm { planet: PlanetName::Mercury, start_long: 30*n+7, end_long: 30*n+14 }
//             } else if zodiac_degree < 20.0 {
//                 PtolemyTerm { planet: PlanetName::Venus, start_long: 30*n+14, end_long: 30*n+20 }
//             } else if zodiac_degree < 26.0 {
//                 PtolemyTerm { planet: PlanetName::Jupiter, start_long: 30*n+20, end_long: 30*n+26 }
//             } else {
//                 PtolemyTerm { planet: PlanetName::Mars, start_long: 30*n+26, end_long: 30*n+30 }
//             }
//         }
//         // 室女
//         5 => {
//             if zodiac_degree < 8.0 {
//                 PtolemyTerm { planet: PlanetName::Mercury, start_long: 30*n, end_long: 30*n+8 }
//             } else if zodiac_degree < 14.0 {
//                 PtolemyTerm { planet: PlanetName::Venus, start_long: 30*n+8, end_long: 30*n+14 }
//             } else if zodiac_degree < 19.0 {
//                 PtolemyTerm { planet: PlanetName::Jupiter, start_long: 30*n+14, end_long: 30*n+19 }
//             } else if zodiac_degree < 25.0 {
//                 PtolemyTerm { planet: PlanetName::Saturn, start_long: 30*n+19, end_long: 30*n+25 }
//             } else {
//                 PtolemyTerm { planet: PlanetName::Mars, start_long: 30*n+25, end_long: 30*n+30 }
//             }
//         }
//         // 天秤
//         6 => {
//             if zodiac_degree < 7.0 {
//                 PtolemyTerm { planet: PlanetName::Saturn, start_long: 30*n, end_long: 30*n+7 }
//             } else if zodiac_degree < 12.0 {
//                 PtolemyTerm { planet: PlanetName::Venus, start_long: 30*n+7, end_long: 30*n+12 }
//             } else if zodiac_degree < 20.0 {
//                 PtolemyTerm { planet: PlanetName::Jupiter, start_long: 30*n+12, end_long: 30*n+20 }
//             } else if zodiac_degree < 25.0 {
//                 PtolemyTerm { planet: PlanetName::Mercury, start_long: 30*n+20, end_long: 30*n+25 }
//             } else {
//                 PtolemyTerm { planet: PlanetName::Mars, start_long: 30*n+25, end_long: 30*n+30 }
//             }
//         }
//         // 天蝎
//         7 => {
//             if zodiac_degree < 7.0 {
//                 PtolemyTerm { planet: PlanetName::Mars, start_long: 30*n, end_long: 30*n+7 }
//             } else if zodiac_degree < 15.0 {
//                 PtolemyTerm { planet: PlanetName::Jupiter, start_long: 30*n+7, end_long: 30*n+15 }
//             } else if zodiac_degree < 22.0 {
//                 PtolemyTerm { planet: PlanetName::Venus, start_long: 30*n+15, end_long: 30*n+22 }
//             } else if zodiac_degree < 28.0 {
//                 PtolemyTerm { planet: PlanetName::Mercury, start_long: 30*n+22, end_long: 30*n+28 }
//             } else {
//                 PtolemyTerm { planet: PlanetName::Saturn, start_long: 30*n+28, end_long: 30*n+30 }
//             }
//         }
//         // 人马
//         8 => {
//             if zodiac_degree < 9.0 {
//                 PtolemyTerm { planet: PlanetName::Jupiter, start_long: 30*n, end_long: 30*n+9 }
//             } else if zodiac_degree < 15.0 {
//                 PtolemyTerm { planet: PlanetName::Venus, start_long: 30*n+9, end_long: 30*n+15 }
//             } else if zodiac_degree < 21.0 {
//                 PtolemyTerm { planet: PlanetName::Mercury, start_long: 30*n+15, end_long: 30*n+21 }
//             } else if zodiac_degree < 26.0 {
//                 PtolemyTerm { planet: PlanetName::Saturn, start_long: 30*n+21, end_long: 30*n+26 }
//             } else {
//                 PtolemyTerm { planet: PlanetName::Mars, start_long: 30*n+26, end_long: 30*n+30 }
//             }
//         }
//         // 摩羯
//         9 => {
//             if zodiac_degree < 7.0 {
//                 PtolemyTerm { planet: PlanetName::Venus, start_long: 30*n, end_long: 30*n+7 }
//             } else if zodiac_degree < 13.0 {
//                 PtolemyTerm { planet: PlanetName::Mercury, start_long: 30*n+7, end_long: 30*n+13 }
//             } else if zodiac_degree < 20.0 {
//                 PtolemyTerm { planet: PlanetName::Jupiter, start_long: 30*n+13, end_long: 30*n+20 }
//             } else if zodiac_degree < 26.0 {
//                 PtolemyTerm { planet: PlanetName::Mars, start_long: 30*n+20, end_long: 30*n+26 }
//             } else {
//                 PtolemyTerm { planet: PlanetName::Saturn, start_long: 30*n+26, end_long: 30*n+30 }
//             }
//         }
//         // 宝瓶
//         10 => {
//             if zodiac_degree < 7.0 {
//                 PtolemyTerm { planet: PlanetName::Saturn, start_long: 30*n, end_long: 30*n+7 }
//             } else if zodiac_degree < 13.0 {
//                 PtolemyTerm { planet: PlanetName::Mercury, start_long: 30*n+7, end_long: 30*n+13 }
//             } else if zodiac_degree < 21.0 {
//                 PtolemyTerm { planet: PlanetName::Venus, start_long: 30*n+13, end_long: 30*n+21 }
//             } else if zodiac_degree < 26.0 {
//                 PtolemyTerm { planet: PlanetName::Jupiter, start_long: 30*n+21, end_long: 30*n+26 }
//             } else {
//                 PtolemyTerm { planet: PlanetName::Mars, start_long: 30*n+26, end_long: 30*n+30 }
//             }
//         }
//         // 双鱼座
//         11 => {
//             if zodiac_degree < 9.0 {
//                 PtolemyTerm { planet: PlanetName::Venus, start_long: 30*n, end_long: 30*n+9 }
//             } else if zodiac_degree < 15.0 {
//                 PtolemyTerm { planet: PlanetName::Jupiter, start_long: 30*n+9, end_long: 30*n+15 }
//             } else if zodiac_degree < 21.0 {
//                 PtolemyTerm { planet: PlanetName::Mercury, start_long: 30*n+15, end_long: 30*n+21 }
//             } else if zodiac_degree < 27.0 {
//                 PtolemyTerm { planet: PlanetName::Mars, start_long: 30*n+21, end_long: 30*n+27 }
//             } else {
//                 PtolemyTerm { planet: PlanetName::Saturn, start_long: 30*n+27, end_long: 0 }
//             }
//         }
//         _ => {
//             unreachable!()
//         }
//     }
// }

pub(crate) const PTOLEMY_TERM: [PtolemyTerm; 60] = [
    //    白羊座
    PtolemyTerm {
        planet: PlanetName::Jupiter,
        long: 0,
    },
    PtolemyTerm {
        planet: PlanetName::Venus,
        long: 7,
    },
    PtolemyTerm {
        planet: PlanetName::Mercury,
        long: 15,
    },
    PtolemyTerm {
        planet: PlanetName::Mars,
        long: 22,
    },
    PtolemyTerm {
        planet: PlanetName::Saturn,
        long: 27,
    },
    // 金牛
    PtolemyTerm {
        planet: PlanetName::Venus,
        long: 30,
    },
    PtolemyTerm {
        planet: PlanetName::Mercury,
        long: 30 * 1 + 9,
    },
    PtolemyTerm {
        planet: PlanetName::Jupiter,
        long: 30 * 1 + 16,
    },
    PtolemyTerm {
        planet: PlanetName::Saturn,
        long: 30 * 1 + 23,
    },
    PtolemyTerm {
        planet: PlanetName::Mars,
        long: 30 * 1 + 27,
    },
    // 双子
    PtolemyTerm {
        planet: PlanetName::Mercury,
        long: 30 * 2,
    },
    PtolemyTerm {
        planet: PlanetName::Jupiter,
        long: 30 * 2 + 8,
    },
    PtolemyTerm {
        planet: PlanetName::Venus,
        long: 30 * 2 + 15,
    },
    PtolemyTerm {
        planet: PlanetName::Saturn,
        long: 30 * 2 + 22,
    },
    PtolemyTerm {
        planet: PlanetName::Mars,
        long: 30 * 2 + 26,
    },
    // 巨蟹
    PtolemyTerm {
        planet: PlanetName::Mars,
        long: 30 * 3,
    },
    PtolemyTerm {
        planet: PlanetName::Jupiter,
        long: 30 * 3 + 7,
    },
    PtolemyTerm {
        planet: PlanetName::Mercury,
        long: 30 * 3 + 14,
    },
    PtolemyTerm {
        planet: PlanetName::Venus,
        long: 30 * 3 + 21,
    },
    PtolemyTerm {
        planet: PlanetName::Saturn,
        long: 30 * 3 + 28,
    },
    // 狮子
    PtolemyTerm {
        planet: PlanetName::Saturn,
        long: 30 * 4,
    },
    PtolemyTerm {
        planet: PlanetName::Mercury,
        long: 30 * 4 + 7,
    },
    PtolemyTerm {
        planet: PlanetName::Venus,
        long: 30 * 4 + 14,
    },
    PtolemyTerm {
        planet: PlanetName::Jupiter,
        long: 30 * 4 + 20,
    },
    PtolemyTerm {
        planet: PlanetName::Mars,
        long: 30 * 4 + 26,
    },
    // 室女
    PtolemyTerm {
        planet: PlanetName::Mercury,
        long: 30 * 5,
    },
    PtolemyTerm {
        planet: PlanetName::Venus,
        long: 30 * 5 + 8,
    },
    PtolemyTerm {
        planet: PlanetName::Jupiter,
        long: 30 * 5 + 14,
    },
    PtolemyTerm {
        planet: PlanetName::Saturn,
        long: 30 * 5 + 19,
    },
    PtolemyTerm {
        planet: PlanetName::Mars,
        long: 30 * 5 + 25,
    },
    // 天秤
    PtolemyTerm {
        planet: PlanetName::Saturn,
        long: 30 * 6,
    },
    PtolemyTerm {
        planet: PlanetName::Venus,
        long: 30 * 6 + 7,
    },
    PtolemyTerm {
        planet: PlanetName::Jupiter,
        long: 30 * 6 + 12,
    },
    PtolemyTerm {
        planet: PlanetName::Mercury,
        long: 30 * 6 + 20,
    },
    PtolemyTerm {
        planet: PlanetName::Mars,
        long: 30 * 6 + 25,
    },
    // 天蝎
    PtolemyTerm {
        planet: PlanetName::Mars,
        long: 30 * 7,
    },
    PtolemyTerm {
        planet: PlanetName::Jupiter,
        long: 30 * 7 + 7,
    },
    PtolemyTerm {
        planet: PlanetName::Venus,
        long: 30 * 7 + 15,
    },
    PtolemyTerm {
        planet: PlanetName::Mercury,
        long: 30 * 7 + 22,
    },
    PtolemyTerm {
        planet: PlanetName::Saturn,
        long: 30 * 7 + 28,
    },
    // 人马
    PtolemyTerm {
        planet: PlanetName::Jupiter,
        long: 30 * 8,
    },
    PtolemyTerm {
        planet: PlanetName::Venus,
        long: 30 * 8 + 9,
    },
    PtolemyTerm {
        planet: PlanetName::Mercury,
        long: 30 * 8 + 15,
    },
    PtolemyTerm {
        planet: PlanetName::Saturn,
        long: 30 * 8 + 21,
    },
    PtolemyTerm {
        planet: PlanetName::Mars,
        long: 30 * 8 + 26,
    },
    // 摩羯
    PtolemyTerm {
        planet: PlanetName::Venus,
        long: 30 * 9,
    },
    PtolemyTerm {
        planet: PlanetName::Mercury,
        long: 30 * 9 + 7,
    },
    PtolemyTerm {
        planet: PlanetName::Jupiter,
        long: 30 * 9 + 13,
    },
    PtolemyTerm {
        planet: PlanetName::Mars,
        long: 30 * 9 + 20,
    },
    PtolemyTerm {
        planet: PlanetName::Saturn,
        long: 30 * 9 + 26,
    },
    // 宝瓶
    PtolemyTerm {
        planet: PlanetName::Saturn,
        long: 30 * 10,
    },
    PtolemyTerm {
        planet: PlanetName::Mercury,
        long: 30 * 10 + 7,
    },
    PtolemyTerm {
        planet: PlanetName::Venus,
        long: 30 * 10 + 13,
    },
    PtolemyTerm {
        planet: PlanetName::Jupiter,
        long: 30 * 10 + 21,
    },
    PtolemyTerm {
        planet: PlanetName::Mars,
        long: 30 * 10 + 26,
    },
    // 双鱼座
    PtolemyTerm {
        planet: PlanetName::Venus,
        long: 30 * 11,
    },
    PtolemyTerm {
        planet: PlanetName::Jupiter,
        long: 30 * 11 + 9,
    },
    PtolemyTerm {
        planet: PlanetName::Mercury,
        long: 30 * 11 + 15,
    },
    PtolemyTerm {
        planet: PlanetName::Mars,
        long: 30 * 11 + 21,
    },
    PtolemyTerm {
        planet: PlanetName::Saturn,
        long: 30 * 11 + 27,
    },
];
