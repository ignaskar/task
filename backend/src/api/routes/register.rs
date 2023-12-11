use actix_web::{HttpResponse, Responder, ResponseError, web};
use actix_web::body::BoxBody;
use actix_web::http::StatusCode;
use actix_web::web::Json;
use thiserror::Error;
use crate::api::contracts;
use crate::api::contracts::{RegisterUserRequest, RegisterUserResponse};
use crate::helpers;
use crate::helpers::validate_request;
use crate::service::{AuthService, UserService};
use crate::service::users::ServiceError;

pub async fn register(user_service: web::Data<UserService>, auth_service: web::Data<AuthService>, request: Json<RegisterUserRequest>) -> Result<impl Responder, RegistrationError> {
    let request = request.into_inner();
    validate_request(&request)?;

    let user = user_service.register(auth_service.into_inner(), request)?;
    let response = contracts::Response::ok(RegisterUserResponse { user: user.into() });

    Ok(HttpResponse::Ok().json(response))
}

#[derive(Error, Debug)]
pub enum RegistrationError {
    #[error("request validation failed")]
    Validation(#[from] helpers::ValidationError),
    #[error(transparent)]
    Service(#[from] ServiceError)
}

impl RegistrationError {
    fn get_errors(&self) -> Vec<contracts::Error> {
        match self {
            RegistrationError::Validation(err) => err.get_validation_errors(),
            RegistrationError::Service(ServiceError::EmailAlreadyExists) => vec![contracts::Error { message: "email already exists".to_string() }],
            RegistrationError::Service(ServiceError::Internal(_)) => vec![contracts::Error { message: "internal server error".to_string() }]
        }
    }
}

impl ResponseError for RegistrationError {
    fn status_code(&self) -> StatusCode {
        match self {
            RegistrationError::Validation(_) => StatusCode::BAD_REQUEST,
            RegistrationError::Service(ServiceError::EmailAlreadyExists) => StatusCode::CONFLICT,
            RegistrationError::Service(ServiceError::Internal(_)) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        HttpResponse::build(self.status_code()).json(contracts::Response::<()>::err(self.get_errors()))
    }
}