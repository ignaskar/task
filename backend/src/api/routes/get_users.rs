use actix_web::{HttpResponse, Responder, web};
use crate::api::contracts;
use crate::api::contracts::GetUsersResponse;
use crate::service::UserService;

pub async fn get_users(service: web::Data<UserService>) -> impl Responder {
    let Ok(users) = service.get_users() else {
        return HttpResponse::InternalServerError().json(contracts::Response::<()>::err(vec![contracts::Error {
            message: "Internal server error".to_string()
            }]));
        };

    let response = contracts::Response::ok(GetUsersResponse {
        users: users.iter().map(Into::into).collect()
    });

    HttpResponse::Ok().json(response)
}
