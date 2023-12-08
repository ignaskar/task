use actix_web::body::MessageBody;
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::error::InternalError;
use actix_web::{HttpResponse, web};
use actix_web_lab::middleware::Next;
use crate::api::contracts;
use crate::service::AuthService;

static HEADER_NAME: &str = "Authorization";
static AUTHORIZATION_SCHEME: &str = "Bearer";

pub async fn requires_authentication(
    req: ServiceRequest,
    next: Next<impl MessageBody>
) -> Result<ServiceResponse<impl MessageBody>, actix_web::Error> {
    let maybe_auth_service = req.app_data::<web::Data<AuthService>>();
    let service = match maybe_auth_service {
        None => return Err(internal_server_error(String::from("something went wrong"))),
        Some(s) => s
    };

    let (request, _) = req.parts();

    if let Some(header_value) = request.headers().get(HEADER_NAME) {
        let authorization_header = match header_value.to_str() {
            Ok(v) => v,
            Err(_) => return Err(unauthorized(String::from("header contains invisible ascii characters")))
        };

        let parts: Vec<&str> = authorization_header.split(' ').collect();
        if parts.len() != 2 {
            return Err(unauthorized(String::from("invalid bearer authentication format")))
        }

        let scheme = parts[0];
        let token = parts[1];

        if scheme != AUTHORIZATION_SCHEME {
            return Err(unauthorized(String::from("invalid authorization scheme")))
        }

        match service.verify_token(token.to_string()) {
            Ok(_) => next.call(req).await,
            Err(e) => Err(unauthorized(e.to_string()))
        }
    } else {
        Err(unauthorized(String::from("missing authorization header")))
    }
}

fn unauthorized(error_message: String) -> actix_web::Error {
    let res = HttpResponse::Unauthorized().json(contracts::Response::<()>::err(vec![
        contracts::Error {
            message: error_message.to_string()
        }
    ]));
    let e = anyhow::anyhow!(error_message.to_string());
    InternalError::from_response(e, res).into()
}

fn internal_server_error(error_message: String) -> actix_web::Error {
    let res = HttpResponse::InternalServerError().json(contracts::Response::<()>::err(vec![
        contracts::Error {
            message: error_message.to_string()
        }
    ]));
    let e = anyhow::anyhow!(error_message.to_string());
    InternalError::from_response(e, res).into()
}