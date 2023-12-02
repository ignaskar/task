use actix_web::{HttpResponse, Responder, ResponseError, web};
use actix_web::body::BoxBody;
use actix_web::http::StatusCode;
use actix_web::web::Json;
use thiserror::Error;
use crate::api::contracts;
use crate::api::contracts::LoginUserRequest;
use crate::service::Service;
use crate::service::users::AuthError;

pub async fn login(service: web::Data<Service>, request: Json<LoginUserRequest>) -> Result<impl Responder, LoginError> {
    service.login(request.into_inner())?;
    Ok(HttpResponse::Ok())
}

#[derive(Error, Debug)]
pub enum LoginError {
    #[error("authentication failed")]
    AuthError(#[from] AuthError),
    #[error("internal server error")]
    Internal(#[from] anyhow::Error),
}

impl LoginError {
    fn get_errors(&self) -> Vec<contracts::Error> {
        match self {
            LoginError::AuthError(e) => vec![contracts::Error { message: e.to_string() }],
            LoginError::Internal(_) => vec![contracts::Error { message: "Internal server error".to_string() }],
        }
    }
}

impl ResponseError for LoginError {
    fn status_code(&self) -> StatusCode {
        match self {
            LoginError::AuthError(_) => StatusCode::UNAUTHORIZED,
            LoginError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR
        }
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        HttpResponse::build(self.status_code()).json(contracts::Response::<()>::err(self.get_errors()))
    }
}