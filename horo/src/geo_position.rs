use crate::error::Error;

#[cfg(feature = "serde")]
use serde::Serialize;

#[cfg(feature = "swagger")]
use utoipa::ToSchema;

#[derive(Clone, Debug)]
#[cfg_attr(feature = "serde", derive(Serialize))]
#[cfg_attr(feature = "swagger", derive(ToSchema))]
pub struct GeoPosition {
    /// 地理经度
    pub long: f64,
    /// 地理纬度
    pub lat: f64,
}

impl GeoPosition {
    pub fn new(long: f64, lat: f64) -> Result<Self, Error> {
        if long < -180.0 || long > 180.0 {
            return Err(Error::InvalidGeoPosition(
                "long must between -180 and 180. ".to_owned(),
            ));
        }
        if lat < -90.0 || lat > 90.0 {
            return Err(Error::InvalidGeoPosition(
                "lat must between -90 and 90. ".to_owned(),
            ));
        }
        Ok(Self { long, lat })
    }
}

#[cfg(test)]
mod test {
    use crate::geo_position::GeoPosition;

    // 测试正确的地理经纬度
    #[test]
    fn test_new_with_correct_geo_position() {
        let correct_geo_position = [(-10.1, -10.1), (10.1, -10.1), (-10.1, 10.1), (10.1, 10.1)];

        for geo in correct_geo_position {
            let g = GeoPosition::new(geo.0, geo.1);
            assert!(g.is_ok());
            let g = g.unwrap();
            assert_eq!(geo.0, g.long, "地理经度");
            assert_eq!(geo.1, g.lat, "地理纬度");
        }
    }

    // 非法大地经纬度
    #[test]
    fn test_new_invalid_geo_position() {
        let invalid_geo_position = [
            (-210.1, -10.1),
            (210.1, -10.1),
            (-10.1, 210.1),
            (10.1, 210.1),
        ];

        for geo in invalid_geo_position {
            assert!(GeoPosition::new(geo.0, geo.1).is_err())
        }
    }
}
