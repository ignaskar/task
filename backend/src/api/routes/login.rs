use crate::api::contracts;
use crate::api::contracts::{LoginUserRequest, LoginUserResponse};
use crate::errors::ServerError;
use crate::helpers::validate_request;
use crate::service::{AuthService, UserService};
use actix_web::web::Json;
use actix_web::{web, HttpResponse, Responder};

pub async fn login(
    user_service: web::Data<UserService>,
    auth_service: web::Data<AuthService>,
    request: Json<LoginUserRequest>,
) -> Result<impl Responder, ServerError> {
    let request = request.into_inner();
    validate_request(&request)?;

    let token = user_service.login(auth_service.into_inner(), request)?;

    Ok(HttpResponse::Ok().json(contracts::Response::ok(LoginUserResponse { token })))
}
