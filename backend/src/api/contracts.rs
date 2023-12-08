use serde::{Deserialize, Serialize};
use uuid::Uuid;
use validator::Validate;

#[derive(Debug, Serialize, Deserialize)]
pub struct Response<T: Serialize> {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<Error>>,
}

impl<T: Serialize> Response<T> {
    pub fn ok(data: T) -> Response<T> {
        Self {
            data: Some(data),
            errors: None,
        }
    }

    pub fn err(errors: Vec<Error>) -> Response<()> {
        Response::<()> {
            data: None,
            errors: Some(errors),
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Error {
    pub message: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetUserResponse {
    pub user: User,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct RegisterUserRequest {
    #[validate(length(min = 1, message = "name must be at least 1 character long"))]
    pub name: String,
    #[validate(email(message = "email must be in a valid format"))]
    pub email: String,
    #[validate(length(min = 8, message = "password must be at least 8 characters long"))]
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct RegisterUserResponse {
    pub user: User,
}

#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct LoginUserRequest {
    #[validate(email(message = "email must be in a valid format"))]
    pub email: String,
    #[validate(length(min = 1, message = "please enter a password"))]
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginUserResponse {
    pub token: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetUsersResponse {
    pub users: Vec<User>,
}