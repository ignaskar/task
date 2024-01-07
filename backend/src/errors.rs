use crate::api::contracts;
use crate::helpers;
use actix_web::body::BoxBody;
use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use thiserror::Error;

#[derive(Error, Debug)]
pub enum ServerError {
    #[error(transparent)]
    Error(#[from] Error),
}

#[derive(Error, Debug)]
pub enum Error {
    #[error("provided email already exists")]
    EmailAlreadyExists,
    #[error("supplied credentials are invalid")]
    InvalidCredentials,
    #[error("request validation failed")]
    Validation(#[from] helpers::ValidationError),
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

impl ResponseError for ServerError {
    fn status_code(&self) -> StatusCode {
        match self {
            ServerError::Error(error) => match error {
                Error::EmailAlreadyExists => StatusCode::CONFLICT,
                Error::InvalidCredentials => StatusCode::UNAUTHORIZED,
                Error::Validation(_) => StatusCode::BAD_REQUEST,
                Error::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
            },
        }
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        HttpResponse::build(self.status_code()).json(build_error_response(self))
    }
}

fn build_error_response(err: &ServerError) -> contracts::Response<()> {
    let errors = match err {
        ServerError::Error(error) => match error {
            Error::EmailAlreadyExists => vec![contracts::Error {
                message: "email already exists".to_string(),
            }],
            Error::InvalidCredentials => vec![contracts::Error {
                message: "invalid credentials supplied".to_string(),
            }],
            Error::Validation(err) => err.get_validation_errors(),
            Error::Internal(_) => vec![contracts::Error {
                message: "something went wrong".to_string(),
            }],
        },
    };

    contracts::Response::<()>::err(errors)
}
