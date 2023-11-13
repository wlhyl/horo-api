use std::fmt::Display;

#[derive(Debug)]
pub enum DateTimeError {
    InvalidDateTime(String),
    InvalidZone(String),
}

impl Display for DateTimeError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            DateTimeError::InvalidDateTime(s) => s,
            DateTimeError::InvalidZone(s) => s,
        };
        write!(f, "{}", s)
    }
}

#[derive(Debug)]
pub enum Error {
    DateTime(DateTimeError),
    Function(String),
    InvalidGeoPosition(String),
    // 无效的小限时间
    InvalidProfectionDateTime(String),
}

impl From<DateTimeError> for Error {
    fn from(value: DateTimeError) -> Self {
        Self::DateTime(value)
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Error::DateTime(e) => match e {
                DateTimeError::InvalidDateTime(s) => s,
                DateTimeError::InvalidZone(s) => s,
            },
            Error::Function(s) => s,
            Error::InvalidGeoPosition(s) => s,
            Error::InvalidProfectionDateTime(s) => s,
        };
        write!(f, "{}", s)
    }
}
