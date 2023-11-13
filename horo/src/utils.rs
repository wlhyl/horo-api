use swe::{swe_calc_ut, swe_close, swe_degnorm, swe_set_ephe_path, Body};

use crate::error::Error;

/// 计算给定时刻的黄赤倾角
pub fn calc_eps(jd_utc: f64, ephe_path: &str) -> Result<f64, Error> {
    swe_set_ephe_path(ephe_path);
    let ret = swe_calc_ut(jd_utc, &Body::SeEclNut, &[]);
    swe_close();

    match ret {
        Ok(v) => Ok(v[0]),
        Err(e) => Err(Error::Function(format!("计算黄赤倾角错误:{e}"))),
    }
}

/// ModPi 把角度限制在[-180, 180]之间
pub fn mod180(r0: f64) -> f64 {
    let mut r = r0;
    while r < -180.0 {
        r += 360.0;
    }
    while r > 180.0 {
        r -= 360.0;
    }
    r
}

/// NewtonIteration 牛顿迭代法求解方程的根
pub fn newton_iteration<F>(init_value: f64, f: F) -> Result<f64, Error>
where
    F: Fn(f64) -> Result<f64, Error>,
{
    let epsilon = 1e-7;
    let delta = 5e-6;
    let mut x = 0.0;
    let mut x0 = init_value;

    for _i in 0..1000 {
        x = x0;
        let fx = f(x)?;

        // 导数
        let fx_delta = f(x + delta)?;

        let fpx = (fx_delta - fx) / delta;
        x0 = x - fx / fpx;
        if (x0 - x).abs() <= epsilon {
            break;
        }
    }
    if (x0 - x).abs() <= epsilon {
        Ok(x)
    } else {
        Err(Error::Function(
            "1000次迭代，求解失败，调整初值重试".to_string(),
        ))
    }
}

/**
 * 计算两个度数之间小于等于180的夹角值
 * @param d0
 * 第一个度数
 * @param d1
 * 第二个度数
 */
pub fn included_angle(d0: f64, d1: f64) -> f64 {
    let x0 = swe_degnorm(d0 - d1);
    let x1 = swe_degnorm(d1 - d0);
    if x0 <= x1 {
        x0
    } else {
        x1
    }
}

#[cfg(test)]
mod tests {
    use super::newton_iteration;
    use crate::{
        horo_date_time::HoroDateTime,
        utils::{calc_eps, included_angle, mod180},
    };
    use std::env;
    use swe::{swe_calc_ut, swe_close, swe_set_ephe_path, Body};

    // 计算黄赤倾角
    #[test]
    fn test_calc_eps_test() {
        dotenv::dotenv().ok();
        let ephe_path = env::var("ephe_path")
            .expect("没设置 ephe_path 环境变量，可在.env文件中设置或export ephe_path=...");

        let t = HoroDateTime::new(2021, 9, 13, 19, 30, 0, 8.0);
        assert!(t.is_ok());
        let t = t.unwrap();
        swe_set_ephe_path(&ephe_path);
        let ret = swe_calc_ut(t.jd_utc, &Body::SeEclNut, &[]);
        swe_close();

        assert!(ret.is_ok(), "计算黄赤倾角错误！");
        let expected = ret.unwrap()[0];

        let actual = calc_eps(t.jd_utc, &ephe_path);
        assert!(actual.is_ok());
        let actual = actual.unwrap();

        assert!(((expected - actual) * 3600.0).abs() < 1.0);
    }

    #[test]
    fn test_mod180() {
        let expected = 186.1 - 360.0;
        let d = 186.1;
        let actual = mod180(d);
        // 精度为1秒
        assert!(
            ((expected - actual) * 3600.0).abs() < 1.0,
            "mod180({})={}, 而非{}",
            d,
            expected,
            actual
        );

        let expected = -186.1 + 360.0;
        let d = -186.1;
        let actual = mod180(d);
        assert!(
            ((expected - actual) * 3600.0).abs() < 1.0,
            "mod180({})={}, 而非{}",
            d,
            expected,
            actual
        );

        let expected = 186.0 - 360.0;
        let d = 186.0 + 360.0 * 2.0;
        let actual = mod180(d);
        assert!(
            ((expected - actual) * 3600.0).abs() < 1.0,
            "mod180({})={}, 而非{}",
            d,
            expected,
            actual
        );

        let expected = -186.0 + 360.0;
        let d = -186.0 - 360.0 * 2.0;
        let actual = mod180(d);
        assert!(
            ((expected - actual) * 3600.0).abs() < 1.0,
            "mod180({})={}, 而非{}",
            d,
            expected,
            actual
        );
    }

    // 测试牛顿迭代
    #[test]
    fn test_newton_iteration() {
        // fx=x^2+x-1 在[0,1]上的根
        let fx = |x| Ok(x * x + x - 1.0);

        let y = newton_iteration(0.0, fx);
        assert!(y.is_ok());

        let x = (-1.0 + 5.0_f64.sqrt()) / 2.0;
        let epsilon = 1e-7;
        let d = (x - y.unwrap()).abs();
        assert!(
            d < epsilon,
            "fx=x^2+x-1 在[0,1]上的根求解失败, 误差{}>={}",
            d,
            epsilon
        );
    }

    // 两个度数间的夹角
    #[test]
    fn test_included_angle() {
        let d0 = 1.0;
        let d1 = 5.0;

        let d = included_angle(d0, d1);
        assert_eq!(d, 4.0);

        let d = included_angle(d1, d0);
        assert_eq!(d, 4.0);
    }
}
