use crate::{Error, GeoPosition, HoroDateTime, Horoscope, HouseName, PlanetConfig, PlanetName};

#[cfg(feature = "serde")]
use serde::Serialize;

#[cfg(feature = "swagger")]
use utoipa::ToSchema;

/// 法达主周期
#[cfg_attr(feature = "serde", derive(Serialize))]
#[cfg_attr(feature = "swagger", derive(ToSchema))]
pub struct FirdariaPeriod {
    pub period: PlanetName,
    pub sub_period: Vec<FirdariaSubPeriod>,
}

/// 法达子周期
#[cfg_attr(feature = "serde", derive(Serialize))]
#[cfg_attr(feature = "swagger", derive(ToSchema))]
pub struct FirdariaSubPeriod {
    pub period: PlanetName,
    pub start_date: HoroDateTime,
}

pub fn firdaria_process(
    native_date: HoroDateTime,
    geo: GeoPosition,
    planets_config: &[PlanetConfig],
    ephe_path: &str,
) -> Result<Vec<FirdariaPeriod>, Error> {
    // 计算原星盘
    let horo = Horoscope::new(
        native_date,
        geo,
        HouseName::Alcabitus,
        planets_config,
        ephe_path,
    )?;

    let firdaria_series = if horo.is_diurnal {
        [
            PlanetName::Sun,
            PlanetName::Venus,
            PlanetName::Mercury,
            PlanetName::Moon,
            PlanetName::Saturn,
            PlanetName::Jupiter,
            PlanetName::Mars,
            PlanetName::NorthNode,
            PlanetName::SouthNode,
        ]
    } else {
        [
            PlanetName::Moon,
            PlanetName::Saturn,
            PlanetName::Jupiter,
            PlanetName::Mars,
            PlanetName::NorthNode,
            PlanetName::SouthNode,
            PlanetName::Sun,
            PlanetName::Venus,
            PlanetName::Mercury,
        ]
    };

    // 法达周期
    let mut firdaria: Vec<FirdariaPeriod> = vec![];
    for planet in firdaria_series {
        let firdaria_period = if planet.is_firdaria_sub_period() {
            let firdaria_start_date = if let Some(f) = firdaria.last() {
                HoroDateTime::new(
                    f.sub_period[0].start_date.year
                        + i32::from(f.period.firdaria_year_number().unwrap()),
                    native_date.month,
                    native_date.day,
                    native_date.hour,
                    native_date.minute,
                    native_date.second,
                    native_date.tz,
                )?
            } else {
                native_date
            };

            let firdaria_end_date = HoroDateTime::new(
                firdaria_start_date.year + i32::from(planet.firdaria_year_number().unwrap()),
                native_date.month,
                native_date.day,
                native_date.hour,
                native_date.minute,
                native_date.second,
                native_date.tz,
            )?;

            let sub_period_days = firdaria_end_date.jd_utc - firdaria_start_date.jd_utc;

            let mut sub_periods: Vec<FirdariaSubPeriod> = vec![];

            while sub_periods.len() < 7 {
                let sub_firdaria = if let Some(f) = sub_periods.last() {
                    let start_date = f.start_date.plus_days(sub_period_days / 7.0)?;
                    let planet = f.period.next_sub_period().unwrap();
                    FirdariaSubPeriod {
                        period: planet,
                        start_date,
                    }
                } else {
                    FirdariaSubPeriod {
                        period: planet.clone(),
                        start_date: firdaria_start_date,
                    }
                };
                sub_periods.push(sub_firdaria);
            }
            FirdariaPeriod {
                period: planet.clone(),
                sub_period: sub_periods,
            }
        } else {
            let firdaria_start_date = if let Some(f) = firdaria.last() {
                HoroDateTime::new(
                    f.sub_period[0].start_date.year
                        + i32::from(f.period.firdaria_year_number().unwrap()),
                    native_date.month,
                    native_date.day,
                    native_date.hour,
                    native_date.minute,
                    native_date.second,
                    native_date.tz,
                )?
            } else {
                native_date
            };

            FirdariaPeriod {
                period: planet.clone(),
                sub_period: vec![FirdariaSubPeriod {
                    period: planet,
                    start_date: firdaria_start_date,
                }],
            }
        };

        firdaria.push(firdaria_period);
    }
    Ok(firdaria)
}

#[cfg(test)]
mod test {
    use crate::{firdaria_process, GeoPosition, HoroDateTime, PlanetConfig, PlanetName};
    use std::env;

    const SUB_PERIOD_SERIES: [PlanetName; 7] = [
          PlanetName::Sun,
          PlanetName::Venus,
          PlanetName::Mercury,
          PlanetName::Moon,
          PlanetName::Saturn,
          PlanetName::Jupiter,          
          PlanetName::Mars
        ];

    #[test]
    fn test_firdaria_diurnal() {
        dotenvy::dotenv().ok();
        let ephe_path = env::var("EPHE_PATH")
            .expect("没设置 EPHE_PATH 环境变量，可在.env文件中设置或export EPHE_PATH=...");

        let native_date = HoroDateTime::new(2024, 3, 1, 12, 0, 0, 8.0);
        assert!(native_date.is_ok());
        let native_date = native_date.unwrap();

        let geo = GeoPosition::new(
            102.0 + 41.0 / 60.0 + 59.0 / 3600.0,
            25.0 + 1.0 / 60.0 + 53.0 / 3600.0,
        );
        assert!(geo.is_ok());
        let geo = geo.unwrap();

        let planets_config = PlanetConfig::default_all_configs();

        let firdaria = firdaria_process(native_date, geo, &planets_config, &ephe_path);
        assert!(firdaria.is_ok());
        let firdaria = firdaria.unwrap();

        assert_eq!(firdaria.len(), 9);

        assert_eq!(firdaria[0].period, PlanetName::Sun);
        assert_eq!(firdaria[1].period, PlanetName::Venus);
        assert_eq!(firdaria[2].period, PlanetName::Mercury);
        assert_eq!(firdaria[3].period, PlanetName::Moon);
        assert_eq!(firdaria[4].period, PlanetName::Saturn);
        assert_eq!(firdaria[5].period, PlanetName::Jupiter);
        assert_eq!(firdaria[6].period, PlanetName::Mars);
        assert_eq!(firdaria[7].period, PlanetName::NorthNode);
        assert_eq!(firdaria[8].period, PlanetName::SouthNode);

        assert_eq!(firdaria[0].sub_period.len(), 7);
        assert_eq!(firdaria[1].sub_period.len(), 7);
        assert_eq!(firdaria[2].sub_period.len(), 7);
        assert_eq!(firdaria[3].sub_period.len(), 7);
        assert_eq!(firdaria[4].sub_period.len(), 7);
        assert_eq!(firdaria[5].sub_period.len(), 7);
        assert_eq!(firdaria[6].sub_period.len(), 7);
        assert_eq!(firdaria[7].sub_period.len(), 1);
        assert_eq!(firdaria[8].sub_period.len(), 1);

        assert_eq!(firdaria[0].sub_period[0].start_date.year, 2024);
        assert_eq!(firdaria[1].sub_period[0].start_date.year, 2024 + 10);
        assert_eq!(firdaria[2].sub_period[0].start_date.year, 2024 + 10 + 8);
        assert_eq!(firdaria[3].sub_period[0].start_date.year, 2024 + 10 + 8 + 13);
        assert_eq!(firdaria[4].sub_period[0].start_date.year, 2024 + 10 + 8 + 13 + 9);
        assert_eq!(firdaria[5].sub_period[0].start_date.year, 2024 + 10 + 8 + 13 + 9 + 11);
        assert_eq!(firdaria[6].sub_period[0].start_date.year, 2024 + 10 + 8 + 13 + 9 + 11 + 12);
        assert_eq!(firdaria[7].sub_period[0].start_date.year, 2024 + 10 + 8 + 13 + 9 + 11 + 12 + 7);
        assert_eq!(firdaria[8].sub_period[0].start_date.year, 2024 + 10 + 8 + 13 + 9 + 11 + 12 + 7 + 3);

        assert_eq!(firdaria[0].sub_period[0].start_date.month,3);
        assert_eq!(firdaria[1].sub_period[0].start_date.month,3);
        assert_eq!(firdaria[2].sub_period[0].start_date.month,3);
        assert_eq!(firdaria[3].sub_period[0].start_date.month,3);
        assert_eq!(firdaria[4].sub_period[0].start_date.month,3);
        assert_eq!(firdaria[5].sub_period[0].start_date.month,3);
        assert_eq!(firdaria[6].sub_period[0].start_date.month,3);
        assert_eq!(firdaria[7].sub_period[0].start_date.month,3);
        assert_eq!(firdaria[8].sub_period[0].start_date.month,3);

        assert_eq!(firdaria[0].sub_period[0].start_date.day,1);
        assert_eq!(firdaria[1].sub_period[0].start_date.day,1);
        assert_eq!(firdaria[2].sub_period[0].start_date.day,1);
        assert_eq!(firdaria[3].sub_period[0].start_date.day,1);
        assert_eq!(firdaria[4].sub_period[0].start_date.day,1);
        assert_eq!(firdaria[5].sub_period[0].start_date.day,1);
        assert_eq!(firdaria[6].sub_period[0].start_date.day,1);
        assert_eq!(firdaria[7].sub_period[0].start_date.day,1);
        assert_eq!(firdaria[8].sub_period[0].start_date.day,1);

        assert_eq!(firdaria[0].sub_period[0].start_date.hour,12);
        assert_eq!(firdaria[1].sub_period[0].start_date.hour,12);
        assert_eq!(firdaria[2].sub_period[0].start_date.hour,12);
        assert_eq!(firdaria[3].sub_period[0].start_date.hour,12);
        assert_eq!(firdaria[4].sub_period[0].start_date.hour,12);
        assert_eq!(firdaria[5].sub_period[0].start_date.hour,12);
        assert_eq!(firdaria[6].sub_period[0].start_date.hour,12);
        assert_eq!(firdaria[7].sub_period[0].start_date.hour,12);
        assert_eq!(firdaria[8].sub_period[0].start_date.hour,12);

        assert_eq!(firdaria[0].sub_period[0].start_date.minute,0);
        assert_eq!(firdaria[1].sub_period[0].start_date.minute,0);
        assert_eq!(firdaria[2].sub_period[0].start_date.minute,0);
        assert_eq!(firdaria[3].sub_period[0].start_date.minute,0);
        assert_eq!(firdaria[4].sub_period[0].start_date.minute,0);
        assert_eq!(firdaria[5].sub_period[0].start_date.minute,0);
        assert_eq!(firdaria[6].sub_period[0].start_date.minute,0);
        assert_eq!(firdaria[7].sub_period[0].start_date.minute,0);
        assert_eq!(firdaria[8].sub_period[0].start_date.minute,0);

        assert_eq!(firdaria[0].sub_period[0].start_date.second,0);
        assert_eq!(firdaria[1].sub_period[0].start_date.second,0);
        assert_eq!(firdaria[2].sub_period[0].start_date.second,0);
        assert_eq!(firdaria[3].sub_period[0].start_date.second,0);
        assert_eq!(firdaria[4].sub_period[0].start_date.second,0);
        assert_eq!(firdaria[5].sub_period[0].start_date.second,0);
        assert_eq!(firdaria[6].sub_period[0].start_date.second,0);
        assert_eq!(firdaria[7].sub_period[0].start_date.second,0);
        assert_eq!(firdaria[8].sub_period[0].start_date.second,0);

        // 测试子周期，前已经测试通过主限开始时间，
        // 因此，后续的测试可以直接使用主限的开始时间，不必额外计算主限的开始时间
        for f in firdaria{
            if f.sub_period.len()==1{
                assert_eq!(f.period, f.sub_period[0].period);
            }else{
                let year_num = f.period.firdaria_year_number();
                assert!(year_num.is_some());
                let year_num = year_num.unwrap();

                let start_date = f.sub_period[0].start_date;

                let end_date = HoroDateTime::new(
                start_date.year+ i32::from(year_num),
                start_date.month,
                start_date.day,
                start_date.hour,
                start_date.minute,
                start_date.second,
                start_date.tz);
                assert!(end_date.is_ok());
                let end_date=end_date.unwrap();

                let days = (end_date.jd_utc - start_date.jd_utc)/7.0;

                // 测试开始时间和行星
                for n in 0u8..7{
                    let date = start_date.plus_days(days * f64::from( n));
                    assert!(date.is_ok());
                    let date=date.unwrap();

                   let k = SUB_PERIOD_SERIES.iter().position(|&p| p==f.period);
                   assert!(k.is_some());
                   let k=k.unwrap();

                   let n: usize = n.into();

                    assert_eq!(f.sub_period[n].period, SUB_PERIOD_SERIES[(k+  n)%7], "第{n}个子周期");

                    // 会因浮点数，有1秒的误差
                    assert!((f.sub_period[n].start_date.jd_utc-date.jd_utc).abs()*24.0<1.0/3600.0, "第{n}个子周期");
                }
            }
        }
    }

    #[test]
    fn test_firdaria_night() {
        dotenvy::dotenv().ok();
        let ephe_path = env::var("EPHE_PATH")
            .expect("没设置 EPHE_PATH 环境变量，可在.env文件中设置或export EPHE_PATH=...");

        let native_date = HoroDateTime::new(2024, 3, 1, 0, 0, 0, 8.0);
        assert!(native_date.is_ok());
        let native_date = native_date.unwrap();

        let geo = GeoPosition::new(
            102.0 + 41.0 / 60.0 + 59.0 / 3600.0,
            25.0 + 1.0 / 60.0 + 53.0 / 3600.0,
        );
        assert!(geo.is_ok());
        let geo = geo.unwrap();

        let planets_config = PlanetConfig::default_all_configs();

        let firdaria = firdaria_process(native_date, geo, &planets_config, &ephe_path);
        assert!(firdaria.is_ok());
        let firdaria = firdaria.unwrap();

        assert_eq!(firdaria.len(), 9);

        assert_eq!(firdaria[0].period, PlanetName::Moon);
        assert_eq!(firdaria[1].period, PlanetName::Saturn);
        assert_eq!(firdaria[2].period, PlanetName::Jupiter);
        assert_eq!(firdaria[3].period, PlanetName::Mars);
        assert_eq!(firdaria[4].period, PlanetName::NorthNode);
        assert_eq!(firdaria[5].period, PlanetName::SouthNode);
        assert_eq!(firdaria[6].period, PlanetName::Sun);
        assert_eq!(firdaria[7].period, PlanetName::Venus);
        assert_eq!(firdaria[8].period, PlanetName::Mercury);

        assert_eq!(firdaria[0].sub_period.len(), 7);
        assert_eq!(firdaria[1].sub_period.len(), 7);
        assert_eq!(firdaria[2].sub_period.len(), 7);
        assert_eq!(firdaria[3].sub_period.len(), 7);
        assert_eq!(firdaria[4].sub_period.len(), 1);
        assert_eq!(firdaria[5].sub_period.len(), 1);
        assert_eq!(firdaria[6].sub_period.len(), 7);
        assert_eq!(firdaria[7].sub_period.len(), 7);
        assert_eq!(firdaria[8].sub_period.len(), 7);

        assert_eq!(firdaria[0].sub_period[0].start_date.year, 2024);
        assert_eq!(firdaria[1].sub_period[0].start_date.year, 2024 + 9);
        assert_eq!(firdaria[2].sub_period[0].start_date.year, 2024 + 9 + 11);
        assert_eq!(firdaria[3].sub_period[0].start_date.year, 2024 + 9 + 11 + 12);
        assert_eq!(firdaria[4].sub_period[0].start_date.year, 2024 + 9 + 11 + 12 + 7);
        assert_eq!(firdaria[5].sub_period[0].start_date.year, 2024 + 9 + 11 + 12 + 7 + 3);
        assert_eq!(firdaria[6].sub_period[0].start_date.year, 2024 + 9 + 11 + 12 + 7 + 3 + 2);
        assert_eq!(firdaria[7].sub_period[0].start_date.year, 2024 + 9 + 11 + 12 + 7 + 3 + 2 + 10);
        assert_eq!(firdaria[8].sub_period[0].start_date.year, 2024 + 9 + 11 + 12 + 7 + 3 + 2 + 10 + 8);

        assert_eq!(firdaria[0].sub_period[0].start_date.month,3);
        assert_eq!(firdaria[1].sub_period[0].start_date.month,3);
        assert_eq!(firdaria[2].sub_period[0].start_date.month,3);
        assert_eq!(firdaria[3].sub_period[0].start_date.month,3);
        assert_eq!(firdaria[4].sub_period[0].start_date.month,3);
        assert_eq!(firdaria[5].sub_period[0].start_date.month,3);
        assert_eq!(firdaria[6].sub_period[0].start_date.month,3);
        assert_eq!(firdaria[7].sub_period[0].start_date.month,3);
        assert_eq!(firdaria[8].sub_period[0].start_date.month,3);

        assert_eq!(firdaria[0].sub_period[0].start_date.day,1);
        assert_eq!(firdaria[1].sub_period[0].start_date.day,1);
        assert_eq!(firdaria[2].sub_period[0].start_date.day,1);
        assert_eq!(firdaria[3].sub_period[0].start_date.day,1);
        assert_eq!(firdaria[4].sub_period[0].start_date.day,1);
        assert_eq!(firdaria[5].sub_period[0].start_date.day,1);
        assert_eq!(firdaria[6].sub_period[0].start_date.day,1);
        assert_eq!(firdaria[7].sub_period[0].start_date.day,1);
        assert_eq!(firdaria[8].sub_period[0].start_date.day,1);
    
        assert_eq!(firdaria[0].sub_period[0].start_date.hour,0);
        assert_eq!(firdaria[1].sub_period[0].start_date.hour,0);
        assert_eq!(firdaria[2].sub_period[0].start_date.hour,0);
        assert_eq!(firdaria[3].sub_period[0].start_date.hour,0);
        assert_eq!(firdaria[4].sub_period[0].start_date.hour,0);
        assert_eq!(firdaria[5].sub_period[0].start_date.hour,0);
        assert_eq!(firdaria[6].sub_period[0].start_date.hour,0);
        assert_eq!(firdaria[7].sub_period[0].start_date.hour,0);
        assert_eq!(firdaria[8].sub_period[0].start_date.hour,0);

        assert_eq!(firdaria[0].sub_period[0].start_date.minute,0);
        assert_eq!(firdaria[1].sub_period[0].start_date.minute,0);
        assert_eq!(firdaria[2].sub_period[0].start_date.minute,0);
        assert_eq!(firdaria[3].sub_period[0].start_date.minute,0);
        assert_eq!(firdaria[4].sub_period[0].start_date.minute,0);
        assert_eq!(firdaria[5].sub_period[0].start_date.minute,0);
        assert_eq!(firdaria[6].sub_period[0].start_date.minute,0);
        assert_eq!(firdaria[7].sub_period[0].start_date.minute,0);
        assert_eq!(firdaria[8].sub_period[0].start_date.minute,0);

        assert_eq!(firdaria[0].sub_period[0].start_date.second,0);
        assert_eq!(firdaria[1].sub_period[0].start_date.second,0);
        assert_eq!(firdaria[2].sub_period[0].start_date.second,0);
        assert_eq!(firdaria[3].sub_period[0].start_date.second,0);
        assert_eq!(firdaria[4].sub_period[0].start_date.second,0);
        assert_eq!(firdaria[5].sub_period[0].start_date.second,0);
        assert_eq!(firdaria[6].sub_period[0].start_date.second,0);
        assert_eq!(firdaria[7].sub_period[0].start_date.second,0);
        assert_eq!(firdaria[8].sub_period[0].start_date.second,0);


        // 测试子周期，前已经测试通过主限开始时间，
        // 因此，后续的测试可以直接使用主限的开始时间，不必额外计算主限的开始时间
        for f in firdaria{
            if f.sub_period.len()==1{
                assert_eq!(f.period, f.sub_period[0].period);
            }else{
                let year_num = f.period.firdaria_year_number();
                assert!(year_num.is_some());
                let year_num = year_num.unwrap();

                let start_date = f.sub_period[0].start_date;

                let end_date = HoroDateTime::new(
                start_date.year+ i32::from(year_num),
                start_date.month,
                start_date.day,
                start_date.hour,
                start_date.minute,
                start_date.second,
                start_date.tz);
                assert!(end_date.is_ok());
                let end_date=end_date.unwrap();

                let days = (end_date.jd_utc - start_date.jd_utc)/7.0;

                // 测试开始时间和行星
                for n in 0u8..7{
                    let date = start_date.plus_days(days * f64::from( n));
                    assert!(date.is_ok());
                    let date=date.unwrap();

                   let k = SUB_PERIOD_SERIES.iter().position(|&p| p==f.period);
                   assert!(k.is_some());
                   let k=k.unwrap();

                   let n: usize = n.into();

                    assert_eq!(f.sub_period[n].period, SUB_PERIOD_SERIES[(k+  n)%7], "第{n}个子周期");

                    // assert_eq!(f.sub_period[n].start_date.year, date.year, "第{n}个子周期");
                    // assert_eq!(f.sub_period[n].start_date.month, date.month, "第{n}个子周期");
                    // assert_eq!(f.sub_period[n].start_date.day, date.day, "第{n}个子周期");
                    // assert_eq!(f.sub_period[n].start_date.hour, date.hour, "第{n}个子周期");
                    // assert_eq!(f.sub_period[n].start_date.minute, date.minute, "第{n}个子周期");
                    // assert_eq!(f.sub_period[n].start_date.second, date.second, "第{n}个子周期");
                    // 不直接比较year, month,day,hour,minute,second，因为plus_days()是从jd_utc反算回日期，时间
                    // 会因浮点数，有1秒的误差
                    assert!((f.sub_period[n].start_date.jd_utc-date.jd_utc).abs()*24.0<1.0/3600.0, "第{n}个子周期");
                }
            }
        }
        
    }
    
}
