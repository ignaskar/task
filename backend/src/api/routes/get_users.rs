use actix_web::{HttpResponse, Responder, ResponseError, web};
use actix_web::body::BoxBody;
use actix_web::http::StatusCode;
use thiserror::Error;
use crate::api::contracts;
use crate::api::contracts::GetUsersResponse;
use crate::service::Service;

pub async fn get_users(service: web::Data<Service>) -> Result<impl Responder, GetUsersError> {
    let users = service.get_users()?;
    let response = contracts::Response::ok(GetUsersResponse {
        users: users.iter().map(|u| u.into()).collect()
    });

    Ok(HttpResponse::Ok().json(response))
}

#[derive(Error, Debug)]
pub enum GetUsersError {
    #[error(transparent)]
    Internal(#[from] anyhow::Error)
}

impl ResponseError for GetUsersError {
    fn status_code(&self) -> StatusCode {
        match self {
            GetUsersError::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR
        }
    }

    fn error_response(&self) -> HttpResponse<BoxBody> {
        HttpResponse::build(self.status_code()).json(contracts::Response::<()>::err(vec![
            contracts::Error {
                message: "Internal server error".to_string()
            }
        ]))
    }
}