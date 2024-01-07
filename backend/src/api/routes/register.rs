use crate::api::contracts;
use crate::api::contracts::{RegisterUserRequest, RegisterUserResponse};
use crate::errors::ServerError;
use crate::helpers::validate_request;
use crate::service::{AuthService, UserService};
use actix_web::web::Json;
use actix_web::{web, HttpResponse, Responder};

pub async fn register(
    user_service: web::Data<UserService>,
    auth_service: web::Data<AuthService>,
    request: Json<RegisterUserRequest>,
) -> Result<impl Responder, ServerError> {
    let request = request.into_inner();
    validate_request(&request)?;

    let user = user_service.register(auth_service.into_inner(), request)?;
    let response = contracts::Response::ok(RegisterUserResponse { user: user.into() });

    Ok(HttpResponse::Ok().json(response))
}
