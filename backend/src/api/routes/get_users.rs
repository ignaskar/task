use crate::api::contracts;
use crate::api::contracts::GetUsersResponse;
use crate::errors::ServerError;
use crate::service::UserService;
use actix_web::{web, HttpResponse, Responder};

pub async fn get_users(service: web::Data<UserService>) -> Result<impl Responder, ServerError> {
    let users = service.get_users()?;

    let response = contracts::Response::ok(GetUsersResponse {
        users: users.iter().map(Into::into).collect(),
    });

    Ok(HttpResponse::Ok().json(response))
}
