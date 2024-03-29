use crate::repository::Repository;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::PgConnection;

pub mod auth;
pub mod users;

#[derive(Debug, Clone)]
pub struct UserService {
    db_pool: Pool<ConnectionManager<PgConnection>>,
    repo: Repository,
}

#[derive(Debug, Clone)]
pub struct AuthService {
    options: AuthOptions,
}

#[derive(Debug, Clone)]
pub struct AuthOptions {
    pub encoding_key: String,
    pub audience: String,
    pub token_expiration_in_seconds: u64,
}

impl AuthService {
    pub fn new(options: AuthOptions) -> Self {
        Self { options }
    }
}

impl UserService {
    pub fn new(db_pool: Pool<ConnectionManager<PgConnection>>) -> Self {
        Self {
            db_pool,
            repo: Repository {},
        }
    }
}
