use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize)]
pub struct Response<T: Serialize> {
    pub data: Option<T>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<Error>>
}

impl<T: Serialize> Response<T> {
    pub fn ok(data: T) -> Response<T> {
        Self {
            data: Some(data),
            errors: None
        }
    }

    pub fn err(errors: Vec<Error>) -> Response<()> {
        Response::<()> {
            data: None,
            errors: Some(errors)
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Error {
    pub message: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetUserResponse {
    pub user: User
}

#[derive(Debug, Serialize, Deserialize)]
pub struct RegisterUserRequest {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub password: String
}

#[derive(Debug, Serialize, Deserialize)]
// TODO [IK]: implement once auth mechanism is decided
pub struct RegisterUserResponse{}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginUserRequest{
    pub email: String,
    pub password: String
}

#[derive(Debug, Serialize, Deserialize)]
// TODO [IK]: implement once auth mechanism is decided
pub struct LoginUserResponse{}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetUsersResponse {
    pub users: Vec<User>
}