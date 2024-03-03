use std::fmt::Display;

#[derive(Debug)]
pub enum Error {
    InvalidGeoPosition(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Error::InvalidGeoPosition(s) => s,
        };
        write!(f, "{}", s)
    }
}
