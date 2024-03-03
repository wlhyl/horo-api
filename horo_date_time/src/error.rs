use std::fmt::Display;

#[derive(Debug)]
pub enum Error {
    InvalidDateTime(String),
    InvalidZone(String),
    Function(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = match self {
            Error::InvalidDateTime(s) => s,
            Error::InvalidZone(s) => s,
            Error::Function(s) => s,
        };
        write!(f, "{}", s)
    }
}
