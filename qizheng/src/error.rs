use std::fmt::Display;

#[derive(Debug)]
pub enum Error {
    InvalidDateTime(String),
    InvalidZone(String),
    Function(String),
    // 无效的推运时间
    InvalidProcessDateTime(String),
}

impl From<horo_date_time::Error> for Error {
    fn from(value: horo_date_time::Error) -> Self {
        match value {
            horo_date_time::Error::InvalidDateTime(s) => Self::InvalidDateTime(s),
            horo_date_time::Error::InvalidZone(s) => Self::InvalidZone(s),
            horo_date_time::Error::Function(s) => Self::Function(s),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Error::Function(s) => s,
            Error::InvalidProcessDateTime(s) => s,
            Error::InvalidDateTime(s) => s,
            Error::InvalidZone(s) => s,
        };
        write!(f, "{}", s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use horo_date_time::Error as HoroDateTimeError;

    #[test]
    fn test_from_horo_date_time_error() {
        let err = HoroDateTimeError::InvalidDateTime("invalid date".to_string());
        let converted_err: Error = err.into();
        assert!(matches!(converted_err, Error::InvalidDateTime(_)));

        let err = HoroDateTimeError::InvalidZone("invalid zone".to_string());
        let converted_err: Error = err.into();
        assert!(matches!(converted_err, Error::InvalidZone(_)));

        let err = HoroDateTimeError::Function("function error".to_string());
        let converted_err: Error = err.into();
        assert!(matches!(converted_err, Error::Function(_)));
    }

    #[test]
    fn test_display_error() {
        let err = Error::Function("test function".to_string());
        assert_eq!(format!("{}", err), "test function");

        let err = Error::InvalidProcessDateTime("invalid process time".to_string());
        assert_eq!(format!("{}", err), "invalid process time");

        let err = Error::InvalidDateTime("invalid date time".to_string());
        assert_eq!(format!("{}", err), "invalid date time");

        let err = Error::InvalidZone("invalid zone".to_string());
        assert_eq!(format!("{}", err), "invalid zone");
    }
}
