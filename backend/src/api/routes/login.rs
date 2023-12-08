use actix_web::{HttpResponse, Responder, ResponseError, web};
use actix_web::body::BoxBody;
use actix_web::http::StatusCode;
use actix_web::web::Json;
use thiserror::Error;
use crate::api::contracts;
use crate::api::contracts::{LoginUserRequest, LoginUserResponse};
use crate::helpers;
use crate::helpers::validate_request;
use crate::service::{AuthService, UserService};
use crate::service::users::AuthError;

pub async fn login(user_service: web::Data<UserService>, auth_service: web::Data<AuthService>, request: Json<LoginUserRequest>) -> Result<impl Responder, LoginError> {
    let req = request.into_inner();
    validate_request(&req)?;

    let token = user_service.login(auth_service.into_inner(), req)?;

    Ok(HttpResponse::Ok().json(contracts::Response::ok(LoginUserResponse {
        token
    })))
}

#[derive(Error, Debug)]
pub enum LoginError {
    #[error("authentication failed")]
    AuthError(#[from] AuthError),
    #[error("request validation failed")]
    Validation(#[from] helpers::ValidationError),
    #[error("internal server error")]
    Internal(#[from] anyhow::Error),
}

impl LoginError {
    fn get_errors(&self) -> Vec<contracts::Error> {
        match self {
            LoginError::Validation(e) => e.get_validation_errors(),
            LoginError::AuthError(e) => vec![contracts::Error { message: e.to_string() }],
            LoginError::Internal(_) => vec![contracts::Error { message: "Internal server error".to_string() }],
        }
    }
}

impl ResponseError for LoginError {
    fn status_code(&self) -> StatusCode {
        match self {
            LoginError::Validation(_) => StatusCode::BAD_REQUEST,
            LoginError::AuthError(_) => StatusCode::UNAUTHORIZED,
            LoginError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR
        }
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        HttpResponse::build(self.status_code()).json(contracts::Response::<()>::err(self.get_errors()))
    }
}