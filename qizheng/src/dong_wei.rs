use crate::{DistanceStarLong, Error, LunarMansionsName, lunar_mansions::calc_xiu_degree};

use horo_date_time::{HoroDateTime, horo_date_time};
#[cfg(feature = "serde")]
use serde::Serialize;

use swe::swe_degnorm;
#[cfg(feature = "swagger")]
use utoipa::ToSchema;

#[cfg_attr(feature = "serde", derive(Serialize))]
#[cfg_attr(feature = "swagger", derive(ToSchema))]
/// 洞微大限
pub struct DongWei {
    /// 洞微大限每一年的黄道经度，从0岁起至洞微大限总年数，洞微大限总年数略去小数部分，起算点为每年的公历生日
    long_of_per_year: Vec<f64>,
    /// 当前推运时间的洞微大限黄道经度
    long: f64,
    /// 当前推运时间的洞微大限黄道经度所在宿名
    xiu: LunarMansionsName,
    /// 当前推运时间的洞微大限道经度的入宿度数
    xiu_degree: f64,
}

impl DongWei {
    pub fn new(
        long_of_per_year: Vec<f64>,
        long: f64,
        xiu: LunarMansionsName,
        xiu_degree: f64,
    ) -> Self {
        Self {
            long_of_per_year,
            long,
            xiu,
            xiu_degree,
        }
    }
}

/// 计算在特定日期的洞微大限经度
fn calc_dong_wei_long_at_date(
    date: &HoroDateTime,
    first_house_long: f64,
    date_of_per_house_for_dong_wei: &[HoroDateTime],
) -> Result<f64, Error> {
    date_of_per_house_for_dong_wei
        .windows(2)
        .enumerate()
        .find_map(|(index, window)| {
            let start_date = &window[0];
            let next_start_date = &window[1];

            if date.jd_utc < next_start_date.jd_utc {
                let long = swe_degnorm(
                    first_house_long + 30.0
                        - (index * 30) as f64
                        - 30.0 * (date.jd_utc - start_date.jd_utc)
                            / (next_start_date.jd_utc - start_date.jd_utc),
                );
                Some(Ok(long))
            } else {
                None
            }
        })
        .unwrap_or(Err(Error::InvalidProcessDateTime(
            "推运时间超出洞微大限的计算范围".to_string(),
        )))
}

/// 计算洞微大限
pub(crate) fn calc_dong_wei(
    ming_du_long: f64,
    first_house_long: f64,
    native_date: &HoroDateTime,
    process_date: &HoroDateTime,
    distance_star_long: &[DistanceStarLong],
) -> Result<DongWei, Error> {
    // 各宫位的洞微所管年数
    let mut ages_of_per_house_for_dong_wei = [
        15.0, 10.0, 11.0, 15.0, 8.0, 7.0, 11.0, 4.5, 4.5, 4.5, 5.0, 5.0,
    ];

    ages_of_per_house_for_dong_wei[0] = (ming_du_long - first_house_long) / 3.0 + 10.0;

    // 计算给定时间洞微大限所在的黄道经度

    let mut date_of_per_house_for_dong_wei = vec![*native_date];
    for age in ages_of_per_house_for_dong_wei {
        // 年龄数的整数部分
        let age_int_part = age.floor();
        // 非整数年，如:4.5年，(下一年的jd_utc-本年的jd_utc)*年龄的小数部分，得到此限开始的jd_utc

        let start_date_of_current_house = date_of_per_house_for_dong_wei.last().unwrap();
        // 计算一下宫位的起运时间
        // 由于可能出现2月29日，因此使用horo_date_time函数
        let current_start_date = horo_date_time(
            start_date_of_current_house.year + (age_int_part as i32),
            start_date_of_current_house.month,
            start_date_of_current_house.day,
            start_date_of_current_house.hour,
            start_date_of_current_house.minute,
            start_date_of_current_house.second,
            start_date_of_current_house.tz,
            false,
        )?;

        let next_start_date = horo_date_time(
            start_date_of_current_house.year + (age_int_part as i32) + 1,
            start_date_of_current_house.month,
            start_date_of_current_house.day,
            start_date_of_current_house.hour,
            start_date_of_current_house.minute,
            start_date_of_current_house.second,
            start_date_of_current_house.tz,
            false,
        )?;

        let start_jd_utc = current_start_date.jd_utc
            + (age - age_int_part) * (next_start_date.jd_utc - current_start_date.jd_utc);

        // 计算下一个宫位的起运时间
        let start_date_of_next_house =
            HoroDateTime::from_jd_zone(start_jd_utc, current_start_date.tz)?;

        date_of_per_house_for_dong_wei.push(start_date_of_next_house);
    }

    // 计算每一年的洞微大限
    let dong_wei_long_per_year: Result<Vec<_>, Error> = (native_date.year
        ..date_of_per_house_for_dong_wei.last().unwrap().year)
        .into_iter()
        .map(|year| {
            let date = horo_date_time(
                year,
                native_date.month,
                native_date.day,
                native_date.hour,
                native_date.minute,
                native_date.second,
                native_date.tz,
                false,
            )?;

            calc_dong_wei_long_at_date(&date, first_house_long, &date_of_per_house_for_dong_wei)
        })
        .collect();

    let dong_wei_long_per_year = dong_wei_long_per_year?;

    // 推运时刻的洞微大限
    let process_dong_wei_long = calc_dong_wei_long_at_date(
        &process_date,
        first_house_long,
        &date_of_per_house_for_dong_wei,
    )?;

    let (process_dong_wei_xiu, process_dong_wei_xiu_degree) =
        calc_xiu_degree(process_dong_wei_long, distance_star_long)?;

    let dong_wei = DongWei::new(
        dong_wei_long_per_year,
        process_dong_wei_long,
        process_dong_wei_xiu,
        process_dong_wei_xiu_degree,
    );

    Ok(dong_wei)
}

#[cfg(test)]
mod tests {
    use crate::{
        DistanceStarConfig, PlanetConfig, PlanetName, dong_wei::calc_dong_wei,
        lunar_mansions::calc_distance_star_long, planet::calc_planets,
    };
    use geo_position::GeoPosition;
    use horo_date_time::horo_date_time;
    use std::env;
    use swe::{HouseSystem, swe_houses};

    #[test]
    fn test_calc_dong_wei() {
        dotenvy::dotenv().ok();
        let ephe_path = env::var("EPHE_PATH").unwrap();
        let native_date = horo_date_time(1983, 10, 27, 18, 30, 0, 8.0, false).unwrap();
        let process_date = horo_date_time(2023, 10, 27, 18, 30, 0, 8.0, false).unwrap();
        let geo = GeoPosition::new(116.383333, 39.9).unwrap();
        let planets_config = PlanetConfig::default_all_configs();
        let distance_star_config = DistanceStarConfig::default_all_configs();

        let distance_star_long =
            calc_distance_star_long(native_date.jd_utc, &distance_star_config, &ephe_path).unwrap();

        let native_planets = calc_planets(
            native_date.jd_utc,
            &distance_star_long,
            &planets_config,
            &ephe_path,
        )
        .unwrap();

        let (_, ascmc) =
            swe_houses(native_date.jd_ut1, geo.lat, geo.long, &HouseSystem::B).unwrap();

        let asc_long = ascmc[0];
        let first_house_long = (asc_long / 30.0).floor() * 30.0;

        let sun_long = native_planets
            .iter()
            .find_map(|p| {
                if p.name == PlanetName::日 {
                    Some(p.long)
                } else {
                    None
                }
            })
            .unwrap();

        let ming_du_long = sun_long - (sun_long / 30.0).floor() * 30.0 + first_house_long;

        let dong_wei = calc_dong_wei(
            ming_du_long,
            first_house_long,
            &native_date,
            &process_date,
            &distance_star_long,
        )
        .unwrap();

        insta::assert_yaml_snapshot!(dong_wei);
    }

    #[test]
    fn test_calc_dong_wei_long_at_date() {
        use super::{Error, calc_dong_wei_long_at_date};

        let first_house_long = 210.0;
        // 为了简化，我们伪造一个时间序列
        let date_of_per_house_for_dong_wei = vec![
            horo_date_time(1983, 10, 27, 18, 30, 0, 8.0, false).unwrap(), // 240度
            horo_date_time(1998, 10, 27, 18, 30, 0, 8.0, false).unwrap(), // 15年後,210度
            horo_date_time(2008, 10, 27, 18, 30, 0, 8.0, false).unwrap(), // 10年後, 180度
        ];

        // 1. 成功案例: 在第二個區間的中間
        let process_date = horo_date_time(2003, 10, 27, 18, 30, 0, 8.0, false).unwrap();
        let long = calc_dong_wei_long_at_date(
            &process_date,
            first_house_long,
            &date_of_per_house_for_dong_wei,
        )
        .unwrap();
        // 1998年是210度，2008年是180度。2003年是中間，所以是195度
        // 实际计算值是195度14秒多，这是由于用30度等分jd日数造成的
        assert!((long - 195.0).abs() * 60.0 < 1.0, "long={long}");

        // 2. 邊界案例: date 等于 start_date of the second window
        let process_date = horo_date_time(1998, 10, 27, 18, 30, 0, 8.0, false).unwrap();
        let long = calc_dong_wei_long_at_date(
            &process_date,
            first_house_long,
            &date_of_per_house_for_dong_wei,
        )
        .unwrap();
        // 應該是第二個區間的開始，也就是180度
        assert!((long - 210.0).abs() < 1e-9);

        // 3. 錯誤案例: 超出範圍
        let process_date = horo_date_time(2023, 10, 27, 18, 30, 0, 8.0, false).unwrap();
        let result = calc_dong_wei_long_at_date(
            &process_date,
            first_house_long,
            &date_of_per_house_for_dong_wei,
        );
        assert!(matches!(result, Err(Error::InvalidProcessDateTime(_))));
    }
}
