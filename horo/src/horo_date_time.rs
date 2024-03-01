use swe::{
    swe_date_conversion, swe_julday, swe_revjul, swe_utc_time_zone, swe_utc_to_jd, Calendar,
};

use crate::error::{DateTimeError, Error};

#[cfg(feature = "serde")]
use serde::Serialize;

#[cfg(feature = "swagger")]
use utoipa::ToSchema;

/// 无公无0年，公元前1年的下一年为公元1年
/// 公元1582年10月15日00:00:00为格里高利历，儒略日=2299160.5
/// 之前为儒略历
/// 闰秒日期时间，处理为下一个整点
#[derive(Clone, Debug, Copy)]
#[cfg_attr(feature = "serde", derive(Serialize))]
#[cfg_attr(feature = "swagger", derive(ToSchema))]
pub struct HoroDateTime {
    /// 年
    pub year: i32,
    /// 月
    pub month: u8,
    /// 日
    pub day: u8,
    /// 时
    pub hour: u8,
    /// 分
    pub minute: u8,
    /// 秒
    pub second: u8,
    /// 毫秒
    /// 1秒 = 1000毫秒
    /// 用于存储jd转换日期后秒的小数部分
    #[cfg_attr(feature = "serde", serde(skip_serializing))]
    pub ms: f64,
    /// 时区
    /// 东为正，西为负
    pub tz: f64,
    /// UTC时的儒略日
    #[cfg_attr(feature = "serde", serde(skip_serializing))]
    pub jd_utc: f64,
    /// ET时的儒略日
    #[cfg_attr(feature = "serde", serde(skip_serializing))]
    pub jd_et: f64,
    /// UT1时的儒略日
    #[cfg_attr(feature = "serde", serde(skip_serializing))]
    pub jd_ut1: f64,
}

impl HoroDateTime {
    pub fn from_jd_zone(jd: f64, time_zone: f64) -> Result<Self, Error> {
        if jd < 0.0 {
            return Err(DateTimeError::InvalidDateTime(format!("jd={}超出支持范围", jd)).into());
        }
        if time_zone < -12.0 || time_zone > 12.0 {
            return Err(DateTimeError::InvalidZone(format!(
                "{}, There is no such time zone.",
                time_zone
            ))
            .into());
        }

        let t_utc = swe_revjul(
            jd,
            if jd < 2299160.5 {
                Calendar::Julian
            } else {
                Calendar::Gregorian
            },
        );

        let y = t_utc.0;
        let m = t_utc.1;
        let d = t_utc.2;
        let h = t_utc.3.floor() as i32;
        let mi = ((t_utc.3 - f64::from(h)) * 60.0) as i32;
        let s = ((t_utc.3 - f64::from(h)) * 60.0 - f64::from(mi)) * 60.0;

        // 计算jd_ut1
        let jd_et_ut1 = match swe_utc_to_jd(
            y,
            m,
            d,
            h,
            mi,
            s,
            if jd < 2299160.5 {
                Calendar::Julian
            } else {
                Calendar::Gregorian
            },
        ) {
            Ok(v) => v,
            Err(e) => return Err(Error::Function(format!("swe_utc_to_jd()调用失败:{}", e))),
        };

        let t_local = swe_utc_time_zone(y, m, d, h, mi, s, -time_zone);

        let year = if t_local.0 <= 0 {
            t_local.0 - 1
        } else {
            t_local.0
        };
        let month = t_local.1 as u8;
        let day = t_local.2 as u8;
        let hour = t_local.3 as u8;
        let minute = t_local.4 as u8;
        let second = t_local.5 as u8;
        let ms = (t_local.5 - f64::from(second)) * 1000.0;

        // 2020年12月21日，5：29：59，引起60秒异常
        // 四舍五入后，可能会得到60秒，舍弃多出的1秒

        Ok(Self {
            year,
            month,
            day,
            hour,
            minute,
            second,
            ms,
            jd_utc: jd,
            tz: time_zone,
            jd_et: jd_et_ut1[0],
            jd_ut1: jd_et_ut1[1],
        })
    }

    pub fn new(
        year: i32,
        month: u8,
        day: u8,
        hour: u8,
        minute: u8,
        second: u8,
        time_zone: f64,
    ) -> Result<Self, Error> {
        if time_zone < -12.0 || time_zone > 12.0 {
            let msg = format!("{},There is no such time zone.", time_zone);
            return Err(DateTimeError::InvalidZone(msg).into());
        }
        if year == 1582 && month == 10 && day > 4 && day < 15 {
            let msg = format!("{year}-{month}-{day} ${hour}:{minute}:{second} 没有此日期");
            return Err(DateTimeError::InvalidDateTime(msg).into());
        }

        if hour > 23 || minute > 59 || second > 60 {
            let msg = format!("{year}-{month}-{day} {hour}:{minute}:{second} 没有此日期");
            return Err(DateTimeError::InvalidDateTime(msg).into());
        }

        if second == 60 && (!is_leap_seconds(year, month, day, hour, minute, second)) {
            let msg = format!(
                "{}-{}-{} {}:{}:{} 没有此日期",
                year, month, day, hour, minute, second
            );
            return Err(DateTimeError::InvalidDateTime(msg).into());
        }

        // 计算儒略日，并判断时间是否合法
        // 1582年10月15日00:00:00起为格里高利历
        let calendar = if year < 1582 {
            Calendar::Julian
        } else if year == 1582 && month < 10 {
            Calendar::Julian
        } else if year == 1582 && month == 10 && day < 15 {
            Calendar::Julian
        } else {
            Calendar::Gregorian
        };

        //这一步仅用作判断时间的正确性
        //假定此时间是格林尼治时间
        // swe_date_conversio通过swe_julday计算jd，
        // 再以swe_revjul反算，作比较，判断时间是否正确
        // 但swe_julday与swe_revjul会将闰秒视作下一秒，
        //即2016-12-31 23:59:60视作2017-1-1 00:00:00
        // 这两个函数使用的是日期与jd的转换公式，不能处理闰秒
        let dhour = if second == 60 {
            f64::from(hour) + f64::from(minute) / 60.0 + f64::from(second - 1) / 3600.0
        } else {
            f64::from(hour) + f64::from(minute) / 60.0 + f64::from(second) / 3600.0
        };
        // swe_date_conversion 不合法的时间，返回null，正确的时间，返回儒略日
        if swe_date_conversion(
            if year < 0 { year + 1 } else { year },
            month.into(),
            day.into(),
            dhour,
            calendar.clone(),
        )
        .is_err()
        {
            let msg = format!("{year}-{month}-{day} {hour}:{minute}:{second} 没有此日期");

            return Err(DateTimeError::InvalidDateTime(msg).into());
        }
        let t_utc = swe_utc_time_zone(
            if year < 0 { year + 1 } else { year },
            month.into(),
            day.into(),
            hour.into(),
            minute.into(),
            second.into(),
            time_zone,
        );
        let jd_utc = swe_julday(
            t_utc.0,
            t_utc.1,
            t_utc.2,
            f64::from(t_utc.3) + f64::from(t_utc.4) / 60.0 + t_utc.5 / 3600.0,
            calendar.clone(),
        );
        let jd_et_ut1 = swe_utc_to_jd(
            t_utc.0, t_utc.1, t_utc.2, t_utc.3, t_utc.4, t_utc.5, calendar,
        );

        let jd_et_ut1 = match jd_et_ut1 {
            Ok(v) => v,
            Err(e) => {
                let msg = format!("swe_utc_to_jd()错误。{}", e);
                return Err(Error::Function(msg));
            }
        };
        Ok(Self {
            year,
            month,
            day,
            hour,
            minute,
            second,
            ms: 0.0,
            tz: time_zone,
            jd_utc,
            jd_et: jd_et_ut1[0],
            jd_ut1: jd_et_ut1[1],
        })
    }

    /// 将时间+day天
    pub fn plus_days(&self, days: f64) -> Result<HoroDateTime, Error> {
        HoroDateTime::from_jd_zone(self.jd_utc + days, self.tz)
    }
}

/*
* 闰秒实施的月份
* 6月30日23:59:60
* 12月31日23:59:60
 */
fn is_leap_seconds(year: i32, month: u8, day: u8, hour: u8, minute: u8, second: u8) -> bool {
    let leap_seconds = [
        19720630, 19721231, 19731231, 19741231, 19751231, 19761231, 19771231, 19781231, 19791231,
        19810630, 19820630, 19830630, 19850630, 19871231, 19891231, 19901231, 19920630, 19930630,
        19940630, 19951231, 19970630, 19981231, 20051231, 20081231, 20120630, 20150630, 20161231,
    ];
    if hour != 23 || minute != 59 || second != 60 {
        return false;
    }
    let t = year * 10000 + i32::from(month) * 100 + i32::from(day);
    leap_seconds.contains(&t)
}

// 此函数将平年2月29日视作3月1日，
// st为夏令时，将夏令时转换成非夏令时
// @param year
// 年
// @param month
// 月
// @param day
// 日
// @param hour
// 时
// @param minute
// 分
// @param second
// 秒
// @param timeZone
// 时区
// @param st
// 夏令时
pub fn horo_date_time(
    year: i32,
    month: u8,
    day: u8,
    hour: u8,
    minute: u8,
    second: u8,
    time_zone: f64,
    st: bool,
) -> Result<HoroDateTime, Error> {
    let mut y = year;
    let mut m = month;
    let mut d = day;
    let mut h = hour;
    let mi = minute;
    let sec = second;
    let tz = time_zone;
    // 以下代码可以将平年2月29日转换成3月1日
    if month == 2 && day == 29 {
        let t = HoroDateTime::new(year, month, 28, 0, 0, 0, 0.0)?;
        let t = HoroDateTime::from_jd_zone(t.jd_utc + 1.0, 0.0)?;
        m = t.month;
        d = t.day;
    }

    //扣除夏令时
    if st {
        let t = swe_utc_time_zone(
            y,
            m.into(),
            d.into(),
            h.into(),
            mi.into(),
            sec.into(),
            tz + 1.0,
        );
        let t = swe_utc_time_zone(t.0, t.1, t.2, t.3, t.4, t.5, -tz);
        y = t.0;
        m = t.1 as u8;
        d = t.2 as u8;
        h = t.3 as u8;
        // 夏令时，减1小时，分，秒不会变
        // mi = t.min
        // sec = t.sec
    }
    HoroDateTime::new(y, m, d, h, mi, sec, tz)
}

#[cfg(test)]
mod test {
    use super::{horo_date_time, HoroDateTime};
    use swe::{swe_julday, swe_utc_to_jd, Calendar};

    const LEAP_SECONDS: [i32; 27] = [
        19720630, 19721231, 19731231, 19741231, 19751231, 19761231, 19771231, 19781231, 19791231,
        19810630, 19820630, 19830630, 19850630, 19871231, 19891231, 19901231, 19920630, 19930630,
        19940630, 19951231, 19970630, 19981231, 20051231, 20081231, 20120630, 20150630, 20161231,
    ];

    /// 测试儒略日构造函数
    #[test]
    fn test_from_jd_zone() {
        let t = HoroDateTime::from_jd_zone(2459312.5, 8.0);
        assert!(t.is_ok());
        let t = t.unwrap();
        assert_eq!(2021, t.year, "年");
        assert_eq!(4, t.month, "月");
        assert_eq!(8, t.day, "日");
        assert_eq!(8, t.hour, "时");
        assert_eq!(0, t.minute, "分");
        assert_eq!(0, t.second, "秒");
        assert_eq!(8.0, t.tz, "时区");
        assert_eq!(2459312.5, t.jd_utc, "儒略日");

        // 测试公元前1
        let t = HoroDateTime::from_jd_zone(1721423.0, 0.0);
        assert!(t.is_ok());
        let t = t.unwrap();
        assert_eq!(-1, t.year, "年");
        assert_eq!(12, t.month, "月");
        assert_eq!(31, t.day, "日");
        assert_eq!(12, t.hour, "时");
        assert_eq!(0, t.minute, "分");
        assert_eq!(0, t.second, "秒");
        assert_eq!(0.0, t.tz, "时区");
        assert_eq!(1721423.0, t.jd_utc, "儒略日");
    }

    // 儒略日构造函数非法时区
    #[test]
    fn test_from_jd_invalid_zone() {
        let zones = [-12.1, -13.0, 12.1, 13.0];
        for tz in zones {
            assert!(HoroDateTime::from_jd_zone(2459312.5, tz).is_err());
        }
    }

    // 儒略日构造函数正确时区
    #[test]
    fn test_from_jd_correct_zone() {
        let zones: Vec<f64> = (0..25).map(|x| f64::from(x - 12)).collect();
        for tz in zones {
            let t = HoroDateTime::from_jd_zone(2459312.5, tz);
            assert!(t.is_ok());
            let t = t.unwrap();
            // 2459312.5 = 2021-4-8 00:00:00 UTC
            assert_eq!(2021, t.year, "年");
            assert_eq!(4, t.month, "月");
            assert_eq!(if tz < 0.0 { 7 } else { 8 }, t.day, "日");
            assert_eq!(
                if tz < 0.0 {
                    (24.0 + tz) as u8
                } else {
                    tz as u8
                },
                t.hour,
                "时"
            );
            assert_eq!(0, t.minute, "分");
            assert_eq!(0, t.second, "秒");
            assert_eq!(tz, t.tz, "时区");
            assert_eq!(2459312.5, t.jd_utc, "儒略日");
        }
    }

    // 正确区分儒略历和格里高利历
    // 公元1582年10月15日00:00:00为格里高利历，儒略日=2299160.5
    #[test]
    fn test_form_jd_zone_15821015() {
        let t = HoroDateTime::from_jd_zone(2299160.5, 0.0);
        assert!(t.is_ok());
        let t = t.unwrap();
        assert_eq!(1582, t.year, "年");
        assert_eq!(10, t.month, "月");
        assert_eq!(15, t.day, "日");
        assert_eq!(0, t.hour, "时");
        assert_eq!(0, t.minute, "分");
        assert_eq!(0, t.second, "秒");
        assert_eq!(0.0, t.tz, "时区");
        assert_eq!(2299160.5, t.jd_utc, "儒略日");
    }

    // date time 构造函数非法时区
    #[test]
    fn test_new_invalid_time_zone() {
        let zones = [-12.1, -13.0, 12.1, 13.0];
        for tz in zones {
            assert!(HoroDateTime::new(2021, 4, 8, 2, 4, 10, tz).is_err())
        }
    }

    // 1582年10月5日-1582年10月14日不存在
    // 不存在的日期
    #[test]
    fn test_new_have_nott_date() {
        // 5日-14日抛异常
        for d in 5..15 {
            assert!(
                HoroDateTime::new(1582, 10, d, 2, 4, 10, 0.0).is_err(),
                "1582-10-{}",
                d
            )
        }

        //能正确处理10月4日，10月15日
        assert!(HoroDateTime::new(1582, 10, 4, 2, 5, 10, 0.0).is_ok());
        assert!(HoroDateTime::new(1582, 10, 15, 2, 15, 10, 0.0).is_ok());
    }

    /// 测试new函数
    // hour < 24
    // minute < 59
    // second <=60
    // 日期与时间
    #[test]
    fn test_new() {
        // 不正确的时
        for h in [24, 25] {
            assert!(
                HoroDateTime::new(2021, 4, 9, h, 4, 10, 0.0).is_err(),
                "{}时",
                h
            )
        }
        // 不正确的分
        for mi in [60, 61, 62] {
            assert!(
                HoroDateTime::new(2021, 4, 9, 0, mi, 10, 0.0).is_err(),
                "{}分",
                mi
            )
        }

        // 不正确的秒
        for s in [61, 62, 63] {
            assert!(
                HoroDateTime::new(2021, 4, 9, 0, 0, s, 0.0).is_err(),
                "{}秒",
                s
            )
        }
        //不存在的闰秒
        assert!(
            HoroDateTime::new(2021, 4, 9, 0, 0, 60, 0.0).is_err(),
            "闰秒"
        );

        //存在的闰秒
        assert!(
            HoroDateTime::new(2016, 12, 31, 23, 59, 60, 0.0).is_ok(),
            "闰秒"
        );

        //不正确的月
        for m in [0, 13, 14] {
            assert!(HoroDateTime::new(2021, m, 9, 0, 0, 0, 0.0).is_err(), "月")
        }

        //不正确的日
        for d in [0, 31, 32, 33] {
            assert!(HoroDateTime::new(2021, 4, d, 0, 0, 0, 0.0).is_err(), "日")
        }

        //不正确的2月29日
        assert!(
            HoroDateTime::new(2021, 2, 29, 0, 0, 0, 0.0).is_err(),
            "不存在的2月29日"
        );
        //不正确的4月31日
        assert!(
            HoroDateTime::new(2021, 4, 31, 0, 0, 0, 0.0).is_err(),
            "不存在的4月31日"
        );

        //正确的日期与时间不抛异常
        assert!(HoroDateTime::new(2021, 4, 8, 0, 0, 0, 0.0).is_ok());
        assert!(HoroDateTime::new(2021, 4, 2, 0, 0, 0, 0.0).is_ok());
        assert!(HoroDateTime::new(2021, 4, 30, 0, 0, 0, 0.0).is_ok());
        assert!(HoroDateTime::new(2021, 5, 31, 0, 0, 0, 0.0).is_ok());
        assert!(HoroDateTime::new(2021, 2, 28, 0, 0, 0, 8.0).is_ok());

        //正确计算儒略日并构造类
        let t = HoroDateTime::new(2021, 4, 8, 20, 54, 27, 8.0);
        assert!(t.is_ok());
        let t = t.unwrap();
        assert_eq!(2021, t.year, "年");
        assert_eq!(4, t.month, "月");
        assert_eq!(8, t.day, "日");
        assert_eq!(20, t.hour, "时");
        assert_eq!(54, t.minute, "分");
        assert_eq!(27, t.second, "秒");
        assert_eq!(8.0, t.tz, "时区");
        assert_eq!(2459313.0378125, t.jd_utc, "儒略日");

        // 公元前1年
        let t = HoroDateTime::new(-1, 12, 31, 20, 36, 27, 8.0);
        let t = t.unwrap();
        assert_eq!(-1, t.year, "公元前1年");
        assert_eq!(12, t.month, "公元前1年12月");
        assert_eq!(31, t.day, "公元前1年12月31日");
        assert_eq!(20, t.hour, "公元前1年12月31日8时");
        assert_eq!(36, t.minute, "公元前1年12月31日20时36分");
        assert_eq!(27, t.second, "公元前1年12月31日20时36分0秒");
        assert_eq!(8.0, t.tz, "公元前1年12月31日20时36分0秒东8区");
        assert_eq!(
            1721423.0253125, t.jd_utc,
            "公元前1年12月31日20时36分0秒东8区儒略日"
        );
    }

    // 测试 plus_days
    #[test]
    fn test_plus_days() {
        //正确计算儒略日并构造类
        let t = HoroDateTime::new(2021, 4, 8, 20, 54, 27, 8.0);
        assert!(t.is_ok());
        let t = t.unwrap();

        let days = 23.0 + (1.0 + 1.0 / 60.0 + 1.0 / 3600.0) / 24.0;
        let t = t.plus_days(days);
        assert!(t.is_ok());
        let t = t.unwrap();

        assert_eq!(2021, t.year, "年");
        assert_eq!(5, t.month, "月");
        assert_eq!(1, t.day, "日");
        assert_eq!(21, t.hour, "时");
        assert_eq!(55, t.minute, "分");
        assert_eq!(28, t.second, "秒");
        assert_eq!(8.0, t.tz, "时区");
        assert_eq!(2459313.0378125 + days, t.jd_utc, "儒略日");
    }

    // 能正确处理闰秒
    #[test]
    fn test_leap_seconds() {
        for date in LEAP_SECONDS {
            let year = date / 10000;
            let month = (date % 10000) / 100;
            let day = (date % 10000) % 100;

            let t_0 = HoroDateTime::new(year, month as u8, day as u8, 23, 59, 60, 0.0);
            assert!(t_0.is_ok());
            let t_0 = t_0.unwrap();
            let t_next = if month == 12 {
                let t = HoroDateTime::new(year + 1, 1, 1, 0, 0, 0, 0.0);
                assert!(t.is_ok());
                t.unwrap()
            } else {
                let t = HoroDateTime::new(year, 7, 1, 0, 0, 0, 0.0);
                assert!(t.is_ok());
                t.unwrap()
            };
            assert_eq!(
                t_0.jd_utc, t_next.jd_utc,
                "${year}-${month}-${day} 23:59:60:"
            )
        }
    }

    // jd_et
    #[test]
    fn test_jd_et() {
        let t = HoroDateTime::new(2021, 4, 8, 20, 59, 59, 0.0);
        assert!(t.is_ok());
        let t = t.unwrap();
        let jds = swe_utc_to_jd(2021, 4, 8, 20, 59, 59.0, Calendar::Gregorian);
        assert!(jds.is_ok());
        let jds = jds.unwrap();
        assert_eq!(jds[0], t.jd_et, "给定日期的jd_et");

        // 给定儒略日
        let t = HoroDateTime::from_jd_zone(2459330.0, 0.0);
        assert!(t.is_ok());
        let t = t.unwrap();
        let jds = swe_utc_to_jd(2021, 4, 25, 12, 0, 0.0, Calendar::Gregorian);
        assert!(jds.is_ok());
        let jds = jds.unwrap();
        assert_eq!(jds[0], t.jd_et, "给定儒略日的jd_et");
    }

    // jd_ut1
    #[test]
    fn test_jd_ut1() {
        let t = HoroDateTime::new(2021, 4, 8, 20, 59, 59, 0.0);
        assert!(t.is_ok());
        let t = t.unwrap();
        let jds = swe_utc_to_jd(2021, 4, 8, 20, 59, 59.0, Calendar::Gregorian);
        assert!(jds.is_ok());
        let jds = jds.unwrap();
        assert_eq!(jds[1], t.jd_ut1, "给定日期的jd_ut");

        // 给定儒略日
        let t = HoroDateTime::from_jd_zone(2459330.0, 0.0);
        assert!(t.is_ok());
        let t = t.unwrap();
        let jds = swe_utc_to_jd(2021, 4, 25, 12, 0, 0.0, Calendar::Gregorian);
        assert!(jds.is_ok());
        let jds = jds.unwrap();
        assert_eq!(jds[1], t.jd_ut1, "给定儒略日的jd_ut")
    }

    // 以时间调用函数horoDateTime
    #[test]
    fn test_horo_date_time() {
        let t = horo_date_time(2021, 4, 8, 20, 54, 27, 8.0, false);
        assert!(t.is_ok());
        let t = t.unwrap();
        assert_eq!(2021, t.year, "年");
        assert_eq!(4, t.month, "月");
        assert_eq!(8, t.day, "日");
        assert_eq!(20, t.hour, "时");
        assert_eq!(54, t.minute, "分");
        assert_eq!(27, t.second, "秒");
        assert_eq!(8.0, t.tz, "时区");
        assert_eq!(2459313.0378125, t.jd_utc, "儒略日");
    }

    // 函数horoDateTime扣除夏令时
    #[test]
    fn test_horo_date_time_no_st() {
        let t = horo_date_time(2021, 4, 8, 0, 54, 27, 8.0, true);
        assert!(t.is_ok());
        let t = t.unwrap();
        let jd = swe_julday(
            2021,
            4,
            7,
            15.0 + 54.0 / 60.0 + 27.0 / 3600.0,
            Calendar::Gregorian,
        );
        assert_eq!(2021, t.year, "年");
        assert_eq!(4, t.month, "月");
        assert_eq!(7, t.day, "日");
        assert_eq!(23, t.hour, "时");
        assert_eq!(54, t.minute, "分");
        assert_eq!(27, t.second, "秒");
        assert_eq!(8.0, t.tz, "时区");
        assert_eq!(jd, t.jd_utc, "儒略日");
    }

    // 函数horoDateTime平年2月29日=3月1日
    #[test]
    fn test_horo_date_time_with_is_not_leap_2_month_29_day() {
        let t = horo_date_time(2021, 2, 29, 8, 54, 27, 8.0, false);
        assert!(t.is_ok());
        let t = t.unwrap();
        let jd = swe_julday(
            2021,
            3,
            1,
            0.0 + 54.0 / 60.0 + 27.0 / 3600.0,
            Calendar::Gregorian,
        );
        assert_eq!(2021, t.year, "年");
        assert_eq!(3, t.month, "月");
        assert_eq!(1, t.day, "日");
        assert_eq!(8, t.hour, "时");
        assert_eq!(54, t.minute, "分");
        assert_eq!(27, t.second, "秒");
        assert_eq!(8.0, t.tz, "时区");
        assert_eq!(jd, t.jd_utc, "儒略日");
    }

    // 函数horoDateTime闰年2月29日=2月29日
    #[test]
    fn test_horo_date_time_with_is_leap_2_month_29_day() {
        let t = horo_date_time(2020, 2, 29, 8, 54, 27, 8.0, false);
        assert!(t.is_ok());
        let t = t.unwrap();
        let jd = swe_julday(
            2020,
            2,
            29,
            0.0 + 54.0 / 60.0 + 27.0 / 3600.0,
            Calendar::Gregorian,
        );
        assert_eq!(2020, t.year, "年");
        assert_eq!(2, t.month, "月");
        assert_eq!(29, t.day, "日");
        assert_eq!(8, t.hour, "时");
        assert_eq!(54, t.minute, "分");
        assert_eq!(27, t.second, "秒");
        assert_eq!(8.0, t.tz, "时区");
        assert_eq!(jd, t.jd_utc, "儒略日");
    }
}
