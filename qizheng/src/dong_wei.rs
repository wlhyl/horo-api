use crate::{DistanceStarLong, Error, LunarMansionsName, lunar_mansions::calc_xiu_degree};

use horo_date_time::{HoroDateTime, horo_date_time};
#[cfg(feature = "serde")]
use serde::Serialize;

use swe::swe_degnorm;
#[cfg(feature = "swagger")]
use utoipa::ToSchema;

/// 二十八宿洞微大限时间
#[cfg_attr(feature = "serde", derive(Serialize))]
#[cfg_attr(feature = "swagger", derive(ToSchema))]
pub struct LunarMansionsDongWeiTime {
    /// 宿名
    lunar_mansions: LunarMansionsName,
    /// 洞微大限时间
    time: HoroDateTime,
}

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
    /// 每个二十八宿距星的洞微大限时间
    lunar_mansions_dong_wei_time: Vec<Option<LunarMansionsDongWeiTime>>,
}
impl DongWei {
    pub fn new(
        long_of_per_year: Vec<f64>,
        long: f64,
        xiu: LunarMansionsName,
        xiu_degree: f64,
        lunar_mansions_dong_wei_time: Vec<Option<LunarMansionsDongWeiTime>>,
    ) -> Self {
        Self {
            long_of_per_year,
            long,
            xiu,
            xiu_degree,
            lunar_mansions_dong_wei_time,
        }
    }
}

/// 计算在特定日期的洞微大限经度
fn calc_dong_wei_long_at_date(
    date: &HoroDateTime,
    first_house_long: f64,
    date_of_per_house_for_dong_wei: &[HoroDateTime],
) -> Result<f64, Error> {
    // 注意： date_of_per_house_for_dong_wei实际有13个元素
    // 最后一个元素是第2宫头的时间
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

/// 计算给定黄道经度的洞微大限时间
///
/// 该函数根据给定的黄道经度，计算出对应的洞微大限时间点。
///
/// # 参数
///
/// - `long`: 给定的黄道经度
/// - `first_house_long`: 第一宫的起始经度
/// - `date_of_per_house_for_dong_wei`: 各宫位起运时间序列
///
/// # 返回值
///
/// 返回对应的儒略日时间，如果给定经度不在洞微大限范围内，则返回错误。
fn calc_date_from_dong_wei_long(
    long: f64,
    first_house_long: f64,
    date_of_per_house_for_dong_wei: &[HoroDateTime],
) -> Result<f64, Error> {
    // 注意： date_of_per_house_for_dong_wei实际有13个元素
    // 最后一个元素是第2宫头的时间

    // 遍历每个宫位的时间区间，找到给定经度对应的时间区间
    for (index, window) in date_of_per_house_for_dong_wei.windows(2).enumerate() {
        let start_date = &window[0];
        let next_start_date = &window[1];

        // 计算当前区间起始的经度
        let start_long = swe_degnorm(first_house_long + 30.0 - (index * 30) as f64);

        // 计算目标经度与起始经度的差值（逆行方向）
        let diff = swe_degnorm(start_long - long);

        // 洞微大限每个宫位区间为30度。如果差值在30度内，说明目标经度在此区间。
        // diff < 30.0 对应于时间上的 [start_date, next_start_date)
        // 当 long 等于区间的结束经度时，diff会等于30.0，此时应归入下一个区间计算
        if diff < 30.0 {
            // 使用线性插值计算对应的儒略日
            let ratio = diff / 30.0;
            let jd = start_date.jd_utc + ratio * (next_start_date.jd_utc - start_date.jd_utc);
            return Ok(jd);
        }
    }

    Err(Error::InvalidProcessDateTime(
        "给定经度超出洞微大限的计算范围".to_string(),
    ))
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

        // 算法：如4.1年，0.1*回归年长度=0.1对应的传略日数
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

    // 计算每个二十八宿距星的洞微大限时间
    let lunar_mansions_dong_wei_time: Result<Vec<_>, Error> = distance_star_long
        .iter()
        .enumerate()
        .map(|(index, star)| {
            let jd_utc = calc_date_from_dong_wei_long(
                star.long,
                first_house_long,
                &date_of_per_house_for_dong_wei,
            )
            .ok();
            if jd_utc.is_none() {
                return Ok(None);
            }
            let time = HoroDateTime::from_jd_zone(jd_utc.unwrap(), native_date.tz)?;
            Ok(Some(LunarMansionsDongWeiTime {
                lunar_mansions: distance_star_long
                    [(distance_star_long.len() - 1 + index) % distance_star_long.len()]
                .lunar_mansions,
                time,
            }))
        })
        .collect();

    let mut lunar_mansions_dong_wei_time = lunar_mansions_dong_wei_time?;

    // 对 lunar_mansions_dong_wei_time 按时间排序
    // Some 在前，None 在后，Some 内部按时间升序
    lunar_mansions_dong_wei_time.sort_by(|a, b| match (a, b) {
        (Some(a_val), Some(b_val)) => a_val
            .time
            .jd_utc
            .partial_cmp(&b_val.time.jd_utc)
            .unwrap_or(std::cmp::Ordering::Equal),
        (Some(_), None) => std::cmp::Ordering::Less,
        (None, Some(_)) => std::cmp::Ordering::Greater,
        (None, None) => std::cmp::Ordering::Equal,
    });

    let dong_wei = DongWei::new(
        dong_wei_long_per_year,
        process_dong_wei_long,
        process_dong_wei_xiu,
        process_dong_wei_xiu_degree,
        lunar_mansions_dong_wei_time,
    );

    Ok(dong_wei)
}

#[cfg(test)]
mod tests {
    use crate::{
        DistanceStarConfig, Error, PlanetConfig, PlanetName,
        dong_wei::{calc_date_from_dong_wei_long, calc_dong_wei},
        lunar_mansions::calc_distance_star_long,
        planet::calc_planets,
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

    #[test]
    fn test_calc_date_from_dong_wei_long() {
        let first_house_long = 210.0;
        // 伪造一个时间序列
        // 1983-10-27: 240 -> 210 (15 years)
        // 1998-10-27: 210 -> 180 (10 years)
        // 2008-10-27: 180 -> 150 (11 years)
        // 2019-10-27
        let date_of_per_house_for_dong_wei = vec![
            horo_date_time(1983, 10, 27, 18, 30, 0, 8.0, false).unwrap(),
            horo_date_time(1998, 10, 27, 18, 30, 0, 8.0, false).unwrap(),
            horo_date_time(2008, 10, 27, 18, 30, 0, 8.0, false).unwrap(),
            horo_date_time(2019, 10, 27, 18, 30, 0, 8.0, false).unwrap(),
        ];

        // 1. 成功案例: 在第二个区间 (210 -> 180) 的中间
        let long = 195.0;
        let jd =
            calc_date_from_dong_wei_long(long, first_house_long, &date_of_per_house_for_dong_wei)
                .unwrap();
        // 1998 + 10/2 = 2003
        let expected_date = horo_date_time(2003, 10, 27, 18, 30, 0, 8.0, false).unwrap();
        assert!(
            (jd - expected_date.jd_utc).abs() < 1.0,
            "jd={jd}, expected_jd={}",
            expected_date.jd_utc
        );

        // 2. 边界案例: long 等于区间的开始
        let long = 210.0;
        let jd =
            calc_date_from_dong_wei_long(long, first_house_long, &date_of_per_house_for_dong_wei)
                .unwrap();
        let expected_date = horo_date_time(1998, 10, 27, 18, 30, 0, 8.0, false).unwrap();
        assert!((jd - expected_date.jd_utc).abs() < 1e-9);

        // 3. 边界案例: long 等于整个推运的终点
        let long = 150.0;
        let result =
            calc_date_from_dong_wei_long(long, first_house_long, &date_of_per_house_for_dong_wei);
        assert!(matches!(result, Err(Error::InvalidProcessDateTime(_))));

        // 4. 错误案例: 超出范围 (小于终点)
        let long = 149.0;
        let result =
            calc_date_from_dong_wei_long(long, first_house_long, &date_of_per_house_for_dong_wei);
        assert!(matches!(result, Err(Error::InvalidProcessDateTime(_))));

        // 5. 错误案例: 超出范围 (大于起点)
        let long = 241.0;
        let result =
            calc_date_from_dong_wei_long(long, first_house_long, &date_of_per_house_for_dong_wei);
        // 第一个区间从240.0开始，所以241超出了范围。
        // 但由于逻辑`swe_degnorm(start_long - long)`会进行角度归一化，
        // `swe_degnorm(240 - 241)`的结果是359，大于30，因此会被判定为超出范围。
        // 这个判断是正确的。
        assert!(matches!(result, Err(Error::InvalidProcessDateTime(_))));
    }
}
