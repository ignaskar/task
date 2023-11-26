use actix_web::{HttpResponse, Responder};
use actix_web::web::Json;
use crate::api::contracts::{GetUserResponse, RegisterUserRequest, Response, User};
use crate::errors::Error;
use crate::helpers::validate_request;

pub async fn register(request: Json<RegisterUserRequest>) -> Result<impl Responder, Error> {
    let inner = request.into_inner();
    validate_request(&inner)?;

    let output = GetUserResponse {
        user: User {
            id: inner.id,
            email: inner.email,
            name: inner.name
        }
    };

    let res = Response::ok(output);
    Ok(HttpResponse::Ok().json(res))
}