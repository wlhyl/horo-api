use std::{collections::HashMap, fmt};

use actix_web::{http::StatusCode, HttpResponse, ResponseError};
use horo::Error as HoroError;

#[derive(Debug)]
pub enum Error {
    BadRequest(String),
    InternalServerError(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Error::BadRequest(s) => s,
            Error::InternalServerError(s) => s,
        };
        write!(f, "{}", s)
    }
}

impl ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        match self {
            Error::BadRequest(_) => StatusCode::BAD_REQUEST,
            Error::InternalServerError(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        let mut result = HashMap::new();
        result.insert("error", format!("{}", self));
        HttpResponse::build(self.status_code()).json(result)
    }
}

impl From<HoroError> for Error {
    fn from(value: HoroError) -> Self {
        match value {
            HoroError::DateTime(e) => Error::BadRequest(e.to_string()),
            HoroError::Function(s) => Error::InternalServerError(s),
            HoroError::InvalidGeoPosition(s) => Error::BadRequest(s),
            HoroError::InvalidProfectionDateTime(s) => Error::BadRequest(s),
        }
    }
}
