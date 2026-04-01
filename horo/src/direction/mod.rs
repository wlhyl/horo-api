mod asc;
mod mc;
mod planet;
mod term;
mod utils;

use geo_position::GeoPosition;
use horo_date_time::{HoroDateTime, horo_date_time};

use crate::{
    Error, Horoscope, HouseName, PlanetConfig, PlanetName,
    direction::{
        asc::{asc_direction, dsc_direction},
        mc::{ic_direction, mc_direction},
        planet::planet_direction,
        utils::{planet_to_planet_direction, promittors_of_planets},
    },
};

const MAX_ARC: f64 = 120.0;

#[cfg(feature = "serde")]
use serde::Serialize;

#[cfg(feature = "swagger")]
use utoipa::ToSchema;

#[derive(Clone, Debug, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize))]
#[cfg_attr(feature = "swagger", derive(ToSchema))]
pub struct Direction {
    pub significator: PlanetName,
    pub promittor: Promittor,
    pub arc: f64,
    pub date: HoroDateTime,
}

impl Direction {
    pub fn new(
        significator: PlanetName,
        promittor: Promittor,
        arc: f64,
        date: HoroDateTime,
    ) -> Self {
        Self {
            significator,
            promittor,
            arc,
            date,
        }
    }
}

#[derive(Clone, Debug, Copy, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(Serialize), serde(rename_all = "camelCase"))]
#[cfg_attr(feature = "swagger", derive(ToSchema))]
pub enum Promittor {
    // 合相
    Conjunction(PlanetName),
    // 120度相
    SinisterTrine(PlanetName),
    DexterTrine(PlanetName),
    // 60度相
    SinisterSextile(PlanetName),
    DexterSextile(PlanetName),
    // 90度相
    SinisterSquare(PlanetName),
    DexterSquare(PlanetName),
    // 180度相
    Opposition(PlanetName),
    // 界
    Term(PlanetName, u16),

    // 映点
    Antiscoins(PlanetName),
    // 反映点
    Contraantiscias(PlanetName),
}
// 推运点
// ASC
// MC
// 月亮
// 太阳
// 福点
// 以下行星一般不推运
// 土星，可用于何时买房,死亡
pub fn direction_process(
    native_date: HoroDateTime,
    geo: GeoPosition,
    planets_config: &[PlanetConfig],
    ephe_path: &str,
) -> Result<Vec<Direction>, Error> {
    // 计算原星盘
    let horo = Horoscope::new(
        native_date,
        geo,
        HouseName::Regiomontanus,
        planets_config,
        ephe_path,
    )?;

    let promittors = promittors_of_planets(&horo);

    let mut directions = mc_direction(&horo, &promittors)?;
    directions.extend(asc_direction(&horo, &promittors)?);
    directions.extend(dsc_direction(&horo, &promittors)?);
    directions.extend(ic_direction(&horo, &promittors)?);

    for significator in &horo.planets {
        directions.extend(planet_direction(&horo, significator, &promittors)?);
    }

    directions.extend(planet_direction(&horo, &horo.part_of_fortune, &promittors)?);

    directions.sort_by(|a, b| a.arc.abs().total_cmp(&b.arc.abs()));

    Ok(directions)
}

fn arc_to_date(arc: f64, native_date: &HoroDateTime) -> Result<HoroDateTime, Error> {
    let y = arc.floor();

    let t0 = horo_date_time(
        native_date.year + y as i32,
        native_date.month,
        native_date.day,
        native_date.hour,
        native_date.minute,
        native_date.second,
        native_date.tz,
        false,
    )?;

    let t1 = horo_date_time(
        native_date.year + y as i32 + 1,
        native_date.month,
        native_date.day,
        native_date.hour,
        native_date.minute,
        native_date.second,
        native_date.tz,
        false,
    )?;

    let days = (t1.jd_utc - t0.jd_utc) * (arc - y);

    Ok(t0.plus_days(days)?)
}

#[cfg(test)]
mod tests {
    use super::*;
    use horo_date_time::HoroDateTime;
    use std::env;

    pub(crate) fn get_ephe_path() -> String {
        dotenvy::dotenv().ok();
        env::var("EPHE_PATH")
            .expect("没设置 EPHE_PATH 环境变量，可在.env文件中设置或export EPHE_PATH=...")
    }

    #[test]
    fn test_arc_to_date_zero_arc() {
        let native_date = HoroDateTime::new(2000, 1, 1, 12, 0, 0, 8.0).unwrap();
        let result = arc_to_date(0.0, &native_date).unwrap();

        assert!((native_date.jd_utc - result.jd_utc).abs() * 24.0 * 3600.0 < 1.0);
    }

    #[test]
    fn test_arc_to_date_one_degree() {
        let native_date = HoroDateTime::new(2000, 1, 1, 12, 0, 0, 8.0).unwrap();
        let result = arc_to_date(1.0, &native_date.clone()).unwrap();

        assert_eq!(result.year, 2001);
        assert_eq!(result.month, 1);
        assert_eq!(result.day, 1);
        assert!(
            (result.jd_utc
                - result.jd_utc.floor()
                - (native_date.jd_utc - native_date.jd_utc.floor()))
            .abs()
                * 24.0
                * 3600.0
                < 1.0
        );
    }

    // 测试小数弧度的日期转换
    #[test]
    fn test_arc_to_date_fractional_arc() {
        let native_date = HoroDateTime::new(2000, 1, 1, 12, 0, 0, 8.0).unwrap();
        let result = arc_to_date(10.5, &native_date.clone()).unwrap();

        assert_eq!(result.year, 2010);
        assert_eq!(result.month, 7);
        let day_diff = (i32::from(result.day) - 1).abs();
        assert!(
            day_diff <= 1,
            "日期应该在7月1日左右，实际为: {}",
            result.day
        );
    }

    #[test]
    fn test_arc_to_date_leap_year() {
        let native_date = HoroDateTime::new(2000, 2, 29, 12, 0, 0, 8.0).unwrap();
        let result = arc_to_date(4.0, &native_date.clone()).unwrap();

        assert_eq!(result.year, 2004);
        assert_eq!(result.month, 2);
        assert_eq!(result.day, 29);
    }

    #[test]
    fn test_direction_new() {
        let date = HoroDateTime::new(2000, 1, 1, 12, 0, 0, 8.0).unwrap();
        let direction = Direction::new(
            PlanetName::MC,
            Promittor::Conjunction(PlanetName::Sun),
            45.0,
            date,
        );

        assert_eq!(direction.significator, PlanetName::MC);
        assert!(matches!(
            direction.promittor,
            Promittor::Conjunction(PlanetName::Sun)
        ));
        assert!((direction.arc - 45.0).abs() < 1e-10);
        assert!((direction.date.jd_utc - date.jd_utc).abs() * 24.0 * 3600.0 < 1.0);
    }
}
