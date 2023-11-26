use diesel::prelude::*;
use uuid::Uuid;
use crate::api::contracts;

#[derive(Queryable, Selectable, Insertable)]
#[diesel(table_name = crate::schema::users)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct User {
    pub id: Uuid,
    pub name: String,
    pub email: String,
    pub password_hash: Vec<u8>,
}

impl From<User> for contracts::User {
    fn from(value: User) -> Self {
        Self {
            id: value.id,
            name: value.name,
            email: value.email
        }
    }
}