use actix_web::{HttpResponse, Responder, web};
use crate::api::contracts;
use crate::api::contracts::GetUsersResponse;
use crate::service::Service;

pub async fn get_users(service: web::Data<Service>) -> impl Responder {
    let get_users_result = service.get_users();

    let users = match get_users_result {
        Ok(users) => users,
        Err(_) => {
            return HttpResponse::InternalServerError().json(contracts::Response::<()>::err(vec![contracts::Error{
                message: "Internal server error".to_string()
            }]))
        }
    };

    let response = contracts::Response::ok(GetUsersResponse {
        users: users.iter().map(|u| u.into()).collect()
    });

    HttpResponse::Ok().json(response)
}