use std::fmt::{Display, Formatter};
use actix_web::http::StatusCode;
use actix_web::{HttpResponse, ResponseError};
use actix_web::body::BoxBody;
use validator::ValidationErrors;
use crate::api::contracts;

#[derive(Debug)]
pub enum Error {
    Validation(ValidationErrors),
    Unauthorized,
    Internal(String)
}

impl Error {
    fn get_errors(&self) -> Vec<contracts::Error> {
        match self {
            Error::Validation(e) => {
                e.field_errors().values().flat_map(|errs| {
                        errs
                            .iter()
                            .map(|err| {
                                let message = match err.clone().message {
                                    None => format!("{}", err),
                                    Some(msg) => msg.to_string()
                                };

                                contracts::Error {
                                    message
                                }
                            })
                    })
                    .collect()
            }
            Error::Unauthorized => {
                vec![contracts::Error {
                    message: String::from("unauthorized")
                }]
            }
            Error::Internal(err) => {
                vec![contracts::Error {
                    message: err.to_string()
                }]
            }
        }
    }
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", self)
    }
}

impl ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        match self {
            Error::Unauthorized => StatusCode::UNAUTHORIZED,
            Error::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Error::Validation(_) => StatusCode::BAD_REQUEST
        }
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        HttpResponse::build(self.status_code()).json(contracts::Response::<()>::err(self.get_errors()))
    }
}