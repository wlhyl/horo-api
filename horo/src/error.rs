use std::fmt::Display;

#[derive(Debug)]
pub enum Error {
    InvalidDateTime(String),
    InvalidZone(String),
    Function(String),
    // 无效的小限时间
    InvalidProfectionDateTime(String),
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

impl From<qizheng::Error> for Error {
    fn from(value: qizheng::Error) -> Self {
        match value {
            qizheng::Error::Function(s) => Error::Function(s),
            qizheng::Error::InvalidProcessDateTime(s) => Error::InvalidProfectionDateTime(s),
            qizheng::Error::InvalidDateTime(s) => Error::InvalidDateTime(s),
            qizheng::Error::InvalidZone(s) => Error::InvalidZone(s),
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Error::Function(s) => s,
            Error::InvalidProfectionDateTime(s) => s,
            Error::InvalidDateTime(s) => s,
            Error::InvalidZone(s) => s,
        };
        write!(f, "{}", s)
    }
}
