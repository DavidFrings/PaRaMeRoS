use actix_web::{HttpResponse, ResponseError, http::StatusCode};
use derive_more::Display;

#[derive(Debug, Display)]
#[display("{}", message)]
pub struct HttpError {
    message: String,
    status: StatusCode,
}

impl HttpError {
    pub fn new(msg: impl Into<String>, status: StatusCode) -> Self {
        Self {
            message: msg.into(),
            status,
        }
    }
}

impl ResponseError for HttpError {
    fn status_code(&self) -> StatusCode {
        self.status
    }

    fn error_response(&self) -> HttpResponse {
        HttpResponse::build(self.status).body(self.message.clone())
    }
}

pub fn _bad_gateway(msg: impl Into<String>) -> HttpError {
    HttpError::new(msg, StatusCode::BAD_GATEWAY)
}

pub fn internal_error(msg: impl Into<String>) -> HttpError {
    HttpError::new(msg, StatusCode::INTERNAL_SERVER_ERROR)
}

pub fn unauthorized(msg: impl Into<String>) -> HttpError {
    HttpError::new(msg, StatusCode::UNAUTHORIZED)
}