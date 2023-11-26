use actix_web::HttpResponse;
use actix_web::web::Json;
use crate::api::contracts::{GetUserResponse, RegisterUserRequest, Response, User};

pub async fn register(request: Json<RegisterUserRequest>) -> HttpResponse {
    let inner = request.into_inner();
    let output = GetUserResponse {
        user: User {
            id: inner.id,
            email: inner.email,
            name: inner.name
        }
    };

    let res = Response::ok(output);
    HttpResponse::Ok().json(res)
}