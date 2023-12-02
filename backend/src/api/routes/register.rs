use actix_web::{HttpResponse, Responder, ResponseError, web};
use actix_web::body::BoxBody;
use actix_web::http::StatusCode;
use actix_web::web::Json;
use thiserror::Error;
use crate::api::contracts;
use crate::api::contracts::{RegisterUserRequest, RegisterUserResponse};
use crate::helpers;
use crate::helpers::validate_request;
use crate::service::Service;

pub async fn register(service: web::Data<Service>, request: Json<RegisterUserRequest>) -> Result<impl Responder, RegistrationError> {
    let inner = request.into_inner();
    validate_request(&inner)?;

    let user = service.register(inner)?;
    let response = contracts::Response::ok(RegisterUserResponse { user: user.into() });

    Ok(HttpResponse::Ok().json(response))
}

#[derive(Error, Debug)]
pub enum RegistrationError {
    #[error("request validation failed")]
    Validation(#[from] helpers::ValidationError),
    #[error(transparent)]
    Internal(#[from] anyhow::Error),
}

impl RegistrationError {
    fn get_errors(&self) -> Vec<contracts::Error> {
        match self {
            RegistrationError::Validation(err) => {
                err.get_validation_errors()
            }
            _ => vec![contracts::Error { message: "Internal server error".to_string() }]
        }
    }
}

impl ResponseError for RegistrationError {
    fn status_code(&self) -> StatusCode {
        match self {
            RegistrationError::Validation(_) => StatusCode::BAD_REQUEST,
            RegistrationError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR
        }
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        HttpResponse::build(self.status_code()).json(contracts::Response::<()>::err(self.get_errors()))
    }
}