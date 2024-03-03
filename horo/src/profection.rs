use crate::Error;
use horo_date_time::{horo_date_time, HoroDateTime};

#[cfg(feature = "serde")]
use serde::Serialize;

#[cfg(feature = "swagger")]
use utoipa::ToSchema;

#[cfg_attr(feature = "serde", derive(Serialize))]
#[cfg_attr(feature = "swagger", derive(ToSchema))]
pub struct Profection {
    /// 年小限所在宫位
    pub year_house: u8,
    /// 月小限所在宫位
    pub month_house: u8,
    /// 日小限所在宫位
    pub day_house: u8,
    /// 每宫对应的日小限开始时间
    pub date_per_house: Vec<HoroDateTime>,
}

impl Profection {
    pub fn new(native_date: HoroDateTime, process_date: HoroDateTime) -> Result<Self, Error> {
        if process_date.jd_utc < native_date.jd_utc {
            let msg = "Profections date muste be greate or equal birthday.".to_string();
            return Err(Error::InvalidProfectionDateTime(msg));
        }

        // 小限年的生日
        let profection_birthday = horo_date_time(
            process_date.year,
            native_date.month,
            native_date.day,
            native_date.hour,
            native_date.minute,
            native_date.second,
            native_date.tz,
            false,
        )?;

        //年小限所在年数
        let profection_year = if process_date.jd_utc < profection_birthday.jd_utc {
            process_date.year - 1
        } else {
            process_date.year
        };

        let n = (profection_year - native_date.year) % 12;

        // 求得年小限所在宫位
        let mut profection_year_house = (n + 1) % 12;
        if profection_year_house == 0 {
            profection_year_house = 12
        }

        let profection_year_house = profection_year_house as u8;

        // 计算月小限
        // 月份的生日
        let profection_birthday_month = horo_date_time(
            process_date.year,
            process_date.month,
            native_date.day,
            native_date.hour,
            native_date.minute,
            native_date.second,
            native_date.tz,
            false,
        )?;

        // 小限所在月分
        let profection_month = if process_date.jd_utc < profection_birthday_month.jd_utc {
            let month = process_date.month - 1;
            if month == 0 {
                12
            } else {
                month
            }
        } else {
            process_date.month
        };

        // n==0，必是profection_month==native_date
        let n = (12 + profection_month - native_date.month) % 12;
        // 得到月小限所在宫位
        let mut profection_month_house = (profection_year_house + n) % 12;
        if profection_month_house == 0 {
            profection_month_house = 12;
        }

        // 天小限
        // 小限所在月与出生日相同的时间
        let profection_birthday_day = horo_date_time(
            process_date.year,
            profection_month,
            native_date.day,
            native_date.hour,
            native_date.minute,
            native_date.second,
            native_date.tz,
            false,
        )?;

        // 下一个小限所在月与出生日相同的时间
        let profection_birthday_next_day = horo_date_time(
            if profection_month == 12 {
                process_date.year + 1
            } else {
                process_date.year
            },
            if profection_month == 12 {
                1
            } else {
                profection_month + 1
            },
            native_date.day,
            native_date.hour,
            native_date.minute,
            native_date.second,
            native_date.tz,
            false,
        )?;
        // 两个小限月之间相差的天数
        let profection_month_days =
            profection_birthday_next_day.jd_utc - profection_birthday_day.jd_utc;
        // 小限天数
        let profection_days = process_date.jd_utc - profection_birthday_day.jd_utc;

        let n = 12.0 * profection_days / profection_month_days;
        let n = n as u8; // 0.24为月小限所在宫位，因此向下取整

        // 日小限所在宫位
        let mut profection_day_house = (profection_month_house + n) % 12;
        if profection_day_house == 0 {
            profection_day_house = 12;
        }

        // 每个宫位对应的天数，用于计算每宫对应的日小限区间
        let n = profection_month_days / 12.0;

        let mut date_per_house = vec![];
        for i in 0..12 {
            let day = profection_birthday_day.plus_days(n * f64::from(i))?;
            date_per_house.push(day);
        }

        Ok(Self {
            year_house: profection_year_house,
            month_house: profection_month_house,
            day_house: profection_day_house,
            date_per_house,
        })
    }
}

#[cfg(test)]
mod tests {
    use crate::{profection::Profection, HoroDateTime};

    #[test]
    fn test_profection_new() {
        // 出生时间在10月，方便测试：12月+1=下一年1月
        // 如果出生时间在1月，不能测试到此种情况
        let native_date = HoroDateTime::new(2023, 10, 15, 19, 45, 1, 8.0);
        let profection_date = HoroDateTime::new(2022, 10, 15, 19, 45, 1, 8.0);

        assert!(native_date.is_ok());
        assert!(profection_date.is_ok());

        let native_date = native_date.unwrap();
        let profection_date = profection_date.unwrap();

        // 不正确的参数
        let profection = Profection::new(native_date, profection_date);
        assert!(profection.is_err());

        // 推运时间与出生时间相同
        let native_date = HoroDateTime::new(2023, 10, 15, 19, 45, 1, 8.0);
        let profection_date = HoroDateTime::new(2023, 10, 15, 19, 45, 1, 8.0);

        assert!(native_date.is_ok());
        assert!(profection_date.is_ok());

        let native_date = native_date.unwrap();
        let profection_date = profection_date.unwrap();

        let profection = Profection::new(native_date, profection_date);
        assert!(profection.is_ok());
        let profection = profection.unwrap();
        assert_eq!(profection.year_house, 1);
        assert_eq!(profection.month_house, 1);
        assert_eq!(profection.day_house, 1);
        assert_eq!(profection.date_per_house.len(), 12);

        // 出后第2年，但未到生日
        let native_date = HoroDateTime::new(2023, 10, 15, 19, 45, 1, 8.0);
        let profection_date = HoroDateTime::new(2024, 8, 14, 18, 40, 1, 8.0);

        assert!(native_date.is_ok());
        assert!(profection_date.is_ok());

        let native_date = native_date.unwrap();
        let profection_date = profection_date.unwrap();

        let profection = Profection::new(native_date, profection_date);
        assert!(profection.is_ok());
        let profection = profection.unwrap();
        assert_eq!(profection.year_house, 1);
        assert_eq!(profection.month_house, 10);
        assert_eq!(profection.day_house, 9);
        assert_eq!(profection.date_per_house.len(), 12);

        // 出后第2年，已过生日
        let native_date = HoroDateTime::new(2023, 10, 15, 19, 45, 1, 8.0);
        let profection_date = HoroDateTime::new(2024, 12, 14, 18, 40, 1, 8.0);

        assert!(native_date.is_ok());
        assert!(profection_date.is_ok());

        let native_date = native_date.unwrap();
        let profection_date = profection_date.unwrap();

        let profection = Profection::new(native_date, profection_date);
        assert!(profection.is_ok());
        let profection = profection.unwrap();
        assert_eq!(profection.year_house, 2);
        assert_eq!(profection.month_house, 3);
        assert_eq!(profection.day_house, 2);
        assert_eq!(profection.date_per_house.len(), 12);

        // 出后第12年，在生日
        let native_date = HoroDateTime::new(2023, 10, 15, 19, 45, 1, 8.0);
        let profection_date = HoroDateTime::new(2035, 10, 15, 19, 45, 1, 8.0);

        assert!(native_date.is_ok());
        assert!(profection_date.is_ok());

        let native_date = native_date.unwrap();
        let profection_date = profection_date.unwrap();

        let profection = Profection::new(native_date, profection_date);
        assert!(profection.is_ok());
        let profection = profection.unwrap();
        assert_eq!(profection.year_house, 1);
        assert_eq!(profection.month_house, 1);
        assert_eq!(profection.day_house, 1);
        assert_eq!(profection.date_per_house.len(), 12);

        // 出后第13年，但未到生日
        let native_date = HoroDateTime::new(2023, 10, 15, 19, 45, 1, 8.0);
        let profection_date = HoroDateTime::new(2036, 8, 14, 18, 40, 1, 8.0);

        assert!(native_date.is_ok());
        assert!(profection_date.is_ok());

        let native_date = native_date.unwrap();
        let profection_date = profection_date.unwrap();

        let profection = Profection::new(native_date, profection_date);
        assert!(profection.is_ok());
        let profection = profection.unwrap();
        assert_eq!(profection.year_house, 1);
        assert_eq!(profection.month_house, 10);
        assert_eq!(profection.day_house, 9);
        assert_eq!(profection.date_per_house.len(), 12);

        // 出后第13年，已过生日
        let native_date = HoroDateTime::new(2023, 10, 15, 19, 45, 1, 8.0);
        let profection_date = HoroDateTime::new(2036, 12, 14, 18, 40, 1, 8.0);

        assert!(native_date.is_ok());
        assert!(profection_date.is_ok());

        let native_date = native_date.unwrap();
        let profection_date = profection_date.unwrap();

        let profection = Profection::new(native_date, profection_date);
        assert!(profection.is_ok());
        let profection = profection.unwrap();
        assert_eq!(profection.year_house, 2);
        assert_eq!(profection.month_house, 3);
        assert_eq!(profection.day_house, 2);
        assert_eq!(profection.date_per_house.len(), 12);

        // 出后第13年，已过生日，每宫对应的日小限开始时间，11月小月,
        let native_date = HoroDateTime::new(2023, 10, 15, 19, 45, 1, 8.0);
        let profection_date = HoroDateTime::new(2036, 12, 14, 18, 40, 1, 8.0);

        assert!(native_date.is_ok());
        assert!(profection_date.is_ok());

        let native_date = native_date.unwrap();
        let profection_date = profection_date.unwrap();

        let profection = Profection::new(native_date, profection_date);
        assert!(profection.is_ok());
        let profection = profection.unwrap();

        // 月小限开始时间
        let profection_month_date = HoroDateTime::new(2036, 11, 15, 19, 45, 1, 8.0);
        assert!(profection_month_date.is_ok());
        let profection_month_date = profection_month_date.unwrap();
        for i in 0..12 {
            let day = 2.5 * f64::from(i as u8);
            let date = profection_month_date.plus_days(day);
            assert!(date.is_ok());
            let date = date.unwrap();
            assert_eq!(profection.date_per_house[i].year, date.year);
            assert_eq!(profection.date_per_house[i].month, date.month);
            assert_eq!(profection.date_per_house[i].day, date.day);
            assert_eq!(profection.date_per_house[i].hour, date.hour);
            assert_eq!(profection.date_per_house[i].minute, date.minute);
            assert_eq!(profection.date_per_house[i].second, date.second);
            assert_eq!(profection.date_per_house[i].tz, date.tz);
        }

        // 出后第13年，已过生日，每宫对应的日小限开始时间，12月大月,
        let native_date = HoroDateTime::new(2023, 10, 15, 19, 45, 1, 8.0);
        let profection_date = HoroDateTime::new(2036, 12, 17, 18, 40, 1, 8.0);

        assert!(native_date.is_ok());
        assert!(profection_date.is_ok());

        let native_date = native_date.unwrap();
        let profection_date = profection_date.unwrap();

        let profection = Profection::new(native_date, profection_date);
        assert!(profection.is_ok());
        let profection = profection.unwrap();

        // 月小限开始时间
        let profection_month_date = HoroDateTime::new(2036, 12, 15, 19, 45, 1, 8.0);
        assert!(profection_month_date.is_ok());
        let profection_month_date = profection_month_date.unwrap();
        for i in 0..12 {
            let day = 31.0 / 12.0 * f64::from(i as u8);
            let date = profection_month_date.plus_days(day);
            assert!(date.is_ok());
            let date = date.unwrap();
            assert_eq!(profection.date_per_house[i].year, date.year);
            assert_eq!(profection.date_per_house[i].month, date.month);
            assert_eq!(profection.date_per_house[i].day, date.day);
            assert_eq!(profection.date_per_house[i].hour, date.hour);
            assert_eq!(profection.date_per_house[i].minute, date.minute);
            assert_eq!(profection.date_per_house[i].second, date.second);
            assert_eq!(profection.date_per_house[i].tz, date.tz);
        }
    }
}
