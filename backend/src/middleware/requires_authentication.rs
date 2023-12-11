use crate::api::contracts;
use crate::service::AuthService;
use actix_service::{Service, Transform};
use actix_web::body::{EitherBody, MessageBody};
use actix_web::dev::{ServiceRequest, ServiceResponse};
use actix_web::error::InternalError;
use actix_web::http::header::HeaderMap;
use actix_web::{web, Error, HttpResponse};
use anyhow::anyhow;
use std::future::Future;
use std::pin::Pin;
use std::rc::Rc;
use std::task::{Context, Poll};

static HEADER_NAME: &str = "Authorization";
static AUTHORIZATION_SCHEME: &str = "Bearer";

pub struct AuthenticationMiddleware<S> {
    service: Rc<S>,
}

impl<S, B> Service<ServiceRequest> for AuthenticationMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: MessageBody + 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Response, Self::Error>>>>;

    fn poll_ready(&self, ctx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        self.service.poll_ready(ctx)
    }

    fn call(&self, req: ServiceRequest) -> Self::Future {
        let service = Rc::clone(&self.service);
        Box::pin(async move {
            let auth_service = match req.app_data::<web::Data<AuthService>>() {
                None => {
                    return Ok(req
                        .error_response::<actix_web::Error>(internal_server_error(
                            "internal server error".to_string(),
                        ))
                        .map_into_right_body());
                }
                Some(s) => s,
            };

            let token = match parse_auth_token_from_header(req.headers()) {
                Ok(t) => t,
                Err(e) => {
                    return Ok(req
                        .error_response::<actix_web::Error>(unauthorized(e.to_string()))
                        .map_into_right_body())
                }
            };

            match auth_service.verify_token(token) {
                Ok(_) => service.call(req).await.map(ServiceResponse::map_into_left_body),
                Err(e) => Ok(req
                    .error_response::<actix_web::Error>(unauthorized(e.to_string()))
                    .map_into_right_body()),
            }
        })
    }
}

pub struct RequiresAuthentication;

impl<S, B> Transform<S, ServiceRequest> for RequiresAuthentication
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    S::Future: 'static,
    B: MessageBody + 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Transform = AuthenticationMiddleware<S>;
    type InitError = ();
    type Future = futures::future::Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        futures::future::ok(AuthenticationMiddleware {
            service: Rc::new(service),
        })
    }
}

fn parse_auth_token_from_header(headers: &HeaderMap) -> Result<String, anyhow::Error> {
    let auth_header = headers
        .get(HEADER_NAME)
        .ok_or_else(|| anyhow!("authorization header is not present"))?;
    let auth_header = auth_header.to_str()?;
    if let Some(token) = auth_header.strip_prefix(AUTHORIZATION_SCHEME) {
        Ok(token.trim().to_string())
    } else {
        Err(anyhow!(
            "authorization header value does not start with 'Bearer'"
        ))
    }
}

fn unauthorized(error_message: String) -> actix_web::Error {
    let res =
        HttpResponse::Unauthorized().json(contracts::Response::<()>::err(vec![contracts::Error {
            message: error_message.to_string(),
        }]));
    let e = anyhow!(error_message.to_string());
    InternalError::from_response(e, res).into()
}

fn internal_server_error(error_message: String) -> actix_web::Error {
    let res = HttpResponse::InternalServerError().json(contracts::Response::<()>::err(vec![
        contracts::Error {
            message: error_message.to_string(),
        },
    ]));
    let e = anyhow!(error_message.to_string());
    InternalError::from_response(e, res).into()
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::service::AuthOptions;
    use actix_web::body::BoxBody;
    use actix_web::http::StatusCode;
    use actix_web::test::TestRequest;
    use actix_web::{http, test, App};
    use uuid::Uuid;

    #[actix_web::test]
    async fn requires_authentication_should_return_internal_server_error_when_unable_to_resolve_auth_service(
    ) {
        // Arrange

        let app = test::init_service(App::new().wrap(RequiresAuthentication)).await;

        // Act

        let actual = TestRequest::get().send_request(&app).await;

        // Assert

        assert_eq!(actual.status(), StatusCode::INTERNAL_SERVER_ERROR);
        deserialize_body_and_match_error(actual, "internal server error").await;
    }

    #[actix_web::test]
    async fn requires_authentication_should_return_unauthorized_when_no_authorization_header_present(
    ) {
        // Arrange

        let app = test::init_service(
            App::new()
                .wrap(RequiresAuthentication)
                .app_data(web::Data::new(get_test_auth_service().clone())),
        )
        .await;

        // Act

        let actual = TestRequest::get().send_request(&app).await;

        // Assert

        assert_eq!(actual.status(), StatusCode::UNAUTHORIZED);
        deserialize_body_and_match_error(actual, "authorization header is not present").await;
    }

    #[actix_web::test]
    async fn requires_authentication_should_return_unauthorized_when_authorization_header_does_not_start_with_bearer(
    ) {
        // Arrange

        let app = test::init_service(
            App::new()
                .wrap(RequiresAuthentication)
                .app_data(web::Data::new(get_test_auth_service().clone())),
        )
        .await;

        // Act

        let actual = TestRequest::get()
            .append_header((http::header::AUTHORIZATION, "123"))
            .send_request(&app)
            .await;

        // Assert

        assert_eq!(actual.status(), StatusCode::UNAUTHORIZED);
        deserialize_body_and_match_error(
            actual,
            "authorization header value does not start with 'Bearer'",
        )
        .await;
    }

    #[actix_web::test]
    async fn requires_authentication_should_return_unauthorized_when_token_verification_failed() {
        // Arrange

        let app = test::init_service(
            App::new()
                .wrap(RequiresAuthentication)
                .app_data(web::Data::new(get_test_auth_service().clone())),
        )
        .await;

        // Act

        let actual = TestRequest::get()
            .append_header((http::header::AUTHORIZATION, "Bearer but-incorrect-token"))
            .send_request(&app)
            .await;

        // Assert

        assert_eq!(actual.status(), StatusCode::UNAUTHORIZED);
        deserialize_body_and_match_error(actual, "InvalidToken").await;
    }

    #[actix_web::test]
    async fn requires_authentication_should_succeed_when_valid_token_provided() {
        // Arrange

        let auth_service = get_test_auth_service();
        let token = match auth_service.generate_token(Uuid::new_v4()) {
            Ok(t) => t,
            Err(_) => panic!(),
        };

        let app = test::init_service(
            App::new()
                .wrap(RequiresAuthentication)
                .route("/", web::get().to(HttpResponse::Ok))
                .app_data(web::Data::new(auth_service.clone())),
        )
        .await;

        // Act

        let actual = TestRequest::get()
            .append_header((http::header::AUTHORIZATION, format!("Bearer {}", token)))
            .send_request(&app)
            .await;

        // Assert

        assert_eq!(actual.status(), StatusCode::OK);
    }

    fn get_test_auth_service() -> AuthService {
        AuthService::new(AuthOptions {
            encoding_key: "test".to_string(),
            audience: "test".to_string(),
            token_expiration_in_seconds: 100,
        })
    }

    async fn deserialize_body_and_match_error(
        response: ServiceResponse<EitherBody<BoxBody>>,
        message: &str,
    ) {
        let body: contracts::Response<()> = test::read_body_json(response).await;

        assert_eq!(body.errors.unwrap().get(0).unwrap().message, message);
    }
}
