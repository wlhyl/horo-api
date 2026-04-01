use swe::{CalcFlag, swe_azalt, swe_cotrans, swe_degnorm};

use crate::{
    Error, Horoscope, Planet, PlanetConfig, PlanetName, Promittor,
    direction::term::{PTOLEMY_TERM, PtolemyTerm},
};

/// 计算斜升差(Ascensional Difference)
/// 公式: AD = arcsin(tan(D) * tan(φ))
/// * dec: 天体赤纬，单位：度
/// * geo_lat: 地理纬度，单位：度
/// 返回：斜升差（度），如果结果不是有限值则返回错误
#[inline]
pub(super) fn calc_ad(dec: f64, geo_lat: f64) -> Result<f64, Error> {
    let dec_rad = dec.to_radians();
    let lat_rad = geo_lat.to_radians();
    let result = (dec_rad.tan() * lat_rad.tan()).asin().to_degrees();

    if result.is_finite() {
        Ok(result)
    } else {
        Err(Error::Function(format!(
            "calc_ad 计算结果无效: dec={}, geo_lat={}, result={}",
            dec, geo_lat, result
        )))
    }
}

/// 计算ASC的斜赤经(Oblique Ascension)
/// 公式: ASC_OA = MC赤经 + 90°
/// * mc_ra: MC的赤经，单位：度
#[inline]
pub(super) fn calc_asc_oa(mc_ra: f64) -> f64 {
    swe_degnorm(mc_ra + 90.0)
}

/// 计算promittor的斜赤经(Oblique Ascension)
/// 公式: promittor_OA = 赤经 - AD
/// * ra: promittor的赤经，单位：度
/// * dec: promittor的赤纬，单位：度
/// * geo_lat: 地理纬度，单位：度
/// 返回：斜赤经（度），如果计算失败则返回错误
#[inline]
pub(super) fn calc_promittor_oa(ra: f64, dec: f64, geo_lat: f64) -> Result<f64, Error> {
    let ad = calc_ad(dec, geo_lat)?;
    Ok(swe_degnorm(ra - ad))
}

/// 斜下降(Oblique Descension)
#[inline]
pub(super) fn calc_promittor_od(ra: f64, dec: f64, geo_lat: f64) -> Result<f64, Error> {
    let ad = calc_ad(dec, geo_lat)?;
    Ok(swe_degnorm(ra + ad))
}

pub(super) fn planet_to_planet_direction(
    horo: &Horoscope,
    significator: &Planet,
    promittor: &Planet,
) -> Result<f64, Error> {
    // 方位角： 从南点向西测量
    let azalt = swe_azalt(
        horo.date.jd_ut1,
        CalcFlag::ECL2HOR,
        &[horo.geo.long, horo.geo.lat, 0.0],
        0.0,
        0.0,
        &[significator.long, significator.lat, 0.0],
    );

    let significator_ad = calc_ad(significator.dec, horo.geo.lat)?;

    // 判断天体的象限
    let (significator_sa, significator_md, t, v, r) =
        if azalt[0] >= 0.0 && azalt[0] < 180.0 && azalt[1] >= 0.0 {
            // 第三限

            //  计算 significator 的白天半弧
            let significator_sa = 90.0 + significator_ad;

            // 计算子午距
            let significator_md = swe_degnorm(horo.mc.ra - significator.ra);

            let t = 1.0;

            let v = 1.0;
            let r = horo.mc.ra;

            (significator_sa, significator_md, t, v, r)
        } else if azalt[0] >= 0.0 && azalt[0] < 180.0 && azalt[1] < 0.0 {
            // 第二限
            // 计算 significator 的夜间半弧
            let significator_sa = 90.0 - significator_ad;

            let significator_md = swe_degnorm(significator.ra - horo.ic.ra);

            let t = -1.0;
            let v = -1.0;
            let r = horo.ic.ra;

            (significator_sa, significator_md, t, v, r)
        } else if azalt[0] >= 180.0 && azalt[1] >= 0.0 {
            // 第四限
            let significator_sa = 90.0 + significator_ad;

            // 计算子午距
            let significator_md = swe_degnorm(significator.ra - horo.mc.ra);

            let t = -1.0;
            let v = 1.0;
            let r = horo.mc.ra;

            (significator_sa, significator_md, t, v, r)
        } else {
            // 第一限
            // 计算 significator 的夜间半弧
            let significator_sa = 90.0 - significator_ad;

            let significator_md = swe_degnorm(horo.ic.ra - significator.ra);

            let t = 1.0;
            let v = -1.0;
            let r = horo.ic.ra;

            (significator_sa, significator_md, t, v, r)
        };

    let promittor_ad = calc_ad(promittor.dec, horo.geo.lat)?;
    // let arc = ra_p - r + t * (90.0 + v * ad_p) * md / sa;
    let arc = promittor.ra - r + t * (90.0 + v * promittor_ad) * significator_md / significator_sa;

    Ok(swe_degnorm(arc))
}

pub(crate) fn promittors_of_planets(horo: &Horoscope) -> Vec<(Promittor, Planet)> {
    let eps = horo.eps;

    // 相位点使用此值，这个值实际不会被用到，仅起占位作用
    let planet_config = PlanetConfig::default_config(&PlanetName::ASC);

    let promittors: Vec<(Promittor, Planet)> = horo
        .planets
        .iter()
        .chain(std::iter::once(&horo.part_of_fortune))
        .flat_map(|planet| {
            let mut promittors = vec![];
            // 计算合相
            promittors.push((Promittor::Conjunction(planet.name), *planet));
            // let conjunction_equator = swe_cotrans(planet.long, 0.0, 1.0, -eps);
            // promittors.push((
            //     planet.name,
            //     conjunction_equator[0],
            //     DirectionType::Conjunction,
            // ));

            // 映点
            let antiscoins_long = swe_degnorm(180.0 - planet.long);
            let equator = swe_cotrans(antiscoins_long, 0.0, 1.0, -eps);
            let p = Planet::new(
                planet.name,
                antiscoins_long,
                0.0,
                0.0,
                equator[0],
                equator[1],
                &planet_config,
            );
            promittors.push((Promittor::Antiscoins(planet.name), p));

            // 南北交点不用计算反映点，也不需要计算基它相位
            if planet.name == PlanetName::NorthNode || planet.name == PlanetName::SouthNode {
                return promittors;
            }

            // 反映点
            let contraantiscias_long = swe_degnorm(180.0 + antiscoins_long);
            let equator = swe_cotrans(contraantiscias_long, 0.0, 1.0, -eps);
            let p = Planet::new(
                planet.name,
                contraantiscias_long,
                0.0,
                0.0,
                equator[0],
                equator[1],
                &planet_config,
            );
            promittors.push((Promittor::Contraantiscias(planet.name), p));

            let aspect_promittors = [-60, 60, -120, 120, -90, 90, 180]
                .into_iter()
                .map(|aspect| {
                    let aspect_long = swe_degnorm(planet.long + aspect as f64);
                    let equator = swe_cotrans(aspect_long, 0.0, 1.0, -eps);
                    let p = Planet::new(
                        planet.name,
                        aspect_long,
                        0.0,
                        0.0,
                        equator[0],
                        equator[1],
                        &planet_config,
                    );

                    let promittor = match aspect {
                        60 => Promittor::SinisterSextile(planet.name),
                        -60 => Promittor::DexterSextile(planet.name),
                        120 => Promittor::SinisterTrine(planet.name),
                        -120 => Promittor::DexterTrine(planet.name),
                        90 => Promittor::SinisterSquare(planet.name),
                        -90 => Promittor::DexterSquare(planet.name),
                        180 => Promittor::Opposition(planet.name),
                        _ => unreachable!(),
                    };

                    (promittor, p)
                });

            promittors.extend(aspect_promittors);

            promittors
        })
        .collect();

    // 托勒密界
    let ptolemy_term: Vec<(Promittor, Planet)> = PTOLEMY_TERM
        .into_iter()
        .map(|PtolemyTerm { planet, long }| {
            let equator = swe_cotrans(f64::from(long), 0.0, 1.0, -eps);
            let ra = equator[0];
            let dec = equator[1];
            let p = Planet::new(
                planet,
                f64::from(long),
                0.0, // 黄纬
                0.0, // 速度
                ra,
                dec,
                &planet_config,
            );
            (Promittor::Term(planet, long), p)
        })
        .collect();

    promittors
        .into_iter()
        .chain(ptolemy_term)
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{HouseName, PlanetConfig, direction::tests::get_ephe_path};
    use geo_position::GeoPosition;
    use horo_date_time::HoroDateTime;

    const EPS: f64 = 1e-10;

    #[test]
    fn test_calc_ad_zero_dec() {
        let result = calc_ad(0.0, 45.0).unwrap();
        assert!(result.abs() < EPS, "赤纬为0时，AD应为0，实际为: {}", result);
    }

    #[test]
    fn test_calc_ad_zero_lat() {
        let result = calc_ad(23.5, 0.0).unwrap();
        assert!(result.abs() < EPS, "纬度为0时，AD应为0，实际为: {}", result);
    }

    #[test]
    fn test_calc_ad_both_zero() {
        let result = calc_ad(0.0, 0.0).unwrap();
        assert!(
            result.abs() < EPS,
            "赤纬和纬度都为0时，AD应为0，实际为: {}",
            result
        );
    }

    #[test]
    fn test_calc_ad_typical_values() {
        let dec = 23.44;
        let lat = 40.0;
        let result = calc_ad(dec, lat).unwrap();

        let dec_rad = dec.to_radians();
        let lat_rad = lat.to_radians();
        let expected = (dec_rad.tan() * lat_rad.tan()).asin().to_degrees();

        assert!(
            (result - expected).abs() < EPS,
            "AD计算错误: 期望 {}, 实际 {}",
            expected,
            result
        );
    }

    #[test]
    fn test_calc_ad_negative_dec() {
        let dec = -23.44;
        let lat = 40.0;
        let result = calc_ad(dec, lat).unwrap();

        assert!(result < 0.0, "负赤纬应产生负AD，实际为: {}", result);
    }

    #[test]
    fn test_calc_ad_negative_lat() {
        let dec = 23.44;
        let lat = -40.0;
        let result = calc_ad(dec, lat).unwrap();

        assert!(result < 0.0, "负纬度应产生负AD，实际为: {}", result);
    }

    #[test]
    fn test_calc_asc_oa_basic() {
        let mc_ra = 180.0;
        let result = calc_asc_oa(mc_ra);
        assert!((result - 270.0).abs() < EPS, "MC赤经180°时，ASC_OA应为270°");
    }

    #[test]
    fn test_calc_asc_oa_zero() {
        let mc_ra = 0.0;
        let result = calc_asc_oa(mc_ra);
        assert!((result - 90.0).abs() < EPS, "MC赤经0°时，ASC_OA应为90°");
    }

    #[test]
    fn test_calc_asc_oa_normalization() {
        let mc_ra = 300.0;
        let result = calc_asc_oa(mc_ra);
        assert!(
            (result - 30.0).abs() < EPS,
            "MC赤经300°时，ASC_OA应为30°（归一化后）"
        );
    }

    #[test]
    fn test_calc_promittor_oa_basic() {
        let ra = 180.0;
        let dec = 0.0;
        let geo_lat = 45.0;
        let result = calc_promittor_oa(ra, dec, geo_lat).unwrap();

        assert!(
            (result - 180.0).abs() < EPS,
            "赤纬为0时，OA应等于RA，实际为: {}",
            result
        );
    }

    #[test]
    fn test_calc_promittor_oa_with_ad() {
        let ra = 0.0;
        let dec = 23.44;
        let geo_lat = 40.0;
        let result = calc_promittor_oa(ra, dec, geo_lat).unwrap();

        let ad = calc_ad(dec, geo_lat).unwrap();
        let expected = swe_degnorm(ra - ad);

        assert!(
            (result - expected).abs() < EPS,
            "OA计算错误: 期望 {}, 实际 {}",
            expected,
            result
        );
    }

    #[test]
    fn test_calc_promittor_oa_normalization() {
        let ra = 10.0;
        let dec = 23.44;
        let geo_lat = 40.0;
        let result = calc_promittor_oa(ra, dec, geo_lat).unwrap();

        assert!(
            result >= 0.0 && result < 360.0,
            "OA应在0-360°范围内，实际为: {}",
            result
        );
    }

    #[test]
    fn test_calc_promittor_oa_negative_dec() {
        let ra = 180.0;
        let dec = -23.44;
        let geo_lat = 40.0;
        let result_pos = calc_promittor_oa(ra, 23.44, geo_lat).unwrap();
        let result_neg = calc_promittor_oa(ra, dec, geo_lat).unwrap();

        assert!(
            (result_pos - result_neg).abs() > 1.0,
            "正负赤纬应产生不同的OA"
        );
    }

    #[test]
    fn test_calc_ad_extreme_lat_returns_error() {
        let dec = 23.44;
        let lat = 89.0;
        let result = calc_ad(dec, lat);
        assert!(result.is_err(), "极地纬度应返回错误");
    }

    #[test]
    fn test_calc_ad_symmetry() {
        let dec = 23.44;
        let lat = 40.0;
        let result1 = calc_ad(dec, lat).unwrap();
        let result2 = calc_ad(lat, dec).unwrap();

        assert!(
            (result1 - result2).abs() < EPS,
            "tan(dec)*tan(lat) 应具有对称性"
        );
    }

    #[test]
    fn test_calc_ad_invalid_returns_error() {
        let result = calc_ad(90.0, 90.0);
        assert!(result.is_err(), "极端值应返回错误");
    }

    fn create_test_horoscope(
        year: i32,
        month: u8,
        day: u8,
        hour: u8,
        minute: u8,
        second: u8,
        tz: f64,
        geo_long: f64,
        geo_lat: f64,
    ) -> Horoscope {
        let ephe_path = get_ephe_path();
        let date = HoroDateTime::new(year, month, day, hour, minute, second, tz).unwrap();
        let geo = GeoPosition::new(geo_long, geo_lat).unwrap();
        let planet_configs = PlanetConfig::default_all_configs();

        Horoscope::new(
            date,
            geo,
            HouseName::Regiomontanus,
            &planet_configs,
            &ephe_path,
        )
        .unwrap()
    }

    #[test]
    fn test_planet_direction_basic() {
        let horo = create_test_horoscope(2000, 1, 1, 12, 0, 0, 8.0, 116.4, 39.9);

        let sun = horo
            .planets
            .iter()
            .find(|p| p.name == PlanetName::Sun)
            .unwrap();
        let moon = horo
            .planets
            .iter()
            .find(|p| p.name == PlanetName::Moon)
            .unwrap();

        let arc = planet_to_planet_direction(&horo, sun, moon).unwrap();

        assert!(arc.is_finite(), "弧度应为有限值，实际为: {}", arc);
        assert!(
            arc >= 0.0 && arc < 360.0,
            "弧度应在0-360°范围内，实际为: {}",
            arc
        );
    }

    #[test]
    fn test_planet_direction_same_planet() {
        let horo = create_test_horoscope(2000, 1, 1, 12, 0, 0, 8.0, 116.4, 39.9);

        let sun = horo
            .planets
            .iter()
            .find(|p| p.name == PlanetName::Sun)
            .unwrap();

        let arc = planet_to_planet_direction(&horo, sun, sun).unwrap();

        assert!(arc.is_finite(), "同一行星的弧度应为有限值，实际为: {}", arc);
    }

    #[test]
    fn test_planet_direction_quadrant_day_above_horizon() {
        let horo = create_test_horoscope(2021, 6, 21, 12, 0, 0, 8.0, 116.4, 39.9);

        let sun = horo
            .planets
            .iter()
            .find(|p| p.name == PlanetName::Sun)
            .unwrap();

        let jupiter = horo
            .planets
            .iter()
            .find(|p| p.name == PlanetName::Jupiter)
            .unwrap();

        let arc = planet_to_planet_direction(&horo, sun, jupiter).unwrap();

        assert!(arc.is_finite(), "夏至日正午太阳在地平线上的弧度应为有限值");
    }

    #[test]
    fn test_planet_direction_quadrant_night_below_horizon() {
        let horo = create_test_horoscope(2021, 6, 21, 0, 0, 0, 8.0, 116.4, 39.9);

        let sun = horo
            .planets
            .iter()
            .find(|p| p.name == PlanetName::Sun)
            .unwrap();

        let moon = horo
            .planets
            .iter()
            .find(|p| p.name == PlanetName::Moon)
            .unwrap();

        let arc = planet_to_planet_direction(&horo, sun, moon).unwrap();

        assert!(arc.is_finite(), "午夜太阳在地平线下的弧度应为有限值");
    }

    #[test]
    fn test_planet_direction_different_latitudes() {
        let horo_equator = create_test_horoscope(2000, 1, 1, 12, 0, 0, 8.0, 116.4, 0.0);
        let horo_mid_lat = create_test_horoscope(2000, 1, 1, 12, 0, 0, 8.0, 116.4, 45.0);
        let horo_high_lat = create_test_horoscope(2000, 1, 1, 12, 0, 0, 8.0, 116.4, 60.0);

        let sun_eq = horo_equator
            .planets
            .iter()
            .find(|p| p.name == PlanetName::Sun)
            .unwrap();
        let moon_eq = horo_equator
            .planets
            .iter()
            .find(|p| p.name == PlanetName::Moon)
            .unwrap();

        let sun_mid = horo_mid_lat
            .planets
            .iter()
            .find(|p| p.name == PlanetName::Sun)
            .unwrap();
        let moon_mid = horo_mid_lat
            .planets
            .iter()
            .find(|p| p.name == PlanetName::Moon)
            .unwrap();

        let sun_high = horo_high_lat
            .planets
            .iter()
            .find(|p| p.name == PlanetName::Sun)
            .unwrap();
        let moon_high = horo_high_lat
            .planets
            .iter()
            .find(|p| p.name == PlanetName::Moon)
            .unwrap();

        let arc_eq = planet_to_planet_direction(&horo_equator, sun_eq, moon_eq).unwrap();
        let arc_mid = planet_to_planet_direction(&horo_mid_lat, sun_mid, moon_mid).unwrap();
        let arc_high = planet_to_planet_direction(&horo_high_lat, sun_high, moon_high).unwrap();

        assert!(
            arc_eq.is_finite() && arc_mid.is_finite() && arc_high.is_finite(),
            "不同纬度的弧度应为有限值"
        );
    }

    #[test]
    fn test_planet_direction_consistency() {
        let horo = create_test_horoscope(2000, 1, 1, 12, 0, 0, 8.0, 116.4, 39.9);

        let sun = horo
            .planets
            .iter()
            .find(|p| p.name == PlanetName::Sun)
            .unwrap();
        let moon = horo
            .planets
            .iter()
            .find(|p| p.name == PlanetName::Moon)
            .unwrap();

        let arc1 = planet_to_planet_direction(&horo, sun, moon).unwrap();
        let arc2 = planet_to_planet_direction(&horo, sun, moon).unwrap();

        assert!((arc1 - arc2).abs() < EPS, "相同输入应产生相同输出");
    }

    #[test]
    fn test_promittors_of_planets_basic() {
        let horo = create_test_horoscope(2000, 1, 1, 12, 0, 0, 8.0, 116.4, 39.9);

        let promittors = promittors_of_planets(&horo);

        assert!(!promittors.is_empty(), "应生成promittor列表");
    }

    #[test]
    fn test_promittors_of_planets_contains_conjunctions() {
        let horo = create_test_horoscope(2000, 1, 1, 12, 0, 0, 8.0, 116.4, 39.9);

        let promittors = promittors_of_planets(&horo);

        let conjunction_count = promittors
            .iter()
            .filter(|(p, _)| matches!(p, Promittor::Conjunction(_)))
            .count();

        assert!(conjunction_count > 0, "应包含合相类型的promittor");
    }

    #[test]
    fn test_promittors_of_planets_contains_aspects() {
        let horo = create_test_horoscope(2000, 1, 1, 12, 0, 0, 8.0, 116.4, 39.9);

        let promittors = promittors_of_planets(&horo);

        let has_sinister_trine = promittors
            .iter()
            .any(|(p, _)| matches!(p, Promittor::SinisterTrine(_)));
        let has_dexter_trine = promittors
            .iter()
            .any(|(p, _)| matches!(p, Promittor::DexterTrine(_)));

        let has_sinister_square = promittors
            .iter()
            .any(|(p, _)| matches!(p, Promittor::SinisterSquare(_)));
        let has_dexter_square = promittors
            .iter()
            .any(|(p, _)| matches!(p, Promittor::DexterSquare(_)));

        let has_sinister_sextile = promittors
            .iter()
            .any(|(p, _)| matches!(p, Promittor::SinisterSextile(_)));
        let has_dexter_sextile = promittors
            .iter()
            .any(|(p, _)| matches!(p, Promittor::DexterSextile(_)));

        let has_opposition = promittors
            .iter()
            .any(|(p, _)| matches!(p, Promittor::Opposition(_)));

        assert!(has_sinister_trine, "应包含左三合相");
        assert!(has_dexter_trine, "应包含右三合相");
        assert!(has_sinister_square, "应包含左刑相");
        assert!(has_dexter_square, "应包含右刑相");
        assert!(has_sinister_sextile, "应包含左六合相");
        assert!(has_dexter_sextile, "应包含右六合相");
        assert!(has_opposition, "应包含冲相");
    }

    #[test]
    fn test_promittors_of_planets_contains_antiscoins() {
        let horo = create_test_horoscope(2000, 1, 1, 12, 0, 0, 8.0, 116.4, 39.9);

        let promittors = promittors_of_planets(&horo);

        let has_antiscoins = promittors
            .iter()
            .any(|(p, _)| matches!(p, Promittor::Antiscoins(_)));
        let has_contraantiscias = promittors
            .iter()
            .any(|(p, _)| matches!(p, Promittor::Contraantiscias(_)));

        assert!(has_antiscoins, "应包含映点");
        assert!(has_contraantiscias, "应包含反映点");
    }

    #[test]
    fn test_promittors_of_planets_contains_terms() {
        let horo = create_test_horoscope(2000, 1, 1, 12, 0, 0, 8.0, 116.4, 39.9);

        let promittors = promittors_of_planets(&horo);

        let has_term = promittors
            .iter()
            .any(|(p, _)| matches!(p, Promittor::Term(_, _)));

        assert!(has_term, "应包含托勒密界");
    }

    #[test]
    fn test_promittors_of_planets_term_count() {
        let horo = create_test_horoscope(2000, 1, 1, 12, 0, 0, 8.0, 116.4, 39.9);

        let promittors = promittors_of_planets(&horo);

        let term_count = promittors
            .iter()
            .filter(|(p, _)| matches!(p, Promittor::Term(_, _)))
            .count();

        assert_eq!(
            term_count,
            PTOLEMY_TERM.len(),
            "托勒密界数量应为 {}",
            PTOLEMY_TERM.len()
        );
    }

    #[test]
    fn test_promittors_of_planets_planet_valid() {
        let horo = create_test_horoscope(2000, 1, 1, 12, 0, 0, 8.0, 116.4, 39.9);

        let promittors = promittors_of_planets(&horo);

        for (_, planet) in &promittors {
            assert!(
                planet.long >= 0.0 && planet.long < 360.0,
                "行星黄经应在0-360°范围内"
            );
            assert!(
                planet.lat >= -90.0 && planet.lat <= 90.0,
                "行星黄纬应在-90°到90°范围内"
            );
            assert!(
                planet.ra >= 0.0 && planet.ra < 360.0,
                "行星赤经应在0-360°范围内"
            );
            assert!(
                planet.dec >= -90.0 && planet.dec <= 90.0,
                "行星赤纬应在-90°到90°范围内"
            );
        }
    }

    #[test]
    fn test_promittors_of_planets_node_no_contraantiscias() {
        let horo = create_test_horoscope(2000, 1, 1, 12, 0, 0, 8.0, 116.4, 39.9);

        let promittors = promittors_of_planets(&horo);

        let node_contraantiscias: Vec<_> = promittors
            .iter()
            .filter(|(p, _)| {
                matches!(
                    p,
                    Promittor::Contraantiscias(PlanetName::NorthNode | PlanetName::SouthNode)
                )
            })
            .collect();

        assert!(node_contraantiscias.is_empty(), "南北交点不应有反映点");
    }

    #[test]
    fn test_promittors_of_planets_expected_count() {
        let horo = create_test_horoscope(2000, 1, 1, 12, 0, 0, 8.0, 116.4, 39.9);

        let promittors = promittors_of_planets(&horo);

        let planet_count = horo.planets.len();

        let conjunction_count = planet_count;
        let antiscoins_count = planet_count;
        let contraantiscias_count = planet_count - 2;
        let aspect_count = (planet_count - 2) * 7;
        let term_count = PTOLEMY_TERM.len();

        let expected_count = conjunction_count
            + antiscoins_count
            + contraantiscias_count
            + aspect_count
            + term_count;

        assert_eq!(
            promittors.len(),
            expected_count,
            "promittor总数应为 {}，实际为 {}",
            expected_count,
            promittors.len()
        );
    }
}
